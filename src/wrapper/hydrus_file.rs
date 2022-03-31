use crate::api_core::common::{
    FileIdentifier, FileMetadataInfo, FileRecord, FileSelection, FileServiceSelection,
    ServiceIdentifier,
};
use crate::api_core::endpoints::adding_tags::{AddTagsRequestBuilder, TagAction};
use crate::error::{Error, Result};
use crate::utils::tag_list_to_string_list;
use crate::wrapper::builders::delete_files_builder::DeleteFilesBuilder;
use crate::wrapper::builders::notes_builder::AddNotesBuilder;
use crate::wrapper::service::ServiceName;
use crate::wrapper::tag::Tag;
use crate::Client;
use chrono::{NaiveDateTime, TimeZone, Utc};
use mime::Mime;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum FileStatus {
    ReadyForImport,
    InDatabase,
    Deleted,
    Unknown,
}

impl Eq for FileStatus {}

impl From<u8> for FileStatus {
    fn from(v: u8) -> FileStatus {
        match v {
            3 => FileStatus::Deleted,
            0 => FileStatus::ReadyForImport,
            _ => FileStatus::InDatabase,
        }
    }
}

#[derive(Clone)]
pub struct HydrusFile {
    pub(crate) client: Client,
    pub id: FileIdentifier,
    pub status: FileStatus,
    pub(crate) metadata: Option<FileMetadataInfo>,
}

impl HydrusFile {
    pub(crate) fn from_id(client: Client, id: u64) -> Self {
        Self {
            client,
            id: FileIdentifier::ID(id),
            status: FileStatus::Unknown,
            metadata: None,
        }
    }

    pub(crate) fn from_raw_status_and_hash<S: ToString>(
        client: Client,
        status: u8,
        hash: S,
    ) -> Self {
        Self {
            client,
            id: FileIdentifier::Hash(hash.to_string()),
            status: status.into(),
            metadata: None,
        }
    }

    pub(crate) fn from_metadata(client: Client, metadata: FileMetadataInfo) -> Self {
        let status = if metadata.is_trashed {
            FileStatus::Deleted
        } else {
            FileStatus::InDatabase
        };

        Self {
            client,
            id: FileIdentifier::Hash(metadata.hash.clone()),
            status,
            metadata: Some(metadata),
        }
    }

    /// Deletes the internally stored metadata about the file retrieves it again
    pub async fn update(&mut self) -> Result<()> {
        self.metadata = None;
        self.metadata().await?;
        Ok(())
    }

    /// Returns the hash of the file
    /// if the file identifier is an id it calls hydrus to resolve the file
    pub async fn hash(&mut self) -> Result<String> {
        match &self.id {
            FileIdentifier::ID(_) => {
                let metadata = self.metadata().await?;
                Ok(metadata.hash.clone())
            }
            FileIdentifier::Hash(hash) => Ok(hash.clone()),
        }
    }

    /// Returns the file size in bytes
    pub async fn size(&mut self) -> Result<Option<u64>> {
        let metadata = self.metadata().await?;

        Ok(metadata.size.clone())
    }

    /// Returns the mime of the file
    pub async fn mime(&mut self) -> Result<Mime> {
        let metadata = self.metadata().await?;
        let mime = metadata
            .mime
            .as_str()
            .parse()
            .map_err(|_| Error::InvalidMime(metadata.mime.clone()))?;

        Ok(mime)
    }

    /// Return the file extension
    pub async fn ext(&mut self) -> Result<String> {
        let metadata = self.metadata().await?;

        Ok(metadata.ext.clone())
    }

    /// Returns the dimensions of the file in pixels
    pub async fn dimensions(&mut self) -> Result<Option<(u32, u32)>> {
        let metadata = self.metadata().await?;
        if let (Some(width), Some(height)) = (&metadata.width, &metadata.height) {
            Ok(Some((*width, *height)))
        } else {
            Ok(None)
        }
    }

    /// Returns the duration of the file in seconds if it's a video
    pub async fn duration(&mut self) -> Result<Option<u64>> {
        let metadata = self.metadata().await?;

        Ok(metadata.duration.clone())
    }

