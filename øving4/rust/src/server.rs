use tokio::net::UdpSocket;

pub struct UDPServer {
    address: String,
    port: u16,
    response_fn: fn(String, &mut [u8]),
    quiet: bool,
}

impl UDPServer {
    pub fn new(
        address: String,
        port: u16,
        response_fn: fn(String, &mut [u8]),
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
        let socket = UdpSocket::bind(fulladdress).await?;
        let mut request_buf = [0;1024];
        let mut response_buf: [u8; 1024];

        if !self.quiet {
            println!("Server lytter på port {}", self.port)
        }

        loop {
            let (len, address) = socket.recv_from(&mut request_buf).await?;

            let request = String::from_utf8_lossy(&request_buf[..len-1]);
            response_buf = [0;1024];

            if !self.quiet {
                println!("Mottok {len} bytes fra {address}: {request}");
            }

            (self.response_fn)(request.to_string(), &mut response_buf);

            if !self.quiet {
                println!("Svarer med {}", String::from_utf8_lossy(&response_buf))
            }

            socket.send_to(&response_buf, address).await?;
        }
    }
}
