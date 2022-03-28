use super::super::common;
use crate::common::test_data::TEST_HASH_1;
use hydrus_api::api_core::common::FileIdentifier;
use std::collections::HashMap;

#[tokio::test]
async fn it_sets_notes() {
    let client = common::get_client();
    common::create_testdata(&client).await;
    let mut test_notes = HashMap::new();
    test_notes.insert("test".to_string(), "value".to_string());
    test_notes.insert("test2".to_string(), "value".to_string());

    client
        .set_notes(FileIdentifier::hash(TEST_HASH_1), test_notes)
        .await
        .unwrap();
}

#[tokio::test]
async fn it_deletes_notes() {
    let client = common::get_client();
    client
        .delete_notes(FileIdentifier::hash(TEST_HASH_1), vec!["test".to_string()])
        .await
        .unwrap();
}
