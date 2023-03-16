use std::error::Error;

mod server;

fn echo(x: String) -> String {
    x
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = server::WebsocketServer::new("10.22.13.226".to_string(), 1312, echo, false);
    server.start().await?;
    Ok(())
}
