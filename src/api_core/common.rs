use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicServiceInfo {
    pub name: String,
    pub service_key: String,
}

impl BasicServiceInfo {
    /// Converts the Service into into an identifier
    /// that can be used for requests consuming service references
    pub fn into_id(self) -> ServiceIdentifier {
        ServiceIdentifier::Key(self.service_key)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub enum ServiceIdentifier {
    Name(String),
    Key(String),
}

impl ServiceIdentifier {
    pub fn name<S: ToString>(name: S) -> Self {
        Self::Name(name.to_string())
    }

    pub fn key<S: ToString>(key: S) -> Self {
        Self::Key(key.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicHashList {
    pub hashes: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct FileMetadataInfo {
    pub file_id: u64,
    pub hash: String,
    pub size: Option<u64>,
    pub mime: String,
    pub ext: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<u64>,
    pub time_modified: Option<u64>,
    pub file_services: FileMetadataServices,
    pub has_audio: Option<bool>,
    pub num_frames: Option<u64>,
    pub num_words: Option<u64>,
    pub is_inbox: bool,
    pub is_local: bool,
    pub is_trashed: bool,
    pub known_urls: Vec<String>,
    #[deprecated]
    pub service_names_to_statuses_to_tags: HashMap<String, HashMap<String, Vec<String>>>,
    pub service_keys_to_statuses_to_tags: HashMap<String, HashMap<String, Vec<String>>>,
    #[deprecated]
    pub service_names_to_statuses_to_display_tags: HashMap<String, HashMap<String, Vec<String>>>,
    pub service_keys_to_statuses_to_display_tags: HashMap<String, HashMap<String, Vec<String>>>,
}

#[derive(Clone, Debug)]
pub enum FileIdentifier {
    ID(u64),
    Hash(String),
}

impl FileIdentifier {
    pub fn hash<S: ToString>(hash: S) -> Self {
        Self::Hash(hash.to_string())
    }
}

#[derive(Clone)]
pub struct FileRecord {
    pub bytes: Vec<u8>,
    pub mime_type: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct FileMetadataServices {
    pub current: HashMap<String, FileMetadataServiceCurrent>,
    pub deleted: HashMap<String, FileMetadataServiceDeleted>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FileMetadataServiceCurrent {
    pub time_imported: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FileMetadataServiceDeleted {
    pub time_deleted: u64,
    pub time_imported: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PageInformation {
    pub name: String,
    pub page_key: String,
    pub page_type: u32,
    #[serde(alias = "focused")]
    pub selected: Option<bool>,
    #[serde(default = "Vec::new")]
    pub pages: Vec<PageInformation>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OptionalStringNumber {
    String(String),
    Number(u64),
    None,
}

impl From<u64> for OptionalStringNumber {
    fn from(value: u64) -> Self {
        Self::Number(value)
    }
}

impl From<String> for OptionalStringNumber {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl OptionalStringNumber {
    pub fn string(&self) -> Option<&str> {
        if let Self::String(s) = &self {
            Some(s)
        } else {
            None
        }
    }

    pub fn number(&self) -> Option<u64> {
        if let Self::Number(n) = &self {
            Some(*n)
        } else {
            None
        }
    }
}
