use crate::api_core::common::FileIdentifier;
use crate::api_core::endpoints::Endpoint;
use std::collections::HashMap;

pub struct SetNotes;

impl Endpoint for SetNotes {
    type Request = SetNotesRequest;
    type Response = ();

    fn path() -> String {
        String::from("add_notes/set_notes")
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct SetNotesRequest {
    notes: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_id: Option<u64>,
}

impl SetNotesRequest {
    pub fn new(id: FileIdentifier, notes: HashMap<String, String>) -> Self {
        let mut request = Self {
            notes,
            ..Default::default()
        };
        match id {
            FileIdentifier::ID(id) => request.file_id = Some(id),
            FileIdentifier::Hash(hash) => request.hash = Some(hash),
        }

        request
    }
}

pub struct DeleteNotes;

impl Endpoint for DeleteNotes {
    type Request = DeleteNotesRequest;
    type Response = ();

    fn path() -> String {
        String::from("add_notes/delete_notes")
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct DeleteNotesRequest {
    note_names: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_id: Option<u64>,
}

impl DeleteNotesRequest {
    pub fn new(id: FileIdentifier, note_names: Vec<String>) -> Self {
        let mut request = Self {
            note_names,
            ..Default::default()
        };
        match id {
            FileIdentifier::ID(id) => request.file_id = Some(id),
            FileIdentifier::Hash(hash) => request.hash = Some(hash),
        }

        request
    }
}
