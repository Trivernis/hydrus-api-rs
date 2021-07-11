use crate::endpoints::adding_files::{STATUS_IMPORT_FAILED, STATUS_IMPORT_VETOED};
use crate::endpoints::adding_urls::AddUrlRequestBuilder;
use crate::endpoints::common::FileIdentifier;
use crate::error::{Error, Result};
use crate::hydrus_file::HydrusFile;
use crate::models::url::Url;
use crate::page::PageIdentifier;
use crate::service::ServiceName;
use crate::tag::Tag;
use crate::utils::tag_list_to_string_list;
use crate::Client;
use std::collections::HashMap;
use std::io::Read;

pub struct ImportBuilder {
    pub(crate) client: Client,
}

impl ImportBuilder {
    pub fn file(self, file: FileImport) -> FileImportBuilder {
        FileImportBuilder {
            client: self.client,
            file,
        }
    }

    pub fn url<S: ToString>(self, url: S) -> UrlImportBuilder {
        UrlImportBuilder {
            client: self.client,
            url: url.to_string(),
            page: None,
            show_page: false,
            filter_tags: vec![],
            service_tag_mappings: Default::default(),
        }
    }
}

pub enum FileImport {
    Path(String),
    Binary(Vec<u8>),
}

impl FileImport {
    pub fn path<S: ToString>(path: S) -> Self {
        Self::Path(path.to_string())
    }

    pub fn binary<R: Read>(reader: &mut R) -> Self {
        let mut bytes = Vec::new();
        let _ = reader.read_to_end(&mut bytes);
        Self::Binary(bytes)
    }
}

pub struct FileImportBuilder {
    client: Client,
    file: FileImport,
}

pub struct UrlImportBuilder {
    client: Client,
    url: String,
    page: Option<PageIdentifier>,
    show_page: bool,
    filter_tags: Vec<Tag>,
    service_tag_mappings: HashMap<String, Vec<Tag>>,
}

impl FileImportBuilder {
    pub async fn run(mut self) -> Result<HydrusFile> {
        let response = match self.file {
            FileImport::Path(path) => self.client.add_file(path).await?,
            FileImport::Binary(b) => self.client.add_binary_file(b).await?,
        };

        if response.status == STATUS_IMPORT_FAILED {
            Err(Error::ImportFailed(response.note))
        } else if response.status == STATUS_IMPORT_VETOED {
            Err(Error::ImportVetoed(response.note))
        } else {
            Ok(HydrusFile {
                client: self.client,
                id: FileIdentifier::Hash(response.hash),
            })
        }
    }
}

impl UrlImportBuilder {
    /// Sets the destination page of the import
    pub fn set_page(mut self, page: PageIdentifier) -> Self {
        self.page = Some(page);

        self
    }

    /// If the destination page of the import should be focussed
    pub fn show_page(mut self, show: bool) -> Self {
        self.show_page = show;

        self
    }

    /// Adds a tag that should be filtered
    pub fn add_filter_tag(mut self, tag: Tag) -> Self {
        self.filter_tags.push(tag);

        self
    }

    /// Adds multiple tags that should be filtered
    pub fn add_filter_tags(mut self, mut tags: Vec<Tag>) -> Self {
        self.filter_tags.append(&mut tags);

        self
    }

    /// Adds an additional tag for the imported file
    pub fn add_additional_tag(self, service: ServiceName, tag: Tag) -> Self {
        self.add_additional_tags(service, vec![tag])
    }

    /// Adds multiple additional tags for the import
    pub fn add_additional_tags(mut self, service: ServiceName, mut tags: Vec<Tag>) -> Self {
        if let Some(service_tags) = self.service_tag_mappings.get_mut(&service.0) {
            service_tags.append(&mut tags);
        } else {
            self.service_tag_mappings.insert(service.0, tags);
        }

        self
    }

    /// Imports the URL
    pub async fn run(mut self) -> Result<Url> {
        let mut request = AddUrlRequestBuilder::default().url(self.url.clone());

        for (service, tags) in self.service_tag_mappings {
            request = request.add_tags(service, tag_list_to_string_list(tags));
        }
        request = request.add_filter_tags(tag_list_to_string_list(self.filter_tags));
        if let Some(page) = self.page {
            request = match page {
                PageIdentifier::Name(n) => request.destination_page_name(n),
                PageIdentifier::Key(k) => request.destination_page_key(k),
            };
        }
        request = request.show_destination_page(self.show_page);

        let response = self.client.add_url(request.build()).await?;

        Ok(Url {
            url: self.url,
            normalised_url: Some(response.normalised_url),
        })
    }
}
