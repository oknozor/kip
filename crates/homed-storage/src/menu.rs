use std::{borrow::Cow, future::Future};

use serde::{Deserialize, Serialize};

use crate::Entity;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct CalendarMenu {
    pub sections: Vec<Section>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct GitHubIssueMenu {
    pub sections: Vec<Section>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct GitHubPrMenu {
    pub sections: Vec<Section>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Section {
    pub title: String,
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Item {
    pub title: String,
}

pub trait MenuSource {
    fn get_menu(&self) -> impl Future<Output = GitHubIssueMenu>;
}

impl Entity<'static> for GitHubIssueMenu {
    fn get_key(&self) -> Cow<'static, str> {
        Cow::Borrowed("github_issues")
    }
}

impl Entity<'static> for GitHubPrMenu {
    fn get_key(&self) -> Cow<'static, str> {
        Cow::Borrowed("github_prs")
    }
}

impl Entity<'static> for CalendarMenu {
    fn get_key(&self) -> Cow<'static, str> {
        Cow::Borrowed("calendar")
    }
}
