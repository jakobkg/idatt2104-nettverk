use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub struct Server {
    address: String,
    port: u16,
    response_fn: fn(String, &mut [u8]),
    quiet: bool,
}

impl Server {
    pub fn new(
        address: String,
        port: u16,
        response_fn: fn(String, &mut [u8]) -> (),
        quiet: bool,
    ) -> Self {
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
            let (mut socket, _) = listener.accept().await?;

            if !self.quiet {
                println!("Ny klient tilkoblet: {}", socket.local_addr().unwrap())
            }

            tokio::spawn(async move {
                let mut requestbuf = [0; 1024]; // 1kB input buffer for requests
                let mut responsebuf: [u8; 1024]; // 1kB output buffer for responses

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

                    // Behandler alt som tekst, så konverterer innholdet i bufferen til en String
                    let message = String::from_utf8_lossy(&requestbuf[0..n - 1]);

                    if !self.quiet {
                        println!("Melding fra klient: {message}");
                    }

                    // Tømmer respons-bufferen før behandling
                    responsebuf = [0; 1024];

                    // Behandler meldingen og genererer svar på en eller annen måte
                    (self.response_fn)(message.into_owned(), &mut responsebuf);

                    if let Err(e) = socket.write_all(&responsebuf).await {
                        eprintln!("Problem ved skrivning til socket: {e:?}");
                        return;
                    }
                }
            });
        }
    }
}
