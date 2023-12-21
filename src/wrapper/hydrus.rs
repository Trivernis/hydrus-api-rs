use crate::api_core::common::{FileIdentifier, ServiceIdentifier};
use crate::api_core::endpoints::searching_and_fetching_files::FullMetadata;
use crate::error::{Error, Result};
use crate::wrapper::address::Address;
use crate::wrapper::builders::delete_files_builder::DeleteFilesBuilder;
use crate::wrapper::builders::import_builder::ImportBuilder;
use crate::wrapper::builders::search_builder::SearchBuilder;
use crate::wrapper::builders::tagging_builder::TaggingBuilder;
use crate::wrapper::hydrus_file::HydrusFile;
use crate::wrapper::page::HydrusPage;
use crate::wrapper::service::Services;
use crate::wrapper::url::Url;
use crate::wrapper::version::Version;
use crate::Client;
use std::fmt::Debug;

use super::builders::tag_search_builder::TagSearchBuilder;

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

    /// Returns the address as an object that can be used to get and set cookies
    pub fn address<S: AsRef<str> + Debug>(&self, address: S) -> Address {
        Address::from_str(self.client.clone(), address.as_ref())
    }

    /// Returns information about a given url in an object that allows
    /// further operations with that url
    pub async fn url<S: AsRef<str> + Debug>(&self, url: S) -> Result<Url> {
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
            .get_file_metadata_by_identifier::<FullMetadata>(identifier)
            .await?;

        Ok(HydrusFile::from_metadata(self.client.clone(), metadata))
    }

    /// Creates a builder to delete files
    pub async fn delete(&self) -> DeleteFilesBuilder {
        DeleteFilesBuilder::new(self.client.clone())
    }

    /// Starts a request to bulk add tags to files
    pub fn tagging(&self) -> TaggingBuilder {
        TaggingBuilder::new(self.client.clone())
    }

    /// Starts a request to search for files
    pub fn search(&self) -> SearchBuilder {
        SearchBuilder::new(self.client.clone())
    }

    /// Starts a search request for tags with additional filter options
    pub fn search_tags<S: ToString>(&self, query: S) -> TagSearchBuilder {
        TagSearchBuilder::new(self.client.clone(), query.to_string())
    }

    /// Returns a hydrus page by page key
    pub async fn page<S: AsRef<str> + Debug>(&self, page_key: S) -> Result<HydrusPage> {
        let info_response = self.client.get_page_info(page_key).await?;

        Ok(HydrusPage::from_info(
            self.client.clone(),
            info_response.page_info,
        ))
    }

    /// Returns the root page in the client
    pub async fn root_page(&self) -> Result<HydrusPage> {
        let pages_response = self.client.get_pages().await?;

        Ok(HydrusPage::from_info(
            self.client.clone(),
            pages_response.pages,
        ))
    }

    /// Sets the user agent hydrus uses for http requests
    pub async fn set_user_agent<S: ToString + Debug>(&self, user_agent: S) -> Result<()> {
        self.client.set_user_agent(user_agent).await
    }

    /// Returns the key for a given service identifier
    pub async fn get_service_key(&self, service: ServiceIdentifier) -> Result<String> {
        let key = match service {
            ServiceIdentifier::Name(n) => self
                .client
                .get_services()
                .await?
                .other
                .values()
                .flatten()
                .filter(|v| *v.name == n)
                .next()
                .ok_or_else(|| Error::Hydrus(String::from("Service not found")))?
                .service_key
                .clone(),
            ServiceIdentifier::Key(k) => k,
        };

        Ok(key)
    }
}
