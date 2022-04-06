use crate::api_core::common::{
    FileIdentifier, FileSelection, FileServiceSelection, ServiceIdentifier,
};
use crate::error::Result;
use crate::Client;

pub struct DeleteFilesBuilder {
    client: Client,
    hashes: Vec<String>,
    ids: Vec<u64>,
    reason: Option<String>,
    service: Option<ServiceIdentifier>,
}

impl DeleteFilesBuilder {
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            hashes: Vec::new(),
            ids: Vec::new(),
            reason: None,
            service: None,
        }
    }

    /// Adds a file to be deleted
    pub fn add_file(mut self, identifier: FileIdentifier) -> Self {
        match identifier {
            FileIdentifier::ID(id) => self.ids.push(id),
            FileIdentifier::Hash(hash) => self.hashes.push(hash),
        }

        self
    }

    /// Adds multiple files to be deleted
    pub fn add_files(self, ids: Vec<FileIdentifier>) -> Self {
        ids.into_iter().fold(self, |acc, id| acc.add_file(id))
    }

    /// Restricts deletion to a single file service
    pub fn service(mut self, service: ServiceIdentifier) -> Self {
        self.service = Some(service);

        self
    }

    /// Adds a reason for why the file was deleted
    pub fn reason<S: ToString>(mut self, reason: S) -> Self {
        self.reason = Some(reason.to_string());

        self
    }

    /// Deletes all files specified in this builder
    pub async fn run(self) -> Result<()> {
        let file_selection = FileSelection {
            hashes: self.hashes,
            file_ids: self.ids,
            ..Default::default()
        };
        let service_selection = self
            .service
            .map(FileServiceSelection::from)
            .unwrap_or_default();

        self.client
            .delete_files(file_selection, service_selection, self.reason)
            .await
    }
}
