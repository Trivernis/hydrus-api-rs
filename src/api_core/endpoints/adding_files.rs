use crate::api_core::common::{FileSelection, FileServiceSelection};
use crate::api_core::endpoints::Endpoint;
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
    #[serde(flatten)]
    pub file_selection: FileSelection,
    #[serde(flatten)]
    pub service_selection: FileServiceSelection,
    pub reason: Option<String>,
}

pub struct DeleteFiles;

impl Endpoint for DeleteFiles {
    type Request = DeleteFilesRequest;
    type Response = ();

    fn path() -> String {
        String::from("add_files/delete_files")
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct UndeleteFilesRequest {
    #[serde(flatten)]
    pub file_selection: FileSelection,
    #[serde(flatten)]
    pub service_selection: FileServiceSelection,
}

pub struct UndeleteFiles;

impl Endpoint for UndeleteFiles {
    type Request = UndeleteFilesRequest;
    type Response = ();

    fn path() -> String {
        String::from("add_files/undelete_files")
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ArchiveFilesRequest {
    #[serde(flatten)]
    pub file_selection: FileSelection,
    #[serde(flatten)]
    pub service_selection: FileServiceSelection,
}

pub struct ArchiveFiles;

impl Endpoint for ArchiveFiles {
    type Request = ArchiveFilesRequest;
    type Response = ();

    fn path() -> String {
        String::from("add_files/archive_files")
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct UnarchiveFilesRequest {
    #[serde(flatten)]
    pub file_selection: FileSelection,
    #[serde(flatten)]
    pub service_selection: FileServiceSelection,
}

pub struct UnarchiveFiles;

impl Endpoint for UnarchiveFiles {
    type Request = UnarchiveFilesRequest;
    type Response = ();

    fn path() -> String {
        String::from("add_files/unarchive_files")
    }
}
