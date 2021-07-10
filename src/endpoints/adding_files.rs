use crate::endpoints::common::BasicHashList;
use crate::endpoints::Endpoint;

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

    fn get_path() -> String {
        String::from("add_files/add_file")
    }
}

pub type DeleteFilesRequest = BasicHashList;

pub struct DeleteFiles;

impl Endpoint for DeleteFiles {
    type Request = DeleteFilesRequest;
    type Response = ();

    fn get_path() -> String {
        String::from("add_files/delete_files")
    }
}

pub type UndeleteFilesRequest = BasicHashList;
pub struct UndeleteFiles;

impl Endpoint for UndeleteFiles {
    type Request = UndeleteFilesRequest;
    type Response = ();

    fn get_path() -> String {
        String::from("add_files/undelete_files")
    }
}

pub type ArchiveFilesRequest = BasicHashList;
pub struct ArchiveFiles;

impl Endpoint for ArchiveFiles {
    type Request = ArchiveFilesRequest;
    type Response = ();

    fn get_path() -> String {
        String::from("add_files/archive_files")
    }
}

pub type UnarchiveFilesRequest = BasicHashList;
pub struct UnarchiveFiles;

impl Endpoint for UnarchiveFiles {
    type Request = UndeleteFilesRequest;
    type Response = ();

    fn get_path() -> String {
        String::from("add_files/unarchive_files")
    }
}
