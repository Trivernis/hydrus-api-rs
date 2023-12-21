use crate::api_core::common::FileMetadataServices;
use crate::api_core::endpoints::Endpoint;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

pub mod file_sort_type {
    pub const SORT_FILE_SIZE: u8 = 0;
    pub const SORT_FILE_DURATION: u8 = 1;
    pub const SORT_FILE_IMPORT_TIME: u8 = 2;
    pub const SORT_FILE_TYPE: u8 = 3;
    pub const SORT_FILE_RANDOM: u8 = 4;
    pub const SORT_FILE_WIDTH: u8 = 5;
    pub const SORT_FILE_HEIGHT: u8 = 6;
    pub const SORT_FILE_RATIO: u8 = 7;
    pub const SORT_FILE_PIXEL_COUNT: u8 = 8;
    pub const SORT_FILE_TAG_COUNT: u8 = 9;
    pub const SORT_FILE_MEDIA_VIEWS: u8 = 10;
    pub const SORT_FILE_MEDIA_VIEWTIME: u8 = 11;
    pub const SORT_FILE_BITRATE: u8 = 12;
    pub const SORT_FILE_HAS_AUDIO: u8 = 13;
    pub const SORT_FILE_MODIFIED_TIME: u8 = 14;
    pub const SORT_FILE_FRAMERATE: u8 = 15;
    pub const SORT_FILE_FRAME_COUNT: u8 = 16;
}

#[derive(Clone, Debug, Default)]
pub struct FileSearchOptions {
    file_service_name: Option<String>,
    file_service_key: Option<String>,
    tag_service_name: Option<String>,
    tag_service_key: Option<String>,
    file_sort_type: Option<u8>,
    file_sort_asc: Option<bool>,
}

impl FileSearchOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn file_service_name<S: ToString>(mut self, name: S) -> Self {
        self.file_service_name = Some(name.to_string());
        self
    }

    pub fn file_service_key<S: ToString>(mut self, key: S) -> Self {
        self.file_service_key = Some(key.to_string());
        self
    }

    pub fn tag_service_name<S: ToString>(mut self, name: S) -> Self {
        self.tag_service_name = Some(name.to_string());
        self
    }

    pub fn tag_service_key<S: ToString>(mut self, key: S) -> Self {
        self.tag_service_key = Some(key.to_string());
        self
    }

    pub fn sort_type(mut self, sort_type: u8) -> Self {
        self.file_sort_type = Some(sort_type);
        self
    }

    pub fn asc(mut self) -> Self {
        self.file_sort_asc = Some(true);
        self
    }

    pub fn desc(mut self) -> Self {
        self.file_sort_asc = Some(false);
        self
    }

    pub(crate) fn into_query_args(self) -> Vec<(&'static str, String)> {
        let mut args = Vec::new();
        if let Some(sort) = self.file_sort_type {
            args.push(("file_sort_type", sort.to_string()));
        }
        if let Some(file_service_name) = self.file_service_name {
            args.push(("file_service_name", file_service_name));
        }
        if let Some(file_service_key) = self.file_service_key {
            args.push(("file_service_key", file_service_key));
        }
        if let Some(tag_service_name) = self.tag_service_name {
            args.push(("tag_service_name", tag_service_name))
        }
        if let Some(tag_service_key) = self.tag_service_key {
            args.push(("tag_service_key", tag_service_key));
        }
        if let Some(sort_asc) = self.file_sort_asc {
            args.push(("file_sort_asc", sort_asc.to_string()))
        }

        args
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchFilesResponse {
    pub file_ids: Vec<u64>,
}

pub struct SearchFiles;

impl Endpoint for SearchFiles {
    type Request = ();
    type Response = SearchFilesResponse;

    fn path() -> String {
        String::from("get_files/search_files")
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchFileHashesResponse {
    pub hashes: Vec<String>,
}

pub struct SearchFileHashes;

impl Endpoint for SearchFileHashes {
    type Request = ();
    type Response = SearchFileHashesResponse;

    fn path() -> String {
        String::from("get_files/search_files")
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct FileMetadataResponse<M: FileMetadataType> {
    pub metadata: Vec<M::Response>,
}

pub struct FileMetadata<M: FileMetadataType>(PhantomData<M>);

impl<M: FileMetadataType> Endpoint for FileMetadata<M> {
    type Request = ();
    type Response = FileMetadataResponse<M>;

    fn path() -> String {
        String::from("get_files/file_metadata")
    }
}

pub struct GetFile;

impl Endpoint for GetFile {
    type Request = ();
    type Response = ();

    fn path() -> String {
        String::from("get_files/file")
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum SearchQueryEntry {
    Tag(String),
    OrChain(Vec<String>),
}

impl<S> From<S> for SearchQueryEntry
where
    S: ToString,
{
    fn from(s: S) -> Self {
        Self::Tag(s.to_string())
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct FileMetadataIdentifiers {
    pub file_id: u64,
    pub hash: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct FileBasicMetadata {
    #[serde(flatten)]
    pub identifiers: FileMetadataIdentifiers,
    pub size: Option<u64>,
    pub mime: String,
    pub ext: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<u64>,
    pub time_modified: Option<u64>,
    pub has_audio: Option<bool>,
    pub num_frames: Option<u64>,
    pub num_words: Option<u64>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct FileFullMetadata {
    #[serde(flatten)]
    pub basic_metadata: FileBasicMetadata,
    pub is_inbox: bool,
    pub is_local: bool,
    pub is_trashed: bool,
    pub file_services: FileMetadataServices,
    pub known_urls: Vec<String>,
    /// map of service keys to tags
    pub tags: HashMap<String, ServiceTags>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ServiceTags {
    pub name: String,
    #[serde(alias = "type")]
    pub service_type: u64,
    pub type_pretty: String,
    pub storage_tags: HashMap<String, Vec<String>>,
    pub display_tags: HashMap<String, Vec<String>>,
}

pub trait FileMetadataType: Clone + Debug {
    type Response: DeserializeOwned + Clone + Debug;

    fn only_identifiers() -> bool;
    fn only_basic_information() -> bool;
}

#[derive(Clone, Debug)]
pub struct FullMetadata;

impl FileMetadataType for FullMetadata {
    type Response = FileFullMetadata;

    fn only_identifiers() -> bool {
        false
    }

    fn only_basic_information() -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub struct BasicMetadata;

impl FileMetadataType for BasicMetadata {
    type Response = FileBasicMetadata;

    fn only_identifiers() -> bool {
        false
    }

    fn only_basic_information() -> bool {
        true
    }
}

#[derive(Clone, Debug)]
pub struct Identifiers;

impl FileMetadataType for Identifiers {
    type Response = FileMetadataIdentifiers;

    fn only_identifiers() -> bool {
        true
    }

    fn only_basic_information() -> bool {
        false
    }
}
