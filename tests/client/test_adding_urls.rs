use super::super::common;
use crate::common::test_data::{get_test_hashes, get_test_urls, TEST_URL_1};
use hydrus_api::api_core::adding_urls::{AddUrlRequestBuilder, URL_TYPE_POST};
use hydrus_api::api_core::common::ServiceIdentifier;

#[tokio::test]
async fn it_returns_files_for_an_url() {
    let client = common::get_client();
    let response = client.get_url_files(TEST_URL_1).await.unwrap();

    assert!(response.normalised_url.len() > 0);
}

#[tokio::test]
async fn it_returns_url_information() {
    let client = common::get_client();
    let info = client.get_url_info(TEST_URL_1).await.unwrap();
    assert!(info.normalised_url.len() > 0);
    assert_eq!(info.url_type, URL_TYPE_POST);
}

#[tokio::test]
async fn it_adds_urls() {
    #![allow(deprecated)]
    let client = common::get_client();
    let request = AddUrlRequestBuilder::default()
        .url(TEST_URL_1)
        .add_tags(
            ServiceIdentifier::name("my tags"),
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
    let client = common::get_client();
    common::create_testdata(&client).await;
    client
        .associate_urls(get_test_urls(), get_test_hashes())
        .await
        .unwrap();
}

#[tokio::test]
async fn it_disassociates_urls() {
    let client = common::get_client();
    common::create_testdata(&client).await;
    client
        .disassociate_urls(get_test_urls(), get_test_hashes())
        .await
        .unwrap();
}
