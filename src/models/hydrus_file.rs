use crate::endpoints::common::FileIdentifier;
use crate::Client;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum FileStatus {
    ReadyForImport,
    InDatabase,
    Deleted,
    Unknown,
}

impl Eq for FileStatus {}

#[derive(Clone)]
pub struct HydrusFile {
    pub(crate) client: Client,
    pub id: FileIdentifier,
    pub status: FileStatus,
}

impl HydrusFile {
    pub(crate) fn from_raw_status_and_hash<S: ToString>(
        client: Client,
        status: u8,
        hash: S,
    ) -> Self {
        let status = if status == 3 {
            FileStatus::Deleted
        } else if status == 0 {
            FileStatus::ReadyForImport
        } else {
            FileStatus::InDatabase
        };
        Self {
            client,
            id: FileIdentifier::Hash(hash.to_string()),
            status,
        }
    }
}
