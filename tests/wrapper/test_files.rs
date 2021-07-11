use super::super::common;
use hydrus_api::endpoints::adding_tags::TagAction;
use hydrus_api::endpoints::common::FileIdentifier;
use hydrus_api::hydrus_file::HydrusFile;
use hydrus_api::service::ServiceName;

async fn get_file() -> HydrusFile {
    let hydrus = common::get_hydrus();
    hydrus
        .file(FileIdentifier::hash(
            "277a138cd1ee79fc1fdb2869c321b848d4861e45b82184487139ef66dd40b62d", // needs to exist
        ))
        .await
        .unwrap()
}

#[tokio::test]
async fn it_associates_with_urls() {
    let mut file = get_file().await;
    file.associate_urls(vec![
        "https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium".to_string(),
    ])
    .await
    .unwrap();
}

#[tokio::test]
async fn it_disassociates_with_urls() {
    let mut file = get_file().await;
    file.disassociate_urls(vec![
        "https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium".to_string(),
    ])
    .await
    .unwrap();
}

#[tokio::test]
async fn it_has_tags_with_services() {
    let mut file = get_file().await;
    let tags = file.services_with_tags().await.unwrap();

    assert!(tags.keys().len() > 0)
}

#[tokio::test]
async fn it_has_tags() {
    let mut file = get_file().await;
    let tags = file.tags().await.unwrap();

    assert!(tags.len() > 0) // test data needs to be prepared this way
}

#[tokio::test]
async fn it_adds_tags() {
    let mut file = get_file().await;
    file.add_tags(
        ServiceName::public_tag_repository(),
        vec!["character:megumin".into(), "ark mage".into()],
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn it_modifies_tags() {
    let mut file = get_file().await;
    file.modify_tags(
        ServiceName::public_tag_repository(),
        TagAction::RescindPendFromRepository,
        vec!["ark mage".into()],
    )
    .await
    .unwrap();
}
