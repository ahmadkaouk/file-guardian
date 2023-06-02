use anyhow::Result;

mod server;
mod store;

#[tokio::main]
async fn main() -> Result<()> {
    // Read the addr from the command line
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:2345".to_string());

    let tcp_server = server::Server::new(&addr);
    tcp_server.run().await?;
    Ok(())
}
