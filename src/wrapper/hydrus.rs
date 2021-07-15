use crate::api_core::common::FileIdentifier;
use crate::api_core::searching_and_fetching_files::FileSearchLocation;
use crate::error::Result;
use crate::utils::tag_list_to_string_list;
use crate::wrapper::builders::import_builder::ImportBuilder;
use crate::wrapper::builders::tagging_builder::TaggingBuilder;
use crate::wrapper::hydrus_file::HydrusFile;
use crate::wrapper::service::Services;
use crate::wrapper::tag::Tag;
use crate::wrapper::url::Url;
use crate::wrapper::version::Version;
use crate::Client;

/// A high level wrapper for the hydrus API for easier management of files, tags
/// urls etc.
pub struct Hydrus {
    client: Client,
}

impl Hydrus {
    /// Creates a new high level Hydrus API client
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Returns the Hydrus and API Version
    pub async fn version(&self) -> Result<Version> {
        let response = self.client.api_version().await?;
        Ok(Version {
            api: response.version,
            hydrus: response.hydrus_version,
        })
    }

    /// Returns a list of available services
    pub async fn services(&self) -> Result<Services> {
        let response = self.client.get_services().await?;

        Ok(Services::from_response(self.client.clone(), response))
    }

    /// Creates an import builder to build an import request to hydrus
    pub fn import(&self) -> ImportBuilder {
        ImportBuilder {
            client: self.client.clone(),
        }
    }

    /// Returns information about a given url in an object that allows
    /// further operations with that url
    pub async fn url<S: AsRef<str>>(&self, url: S) -> Result<Url> {
        let info = self.client.get_url_info(&url).await?;

        Ok(Url {
            client: self.client.clone(),
            normalised_url: info.normalised_url,
            url_type: info.url_type.into(),
            match_name: info.match_name,
            url: url.as_ref().to_string(),
            can_parse: info.can_parse,
        })
    }

    /// Returns a file by identifier to perform further operations on
    pub async fn file(&self, identifier: FileIdentifier) -> Result<HydrusFile> {
        let metadata = self
            .client
            .get_file_metadata_by_identifier(identifier)
            .await?;

        Ok(HydrusFile::from_metadata(self.client.clone(), metadata))
    }

    /// Starts a request to bulk add tags to files
    pub fn tagging(&self) -> TaggingBuilder {
        TaggingBuilder::new(self.client.clone())
    }

    /// Searches for files that have the given tags and returns a list of hydrus files as a result
    pub async fn search(
        &self,
        location: FileSearchLocation,
        tags: Vec<Tag>,
    ) -> Result<Vec<HydrusFile>> {
        let search_result = self
            .client
            .search_files(tag_list_to_string_list(tags), location)
            .await?;
        let files = search_result
            .file_ids
            .into_iter()
            .map(|id| HydrusFile::from_id(self.client.clone(), id))
            .collect();

        Ok(files)
    }
}
