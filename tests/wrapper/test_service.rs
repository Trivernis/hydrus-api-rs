use super::super::common;
use hydrus_api::wrapper::service::{Service, ServiceType, Services};

async fn get_services() -> Services {
    let hydrus = common::get_hydrus();
    hydrus.services().await.unwrap()
}

async fn get_file_service() -> Service {
    let services = get_services().await;
    services
        .get_services(ServiceType::LocalFiles)
        .pop()
        .unwrap()
        .clone()
}

#[tokio::test]
async fn it_searches_for_files() {
    let service = get_file_service().await;
    service
        .search()
        .add_tag("character:rimuru tempest".into())
        .run()
        .await
        .unwrap();
}
