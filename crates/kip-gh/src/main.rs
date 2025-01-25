use std::collections::HashMap;

use anyhow::Result;
use clap::Parser;
use kip_storage::model::Item;
use octocrab::Octocrab;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// GitHub personal access token
    #[arg(short, long)]
    token: String,
    /// Whether to fetch PRs or Issues
    #[arg(value_enum)]
    kind: Kind,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Kind {
    PrCreated,
    PrOpened,
    IssuesAssigned,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let client = GhClient::new(&args.token)?;

    let query = match args.kind {
        Kind::PrCreated => "is:pr is:open author:@me",
        Kind::PrOpened => "author:@me is:pr state:open",
        Kind::IssuesAssigned => "is:issue is:open assignee:@me",
    };

    let menu = client.get_issues_or_pr(query).await?;
    serde_json::to_writer(std::io::stdout(), &menu)?;

    Ok(())
}

pub struct GhClient {
    client: Octocrab,
}

impl GhClient {
    pub fn new(token: &str) -> Result<GhClient> {
        let octocrab = Octocrab::builder().personal_token(token).build()?;
        Ok(GhClient { client: octocrab })
    }
}

impl GhClient {
    async fn get_issues_or_pr(&self, query: &str) -> Result<Vec<Item>, octocrab::Error> {
        let page = self
            .client
            .search()
            .issues_and_pull_requests(query)
            .sort("newest")
            .order("asc")
            .per_page(10)
            .send()
            .await?;

        Ok(page
            .items
            .into_iter()
            .map(|item| Item {
                title: item.title,
                url: item.html_url.to_string(),
                custom: HashMap::new(),
            })
            .collect())
    }
}
