use super::super::common;
use hydrus_api::api_core::common::FileIdentifier;
use hydrus_api::api_core::endpoints::searching_and_fetching_files::file_sort_type::SORT_FILE_PIXEL_COUNT;
use hydrus_api::api_core::endpoints::searching_and_fetching_files::{
    BasicMetadata, FileSearchOptions, FullMetadata, Identifiers, SearchQueryEntry,
};

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
    let response = client
        .get_file_metadata::<Identifiers>(vec![1], vec![])
        .await;
    assert!(response.is_ok());
    let response = client
        .get_file_metadata::<BasicMetadata>(vec![1], vec![])
        .await;
    assert!(response.is_ok());
    let response = client
        .get_file_metadata::<FullMetadata>(vec![1], vec![])
        .await;
    assert!(response.is_ok());
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
