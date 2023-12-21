use crate::api_core::common::ServiceIdentifier;
use crate::api_core::endpoints::adding_tags::{AddTagsRequestBuilder, TagAction};
use crate::error::{Error, Result};
use crate::wrapper::tag::Tag;
use crate::Client;
use std::collections::HashMap;

pub struct TaggingBuilder {
    client: Client,
    hashes: Vec<String>,
    tag_mappings: HashMap<ServiceIdentifier, HashMap<TagAction, Vec<Tag>>>,
}

impl TaggingBuilder {
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            hashes: Vec::new(),
            tag_mappings: Default::default(),
        }
    }

    /// Adds a file that should get the tags defined for this request
    pub fn add_file<S: ToString>(mut self, hash: S) -> Self {
        self.hashes.push(hash.to_string());

        self
    }

    /// Adds a single tag for a given service
    pub fn add_tag(self, service: ServiceIdentifier, action: TagAction, tag: Tag) -> Self {
        self.add_tags(service, action, vec![tag])
    }

    /// Adds tags with actions for the given service
    pub fn add_tags(
        mut self,
        service: ServiceIdentifier,
        action: TagAction,
        mut tags: Vec<Tag>,
    ) -> Self {
        let service_action_mappings =
            if let Some(service_action_mappings) = self.tag_mappings.get_mut(&service) {
                service_action_mappings
            } else {
                self.tag_mappings.insert(service.clone(), HashMap::new());
                self.tag_mappings.get_mut(&service).unwrap()
            };
        if let Some(action_tag_mappings) = service_action_mappings.get_mut(&action) {
            action_tag_mappings.append(&mut tags)
        } else {
            service_action_mappings.insert(action, tags);
        }

        self
    }

    /// Executes the request
    pub async fn run(self) -> Result<()> {
        let mut request = AddTagsRequestBuilder::default().add_hashes(self.hashes);
        for (service, action_tag_mappings) in self.tag_mappings {
            let service_key = match service {
                ServiceIdentifier::Name(n) => self
                    .client
                    .get_services()
                    .await?
                    .other
                    .values()
                    .flatten()
                    .filter(|v| *v.name == n)
                    .next()
                    .ok_or_else(|| Error::Hydrus(String::from("Service not found")))?
                    .service_key
                    .clone(),
                ServiceIdentifier::Key(k) => k,
            };
            for (action, tags) in action_tag_mappings {
                for tag in tags {
                    request = request.add_tag_with_action(
                        service_key.clone(),
                        tag.to_string(),
                        action.clone(),
                    );
                }
            }
        }

        self.client.add_tags(request.build()).await
    }
}
