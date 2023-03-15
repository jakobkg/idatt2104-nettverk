use base64::{engine::general_purpose, Engine as _};
use sha::sha1;
use sha::utils::{Digest, DigestExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub struct WebsocketServer {
    address: String,
    port: u16,
    response_fn: fn(String) -> String,
    quiet: bool,
}

pub struct WebsocketMsg<'a> {
    bytes: &'a [u8],
}

impl<'a> WebsocketMsg<'a> {
    /// Returns `Some(WebsocketMsg)` if the `&[u8]` has length 6 or longer,
    /// or `None` otherwise.
    /// 
    /// TODO: Validation
    pub fn from_bytes(bytes: &'a [u8]) -> Option<Self> {
        // Per spec, the header is at least 6 bytes so a message is expected to be more than 6 bytes long
        if bytes.len() > 6 {
            Some(Self { bytes })
        } else {
            None
        }
    }

    pub fn opcode(&self) -> u8 {
        self.bytes[0] & 0x0F
    }

    pub fn is_text(&self) -> bool {
        self.opcode() == 0x01
    }

    pub fn len(&self) -> u64 {
        // Per spec, if the length is less than 126 it is contained entirely in the second header byte
        let masked_first_byte = self.bytes[1] & 0x7f;

        if masked_first_byte <= 125 {
            (self.bytes[1] & 0x7f) as u64
        } else if masked_first_byte == 126 {
            u16::from_ne_bytes(
                self.bytes[2..4]
                    .try_into()
                    .expect("Failed to parse message length"),
            ) as u64
        } else {
            u64::from_ne_bytes(
                self.bytes[2..10]
                    .try_into()
                    .expect("Failed to parse message length"),
            )
        }
    }

    fn mask_offset(&self) -> usize {
        // According to spec, the mask follows the payload length which can take 1, 3 or 9 bytes
        // As such, mask offset can be derived from the payload length
        let masked_first_byte = self.bytes[1] & 0x7f;
        let base = 2;

        if masked_first_byte <= 125 {
            base
        } else if masked_first_byte == 126 {
            base + 2
        } else {
            base + 8
        }
    }

    fn mask(&self) -> &[u8] {
        &self.bytes[self.mask_offset()..self.mask_offset() + 4]
    }

    fn payload_offset(&self) -> usize {
        self.mask_offset() + 4
    }

    pub fn text(&self) -> Option<String> {
        if !self.is_text() {
            return None;
        } else {
            let mut bytevec = vec![0u8; 0];

            for (idx, byte) in self.bytes
                [self.payload_offset()..(self.payload_offset() + self.len() as usize - 1)]
                .iter()
                .enumerate()
            {
                bytevec.push(byte ^ self.mask()[idx % 4]);
            }

            Some(String::from_utf8_lossy(&bytevec).to_string())
        }
    }
}

impl WebsocketServer {
    const MAGIC_GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

    pub fn new(address: String, port: u16, response_fn: fn(String) -> String, quiet: bool) -> Self {
        Self {
            address,
            port,
            response_fn,
            quiet,
        }
    }

    fn extract_key(header: String) -> String {
        header
            .lines()
            .filter(|line| line.to_lowercase().starts_with("sec-websocket-key: "))
            .take(1)
            .collect::<Vec<&str>>()[0]
            .split(":")
            .collect::<Vec<&str>>()[1]
            .trim()
            .to_string()
    }

    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        let fulladdress = format!("{}:{}", self.address, self.port);
        let listener = TcpListener::bind(&fulladdress).await?;

        if !self.quiet {
            println!("Server listening on port {}", self.port)
        }

        loop {
            let (mut socket, address) = listener.accept().await?;

            if !self.quiet {
                println!("Ny klient tilkoblet: {}", address)
            }

            tokio::spawn(async move {
                let mut requestbuf = [0; usize::pow(2, 16)]; // 64kB input buffer

                let n = match socket.read(&mut requestbuf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Problem ved mottak av handshake: {e:?}");
                        return;
                    }
                };

                let header = match String::from_utf8(requestbuf[0..n - 1].to_vec()) {
                    Ok(header) => header,
                    Err(_) => {
                        eprintln!("Mottatt melding kunne ikke konverteres til UTF8-string");
                        socket.write(b"HTTP/1.0 400 Bad Request\n\n").await.unwrap();
                        return;
                    }
                };
                let header_lower = header.to_lowercase();

                // Sjekk at meldingen som er mottatt inneholder forventede headere
                if !header_lower.contains("upgrade: websocket")
                    || !header_lower.contains("sec-websocket-key: ")
                {
                    // Svar med 400 Bad Request og lukk koblingen om ikke
                    eprintln!("Mottok ingen key, lukker koblingen");
                    socket.write(b"HTTP/1.0 400 Bad Request\n\n").await.unwrap();
                    return;
                };

                // Trengs ikke videre
                drop(header_lower);

                let key = WebsocketServer::extract_key(header.to_string());

                drop(header);

                // Nøkkelen som skal svares med er base64(sha1(key + MAGIC_GUID))
                let response_key = general_purpose::STANDARD.encode(
                    sha1::Sha1::default()
                        .digest((key + WebsocketServer::MAGIC_GUID).as_bytes())
                        .to_bytes(),
                );

                let handshake_response = format!("HTTP/1.1 101 Switching Protocols\nUpgrade: websocket\nConnection: Upgrade\nSec-WebSocket-Accept: {response_key}\n\n");

                match socket.write_all(handshake_response.as_bytes()).await {
                    Ok(_) => println!("Sendte handshake"),
                    Err(_) => println!("Kunne ikke sende handshake"),
                };
                drop(response_key);
                drop(handshake_response);

                // Handshake ferdig, anser websocket-kobling som etablert
                loop {
                    // Les forespørsel
                    let n = match socket.read(&mut requestbuf).await {
                        Ok(n) if n == 0 => return, // Return (lukk tråd) hvis forespørselen er tom
                        Ok(n) => n,                // Noter størrelsen på forespørselen
                        Err(e) => {
                            // Ved error, print og lukk tråden
                            eprintln!("Problem ved lesning fra socket: {e:?}");
                            return;
                        }
                    };

                    // Meldingen som er mottatt må demaskeres før den kan behandles, per RFC6455
                    let message = match WebsocketMsg::from_bytes(&requestbuf[0..n - 1]) {
                        Some(msg) => msg,
                        None => {
                            eprintln!("Kunne ikke lese melding fra bytes");
                            continue;
                        }
                    };

                    // Godtar bare meldinger som inneholder tekst
                    if !message.is_text() {
                        eprintln!(
                            "Mottok melding som ikke er kodet som tekst (første byte ikke 0x81)"
                        );
                        eprintln!("Meldingen var: {:?}", message.bytes);
                        continue;
                    }

                    if !self.quiet {
                        println!("Melding fra klient: {}", message.text().unwrap());
                    }

                    // Behandler meldingen og genererer svar på en eller annen måte
                    let response_msg = (self.response_fn)(message.text().unwrap());
                    let response_header = [0x81, response_msg.len() as u8];

                    let response = [&response_header, response_msg.as_bytes()].concat();

                    if let Err(e) = socket.write_all(&response).await {
                        eprintln!("Problem ved skrivning til socket: {e:?}");
                        return;
                    }
                }
            });
        }
    }
}
