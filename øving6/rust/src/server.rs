use std::cmp::Ordering;

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

impl WebsocketServer {
    pub fn new(address: String, port: u16, response_fn: fn(String) -> String, quiet: bool) -> Self {
        Self {
            address,
            port,
            response_fn,
            quiet,
        }
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
                let mut requestbuf = [0; usize::pow(2, 16)]; // real big input buffer for requests

                let n = match socket.read(&mut requestbuf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Problem ved mottak av handshake: {e:?}");
                        return;
                    }
                };

                let message = String::from_utf8_lossy(&requestbuf[0..n - 1]);
                let message_lower = message.to_lowercase();

                if !message_lower.contains("upgrade: websocket")
                    || !message_lower.contains("sec-websocket-key: ")
                {
                    socket.write(b"HTTP/1.0 400\n").await.unwrap();
                    eprintln!("Mottok ingen key, lukker koblingen");
                    return;
                };

                let key = message
                    .lines()
                    .filter(|line| line.to_lowercase().starts_with("sec-websocket-key: "))
                    .take(1)
                    .collect::<Vec<&str>>()[0]
                    .split(":")
                    .collect::<Vec<&str>>()[1]
                    .trim()
                    .to_string()
                    + "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

                let response_key = sha1::Sha1::default().digest(key.as_bytes()).to_bytes();
                let response_key = general_purpose::STANDARD.encode(response_key);
                println!("{response_key}");

                let handshake_response = format!("HTTP/1.1 101 Switching Protocols\nUpgrade: websocket\nConnection: Upgrade\nSec-WebSocket-Accept: {response_key}\n\n");
                match socket
                    .write_all(handshake_response.as_bytes())
                    .await {
                        Ok(_) => println!("Sendte handshake"),
                        Err(_) => println!("Kunne ikke sende handshake"),
                    };

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
                    let message = &requestbuf[0..n - 1];

                    // Godtar bare meldinger som inneholder tekst, så første byte må være 0x81
                    if message[0].cmp(&0x81) != Ordering::Equal {
                        eprintln!("Mottok melding som ikke er tekst!");
                        eprintln!("Meldingen var: {message:?}");
                        continue;
                    }

                    // Deler opp resten av headeren
                    let _message_len = message[1] & 0x7f;
                    let mask = &message[2..6];
                    let masked_data = &message[6..];

                    let mut data: Vec<u8> = Vec::new();

                    for (idx, byte) in masked_data.iter().enumerate() {
                        data.push(byte ^ mask[idx % 4]);
                    }

                    let message = String::from_utf8_lossy(&data);

                    if !self.quiet {
                        println!("Melding fra klient: {message}");
                    }

                    // Behandler meldingen og genererer svar på en eller annen måte
                    let response_msg = (self.response_fn)(message.into_owned());
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
