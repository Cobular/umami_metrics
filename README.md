# Umami Metric for Rust

![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![CI](https://github.com/Cobular/umami_metrics/actions/workflows/main.yml/badge.svg)](https://github.com/Cobular/umami_metrics/actions)
[![crates-io](https://img.shields.io/crates/v/umami_metrics.svg)](https://crates.io/crates/umami_metrics)
[![api-docs](https://docs.rs/umami_metrics/badge.svg)](https://docs.rs/umami_metrics)

A Rust library for sending metrics events to an instance of https://umami.is/

## Basic Example

```rust
let umami = Umami::new(
    "website_id".to_string(),
    "https://umami_instance.com".to_string(),
);

let res = umami.pageview(
    "/".to_string(),
    "click".to_string(),
    "eee".to_string(),
    "tets".to_string(),
    "asdfasdfasfd".to_string(),
).await;

let res = umami.event(
    "/".to_string(),
    "click".to_string(),
    "eee".to_string(),
    "tets".to_string(),
    "asdfasdfasfd".to_string(),
    "asdfasdf".to_string(),
).await;
```
