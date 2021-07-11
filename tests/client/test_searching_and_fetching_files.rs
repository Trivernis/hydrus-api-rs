use super::super::common;
use hydrus_api::endpoints::common::FileIdentifier;
use hydrus_api::endpoints::searching_and_fetching_files::FileSearchLocation;

#[tokio::test]
async fn is_searches_files() {
    let client = common::get_client();
    client
        .search_files(vec!["beach".to_string()], FileSearchLocation::Archive)
        .await
        .unwrap();
}

#[tokio::test]
async fn it_fetches_file_metadata() {
    let client = common::get_client();
    let response = client
        .get_file_metadata(
            vec![],
            vec!["0000000000000000000000000000000000000000000000000000000000000000".to_string()],
        )
        .await;
    assert!(response.is_ok()); // Even if the file doesn't exist it still returns some information about it
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
