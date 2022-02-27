use super::super::common;
use hydrus_api::api_core::adding_tags::TagAction;
use hydrus_api::wrapper::builders::or_chain_builder::OrChainBuilder;
use hydrus_api::wrapper::builders::search_builder::SortType;
use hydrus_api::wrapper::builders::tag_builder::TagBuilder;
use hydrus_api::wrapper::service::{ServiceName, ServiceType};
use hydrus_api::wrapper::url::UrlType;

#[tokio::test]
async fn it_retrieves_version_info() {
    let hydrus = common::get_hydrus();
    let version = hydrus.version().await.unwrap();
    assert!(version.hydrus > 0);
    assert!(version.api > 0);
}

#[tokio::test]
async fn it_retrieves_services() {
    let hydrus = common::get_hydrus();
    let services = hydrus.services().await.unwrap();

    // assuming hydrus is configured correctly
    assert!(services.get_services(ServiceType::AllKnownFiles).len() > 0);
    assert!(services.get_services(ServiceType::AllKnownTags).len() > 0);
}

#[tokio::test]
async fn it_retrieves_url_information() {
    let hydrus = common::get_hydrus();
    let url = hydrus
        .url("https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium")
        .await
        .unwrap();

    assert_eq!(url.url_type, UrlType::Post)
}

#[tokio::test]
async fn it_searches() {
    let hydrus = common::get_hydrus();
    hydrus
        .search()
        .add_tag("character:megumin".into())
        .add_or_chain(
            OrChainBuilder::new()
                .add_tag("summer".into())
                .add_tag("winter".into())
                .add_tag(TagBuilder::new("inside").negate().build())
                .build(),
        )
        .sort_by(SortType::ModifiedTime)
        .run()
        .await
        .unwrap();
}

#[tokio::test]
async fn it_adds_tags() {
    let hydrus = common::get_hydrus();
    hydrus
        .tagging()
        .add_tag(
            ServiceName::my_tags().into(),
            TagAction::AddToLocalService,
            "summer".into(),
        )
        .add_file("0000000000000000000000000000000000000000000000000000000000000000")
        .run()
        .await
        .unwrap();
}

#[tokio::test]
async fn it_sets_the_user_agent() {
    let hydrus = common::get_hydrus();
    hydrus
        .set_user_agent("Mozilla/5.0 (compatible; Hydrus Client)")
        .await
        .unwrap();
}
