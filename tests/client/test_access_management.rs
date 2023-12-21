use super::super::common;

#[tokio::test]
async fn it_returns_the_api_version() {
    let client = common::get_client();
    let api_version = client.api_version().await.unwrap();
    assert!(api_version.hydrus_version > 0);
    assert!(api_version.version > 0);
}

#[tokio::test]
async fn it_returns_the_session_key() {
    let client = common::get_client();
    let session_key = client.session_key().await.unwrap();
    assert!(session_key.session_key.len() > 0);
}

#[tokio::test]
async fn it_verifies_the_access_key() {
    let client = common::get_client();
    let verification_response = client.verify_access_key().await.unwrap();
    assert!(verification_response.basic_permissions.len() > 0); // needs to be configured in the client but we want at least some permissions for the test
    assert!(verification_response.human_description.len() > 0);
}

#[tokio::test]
async fn it_returns_a_list_of_services() {
    let client = common::get_client();
    let services_response = client.get_services().await.unwrap();
    assert!(services_response.other.keys().len() > 0);
}
