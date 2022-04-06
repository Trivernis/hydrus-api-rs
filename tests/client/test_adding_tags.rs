use super::super::common;
use crate::common::test_data::EMPTY_HASH;
use hydrus_api::api_core::common::ServiceIdentifier;
use hydrus_api::api_core::endpoints::adding_tags::{AddTagsRequestBuilder, TagAction};

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
            ServiceIdentifier::name("my tags"),
            vec!["beach".into(), "summer".into()],
        )
        .add_tag_with_action(
            ServiceIdentifier::name("my tags"),
            "rain",
            TagAction::DeleteFromLocalService,
        )
        .build();
    client.add_tags(request).await.unwrap();
}
