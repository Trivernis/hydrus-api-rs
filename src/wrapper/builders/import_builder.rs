use crate::api_core::common::ServiceIdentifier;
use crate::api_core::endpoints::adding_files::{STATUS_IMPORT_FAILED, STATUS_IMPORT_VETOED};
use crate::api_core::endpoints::adding_urls::AddUrlRequestBuilder;
use crate::error::{Error, Result};
use crate::utils::tag_list_to_string_list;
use crate::wrapper::hydrus_file::HydrusFile;
use crate::wrapper::page::PageIdentifier;
use crate::wrapper::tag::Tag;
use crate::wrapper::url::Url;
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
        UrlImportBuilder::new(self.client.clone(), url)
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

impl FileImportBuilder {
    pub async fn run(self) -> Result<HydrusFile> {
        let response = match self.file {
            FileImport::Path(path) => self.client.add_file(path).await?,
            FileImport::Binary(b) => self.client.add_binary_file(b).await?,
        };

        if response.status == STATUS_IMPORT_FAILED {
            Err(Error::ImportFailed(response.note))
        } else if response.status == STATUS_IMPORT_VETOED {
            Err(Error::ImportVetoed(response.note))
        } else {
            Ok(HydrusFile::from_raw_status_and_hash(
                self.client,
                response.status,
                response.hash,
            ))
        }
    }
}

pub struct UrlImportBuilder {
    client: Client,
    url: String,
    page: Option<PageIdentifier>,
    show_page: bool,
    filter_tags: Vec<Tag>,
    service_tag_mappings: HashMap<ServiceIdentifier, Vec<Tag>>,
}

impl UrlImportBuilder {
    pub fn new<S: ToString>(client: Client, url: S) -> Self {
        Self {
            client,
            url: url.to_string(),
            page: None,
            show_page: false,
            filter_tags: vec![],
            service_tag_mappings: Default::default(),
        }
    }

    /// Sets the destination page of the import
    pub fn page(mut self, page: PageIdentifier) -> Self {
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
    pub fn add_additional_tag(self, service: ServiceIdentifier, tag: Tag) -> Self {
        self.add_additional_tags(service, vec![tag])
    }

    /// Adds multiple additional tags for the import
    pub fn add_additional_tags(mut self, service: ServiceIdentifier, mut tags: Vec<Tag>) -> Self {
        if let Some(service_tags) = self.service_tag_mappings.get_mut(&service) {
            service_tags.append(&mut tags);
        } else {
            self.service_tag_mappings.insert(service, tags);
        }

        self
    }

    /// Imports the URL
    pub async fn run(self) -> Result<Url> {
        let mut request = AddUrlRequestBuilder::default().url(&self.url);

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
        let url_info = self.client.get_url_info(&self.url).await?;

        Ok(Url {
            url: self.url,
            client: self.client,
            normalised_url: response.normalised_url,
            url_type: url_info.url_type.into(),
            match_name: url_info.match_name,
            can_parse: url_info.can_parse,
        })
    }
}
