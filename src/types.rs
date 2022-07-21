//!
//! Event payloads are found at the bottom of this page: <https://umami.is/docs/api>
//!
//! There are 2 kinds, one for pageviews and one for events.
//!
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Pageview {
    pub payload: PageviewPayload,
    pub r#type: &'static str,
}

#[derive(Serialize, Debug)]
pub struct PageviewPayload {
    pub website: String,
    pub url: String,
    pub referrer: String,
    pub hostname: String,
    pub language: String,
    pub screen: String,
}

#[derive(Serialize, Debug)]
pub struct Event {
    pub payload: EventPayload,
    pub r#type: &'static str,
}

#[derive(Serialize, Debug)]
pub struct EventPayload {
    pub website: String,
    pub url: String,
    pub event_type: String,
    pub event_value: String,
    pub hostname: String,
    pub language: String,
    pub screen: String,
}
