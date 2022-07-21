//!
//! Event payloads are found at the bottom of this page: https://umami.is/docs/api
//!
//! There are 2 kinds, one for pageviews and one for events.
//!
use anyhow::Result;
use reqwest::StatusCode;
use serde::Serialize;

pub struct Umami {
    website_id: String,
    url: String,
    client: reqwest::Client,
}

impl Umami {
    pub fn new(website_id: String, mut base_url: String) -> Umami {
        if !(base_url.starts_with("http://") || base_url.starts_with("https://")) {
            base_url = format!("https://{}", base_url);
        }

        if !base_url.ends_with('/') {
            base_url = format!("{}/", base_url);
        }

        base_url = format!("{}api/collect", base_url);

        println!("{}", base_url);

        Umami {
            website_id,
            url: base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn pageview(
        &self,
        url: String,
        referrer: String,
        hostname: String,
        language: String,
        screen: String,
    ) -> Result<()> {
        let pageview = Pageview {
            r#type: "pageview",
            payload: PageviewPayload {
                website: self.website_id.clone(),
                url,
                referrer,
                hostname,
                language,
                screen,
            },
        };

        println!("{:#?}", pageview);

        let pageview = serde_json::to_string(&pageview)?;

        println!("{}", pageview);

        let body = self
            .client
            .post(&self.url)
            .body(pageview)
            .header("User-Agent", "umami-rust/0.1.0")
            .send()
            .await?
            .text()
            .await?;

        Ok(())
    }

    pub async fn event(
        &self,
        url: String,
        event_type: String,
        event_value: String,
        hostname: String,
        language: String,
        screen: String,
    ) -> Result<StatusCode> {
        let event = Event {
            r#type: "event",
            payload: EventPayload {
                website: self.website_id.clone(),
                url,
                event_type,
                event_value,
                hostname,
                language,
                screen,
            },
        };

        let event = serde_json::to_string(&event)?;

        let resp = self
            .client
            .post(&self.url)
            .body(event)
            .header("User-Agent", "umami-rust/0.1.0")
            .header("Content-Type", "text/plain")
            .send()
            .await?;

        let status = resp.status();
        let text = resp.text().await?;

        if status.is_success() {
            Ok(status)
        } else {
            Err(anyhow::anyhow!("{}, {}", status, text))
        }
    }
}

#[derive(Serialize, Debug)]
struct Pageview {
    payload: PageviewPayload,
    r#type: &'static str,
}

#[derive(Serialize, Debug)]
struct PageviewPayload {
    website: String,
    url: String,
    referrer: String,
    hostname: String,
    language: String,
    screen: String,
}

#[derive(Serialize, Debug)]
struct Event {
    payload: EventPayload,
    r#type: &'static str,
}

#[derive(Serialize, Debug)]
struct EventPayload {
    website: String,
    url: String,
    event_type: String,
    event_value: String,
    hostname: String,
    language: String,
    screen: String,
}
