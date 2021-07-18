use super::super::common;

#[tokio::test]
async fn it_returns_all_pages() {
    let client = common::get_client();
    client.get_pages().await.unwrap();
}
