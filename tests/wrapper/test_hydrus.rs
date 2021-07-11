use super::super::common;
use hydrus_api::service::ServiceType;

#[tokio::test]
async fn it_retrieves_version_info() {
    let mut hydrus = common::get_hydrus();
    let version = hydrus.version().await.unwrap();
    assert!(version.hydrus > 0);
    assert!(version.api > 0);
}

#[tokio::test]
async fn it_retrieves_services() {
    let mut hydrus = common::get_hydrus();
    let services = hydrus.services().await.unwrap();

    // assuming hydrus is configured correctly
    assert!(services.get_services(ServiceType::AllKnownFiles).len() > 0);
    assert!(services.get_services(ServiceType::AllKnownTags).len() > 0);
}
