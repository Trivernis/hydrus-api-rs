use super::super::common;

#[tokio::test]
async fn it_returns_all_pages() {
    let client = common::get_client();
    client.get_pages().await.unwrap();
}

#[tokio::test]
async fn it_returns_page_info() {
    let client = common::get_client();
    let result = client
        .get_page_info("0c33d6599c22d5ec12a57b79d8c5a528ebdab7a8c2b462e6d76e2d0512e917fd")
        .await;
    assert!(result.is_err()); // page does not exist
}

#[tokio::test]
async fn it_focuses_pages() {
    let client = common::get_client();
    let result = client
        .focus_page("0c33d6599c22d5ec12a57b79d8c5a528ebdab7a8c2b462e6d76e2d0512e917fd")
        .await;

    assert!(result.is_err()); // page does not exist
}
