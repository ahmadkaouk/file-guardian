use anyhow::Result;

mod server;
mod store;

#[tokio::main]
async fn main() -> Result<()> {
    let tcp_server = server::Server::new("127.0.0.1:2345");
    tcp_server.run().await?;
    Ok(())
}

