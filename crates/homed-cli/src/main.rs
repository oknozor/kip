use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket_path = dirs::runtime_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("homedd.sock");

    let mut stream = UnixStream::connect(socket_path).await?;

    // Send command
    stream.write_all(b"get_issues").await?;
    stream.shutdown().await?;

    // Read response
    let mut response = String::new();
    stream.read_to_string(&mut response).await?;
    println!("Response: {}", response);

    Ok(())
}
