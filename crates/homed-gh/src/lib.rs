use std::error::Error;

use homed_storage::menu::{GitHubIssueMenu, GitHubPrMenu, Item, Section};
use octocrab::Octocrab;

pub struct GhClient {
    client: Octocrab,
}

impl GhClient {
    pub fn new(token: &str) -> Result<GhClient, Box<dyn Error>> {
        let octocrab = Octocrab::builder().personal_token(token).build()?;

        Ok(GhClient { client: octocrab })
    }
}

impl GhClient {
    pub async fn get_pr_menu(&self) -> Result<GitHubPrMenu, Box<dyn Error>> {
        let assigned = self
            .get_issues_or_pr("assignee:@me is:opened", "Assigned")
            .await?;
        let created = self
            .get_issues_or_pr("author:@me is:pr state:open", "Open")
            .await?;
        let mentioned = self
            .get_issues_or_pr("mention:@me is:issue", "Mentioned")
            .await?;

        let menu = GitHubPrMenu {
            sections: vec![assigned, created, mentioned],
        };

        Ok(menu)
    }

    pub async fn get_issue_menu(&self) -> Result<GitHubIssueMenu, Box<dyn Error>> {
        let assigned = self
            .get_issues_or_pr("assignee:oknozor is:pr is:open", "assigned")
            .await?;
        let created = self
            .get_issues_or_pr(" review-requested:@me is:pr is:open", "review requested")
            .await?;
        let mentioned = self
            .get_issues_or_pr("is:pr is:open commenter:@me", "assigned")
            .await?;

        let menu = GitHubIssueMenu {
            sections: vec![assigned, created, mentioned],
        };

        Ok(menu)
    }

    async fn get_issues_or_pr(
        &self,
        query: &str,
        section_title: &str,
    ) -> Result<Section, octocrab::Error> {
        let page = self
            .client
            .search()
            .issues_and_pull_requests(query)
            .sort("newest")
            .order("asc")
            .per_page(10)
            .send()
            .await?;

        let section = Section {
            title: section_title.to_string(),
            items: page
                .items
                .into_iter()
                .map(|item| Item { title: item.title })
                .collect(),
        };

        Ok(section)
    }
}

#[cfg(test)]
mod tests {
    use crate::GhClient;

    #[tokio::test]
    async fn test_get_issues() {
        dotenv::dotenv().ok();
        let token =
            std::env::var("HOMEDD_GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
        let client = GhClient::new(&token).unwrap();
        let menu = client.get_issue_menu().await.unwrap();
        println!("{menu:?}");
    }

    #[tokio::test]
    async fn test_get_pr() {
        dotenv::dotenv().ok();
        let token =
            std::env::var("HOMEDD_GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
        let client = GhClient::new(&token).unwrap();
        let menu = client.get_issue_menu().await.unwrap();
        println!("{menu:#?}");
    }
}
