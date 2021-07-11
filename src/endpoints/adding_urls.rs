use crate::endpoints::Endpoint;
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

    fn get_path() -> String {
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

    fn get_path() -> String {
        String::from("add_urls/get_url_info")
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct AddUrlRequest {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_page_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_page_name: Option<String>,

    pub show_destination_page: bool,
    pub service_names_to_additional_tags: HashMap<String, Vec<String>>,
    pub filterable_tags: Vec<String>,
}

/// A request builder that can be used to create a request for adding urls
/// without having to fill a huge struct manually
///
/// Example:
/// ```
/// use hydrus_api::endpoints::adding_urls::AddUrlRequestBuilder;
///
/// let request = AddUrlRequestBuilder::default()
///     .url("https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium")
///     .add_tags("my tags", vec!["ark mage".to_string(), "grinning".to_string()])
///     .show_destination_page(true)
///     .destination_page_name("Rusty Url Import")
///     .build();
/// ```
pub struct AddUrlRequestBuilder {
    inner: AddUrlRequest,
}

impl Default for AddUrlRequestBuilder {
    fn default() -> Self {
        Self {
            inner: AddUrlRequest {
                url: String::new(),
                destination_page_key: None,
                destination_page_name: None,
                show_destination_page: false,
                service_names_to_additional_tags: Default::default(),
                filterable_tags: vec![],
            },
        }
    }
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

    pub fn add_tags<S: AsRef<str>>(mut self, service: S, mut tags: Vec<String>) -> Self {
        if let Some(entry) = self
            .inner
            .service_names_to_additional_tags
            .get_mut(service.as_ref())
        {
            entry.append(&mut tags);
        } else {
            self.inner
                .service_names_to_additional_tags
                .insert(service.as_ref().to_string(), tags);
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

    fn get_path() -> String {
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

    fn get_path() -> String {
        String::from("add_urls/associate_url")
    }
}
