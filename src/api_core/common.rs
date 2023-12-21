use crate::wrapper::service::ServiceName;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicServiceInfo {
    pub name: String,
    pub service_key: String,
    #[serde(alias = "type")]
    pub service_type: u64,
    pub type_pretty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceItem {
    pub name: String,
    #[serde(alias = "type")]
    pub service_type: u64,
    pub type_pretty: String,
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
    /// Try to avoid using this variant as it will be removed from the interface
    /// in the future
    Name(String),
    /// The key variant of a service which should be the preferred variant.
    Key(String),
}

impl ServiceIdentifier {
    /// Deprecation: use [ServiceIdentifier::key] instead.
    #[deprecated(
        note = "Deprecation in the official interface was mentioned. Use the service keys instead."
    )]
    pub fn name<S: ToString>(name: S) -> Self {
        Self::Name(name.to_string())
    }

    /// Constructs a new type of the key variant.
    pub fn key<S: ToString>(key: S) -> Self {
        Self::Key(key.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicHashList {
    pub hashes: Vec<String>,
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

    pub fn as_hash(&self) -> Option<&String> {
        if let Self::Hash(h) = &self {
            Some(h)
        } else {
            None
        }
    }

    pub fn as_id(&self) -> Option<u64> {
        if let Self::ID(id) = &self {
            Some(*id)
        } else {
            None
        }
    }
}

/// A generic selection for one or multiple files
#[derive(Clone, Debug, Serialize, Default)]
pub struct FileSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) hash: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) hashes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) file_id: Option<u64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) file_ids: Vec<u64>,
}

impl From<FileIdentifier> for FileSelection {
    fn from(id: FileIdentifier) -> Self {
        let mut selection = Self::default();
        match id {
            FileIdentifier::ID(id) => selection.file_id = Some(id),
            FileIdentifier::Hash(hash) => selection.hash = Some(hash),
        }
        selection
    }
}

impl FileSelection {
    /// Creates a new single hash file selection
    pub fn by_hash<S: ToString>(hash: S) -> Self {
        Self {
            hash: Some(hash.to_string()),
            ..Default::default()
        }
    }

    /// Creates a new file selection with a single file id
    pub fn by_file_id(file_id: u64) -> Self {
        Self {
            file_id: Some(file_id),
            ..Default::default()
        }
    }

    /// Creates a new file selection with several hashes
    pub fn by_hashes(mut hashes: Vec<String>) -> Self {
        if hashes.len() == 1 {
            Self::by_hash(hashes.pop().unwrap())
        } else {
            Self {
                hashes,
                ..Default::default()
            }
        }
    }

    /// Creates a new file selection with several IDs
    pub fn by_file_ids(mut file_ids: Vec<u64>) -> Self {
        if file_ids.len() == 1 {
            Self::by_file_id(file_ids.pop().unwrap())
        } else {
            Self {
                file_ids,
                ..Default::default()
            }
        }
    }
}

/// A selection for a single file  service
#[derive(Clone, Debug, Serialize, Default)]
pub struct FileServiceSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) file_service_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) file_service_key: Option<String>,
}

impl FileServiceSelection {
    /// Creates a new file service selection by name
    pub fn by_name<S: ToString>(name: S) -> Self {
        Self {
            file_service_name: Some(name.to_string()),
            ..Default::default()
        }
    }

    /// Creates a new file  service selection by service key
    pub fn by_key<S: ToString>(key: S) -> Self {
        Self {
            file_service_key: Some(key.to_string()),
            ..Default::default()
        }
    }

    /// Selects no service
    pub fn none() -> Self {
        Self::default()
    }
}

impl From<ServiceIdentifier> for FileServiceSelection {
    fn from(id: ServiceIdentifier) -> Self {
        match id {
            ServiceIdentifier::Name(n) => Self::by_name(n),
            ServiceIdentifier::Key(k) => Self::by_key(k),
        }
    }
}

impl From<ServiceName> for FileServiceSelection {
    fn from(name: ServiceName) -> Self {
        Self::by_name(name)
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
    pub name: String,
    #[serde(alias = "type")]
    pub service_type: u64,
    pub type_pretty: String,
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
