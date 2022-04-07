use super::super::common;
use crate::common::create_testdata;
use hydrus_api::api_core::common::FileIdentifier;
use hydrus_api::api_core::endpoints::searching_and_fetching_files::file_sort_type::SORT_FILE_PIXEL_COUNT;
use hydrus_api::api_core::endpoints::searching_and_fetching_files::{
    BasicMetadata, FileSearchOptions, FullMetadata, Identifiers, SearchQueryEntry,
};
use hydrus_api::wrapper::builders::tag_builder::SystemTagBuilder;
use hydrus_api::wrapper::service::ServiceName;

#[tokio::test]
async fn is_searches_files() {
    let client = common::get_client();
    let options = FileSearchOptions::new()
        .sort_type(SORT_FILE_PIXEL_COUNT)
        .tag_service_name("my tags")
        .file_service_name("all known files");
    client
        .search_files(
            vec![
                "beach".into(),
                SearchQueryEntry::OrChain(vec!["summer".to_string(), "winter".to_string()]),
            ],
            options,
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn is_searches_file_hashes() {
    let client = common::get_client();
    let options = FileSearchOptions::new()
        .sort_type(SORT_FILE_PIXEL_COUNT)
        .tag_service_name("my tags")
        .file_service_name("all known files");
    client
        .search_file_hashes(
            vec![
                "beach".into(),
                SearchQueryEntry::OrChain(vec!["summer".to_string(), "winter".to_string()]),
            ],
            options,
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn it_fetches_file_metadata() {
    let client = common::get_client();
    client
        .get_file_metadata::<FullMetadata>(
            vec![],
            vec!["0000000000000000000000000000000000000000000000000000000000000000".to_string()],
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn it_fetches_file_metadata_by_id() {
    let client = common::get_client();
    create_testdata(&client).await;
    let mut files = client
        .search_files(
            vec![SearchQueryEntry::Tag(
                SystemTagBuilder::new().everything().build().to_string(),
            )],
            FileSearchOptions::default().file_service_name(ServiceName::my_files()),
        )
        .await
        .unwrap();
    let test_id = files.file_ids.pop().unwrap();
    let response = client
        .get_file_metadata::<Identifiers>(vec![test_id], vec![])
        .await;
    response.unwrap();
    let response = client
        .get_file_metadata::<BasicMetadata>(vec![test_id], vec![])
        .await;
    response.unwrap();
    let response = client
        .get_file_metadata::<FullMetadata>(vec![test_id], vec![])
        .await;
    response.unwrap();
}

#[tokio::test]
async fn it_fetches_single_files() {
    let client = common::get_client();
    let response = client
        .get_file(FileIdentifier::Hash(
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
        ))
        .await;

    assert!(response.is_err()); // can't find the file
}
