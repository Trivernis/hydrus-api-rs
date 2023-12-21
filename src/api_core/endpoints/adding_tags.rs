use crate::api_core::common::ServiceIdentifier;
use crate::api_core::endpoints::Endpoint;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct CleanTagsResponse {
    pub tags: Vec<String>,
}

pub struct CleanTags;

impl Endpoint for CleanTags {
    type Request = ();
    type Response = CleanTagsResponse;

    fn path() -> String {
        String::from("add_tags/clean_tags")
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AddTagsRequest {
    pub hashes: Vec<String>,
    pub service_keys_to_tags: HashMap<String, Vec<String>>,
    pub service_keys_to_actions_to_tags: HashMap<String, HashMap<String, Vec<String>>>,
}

pub struct AddTags;

impl Endpoint for AddTags {
    type Request = AddTagsRequest;
    type Response = ();

    fn path() -> String {
        String::from("add_tags/add_tags")
    }
}

#[derive(Default)]
pub struct AddTagsRequestBuilder {
    hashes: Vec<String>,
    service_keys_to_tags: HashMap<String, Vec<String>>,
    service_keys_to_actions_to_tags: HashMap<String, HashMap<String, Vec<String>>>,
}

/// List of actions for a given tag
#[derive(Clone, Debug, PartialOrd, PartialEq, Hash)]
pub enum TagAction {
    /// Add to a local tag service.
    AddToLocalService,

    /// Delete from a local tag service.
    DeleteFromLocalService,

    /// Pend to a tag repository.
    PendAddToRepository,

    ///  Rescind a pend from a tag repository.
    RescindPendFromRepository,

    /// Petition from a tag repository. (This is special)
    PetitionFromRepository,

    /// Rescind a petition from a tag repository.
    RescindPetitionFromRepository,
}

impl Eq for TagAction {}

impl TagAction {
    fn into_id(self) -> u8 {
        match self {
            TagAction::AddToLocalService => 0,
            TagAction::DeleteFromLocalService => 1,
            TagAction::PendAddToRepository => 2,
            TagAction::RescindPendFromRepository => 3,
            TagAction::PetitionFromRepository => 4,
            TagAction::RescindPetitionFromRepository => 5,
        }
    }
}

impl AddTagsRequestBuilder {
    /// Adds a file hash to the request
    pub fn add_hash<S: AsRef<str>>(mut self, hash: S) -> Self {
        self.hashes.push(hash.as_ref().into());

        self
    }

    /// Adds multiple file hashes to the request
    pub fn add_hashes(mut self, mut hashes: Vec<String>) -> Self {
        self.hashes.append(&mut hashes);

        self
    }

    /// Adds a single tag for a given service
    pub fn add_tag<S: AsRef<str>>(mut self, service_key: String, tag: S) -> Self {
        let (service, relevant_mappings) = (service_key, &mut self.service_keys_to_tags);
        if let Some(mappings) = relevant_mappings.get_mut(&service) {
            mappings.push(tag.as_ref().into())
        } else {
            relevant_mappings.insert(service, vec![tag.as_ref().into()]);
        }

        self
    }

    /// Adds multiple tags for a given service
    pub fn add_tags(mut self, service_key: String, mut tags: Vec<String>) -> Self {
        let (service, relevant_mappings) = (service_key, &mut self.service_keys_to_tags);
        if let Some(mappings) = relevant_mappings.get_mut(&service) {
            mappings.append(&mut tags);
        } else {
            relevant_mappings.insert(service, tags);
        }

        self
    }

    /// Adds one tag for a given service with a defined action
    pub fn add_tag_with_action<S: AsRef<str>>(
        mut self,
        service_key: String,
        tag: S,
        action: TagAction,
    ) -> Self {
        let (service, relevant_mappings) = (service_key, &mut self.service_keys_to_actions_to_tags);
        let action_id = action.into_id();
        if let Some(actions) = relevant_mappings.get_mut(&service) {
            if let Some(tags) = actions.get_mut(&action_id.to_string()) {
                tags.push(tag.as_ref().into());
            } else {
                actions.insert(action_id.to_string(), vec![tag.as_ref().into()]);
            }
        } else {
            let mut actions = HashMap::new();
            actions.insert(action_id.to_string(), vec![tag.as_ref().into()]);
            relevant_mappings.insert(service, actions);
        }
        self
    }

    /// builds the request
    pub fn build(self) -> AddTagsRequest {
        AddTagsRequest {
            hashes: self.hashes,
            service_keys_to_tags: self.service_keys_to_tags,
            service_keys_to_actions_to_tags: self.service_keys_to_actions_to_tags,
        }
    }
}

pub struct SearchTags;

impl Endpoint for SearchTags {
    type Request = ();

    type Response = SearchTagsResponse;

    fn path() -> String {
        String::from("add_tags/search_tags")
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchTagsResponse {
    pub tags: Vec<TagWithCount>,
}

#[derive(Debug, Deserialize)]
pub struct TagWithCount {
    /// The name of the tag
    pub value: String,
    /// The count of how many times it was found in the database
    pub count: u64,
}

#[derive(Debug, Default)]
pub struct TagSearchOptions {
    /// And optional filter for the service the tags should belong to
    pub tag_service: Option<ServiceIdentifier>,
    /// Controls how the tags in the result should be displayed
    pub display_type: TagDisplayType,
}

#[derive(Debug)]
pub enum TagDisplayType {
    /// Returns tags as stored in the hydrus database
    Storage,
    /// Returns tags as displayed by hydrus
    Display,
}

impl Default for TagDisplayType {
    fn default() -> Self {
        Self::Storage
    }
}

impl TagDisplayType {
    fn to_api_string(&self) -> &'static str {
        match self {
            TagDisplayType::Storage => "storage",
            TagDisplayType::Display => "display",
        }
    }
}

impl TagSearchOptions {
    /// Sets the display type of the search result
    pub fn display_type(mut self, display_type: TagDisplayType) -> Self {
        self.display_type = display_type;
        self
    }

    /// Adds a filter for the tag service that the tags we're searching for
    /// should belong to.
    pub fn tag_service(mut self, tag_service: ServiceIdentifier) -> Self {
        self.tag_service = Some(tag_service);
        self
    }

    pub(crate) fn into_query_args(self) -> Vec<(&'static str, String)> {
        let mut args = Vec::new();

        if let Some(service) = self.tag_service {
            match service {
                ServiceIdentifier::Name(name) => args.push(("tag_service_name", name)),
                ServiceIdentifier::Key(key) => args.push(("tag_service_key", key)),
            }
        }
        args.push((
            "tag_display_type",
            self.display_type.to_api_string().to_string(),
        ));

        args
    }
}
