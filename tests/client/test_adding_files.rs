use crate::common;
use crate::common::create_testdata;
use crate::common::test_data::get_test_hashes;

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
    client.delete_files(get_test_hashes()).await.unwrap();
}

#[tokio::test]
async fn it_undeletes_files() {
    let client = common::get_client();
    create_testdata(&client).await;
    client.undelete_files(get_test_hashes()).await.unwrap();
}

#[tokio::test]
async fn it_archives_files() {
    let client = common::get_client();
    create_testdata(&client).await;
    client.archive_files(get_test_hashes()).await.unwrap();
}

#[tokio::test]
async fn it_unarchives_files() {
    let client = common::get_client();
    create_testdata(&client).await;
    client.unarchive_files(get_test_hashes()).await.unwrap();
}
