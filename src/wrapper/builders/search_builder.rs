use crate::api_core::searching_and_fetching_files::FileSearchOptions;
use crate::error::Result;
use crate::utils::tag_list_to_string_list;
use crate::wrapper::hydrus_file::HydrusFile;
use crate::wrapper::service::ServiceName;
use crate::wrapper::tag::Tag;
use crate::Client;

pub enum SortType {
    FileSize,
    Duration,
    ImportTime,
    FileType,
    Random,
    Width,
    Height,
    Ratio,
    NumberOfPixels,
    NumberOfTags,
    NumberOfMediaViewers,
    MediaViewTime,
    Bitrate,
    HasAudio,
    ModifiedTime,
    Framerate,
    NumberOfFrames,
}

#[derive(Clone, Debug)]
pub struct SearchBuilder {
    client: Client,
    tags: Vec<Tag>,
    options: FileSearchOptions,
}

impl SearchBuilder {
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            tags: Vec::new(),
            options: FileSearchOptions::new(),
        }
    }

    /// Add multiple tags to filter by
    pub fn add_tags(mut self, mut tags: Vec<Tag>) -> Self {
        self.tags.append(&mut tags);
        self
    }

    /// Add a tag to filter by
    pub fn add_tag(mut self, tag: Tag) -> Self {
        self.tags.push(tag);
        self
    }

    /// Sets the sort type
    pub fn sort_by(mut self, sort_type: SortType) -> Self {
        self.options = self.options.sort_type(sort_type as u8);
        self
    }

    /// Sorts descending
    pub fn sort_descending(mut self) -> Self {
        self.options = self.options.desc();
        self
    }

    /// Sorts ascending
    pub fn sort_ascending(mut self) -> Self {
        self.options = self.options.asc();
        self
    }

    /// Sets the file service name to search in
    pub fn file_service_name(mut self, service: ServiceName) -> Self {
        self.options = self.options.file_service_name(service);
        self
    }

    /// Sets the tag service to search by
    pub fn tag_service_name(mut self, service: ServiceName) -> Self {
        self.options = self.options.tag_service_name(service);
        self
    }

    /// Sets the file service key. This option is preferred over
    /// setting it by name because it's faster
    pub fn file_service_key<S: ToString>(mut self, key: S) -> Self {
        self.options = self.options.file_service_key(key);
        self
    }

    /// Sets the tag service key. This option is preferred over
    /// setting it by name because it's faster
    pub fn tag_service_key<S: ToString>(mut self, key: S) -> Self {
        self.options = self.options.tag_service_key(key);
        self
    }

    /// Runs the search
    pub async fn run(self) -> Result<Vec<HydrusFile>> {
        let client = self.client.clone();
        let response = client
            .search_files(tag_list_to_string_list(self.tags), self.options)
            .await?;
        let files = response
            .file_ids
            .into_iter()
            .map(|id| HydrusFile::from_id(client.clone(), id))
            .collect();

        Ok(files)
    }
}
