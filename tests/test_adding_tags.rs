use hydrus_api::paths::adding_tags::{AddTagsRequestBuilder, TagAction};
mod common;

#[tokio::test]
async fn it_cleans_tags() {
    let mut client = common::get_client();
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
    let mut client = common::get_client();
    let request = AddTagsRequestBuilder::default()
        .add_hash("0000000000000000000000000000000000000000000000000000000000000000") // valid hash, I hope no files are affected
        .add_tags("my tags", vec!["beach".into(), "summer".into()])
        .add_tag_with_action("my tags", "rain", TagAction::DeleteFromLocalService)
        .build();
    client.add_tags(request).await.unwrap();
}
