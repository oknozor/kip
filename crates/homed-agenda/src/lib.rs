#[cfg(test)]
mod tests {
    use anyhow::Result;
    use minicaldav::Credentials;
    use ureq::Agent;
    use url::Url;

    // TODO: auth
    #[tokio::test]
    async fn test_calendar() -> Result<()> {
        dotenv::dotenv().ok();
        let url = std::env::var("NEXTCLOUD_CALENDAR_URL").expect("NEXTCLOUD_CALENDAR_URL not set");
        let username = std::env::var("NEXTCLOUD_USERNAME").expect("NEXTCLOUD_USERNAME not set");
        let password = std::env::var("NEXTCLOUD_PASSWORD").expect("NEXTCLOUD_PASSWORD not set");
        let credentials = Credentials::Basic(username, password);

        let agent = Agent::new();
        let url = Url::parse(&url).unwrap();
        let calendars = minicaldav::get_calendars(agent.clone(), &credentials, &url).unwrap();
        for calendar in calendars {
            println!("{:?}", calendar);
            let (events, errors) =
                minicaldav::get_events(agent.clone(), &credentials, &calendar).unwrap();
            for event in events {
                println!("{:?}", event);
            }
            for error in errors {
                println!("Error: {:?}", error);
            }
        }

        Ok(())
    }
}
