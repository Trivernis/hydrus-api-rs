use crate::api_core::common::{FileIdentifier, PageInformation};
use crate::error::Result;
use crate::Client;

#[derive(Clone)]
pub struct HydrusPage {
    client: Client,
    pub key: String,
    pub name: String,
    pub page_type: PageType,
    pub children: Vec<HydrusPage>,
}

impl HydrusPage {
    pub(crate) fn from_info(client: Client, info: PageInformation) -> Self {
        let children = info
            .pages
            .into_iter()
            .map(|i| HydrusPage::from_info(client.clone(), i))
            .collect();

        Self {
            client,
            key: info.page_key,
            name: info.name,
            page_type: PageType::from_raw_type(info.page_type),
            children,
        }
    }

    /// Focuses the page
    pub async fn focus(&self) -> Result<()> {
        self.client.focus_page(&self.key).await
    }

    /// Returns an identifier of the page
    pub fn id(&self) -> PageIdentifier {
        PageIdentifier::key(&self.key)
    }

    /// Adds files to a page
    pub async fn add_files(&self, files: Vec<FileIdentifier>) -> Result<()> {
        let mut hashes = Vec::new();
        let mut ids = Vec::new();

        for file in files {
            match file {
                FileIdentifier::ID(id) => ids.push(id),
                FileIdentifier::Hash(hash) => hashes.push(hash),
            }
        }
        // resolve file ids to hashes
        if ids.len() > 0 && hashes.len() > 0 {
            while let Some(id) = ids.pop() {
                let metadata = self
                    .client
                    .get_file_metadata_by_identifier(FileIdentifier::ID(id))
                    .await?;
                hashes.push(metadata.hash);
            }
        }

        self.client.add_files_to_page(&self.key, ids, hashes).await
    }
}

#[derive(Clone)]
pub enum PageIdentifier {
    Name(String),
    Key(String),
}

impl PageIdentifier {
    pub fn name<S: ToString>(name: S) -> Self {
        Self::Name(name.to_string())
    }

    pub fn key<S: ToString>(key: S) -> Self {
        Self::Key(key.to_string())
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum PageType {
    GalleryDownloader,
    SimpleDownloader,
    HardDriveImport,
    Petitions,
    FileSearch,
    URLDownloader,
    Duplicates,
    ThreadWatcher,
    PageOfPages,
    Unknown,
}

impl PageType {
    pub(crate) fn from_raw_type(raw_type: u32) -> Self {
        match raw_type {
            1 => Self::GalleryDownloader,
            2 => Self::SimpleDownloader,
            3 => Self::HardDriveImport,
            4 => Self::Petitions,
            5 => Self::FileSearch,
            6 => Self::URLDownloader,
            7 => Self::Duplicates,
            8 => Self::ThreadWatcher,
            9 => Self::PageOfPages,
            _ => Self::Unknown,
        }
    }
}
