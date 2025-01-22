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
            .join("homedd.sock");

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
    use homed_storage::menu::{GitHubIssueMenu, GitHubPrMenu};

    match command.trim() {
        "get_issues" => match DB.get_by_key::<GitHubIssueMenu>("github", "github_issues") {
            Some(issues) => serde_json::to_string(&issues)
                .unwrap_or_else(|_| "Error serializing issues".to_string()),
            None => "No issues found".to_string(),
        },
        "get_prs" => match DB.get_by_key::<GitHubPrMenu>("github", "github_prs") {
            Some(prs) => {
                serde_json::to_string(&prs).unwrap_or_else(|_| "Error serializing PRs".to_string())
            }
            None => "No PRs found".to_string(),
        },
        _ => "Unknown command".to_string(),
    }
}
