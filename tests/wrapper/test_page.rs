use super::super::common;
use hydrus_api::wrapper::page::HydrusPage;

async fn get_page() -> HydrusPage {
    let hydrus = common::get_hydrus();

    hydrus.root_page().await.unwrap()
}

#[tokio::test]
async fn it_can_be_focused() {
    let page = get_page().await;
    page.focus().await.unwrap();
}

#[tokio::test]
async fn it_has_a_name() {
    let page = get_page().await;
    assert!(page.name.len() > 0)
}

#[tokio::test]
async fn it_has_a_key() {
    let page = get_page().await;
    assert!(page.key.len() > 0)
}

#[tokio::test]
async fn it_has_a_id() {
    let page = get_page().await;
    page.id();
}
