use crate::builders::import_builder::UrlImportBuilder;
use crate::endpoints::adding_urls::{
    URL_TYPE_FILE, URL_TYPE_GALLERY, URL_TYPE_POST, URL_TYPE_WATCHABLE,
};
use crate::error::Result;
use crate::hydrus_file::HydrusFile;
use crate::Client;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum UrlType {
    Post,
    File,
    Gallery,
    Watchable,
    Unknown,
}

impl Eq for UrlType {}

impl From<u8> for UrlType {
    fn from(value: u8) -> Self {
        match value {
            v if v == URL_TYPE_POST => Self::Post,
            v if v == URL_TYPE_FILE => Self::File,
            v if v == URL_TYPE_GALLERY => Self::Gallery,
            v if v == URL_TYPE_WATCHABLE => Self::Watchable,
            _ => Self::Unknown,
        }
    }
}

#[derive(Clone)]
pub struct Url {
    pub url: String,
    pub(crate) client: Client,
    pub normalised_url: String,
    pub url_type: UrlType,
    pub match_name: String,
    pub can_parse: bool,
}

impl Url {
    /// Returns a list of files associated with the url
    pub async fn files(&mut self) -> Result<Vec<HydrusFile>> {
        let response = self.client.get_url_files(&self.url).await?;
        let files = response
            .url_file_statuses
            .into_iter()
            .map(|file| {
                HydrusFile::from_raw_status_and_hash(self.client.clone(), file.status, file.hash)
            })
            .collect();

        Ok(files)
    }

    /// Creates an import builder for the url
    pub fn import(&mut self) -> UrlImportBuilder {
        UrlImportBuilder::new(self.client.clone(), &self.url)
    }

    /// Associates the url with a list of file hashes
    pub async fn associate(&mut self, hashes: Vec<String>) -> Result<()> {
        self.client
            .associate_urls(vec![self.url.clone()], hashes)
            .await
    }

    /// Disassociates the url with a list of file hashes
    pub async fn disassociate(&mut self, hashes: Vec<String>) -> Result<()> {
        self.client
            .disassociate_urls(vec![self.url.clone()], hashes)
            .await
    }
}
