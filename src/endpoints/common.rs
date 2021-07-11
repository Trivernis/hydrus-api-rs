use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicServiceInfo {
    pub name: String,
    pub service_key: String,
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
    pub has_audio: Option<bool>,
    pub num_frames: Option<u16>,
    pub num_words: Option<u64>,
    pub is_inbox: bool,
    pub is_local: bool,
    pub is_trashed: bool,
    pub known_urls: Vec<String>,
    pub service_names_to_statuses_to_tags: HashMap<String, HashMap<String, Vec<String>>>,
    pub service_names_to_statuses_to_display_tags: HashMap<String, HashMap<String, Vec<String>>>,
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
