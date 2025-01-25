use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Item {
    pub title: String,
    pub url: String,
    #[serde(flatten)]
    pub custom: HashMap<String, serde_json::Value>,
}

pub fn diff_items(old: &[Item], new: &[Item]) -> Vec<Item> {
    let mut diff = Vec::new();
    for item in new {
        if !old.contains(&item) {
            diff.push(item.clone());
        }
    }
    diff
}

#[cfg(test)]
mod test {
    use crate::model::Item;

    #[test]
    fn should_deserilize_items() {
        let items = r#"[{"title":"Verre avec Guillaume Locquet","url":"https://nextcloud.hoohoot.org/apps/calendar/dayGridMonth/2025-01-23","dtend":"2025-01-23 21:00","dtstamp":"2025-01-22 22:36","sequence":"2","status":"CONFIRMED","uid":"e89aa958-53ab-471a-9c44-192a8add9244","created":"2025-01-22 22:33","last-modified":"2025-01-22 22:36","dtstart":"2025-01-23 20:00"}]"#;
        let items: Vec<Item> = serde_json::from_str(items).unwrap();
        assert_eq!(items.len(), 1);
    }
}
