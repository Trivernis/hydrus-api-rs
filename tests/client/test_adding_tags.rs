use super::super::common;
use crate::common::test_data::EMPTY_HASH;
use hydrus_api::api_core::common::ServiceIdentifier;
use hydrus_api::api_core::endpoints::adding_tags::{
    AddTagsRequestBuilder, TagAction, TagDisplayType, TagSearchOptions,
};

#[tokio::test]
async fn it_cleans_tags() {
    let client = common::get_client();
    let response = client
        .clean_tags(vec![
            "summer".into(),
            "rain".into(),
            "beach".into(),
            "safe".into(),
        ])
        .await
        .unwrap();
    assert!(response.tags.len() > 0)
}

#[tokio::test]
async fn it_adds_tags() {
    #![allow(deprecated)]
    let client = common::get_client();
    let request = AddTagsRequestBuilder::default()
        .add_hash(EMPTY_HASH) // valid hash, I hope no files are affected
        .add_tags(
            "6c6f63616c2074616773".into(),
            vec!["beach".into(), "summer".into()],
        )
        .add_tag_with_action(
            "6c6f63616c2074616773".into(),
            "rain",
            TagAction::DeleteFromLocalService,
        )
        .build();
    client.add_tags(request).await.unwrap();
}

/// This test requires that searching for "*" is permitted in hydrus
#[tokio::test]
async fn it_searches_for_tags() {
    #![allow(deprecated)]
    let client = common::get_client();
    let response = client
        .search_tags(
            "*",
            TagSearchOptions::default()
                .display_type(TagDisplayType::Display)
                .tag_service(ServiceIdentifier::name("all known tags")),
        )
        .await
        .unwrap();
    assert!(response.tags.is_empty() == false)
}
