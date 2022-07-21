#![doc(html_root_url = "https://docs.rs/umami_metrics/0.1.0")]

//!
//! A Rust library for sending metrics to [Umami](https://umami.io), a self hosted and privacy preserving metrics stack.
//!
//! Supports both pageview and event metrics.
//!
//! ## Basic Example:
//!
//! ```ignore
//! use umami_metrics::Umami;
//!
//! let umami = Umami::new(
//!     "website_id".to_string(),
//!     "https://umami_instance.com".to_string(),
//! );
//!
//! let status = umami.pageview(
//!     "/".to_string(),
//!     "click".to_string(),
//!     "eee".to_string(),
//!     "tets".to_string(),
//!     "asdfasdfasfd".to_string(),
//! ).await;
//!
//! let status = umami.event(
//!     "/".to_string(),
//!     "click".to_string(),
//!     "eee".to_string(),
//!     "tets".to_string(),
//!     "asdfasdfasfd".to_string(),
//!     "asdfasdf".to_string(),
//! ).await;
//!```
//!  
pub mod types;

use anyhow::Result;
use reqwest::StatusCode;

use crate::types::{Event, EventPayload, Pageview, PageviewPayload};

/// A struct representing an instance of a page tracked by Umami.
///
/// For the methods below, any will be accepted in my testing,
/// so feel free to use them for whatever you want. However,
/// using garbage in say the screen size field may result in
/// less useful tracking data, so consider what fields you use
/// to pack your assorted information.
pub struct Umami {
    website_id: String,
    url: String,
    client: reqwest::Client,
}

impl Umami {
    /// Create a new Umami instance.
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

    /// Track a pageview.
    pub async fn pageview(
        &self,
        url: String,
        referrer: String,
        hostname: String,
        language: String,
        screen: String,
    ) -> Result<StatusCode> {
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

        let pageview = serde_json::to_string(&pageview)?;

        let resp = self
            .client
            .post(&self.url)
            .body(pageview)
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

    /// Track an event
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
