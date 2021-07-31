use crate::api_core::access_management::GetServicesResponse;
use crate::api_core::access_management::{
    SERVICE_TYPE_ALL_KNOWN_FILES, SERVICE_TYPE_ALL_KNOWN_TAGS, SERVICE_TYPE_ALL_LOCAL_FILES,
    SERVICE_TYPE_FILE_REPOSITORIES, SERVICE_TYPE_LOCAL_FILES, SERVICE_TYPE_LOCAL_TAGS,
    SERVICE_TYPE_TAG_REPOSITORIES, SERVICE_TYPE_TRASH,
};
use crate::error::Error;
use crate::Client;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

#[derive(Clone, PartialOrd, PartialEq, Hash)]
pub enum ServiceType {
    LocalTags,
    TagRepositories,
    LocalFiles,
    FileRepositories,
    AllLocalFiles,
    AllKnownFiles,
    AllKnownTags,
    Trash,
}

impl Eq for ServiceType {}

impl TryFrom<String> for ServiceType {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            s if s == SERVICE_TYPE_LOCAL_TAGS => Ok(Self::LocalTags),
            s if s == SERVICE_TYPE_TAG_REPOSITORIES => Ok(Self::TagRepositories),
            s if s == SERVICE_TYPE_LOCAL_FILES => Ok(Self::LocalFiles),
            s if s == SERVICE_TYPE_FILE_REPOSITORIES => Ok(Self::FileRepositories),
            s if s == SERVICE_TYPE_ALL_LOCAL_FILES => Ok(Self::AllLocalFiles),
            s if s == SERVICE_TYPE_ALL_KNOWN_FILES => Ok(Self::AllKnownFiles),
            s if s == SERVICE_TYPE_ALL_KNOWN_TAGS => Ok(Self::AllKnownTags),
            s if s == SERVICE_TYPE_TRASH => Ok(Self::Trash),
            _ => Err(Error::InvalidServiceType(value)),
        }
    }
}

impl ToString for ServiceType {
    fn to_string(&self) -> String {
        match self {
            ServiceType::LocalTags => String::from(SERVICE_TYPE_LOCAL_TAGS),
            ServiceType::TagRepositories => String::from(SERVICE_TYPE_TAG_REPOSITORIES),
            ServiceType::LocalFiles => String::from(SERVICE_TYPE_LOCAL_FILES),
            ServiceType::FileRepositories => String::from(SERVICE_TYPE_FILE_REPOSITORIES),
            ServiceType::AllLocalFiles => String::from(SERVICE_TYPE_ALL_LOCAL_FILES),
            ServiceType::AllKnownFiles => String::from(SERVICE_TYPE_ALL_KNOWN_FILES),
            ServiceType::AllKnownTags => String::from(SERVICE_TYPE_ALL_KNOWN_TAGS),
            ServiceType::Trash => String::from(SERVICE_TYPE_TRASH),
        }
    }
}

#[derive(Clone, PartialOrd, PartialEq, Hash)]
pub struct ServiceName(pub String);

impl Eq for ServiceName {}

impl ServiceName {
    pub fn my_tags() -> Self {
        Self(String::from("my tags"))
    }

    pub fn my_files() -> Self {
        Self(String::from("my files"))
    }

    pub fn public_tag_repository() -> Self {
        Self(String::from("public tag repository"))
    }

    pub fn all_local_files() -> Self {
        Self(String::from("all local files"))
    }

    pub fn all_known_tags() -> Self {
        Self(String::from("all known tags"))
    }

    pub fn all_known_files() -> Self {
        Self(String::from("all known files"))
    }
}

impl Display for ServiceName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone)]
pub struct Service {
    client: Client,
    pub name: ServiceName,
    pub key: String,
    pub service_type: ServiceType,
}

#[derive(Clone)]
pub struct Services {
    inner: HashMap<ServiceType, Vec<Service>>,
}

impl Services {
    /// Creates the services list from a given hydrus response
    pub fn from_response(client: Client, response: GetServicesResponse) -> Self {
        let mut response = response.0;
        let mut mapped_types = HashMap::with_capacity(response.keys().len());
        let keys = response.keys().cloned().collect::<Vec<String>>().clone();

        for service_type in &keys {
            if let Ok(mapped_type) = ServiceType::try_from(service_type.clone()) {
                let basic_services = response.remove(service_type).unwrap();
                let mut service_list = Vec::new();

                for basic_service in basic_services {
                    service_list.push(Service {
                        service_type: mapped_type.clone(),
                        name: ServiceName(basic_service.name),
                        key: basic_service.service_key,
                        client: client.clone(),
                    })
                }

                mapped_types.insert(mapped_type, service_list);
            }
        }

        Self {
            inner: mapped_types,
        }
    }

    /// Returns a list of all services of the given type
    pub fn get_services(&self, service_type: ServiceType) -> Vec<&Service> {
        if let Some(services) = self.inner.get(&service_type) {
            let mut borrowed_services = Vec::with_capacity(services.len());
            for service in services {
                borrowed_services.push(service)
            }
            borrowed_services
        } else {
            Vec::with_capacity(0)
        }
    }
}
