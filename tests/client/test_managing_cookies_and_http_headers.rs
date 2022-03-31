use super::super::common;
use hydrus_api::api_core::endpoints::managing_cookies_and_http_headers::CookieBuilder;

#[tokio::test]
async fn it_returns_cookies_for_a_domain() {
    let client = common::get_client();
    client.get_cookies("trivernis.net").await.unwrap();
}

#[tokio::test]
async fn it_sets_cookies_for_a_domain() {
    let client = common::get_client();
    let cookie = CookieBuilder::default()
        .name("my_cookie")
        .value("my_value")
        .domain("trivernis.net")
        .path("/")
        .build();
    client.set_cookies(vec![cookie]).await.unwrap();
}

#[tokio::test]
async fn it_sets_the_user_agent() {
    let client = common::get_client();
    client
        .set_user_agent("Mozilla/5.0 (compatible; Hydrus Client)")
        .await
        .unwrap();
}
