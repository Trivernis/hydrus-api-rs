use hydrus_api::endpoints::adding_urls::{AddUrlRequestBuilder, URL_TYPE_POST};

mod common;

#[tokio::test]
async fn it_returns_files_for_an_url() {
    let mut client = common::get_client();
    let response = client
        .get_url_files("https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium")
        .await
        .unwrap();

    assert!(response.normalised_url.len() > 0);
}

#[tokio::test]
async fn it_returns_url_information() {
    let mut client = common::get_client();
    let info = client
        .get_url_info("https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium")
        .await
        .unwrap();
    assert!(info.normalised_url.len() > 0);
    assert_eq!(info.url_type, URL_TYPE_POST);
}

#[tokio::test]
async fn it_adds_urls() {
    let mut client = common::get_client();
    let request = AddUrlRequestBuilder::default()
        .url("https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium")
        .add_tags(
            "my tags",
            vec!["ark mage".to_string(), "grinning".to_string()],
        )
        .show_destination_page(true)
        .destination_page_name("Rusty Url Import")
        .build();
    let response = client.add_url(request).await.unwrap();
    assert!(response.normalised_url.len() > 0);
}

#[tokio::test]
async fn it_associates_urls() {
    let mut client = common::get_client();
    client
        .associate_urls(
            vec![
                "https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium"
                    .to_string(),
            ],
            vec!["0000000000000000000000000000000000000000000000000000000000000000".to_string()],
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn it_disassociates_urls() {
    let mut client = common::get_client();
    client
        .disassociate_urls(
            vec![
                "https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium"
                    .to_string(),
            ],
            vec!["0000000000000000000000000000000000000000000000000000000000000000".to_string()],
        )
        .await
        .unwrap();
}
