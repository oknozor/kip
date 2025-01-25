use std::time::Duration;

use kip_storage::model::Item;
use serde::Deserialize;
use thiserror::Error;
use tokio::{
    process::Command,
    sync::{
        self,
        mpsc::{Receiver, Sender},
    },
    task::JoinHandle,
    time::interval,
};
use tracing::error;

#[derive(Debug, Deserialize, Clone)]
pub struct Plugin {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub interval: u64,
    pub timeout: u64,
    pub notify: bool,
}

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("Failed to execute command: {0}")]
    CommandError(#[from] tokio::io::Error),
}

impl Plugin {
    pub async fn run(&self) -> (JoinHandle<Result<(), PluginError>>, Receiver<Vec<Item>>) {
        let plugin = self.clone();
        let (tx, rx) = sync::mpsc::channel(100);
        let handle = tokio::spawn(plugin.task(tx));
        (handle, rx)
    }

    async fn task(self, tx: Sender<Vec<Item>>) -> Result<(), PluginError> {
        let mut interval = interval(Duration::from_secs(self.interval));

        loop {
            interval.tick().await;

            let output = match tokio::time::timeout(
                Duration::from_secs(self.timeout),
                Command::new(&self.command).args(&self.args).output(),
            )
            .await
            {
                Ok(result) => result?,
                Err(_) => {
                    error!("{} plugin timed out after", self.name);
                    continue;
                }
            };

            let out = String::from_utf8_lossy(&output.stdout).to_string();
            let items = serde_json::from_str(&out)
                .expect(&format!("Failed to parse {} plugin output", self.name));
            if let Err(e) = tx.send(items).await {
                error!("Failed to send output: {}", e);
            }
        }
    }
}
