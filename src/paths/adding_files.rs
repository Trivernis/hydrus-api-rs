use crate::paths::common::BasicHashList;
use crate::paths::Path;

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

impl Path for AddFileResponse {
    fn get_path() -> String {
        String::from("add_files/add_file")
    }
}

pub type DeleteFilesRequest = BasicHashList;
pub struct DeleteFilesResponse;

impl Path for DeleteFilesResponse {
    fn get_path() -> String {
        String::from("add_files/delete_files")
    }
}

pub type UndeleteFilesRequest = BasicHashList;
pub struct UndeleteFilesResponse;

impl Path for UndeleteFilesResponse {
    fn get_path() -> String {
        String::from("add_files/undelete_files")
    }
}

pub type ArchiveFilesRequest = BasicHashList;
pub struct ArchiveFilesResponse;

impl Path for ArchiveFilesResponse {
    fn get_path() -> String {
        String::from("add_files/archive_files")
    }
}

pub type UnarchiveFilesRequest = BasicHashList;
pub struct UnarchiveFilesResponse;

impl Path for UnarchiveFilesResponse {
    fn get_path() -> String {
        String::from("add_files/unarchive_files")
    }
}
