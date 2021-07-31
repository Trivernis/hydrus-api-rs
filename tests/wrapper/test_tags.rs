use super::super::common;
use chrono::{Duration, Local};
use hydrus_api::error::Result;
use hydrus_api::wrapper::builders::tag_builder::{
    Comparator, CurrentlyOrPending, FileRelationshipType, FileSizeUnit, IsComparator, PixelUnit,
    SystemTagBuilder, ViewType, WiderTallerEqual,
};
use hydrus_api::wrapper::service::ServiceName;
use hydrus_api::wrapper::tag::Tag;

async fn retrieve_single_tag(tag: Tag) -> Result<()> {
    let hydrus = common::get_hydrus();
    hydrus.search(vec![tag]).await?;

    Ok(())
}

#[tokio::test]
async fn it_returns_everything() {
    let tag = SystemTagBuilder::new().everything().build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_the_inbox() {
    let tag = SystemTagBuilder::new().inbox().build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_archived_files() {
    let tag = SystemTagBuilder::new().archive().build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_duration() {
    let tag = SystemTagBuilder::new().has_duration().build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_without_duration() {
    let tag = SystemTagBuilder::new().no_duration().build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_the_best_from_duplicates() {
    let tag = SystemTagBuilder::new().best_duplicate_quality().build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_worse_duplicates() {
    let tag = SystemTagBuilder::new().not_best_duplicate_quality().build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_audio() {
    let tag = SystemTagBuilder::new().has_audio().build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_without_audio() {
    let tag = SystemTagBuilder::new().no_audio().build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_tags() {
    let tag = SystemTagBuilder::new().has_tags().build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_without_tags() {
    let tag = SystemTagBuilder::new().no_tags().build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_untagged_files() {
    let tag = SystemTagBuilder::new().untagged().build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_number_of_tags() {
    let tag = SystemTagBuilder::new()
        .number_of_tags(Comparator::Greater, 12)
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_height() {
    let tag = SystemTagBuilder::new()
        .height(Comparator::Approximate, 200)
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_width() {
    let tag = SystemTagBuilder::new()
        .width(Comparator::Equal, 200)
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_size_in_gigabytes() {
    let tag = SystemTagBuilder::new()
        .filesize(Comparator::Less, 200, FileSizeUnit::Gigabytes)
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_size_in_megabytes() {
    let tag = SystemTagBuilder::new()
        .filesize(Comparator::Less, 200, FileSizeUnit::Megabytes)
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_size_in_kilobytes() {
    let tag = SystemTagBuilder::new()
        .filesize(Comparator::Less, 200, FileSizeUnit::Kilobytes)
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_size_in_bytes() {
    let tag = SystemTagBuilder::new()
        .filesize(Comparator::Less, 200, FileSizeUnit::Bytes)
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_that_are_similar_to_others() {
    let tag = SystemTagBuilder::new()
        .similar_to(
            vec![String::from(
                "0000000000000000000000000000000000000000000000000000000000000000",
            )],
            20,
        )
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_limits_results() {
    let tag = SystemTagBuilder::new().limit(50).build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_modification_date() {
    let tag = SystemTagBuilder::new()
        .date_modified(Comparator::Greater, Local::now())
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_import_time() {
    let tag = SystemTagBuilder::new()
        .time_imported(Comparator::Less, Local::now())
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_of_a_service() {
    let tag = SystemTagBuilder::new()
        .file_service(
            IsComparator::Is,
            CurrentlyOrPending::CurrentlyIn,
            ServiceName::my_files(),
        )
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_that_are_not_of_a_service() {
    let tag = SystemTagBuilder::new()
        .file_service(
            IsComparator::IsNot,
            CurrentlyOrPending::CurrentlyIn,
            ServiceName::my_files(),
        )
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_pending_to_service() {
    let tag = SystemTagBuilder::new()
        .file_service(
            IsComparator::Is,
            CurrentlyOrPending::PendingTo,
            ServiceName::my_files(),
        )
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_not_pending_to_service() {
    let tag = SystemTagBuilder::new()
        .file_service(
            IsComparator::IsNot,
            CurrentlyOrPending::PendingTo,
            ServiceName::my_files(),
        )
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_number_of_alternate_relationships() {
    let tag = SystemTagBuilder::new()
        .number_of_relationships(Comparator::Approximate, 3, FileRelationshipType::Alternates)
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_number_of_duplicate_relationships() {
    let tag = SystemTagBuilder::new()
        .number_of_relationships(Comparator::Approximate, 3, FileRelationshipType::Duplicates)
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_number_of_false_positive_relationships() {
    let tag = SystemTagBuilder::new()
        .number_of_relationships(
            Comparator::Approximate,
            3,
            FileRelationshipType::FalsePositives,
        )
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_number_of_potential_duplicate_relationships() {
    let tag = SystemTagBuilder::new()
        .number_of_relationships(
            Comparator::Approximate,
            3,
            FileRelationshipType::PotentialDuplicates,
        )
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_wider_than_a_specific_ratio() {
    let tag = SystemTagBuilder::new()
        .ratio(WiderTallerEqual::Wider, (40, 50))
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_taller_than_a_specific_ratio() {
    let tag = SystemTagBuilder::new()
        .ratio(WiderTallerEqual::Taller, (40, 50))
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_taller_with_specific_ratio() {
    let tag = SystemTagBuilder::new()
        .ratio(WiderTallerEqual::Equal, (40, 50))
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_number_of_megapixels() {
    let tag = SystemTagBuilder::new()
        .number_of_pixels(Comparator::Less, 50, PixelUnit::Megapixels)
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_number_of_kilopixels() {
    let tag = SystemTagBuilder::new()
        .number_of_pixels(Comparator::Equal, 50, PixelUnit::Kilopixels)
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_number_of_pixels() {
    let tag = SystemTagBuilder::new()
        .number_of_pixels(Comparator::Greater, 50, PixelUnit::Pixels)
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_number_of_views() {
    let tag = SystemTagBuilder::new()
        .views(ViewType::All, Comparator::Less, 1000)
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_number_of_preview_views() {
    let tag = SystemTagBuilder::new()
        .views(ViewType::Preview, Comparator::Equal, 1000)
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_number_of_media_views() {
    let tag = SystemTagBuilder::new()
        .views(ViewType::Media, Comparator::Greater, 1000)
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_preview_viewtime() {
    let tag = SystemTagBuilder::new()
        .viewtime(
            ViewType::Preview,
            Comparator::Greater,
            Duration::minutes(10),
        )
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_media_viewtime() {
    let tag = SystemTagBuilder::new()
        .viewtime(ViewType::Media, Comparator::Equal, Duration::minutes(10))
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_a_specific_viewtime() {
    let tag = SystemTagBuilder::new()
        .viewtime(ViewType::All, Comparator::Less, Duration::hours(10))
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_urls_matching_a_regex() {
    let tag = SystemTagBuilder::new()
        .has_url_matching_regex(".*pixiv.net.*")
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_urls_not_matching_a_regex() {
    let tag = SystemTagBuilder::new()
        .does_not_have_url_matching_regex(".*pixiv.net.*")
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_urls_matching_a_class() {
    let tag = SystemTagBuilder::new()
        .has_url_with_class("pixiv file page")
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_urls_not_matching_a_class() {
    let tag = SystemTagBuilder::new()
        .does_not_have_url_with_class("pixiv file page")
        .build();
    retrieve_single_tag(tag).await.unwrap();
}

#[tokio::test]
async fn it_returns_files_with_namespace_properties() {
    let tag = SystemTagBuilder::new()
        .tag_namespace_as_number("page", Comparator::Approximate, 5)
        .build();
    retrieve_single_tag(tag).await.unwrap();
}
