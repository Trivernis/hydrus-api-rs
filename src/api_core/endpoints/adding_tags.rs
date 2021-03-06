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
    pub service_names_to_tags: HashMap<String, Vec<String>>,
    pub service_keys_to_tags: HashMap<String, Vec<String>>,
    pub service_names_to_actions_to_tags: HashMap<String, HashMap<String, Vec<String>>>,
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
    service_names_to_tags: HashMap<String, Vec<String>>,
    service_keys_to_tags: HashMap<String, Vec<String>>,
    service_names_to_actions_to_tags: HashMap<String, HashMap<String, Vec<String>>>,
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
    pub fn add_tag<S: AsRef<str>>(mut self, service_id: ServiceIdentifier, tag: S) -> Self {
        let (service, relevant_mappings) = match service_id {
            ServiceIdentifier::Name(name) => (name, &mut self.service_names_to_tags),
            ServiceIdentifier::Key(key) => (key, &mut self.service_keys_to_tags),
        };
        if let Some(mappings) = relevant_mappings.get_mut(&service) {
            mappings.push(tag.as_ref().into())
        } else {
            relevant_mappings.insert(service, vec![tag.as_ref().into()]);
        }

        self
    }

    /// Adds multiple tags for a given service
    pub fn add_tags(mut self, service_id: ServiceIdentifier, mut tags: Vec<String>) -> Self {
        let (service, relevant_mappings) = match service_id {
            ServiceIdentifier::Name(name) => (name, &mut self.service_names_to_tags),
            ServiceIdentifier::Key(key) => (key, &mut self.service_keys_to_tags),
        };
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
        service_id: ServiceIdentifier,
        tag: S,
        action: TagAction,
    ) -> Self {
        let (service, relevant_mappings) = match service_id {
            ServiceIdentifier::Name(name) => (name, &mut self.service_names_to_actions_to_tags),
            ServiceIdentifier::Key(key) => (key, &mut self.service_keys_to_actions_to_tags),
        };
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
            service_names_to_tags: self.service_names_to_tags,
            service_keys_to_tags: self.service_keys_to_tags,
            service_names_to_actions_to_tags: self.service_names_to_actions_to_tags,
            service_keys_to_actions_to_tags: self.service_keys_to_actions_to_tags,
        }
    }
}
