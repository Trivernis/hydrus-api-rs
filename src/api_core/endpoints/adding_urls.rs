use crate::api_core::common::ServiceIdentifier;
use crate::api_core::endpoints::Endpoint;
use serde::Serialize;
use std::collections::HashMap;

pub static URL_TYPE_POST: u8 = 0;
pub static URL_TYPE_FILE: u8 = 1;
pub static URL_TYPE_GALLERY: u8 = 2;
pub static URL_TYPE_WATCHABLE: u8 = 4;
pub static URL_TYPE_UNKNOWN: u8 = 5;

#[derive(Clone, Debug, Deserialize)]
pub struct GetUrlFilesResponse {
    pub normalised_url: String,
    pub url_file_statuses: Vec<UrlFileStatus>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UrlFileStatus {
    pub status: u8,
    pub hash: String,
    pub note: String,
}

pub struct GetUrlFiles;

impl Endpoint for GetUrlFiles {
    type Request = ();
    type Response = GetUrlFilesResponse;

    fn path() -> String {
        String::from("add_urls/get_url_files")
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetUrlInfoResponse {
    pub normalised_url: String,
    pub url_type: u8,
    pub url_type_string: String,
    pub match_name: String,
    pub can_parse: bool,
}

pub struct GetUrlInfo;

impl Endpoint for GetUrlInfo {
    type Request = ();
    type Response = GetUrlInfoResponse;

    fn path() -> String {
        String::from("add_urls/get_url_info")
    }
}

#[derive(Clone, Default, Debug, Serialize)]
pub struct AddUrlRequest {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_page_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_page_name: Option<String>,

    pub show_destination_page: bool,
    pub service_names_to_additional_tags: HashMap<String, Vec<String>>,
    pub service_keys_to_additional_tags: HashMap<String, Vec<String>>,
    pub filterable_tags: Vec<String>,
}

/// A request builder that can be used to create a request for adding urls
/// without having to fill a huge struct manually
///
/// Example:
/// ```
/// use hydrus_api::api_core::endpoints::adding_urls::AddUrlRequestBuilder;
/// use hydrus_api::api_core::common::ServiceIdentifier;
///
/// let request = AddUrlRequestBuilder::default()
///     .url("https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium")
///     .add_tags(ServiceIdentifier::name("my tags"), vec!["ark mage".to_string(), "grinning".to_string()])
///     .show_destination_page(true)
///     .destination_page_name("Rusty Url Import")
///     .build();
/// ```
#[derive(Default)]
pub struct AddUrlRequestBuilder {
    inner: AddUrlRequest,
}

impl AddUrlRequestBuilder {
    pub fn url<S: ToString>(mut self, url: S) -> Self {
        self.inner.url = url.to_string();

        self
    }

    pub fn destination_page_key<S: ToString>(mut self, page_key: S) -> Self {
        self.inner.destination_page_key = Some(page_key.to_string());

        self
    }

    pub fn destination_page_name<S: ToString>(mut self, page_name: S) -> Self {
        self.inner.destination_page_name = Some(page_name.to_string());

        self
    }

    pub fn show_destination_page(mut self, show: bool) -> Self {
        self.inner.show_destination_page = show;

        self
    }

    pub fn add_tags(mut self, service_id: ServiceIdentifier, mut tags: Vec<String>) -> Self {
        let (service, mappings) = match service_id {
            ServiceIdentifier::Name(name) => {
                (name, &mut self.inner.service_names_to_additional_tags)
            }
            ServiceIdentifier::Key(key) => (key, &mut self.inner.service_keys_to_additional_tags),
        };
        if let Some(entry) = mappings.get_mut(&service) {
            entry.append(&mut tags);
        } else {
            mappings.insert(service, tags);
        }

        self
    }

    pub fn add_filter_tags(mut self, mut filter_tags: Vec<String>) -> Self {
        self.inner.filterable_tags.append(&mut filter_tags);

        self
    }

    pub fn build(self) -> AddUrlRequest {
        self.inner
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct AddUrlResponse {
    pub human_result_text: String,
    pub normalised_url: String,
}

pub struct AddUrl;

impl Endpoint for AddUrl {
    type Request = AddUrlRequest;
    type Response = AddUrlResponse;

    fn path() -> String {
        String::from("add_urls/add_url")
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct AssociateUrlRequest {
    pub urls_to_add: Vec<String>,
    pub urls_to_delete: Vec<String>,
    pub hashes: Vec<String>,
}
pub struct AssociateUrl;

impl Endpoint for AssociateUrl {
    type Request = AssociateUrlRequest;
    type Response = ();

    fn path() -> String {
        String::from("add_urls/associate_url")
    }
}
