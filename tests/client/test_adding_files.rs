use super::super::common;

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
    client.delete_files(vec![]).await.unwrap();
}

#[tokio::test]
async fn it_undeletes_files() {
    let client = common::get_client();
    client.undelete_files(vec![]).await.unwrap();
}

#[tokio::test]
async fn it_archives_files() {
    let client = common::get_client();
    client.archive_files(vec![]).await.unwrap();
}

#[tokio::test]
async fn it_unarchives_files() {
    let client = common::get_client();
    client.unarchive_files(vec![]).await.unwrap();
}
