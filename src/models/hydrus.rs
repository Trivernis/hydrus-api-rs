use crate::builders::import_builder::ImportBuilder;
use crate::endpoints::common::FileIdentifier;
use crate::error::Result;
use crate::hydrus_file::HydrusFile;
use crate::models::url::Url;
use crate::models::version::Version;
use crate::service::Services;
use crate::Client;

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
}