    /// Returns the number of frames of the file if it's a video
    pub async fn num_frames(&mut self) -> Result<Option<u64>> {
        let metadata = self.metadata().await?;

        Ok(metadata.num_frames.clone())
    }

    /// Returns if the file has audio
    pub async fn has_audio(&mut self) -> Result<bool> {
        let metadata = self.metadata().await?;

        Ok(metadata.has_audio.unwrap_or(false))
    }

    /// Returns if the file is currently in the inbox
    pub async fn in_inbox(&mut self) -> Result<bool> {
        let metadata = self.metadata().await?;

        Ok(metadata.is_inbox)
    }

    /// Returns if the file is stored locally
    pub async fn stored_locally(&mut self) -> Result<bool> {
        let metadata = self.metadata().await?;

        Ok(metadata.is_local)
    }

    /// Returns if the file has been moved to the trash
    pub async fn moved_to_trashed(&mut self) -> Result<bool> {
        let metadata = self.metadata().await?;

        Ok(metadata.is_trashed)
    }

    /// Returns all urls associated with the file
    pub async fn urls(&mut self) -> Result<&Vec<String>> {
        let metadata = self.metadata().await?;

        Ok(&metadata.known_urls)
    }

    /// Returns the modified time of the file
    pub async fn time_modified(&mut self) -> Result<Option<NaiveDateTime>> {
        let metadata = self.metadata().await?;
        let naive_time_modified = metadata
            .time_modified
            .map(|m| Utc.timestamp_millis(m as i64).naive_utc());

        Ok(naive_time_modified)
    }

    /// Returns the imported time of the file for a given file service key
    pub async fn time_imported<S: AsRef<str>>(
        &mut self,
        service_key: S,
    ) -> Result<Option<NaiveDateTime>> {
        let metadata = self.metadata().await?;
        let naive_time_imported = metadata
            .file_services
            .current
            .get(service_key.as_ref())
            .map(|s| s.time_imported)
            .or_else(|| {
                metadata
                    .file_services
                    .deleted
                    .get(service_key.as_ref())
                    .map(|s| s.time_imported)
            })
            .map(|millis| Utc.timestamp_millis(millis as i64).naive_utc());

        Ok(naive_time_imported)
    }

    /// Returns the time the file was deleted for a specified file service
    pub async fn time_deleted<S: AsRef<str>>(
        &mut self,
        service_key: S,
    ) -> Result<Option<NaiveDateTime>> {
        let metadata = self.metadata().await?;
        let naive_time_deleted = metadata
            .file_services
            .deleted
            .get(service_key.as_ref())
            .map(|service| service.time_deleted)
            .map(|millis| Utc.timestamp_millis(millis as i64).naive_utc());

        Ok(naive_time_deleted)
    }

    /// Creates a request builder to delete the file
    pub fn delete(&mut self) -> DeleteFilesBuilder {
        self.metadata = None;
        DeleteFilesBuilder::new(self.client.clone()).add_file(self.id.clone())
    }

    /// Undeletes the file
    pub async fn undelete(&mut self) -> Result<()> {
        let hash = self.hash().await?;
        self.metadata = None;
        self.client
            .undelete_files(FileSelection::by_hash(hash), FileServiceSelection::none())
            .await
    }

    /// Associates the file with a list of urls
    pub async fn associate_urls(&mut self, urls: Vec<String>) -> Result<()> {
        let hash = self.hash().await?;
        self.client.associate_urls(urls, vec![hash]).await
    }

    /// Disassociates the file with a list of urls
    pub async fn disassociate_urls(&mut self, urls: Vec<String>) -> Result<()> {
        let hash = self.hash().await?;
        self.client.disassociate_urls(urls, vec![hash]).await
    }

