use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} get <key>", args[0]);
        return Ok(());
    }

    let socket_path = dirs::runtime_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("kipd.sock");

    let mut stream = UnixStream::connect(socket_path).await?;

    // Join all arguments with spaces to create the command
    let command = args[1..].join(" ");

    stream.write_all(command.as_bytes()).await?;
    stream.shutdown().await?;

    let mut response = String::new();
    stream.read_to_string(&mut response).await?;
    println!("{}", response);

    Ok(())
}
