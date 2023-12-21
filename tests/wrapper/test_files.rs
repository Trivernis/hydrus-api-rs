use super::super::common;
use crate::common::test_data::TEST_HASH_2;
use crate::common::{create_testdata, get_client};
use hydrus_api::api_core::common::FileIdentifier;
use hydrus_api::api_core::endpoints::adding_tags::TagAction;
use hydrus_api::wrapper::hydrus_file::HydrusFile;
use hydrus_api::wrapper::service::ServiceName;

async fn get_file() -> HydrusFile {
    let client = get_client();
    create_testdata(&client).await;
    let hydrus = common::get_hydrus();
    hydrus
        .file(FileIdentifier::hash(
            TEST_HASH_2, // needs to exist
        ))
        .await
        .unwrap()
}

#[tokio::test]
async fn it_associates_with_urls() {
    let mut file = get_file().await;
    file.associate_urls(vec![
        "https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium".to_string(),
    ])
    .await
    .unwrap();
}

#[tokio::test]
async fn it_disassociates_with_urls() {
    let mut file = get_file().await;
    file.disassociate_urls(vec![
        "https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium".to_string(),
    ])
    .await
    .unwrap();
}

#[tokio::test]
async fn it_has_tags_with_services() {
    let mut file = get_file().await;
    let tags = file.services_with_tags().await.unwrap();

    assert!(tags.keys().len() > 0)
}

#[tokio::test]
async fn it_has_tags() {
    let mut file = get_file().await;
    let tags = file.tags().await.unwrap();

    assert!(tags.len() > 0) // test data needs to be prepared this way
}

#[tokio::test]
async fn it_adds_tags() {
    let mut file = get_file().await;
    file.add_tags(
        "6c6f63616c2074616773".into(),
        vec!["character:megumin".into(), "ark mage".into()],
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn it_modifies_tags() {
    let mut file = get_file().await;
    file.modify_tags(
        "6c6f63616c2074616773".into(),
        TagAction::DeleteFromLocalService,
        vec!["ark mage".into()],
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn it_adds_notes() {
    let file = get_file().await;
    file.add_notes()
        .add_note("My Note", "My notes content")
        .add_notes(vec![("My note 2", "More content")])
        .run()
        .await
        .unwrap();
}

#[tokio::test]
async fn it_deletes_notes() {
    let file = get_file().await;
    file.delete_note("My Note").await.unwrap();
}

#[tokio::test]
async fn it_retrieves_content() {
    let file = get_file().await;
    let file = file.retrieve().await.unwrap();

    assert!(file.bytes.len() > 0) // assuming it exists
}

#[tokio::test]
async fn it_retrieves_metadata() {
    let mut file = get_file().await;
    assert!(file.dimensions().await.unwrap().is_some());
    assert!(file.duration().await.unwrap().is_none());
    assert!(file.time_modified().await.is_ok());
    assert!(file.time_deleted("000").await.is_ok());
    assert!(file.time_imported("000").await.is_ok());
}

#[tokio::test]
async fn it_deletes() {
    let mut file = get_file().await;
    file.delete()
        .reason("I just don't like that file")
        .run()
        .await
        .unwrap();
    file.undelete(ServiceName::my_files().into()).await.unwrap();
}
