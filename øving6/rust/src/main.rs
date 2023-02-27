use std::error::Error;

mod server;

fn echo(x: String) -> String {
    x
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = server::WebsocketServer::new("127.0.0.1".to_string(), 1312, echo, false);
    server.start().await?;
    Ok(())
}
