#[cfg(test)]
mod tests {
    use anyhow::Result;
    use chrono::{Local, NaiveDateTime, NaiveTime, TimeZone};
    use minicaldav::Credentials;
    use ureq::Agent;
    use url::Url;

    // TODO: auth
    #[tokio::test]
    async fn test_calendar() -> Result<()> {
        dotenv::dotenv().ok();
        let url =
            std::env::var("HOMEDD_NEXTCLOUD_CALENDAR_URL").expect("NEXTCLOUD_CALENDAR_URL not set");
        let username =
            std::env::var("HOMEDD_NEXTCLOUD_USERNAME").expect("NEXTCLOUD_USERNAME not set");
        let password =
            std::env::var("HOMEDD_NEXTCLOUD_PASSWORD").expect("NEXTCLOUD_PASSWORD not set");
        let credentials = Credentials::Basic(username, password);

        let agent = Agent::new();
        let url = Url::parse(&url).unwrap();
        let calendars = minicaldav::get_calendars(agent.clone(), &credentials, &url).unwrap();

        for calendar in calendars {
            let (events, errors) =
                minicaldav::get_events(agent.clone(), &credentials, &calendar).unwrap();
            for event in events {
                // Find the DTSTART property in the VEVENT
                if let Some(vevent) = event.ical().children.iter().find(|c| c.name == "VEVENT") {
                    let start_date = vevent
                        .properties
                        .iter()
                        .find(|p| p.name == "DTSTART")
                        .and_then(|dt| {
                            NaiveDateTime::parse_from_str(&dt.value, "%Y%m%dT%H%M%S").ok()
                        });

                    let summary = vevent
                        .properties
                        .iter()
                        .find(|p| p.name == "SUMMARY")
                        .map(|s| &s.value);

                    if let (Some(dt), Some(summary)) = (start_date, summary) {
                        println!("Event: {} - {}", summary, dt.format("%Y-%m-%d %H:%M:%S"));
                    }
                }
            }
        }

        Ok(())
    }
}
