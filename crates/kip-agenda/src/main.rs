use anyhow::Result;
use chrono::NaiveDateTime;
use clap::Parser;
use kip_storage::model::Item;
use minicaldav::Credentials;
use tracing::error;
use ureq::Agent;
use url::Url;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    url: String,
    #[arg(long)]
    username: String,
    #[arg(long)]
    password: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let credentials = Credentials::Basic(cli.username, cli.password);
    let agent = Agent::new();
    let url = Url::parse(&cli.url).unwrap();
    let caldav_url = url.join("remote.php/dav").unwrap();
    let calendars = minicaldav::get_calendars(agent.clone(), &credentials, &caldav_url).unwrap();
    let mut items = vec![];
    let mut events_sorted = vec![];

    for calendar in calendars {
        let (events, errors) =
            minicaldav::get_events(agent.clone(), &credentials, &calendar).unwrap();

        for error in errors {
            error!("Error: {}", error);
        }

        for event in &events {
            let ical = event.ical();
            if let Some(vevent) = ical.children.iter().find(|c| c.name == "VEVENT") {
                let start_date = vevent
                    .properties
                    .iter()
                    .find(|p| p.name == "DTSTART")
                    .and_then(|dt| NaiveDateTime::parse_from_str(&dt.value, "%Y%m%dT%H%M%S").ok());

                if let Some(start_date) = start_date {
                    events_sorted.push((start_date, vevent.clone()));
                }
            }
        }
    }

    events_sorted.sort_by(|a, b| b.0.cmp(&a.0));
    let events = events_sorted.into_iter().take(10).collect::<Vec<_>>();

    for (start_date, event) in events {
        let formatted_date = start_date.format("%Y-%m-%d").to_string();
        let web_path = format!("apps/calendar/dayGridMonth/{formatted_date}");
        let web_url = url.join(&web_path).unwrap();

        let Some(summary) = event
            .properties
            .iter()
            .find(|p| p.name == "SUMMARY")
            .map(|s| &s.value)
        else {
            continue;
        };

        let item = Item {
            title: summary.to_owned(),
            url: web_url.to_string(),
            custom: event
                .properties
                .iter()
                .filter(|p| p.name != "SUMMARY")
                .map(|p| {
                    let value = if ["CREATED", "DTEND", "DTSTART", "DTSTAMP", "LAST-MODIFIED"]
                        .contains(&p.name.as_str())
                    {
                        match NaiveDateTime::parse_from_str(&p.value, "%Y%m%dT%H%M%S") {
                            Ok(dt) => {
                                serde_json::Value::String(dt.format("%Y-%m-%d %H:%M").to_string())
                            }
                            Err(e1) => {
                                match NaiveDateTime::parse_from_str(&p.value, "%Y%m%dT%H%M%SZ") {
                                    Ok(dt) => serde_json::Value::String(
                                        dt.format("%Y-%m-%d %H:%M").to_string(),
                                    ),
                                    Err(e2) => {
                                        error!("Failed to parse '{}': {} or {}", p.value, e1, e2);
                                        serde_json::Value::String(p.value.clone())
                                    }
                                }
                            }
                        }
                    } else {
                        serde_json::Value::String(p.value.clone())
                    };
                    (p.name.to_lowercase(), value)
                })
                .collect(),
        };

        items.push(item);
    }

    serde_json::to_writer(std::io::stdout(), &items)?;
    Ok(())
}
