use crate::{
    api_core::{
        common::ServiceIdentifier,
        endpoints::adding_tags::{TagDisplayType, TagSearchOptions, TagWithCount},
    },
    error::Result,
    wrapper::tag::Tag,
    Client,
};

pub struct TagSearchBuilder {
    client: Client,
    query: String,
    options: TagSearchOptions,
}

impl TagSearchBuilder {
    pub(crate) fn new(client: Client, query: String) -> Self {
        Self {
            client,
            query,
            options: TagSearchOptions::default(),
        }
    }

    /// Returns a list of tags as displayed in hydrus
    /// rather than what is stored in the database.
    pub fn as_displayed(mut self) -> Self {
        self.options = self.options.display_type(TagDisplayType::Display);
        self
    }

    /// Adds an additioinal filter for the tag service
    pub fn tag_service(mut self, tag_service: ServiceIdentifier) -> Self {
        self.options = self.options.tag_service(tag_service);
        self
    }

    /// Runs the search
    pub async fn run(self) -> Result<Vec<(u64, Tag)>> {
        let tags = self
            .client
            .search_tags(self.query, self.options)
            .await?
            .tags
            .into_iter()
            .map(|TagWithCount { value, count }| (count, Tag::from(value)))
            .collect();

        Ok(tags)
    }
}
