use crate::api_core::common::FileMetadataInfo;
use crate::api_core::Endpoint;

#[derive(Debug, Clone, Deserialize)]
pub struct SearchFilesResponse {
    pub file_ids: Vec<u64>,
}

pub enum FileSearchLocation {
    Inbox,
    Archive,
}

impl FileSearchLocation {
    pub fn is_inbox(&self) -> bool {
        if let &Self::Inbox = &self {
            true
        } else {
            false
        }
    }

    pub fn is_archive(&self) -> bool {
        if let &Self::Archive = &self {
            true
        } else {
            false
        }
    }
}

pub struct SearchFiles;

impl Endpoint for SearchFiles {
    type Request = ();
    type Response = SearchFilesResponse;

    fn path() -> String {
        String::from("get_files/search_files")
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct FileMetadataResponse {
    pub metadata: Vec<FileMetadataInfo>,
}

pub struct FileMetadata;

impl Endpoint for FileMetadata {
    type Request = ();
    type Response = FileMetadataResponse;

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