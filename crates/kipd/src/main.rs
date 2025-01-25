use config::Settings;
use kip_storage::model::{diff_items, Item};
use kip_storage::DB;
use notify_rust::Notification;
use std::error::Error;
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tracing::{debug, error, info};

mod config;
mod socket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = Settings::new()?;
    tracing_subscriber::fmt::init();
    info!("Starting kipd...");

    let mut plugin_handles = vec![];
    let mut plugin_receivers = vec![];

    for (plugin_name, plugin) in settings.plugins {
        info!("Starting plugin: {}", plugin_name);
        let (handle, receiver) = plugin.run().await;
        plugin_handles.push((plugin_name.clone(), handle));
        plugin_receivers.push((plugin_name, receiver));
    }

    let socket_server = socket::SocketServer::new().await?;
    let plugin_handler = handle_plugin_output(plugin_receivers);

    select! {
        result = socket_server.handle_connections() => {
            if let Err(e) = result {
                error!("Socket server error: {}", e);
            }
        },
        _ = plugin_handler => {
            info!("Plugin handler completed");
        },
    };

    Ok(())
}

fn send_notification(item: Item) {
    if let Err(e) = Notification::new()
        .summary(&item.title)
        .body(&item.url)
        .timeout(5000)
        .show()
    {
        error!("Failed to send notification: {}", e);
    }
}

async fn handle_plugin_output(receivers: Vec<(String, Receiver<Vec<Item>>)>) {
    let mut handles = vec![];

    for (plugin_name, mut receiver) in receivers {
        let handle = tokio::spawn(async move {
            while let Some(new_data) = receiver.recv().await {
                let old_data = DB.get_by_key(&plugin_name);

                if let Some(old_data) = old_data {
                    if old_data == new_data {
                        debug!("No new data from plugin {}", plugin_name);
                        continue;
                    }

                    diff_items(&old_data, &new_data)
                        .into_iter()
                        .for_each(send_notification);
                };

                match DB.insert(&plugin_name, &new_data) {
                    Ok(_) => info!("Stored data from plugin {}", plugin_name),
                    Err(e) => error!("Failed to store data from plugin {}: {}", plugin_name, e),
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        if let Err(e) = handle.await {
            error!("Plugin task failed: {}", e);
        }
    }
}
