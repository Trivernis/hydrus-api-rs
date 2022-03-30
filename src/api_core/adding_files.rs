use crate::api_core::common::{BasicHashList, ServiceIdentifier};
use crate::api_core::Endpoint;
use serde::Serialize;

pub static STATUS_IMPORT_SUCCESS: u8 = 1;
pub static STATUS_IMPORT_ALREADY_EXISTS: u8 = 2;
pub static STATUS_IMPORT_PREVIOUSLY_DELETED: u8 = 3;
pub static STATUS_IMPORT_FAILED: u8 = 4;
pub static STATUS_IMPORT_VETOED: u8 = 5;

#[derive(Debug, Clone, Serialize)]
pub struct AddFileRequest {
    pub path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddFileResponse {
    pub status: u8,
    pub hash: String,
    pub note: String,
}

pub struct AddFile;

impl Endpoint for AddFile {
    type Request = AddFileRequest;
    type Response = AddFileResponse;

    fn path() -> String {
        String::from("add_files/add_file")
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct DeleteFilesRequest {
    /// The files by hashes to delete
    pub hashes: Vec<String>,
    /// The files by file ids to delete
    pub file_ids: Vec<u64>,
    pub file_service_name: Option<String>,
    pub file_service_key: Option<String>,
    pub reason: Option<String>,
}

impl DeleteFilesRequest {
    pub fn new(hashes: Vec<String>, file_ids: Vec<u64>) -> Self {
        Self {
            hashes,
            file_ids,
            file_service_key: None,
            file_service_name: None,
            reason: None,
        }
    }

    /// Sets the service to delete from. If none is given it deletes
    /// from all files.
    pub fn set_service(&mut self, service: ServiceIdentifier) {
        match service {
            ServiceIdentifier::Name(name) => self.file_service_name = Some(name),
            ServiceIdentifier::Key(key) => self.file_service_key = Some(key),
        }
    }

    /// Sets the reason for deletion
    pub fn set_reason<S: ToString>(&mut self, reason: S) {
        self.reason = Some(reason.to_string());
    }
}

pub struct DeleteFiles;

impl Endpoint for DeleteFiles {
    type Request = DeleteFilesRequest;
    type Response = ();

    fn path() -> String {
        String::from("add_files/delete_files")
    }
}

pub type UndeleteFilesRequest = BasicHashList;
pub struct UndeleteFiles;

impl Endpoint for UndeleteFiles {
    type Request = UndeleteFilesRequest;
    type Response = ();

    fn path() -> String {
        String::from("add_files/undelete_files")
    }
}

pub type ArchiveFilesRequest = BasicHashList;
pub struct ArchiveFiles;

impl Endpoint for ArchiveFiles {
    type Request = ArchiveFilesRequest;
    type Response = ();

    fn path() -> String {
        String::from("add_files/archive_files")
    }
}

pub type UnarchiveFilesRequest = BasicHashList;
pub struct UnarchiveFiles;

impl Endpoint for UnarchiveFiles {
    type Request = UndeleteFilesRequest;
    type Response = ();

    fn path() -> String {
        String::from("add_files/unarchive_files")
    }
}
