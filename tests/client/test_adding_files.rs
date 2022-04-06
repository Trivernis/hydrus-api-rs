use crate::common;
use crate::common::create_testdata;
use crate::common::test_data::{get_test_hashes, TEST_HASH_1};
use hydrus_api::api_core::common::FileSelection;
use hydrus_api::wrapper::service::ServiceName;

#[tokio::test]
async fn it_adds_files() {
    let client = common::get_client();
    let result = client.add_file("/does/not/exist").await;
    assert!(result.is_err()); // because the path does not exist
}

#[tokio::test]
async fn it_adds_binary_files() {
    let client = common::get_client();
    let result = client
        .add_binary_file(vec![0u8, 0u8, 0u8, 0u8])
        .await
        .unwrap();
    assert_eq!(result.status, 4); // should fail because the filetype is unknown
}

#[tokio::test]
async fn it_deletes_files() {
    let client = common::get_client();
    create_testdata(&client).await;
    client
        .delete_files(
            FileSelection::by_hashes(get_test_hashes()),
            ServiceName::my_files().into(),
            Some("Test".to_string()),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn it_undeletes_files() {
    let client = common::get_client();
    create_testdata(&client).await;
    client
        .undelete_files(
            FileSelection::by_hashes(get_test_hashes()),
            ServiceName::my_files().into(),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn it_archives_files() {
    let client = common::get_client();
    create_testdata(&client).await;
    client
        .archive_files(
            FileSelection::by_hashes(vec![TEST_HASH_1.to_string()]),
            ServiceName::my_files().into(),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn it_unarchives_files() {
    let client = common::get_client();
    create_testdata(&client).await;
    client
        .unarchive_files(
            FileSelection::by_hashes(get_test_hashes()),
            ServiceName::my_files().into(),
        )
        .await
        .unwrap();
}
