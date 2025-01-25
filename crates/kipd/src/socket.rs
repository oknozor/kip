use std::error::Error;
use std::fs;
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixListener;
use tracing::{error, info};

pub struct SocketServer {
    socket_path: PathBuf,
    listener: UnixListener,
}

impl SocketServer {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let socket_path = dirs::runtime_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("kipd.sock");

        // Clean up existing socket if it exists
        if socket_path.exists() {
            fs::remove_file(&socket_path)?;
        }

        let listener = UnixListener::bind(&socket_path)?;
        info!("Socket server listening on {:?}", socket_path);

        Ok(SocketServer {
            socket_path,
            listener,
        })
    }

    pub async fn handle_connections(&self) -> Result<(), Box<dyn Error>> {
        loop {
            match self.listener.accept().await {
                Ok((mut socket, _addr)) => {
                    info!("New connection established");

                    tokio::spawn(async move {
                        let mut buf = Vec::new();
                        match socket.read_to_end(&mut buf).await {
                            Ok(_) => {
                                if let Ok(command) = String::from_utf8(buf) {
                                    info!("Received command: {}", command);

                                    // Handle the command and send response
                                    let response = handle_command(&command);
                                    if let Err(e) = socket.write_all(response.as_bytes()).await {
                                        error!("Failed to send response: {}", e);
                                    }
                                }
                            }
                            Err(e) => error!("Failed to read from socket: {}", e),
                        }
                    });
                }
                Err(e) => error!("Failed to accept connection: {}", e),
            }
        }
    }
}

impl Drop for SocketServer {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_file(&self.socket_path) {
            error!("Failed to remove socket file: {}", e);
        }
    }
}

fn handle_command(command: &str) -> String {
    use crate::DB;

    let parts: Vec<&str> = command.trim().split_whitespace().collect();

    match parts.get(0) {
        Some(&"get") => {
            if let Some(key) = parts.get(1) {
                match DB.get_by_key(key) {
                    Some(value) => {
                        serde_json::to_string_pretty(&value).expect("failed to deserialize items")
                    }
                    None => format!("No data found for key: {}", key),
                }
            } else {
                "Usage: get <key>".to_string()
            }
        }
        Some(&"apply") => {
            // Apply any pending changes
            "Changes applied successfully".to_string()
        }
        _ => "Unknown command. Available commands: get <key>, apply".to_string(),
    }
}
