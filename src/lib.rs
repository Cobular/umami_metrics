pub mod types;

#[cfg(test)]
mod tests {
    use crate::types::Umami;

    #[tokio::test]
    async fn send_event() {
        let umami = Umami::new(
            "fdcdf862-ac50-4785-a98f-af48d9ed2868".to_string(),
            "https://umami.cobular.com".to_string(),
        );

        let res = umami.event(
            "/".to_string(),
            "click".to_string(),
            "eee".to_string(),
            "tets".to_string(),
            "asdfasdfasfd".to_string(),
            "asdfasdf".to_string(),
        ).await;

        assert!(res.is_ok());
    }
    

    #[tokio::test]
    async fn send_pageview() {
        let umami = Umami::new(
            "fdcdf862-ac50-4785-a98f-af48d9ed2868".to_string(),
            "https://umami.cobular.com".to_string(),
        );

        let res = umami.pageview(
            "/".to_string(),
            "click".to_string(),
            "eee".to_string(),
            "tets".to_string(),
            "asdfasdfasfd".to_string(),
        ).await;

        assert!(res.is_ok());
    }
}
