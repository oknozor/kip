use homed_gh::GhClient;
use homed_storage::menu::{GitHubIssueMenu, GitHubPrMenu};
use homed_storage::{Database, DB};
use notify_rust::Notification;
use std::error::Error;
use tokio::select;
use tokio::time::{interval, Duration};
use tracing::{error, info};

const MENU_COLLECTION: &str = "github";
const FETCH_INTERVAL_SECS: u64 = 300;

mod socket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    info!("Starting homedd...");

    let gh_client = match GhClient::new() {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to initialize GitHub client: {}", e);
            return Err(e);
        }
    };

    info!("GitHub and Calendar clients initialized successfully");
    let mut interval = interval(Duration::from_secs(FETCH_INTERVAL_SECS));

    fetch_and_store_github_data(&gh_client, &DB).await;

    info!(
        "Entering main loop, fetching every {} seconds",
        FETCH_INTERVAL_SECS
    );
    let socket_server = socket::SocketServer::new().await?;

    select! {
        _ = async {
            loop {
                interval.tick().await;
                fetch_and_store_github_data(&gh_client, &DB).await;
            }
        } => {},
        result = socket_server.handle_connections() => {
            if let Err(e) = result {
                error!("Socket server error: {}", e);
            }
        }
    };

    Ok(())
}

fn send_notification(title: &str, body: &str) {
    if let Err(e) = Notification::new()
        .summary(title)
        .body(body)
        .timeout(5000)
        .show()
    {
        error!("Failed to send notification: {}", e);
    }
}

async fn fetch_and_store_github_data(client: &GhClient, db: &Database) {
    info!("Fetching GitHub data...");

    match client.get_issue_menu().await {
        Ok(new_issues) => {
            info!("Successfully fetched issues");
            let menu = db.get_by_key::<GitHubIssueMenu>(MENU_COLLECTION, "github_issues");
            match menu {
                Some(old_issues) => {
                    if old_issues != new_issues {
                        match db.insert(MENU_COLLECTION, &new_issues) {
                            Err(e) => {
                                error!("Failed to store issues: {}", e);
                            }
                            Ok(_) => {
                                info!("Stored updated issues");
                                send_notification(
                                    "GitHub Issues Updated",
                                    "There are changes in your GitHub issues",
                                );
                            }
                        }
                    } else {
                        info!("No changes in issues");
                    }
                }
                None => {
                    if let Err(e) = db.insert(MENU_COLLECTION, &new_issues) {
                        error!("Failed to store initial issues: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to fetch issues: {}", e);
        }
    }

    match client.get_pr_menu().await {
        Ok(new_prs) => {
            info!("Successfully fetched PRs");
            match db.get_by_key::<GitHubPrMenu>(MENU_COLLECTION, "github_prs") {
                Some(old_prs) => {
                    if old_prs != new_prs {
                        match db.insert(MENU_COLLECTION, &new_prs) {
                            Err(e) => {
                                error!("Failed to store PRs: {}", e);
                            }
                            Ok(_) => {
                                info!("Stored updated PRs");
                                send_notification(
                                    "GitHub PRs Updated",
                                    "There are changes in your GitHub pull requests",
                                );
                            }
                        }
                    } else {
                        info!("No changes in PRs");
                    }
                }
                None => {
                    if let Err(e) = db.insert(MENU_COLLECTION, &new_prs) {
                        error!("Failed to store initial PRs: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to fetch PRs: {}", e);
        }
    }
}