    /// Returns map mapping lists of tags to services.
    ///
    /// Deprecation: Use [HydrusFile::services_with_tags] instead.
    #[deprecated(note = "Deprecated in the official API. Use services_with_tags instead.")]
    pub async fn service_names_with_tags(&mut self) -> Result<HashMap<ServiceName, Vec<Tag>>> {
        let metadata = self.metadata().await?;
        let mut tag_mappings = HashMap::new();

        #[allow(deprecated)]
        for (service, status_tags) in &metadata.service_names_to_statuses_to_tags {
            let mut tag_list = Vec::new();

            for (_, tags) in status_tags {
                tag_list.append(&mut tags.into_iter().map(|t| t.into()).collect())
            }
            tag_mappings.insert(ServiceName(service.clone()), tag_list);
        }

        Ok(tag_mappings)
    }

    /// Returns a mapping with service ids mapped to tags
    pub async fn services_with_tags(&mut self) -> Result<HashMap<ServiceIdentifier, Vec<Tag>>> {
        let metadata = self.metadata().await?;
        let mut tag_mappings = HashMap::new();

        for (service, status_tags) in &metadata.service_keys_to_statuses_to_tags {
            let mut tag_list = Vec::new();

            for (_, tags) in status_tags {
                tag_list.append(&mut tags.into_iter().map(|t| t.into()).collect())
            }
            tag_mappings.insert(ServiceIdentifier::Key(service.clone()), tag_list);
        }

        Ok(tag_mappings)
    }

    /// Returns a list of all tags assigned to the file
    pub async fn tags(&mut self) -> Result<Vec<Tag>> {
        let mut tag_list = Vec::new();
        let tag_mappings = self.services_with_tags().await?;

        for (_, mut tags) in tag_mappings {
            tag_list.append(&mut tags);
        }

        Ok(tag_list)
    }

    /// Adds tags for a specific service to the file
    pub async fn add_tags(&mut self, service: ServiceIdentifier, tags: Vec<Tag>) -> Result<()> {
        let hash = self.hash().await?;
        let request = AddTagsRequestBuilder::default()
            .add_hash(hash)
            .add_tags(service, tag_list_to_string_list(tags))
            .build();

        self.client.add_tags(request).await
    }

    /// Allows modification of tags by using the defined tag actions
    pub async fn modify_tags(
        &mut self,
        service: ServiceIdentifier,
        action: TagAction,
        tags: Vec<Tag>,
    ) -> Result<()> {
        let hash = self.hash().await?;
        let mut reqwest = AddTagsRequestBuilder::default().add_hash(hash);

        for tag in tags {
            reqwest = reqwest.add_tag_with_action(service.clone(), tag.to_string(), action.clone());
        }

        self.client.add_tags(reqwest.build()).await
    }

    /// Creates a builder to add notes to the file
    pub fn add_notes(&self) -> AddNotesBuilder {
        AddNotesBuilder::new(self.client.clone(), self.id.clone())
    }

    /// Deletes a single note from the file
    pub async fn delete_note<S1: ToString>(&self, name: S1) -> Result<()> {
        self.client
            .delete_notes(self.id.clone(), vec![name.to_string()])
            .await
    }

    /// Deletes multiple notes from the file
    pub async fn delete_notes<I: IntoIterator<Item = S>, S: ToString>(
        &self,
        names: I,
    ) -> Result<()> {
        let names = names.into_iter().map(|n: S| n.to_string()).collect();
        self.client.delete_notes(self.id.clone(), names).await
    }

    /// Retrieves the file record bytes
    pub async fn retrieve(&self) -> Result<FileRecord> {
        self.client.get_file(self.id.clone()).await
    }

    /// Returns the metadata for the given file
    /// if there's already known metadata about the file it uses that
    async fn metadata(&mut self) -> Result<&FileMetadataInfo> {
        if self.metadata.is_none() {
            let metadata = self
                .client
                .get_file_metadata_by_identifier(self.id.clone())
                .await?;
            self.status = if metadata.is_trashed {
                FileStatus::Deleted
            } else {
                FileStatus::InDatabase
            };
            self.metadata = Some(metadata);
        }

        Ok(self.metadata.as_ref().unwrap())
    }
}
