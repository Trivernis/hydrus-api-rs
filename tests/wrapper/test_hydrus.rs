use super::super::common;
use hydrus_api::service::ServiceType;
use hydrus_api::url::UrlType;

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

#[tokio::test]
async fn it_retrieves_url_information() {
    let mut hydrus = common::get_hydrus();
    let url = hydrus
        .url("https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium")
        .await
        .unwrap();

    assert_eq!(url.url_type, UrlType::Post)
}
