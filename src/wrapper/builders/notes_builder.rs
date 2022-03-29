use crate::api_core::common::FileIdentifier;
use crate::error::Result;
use crate::Client;
use std::collections::HashMap;

/// Builder to create a request for adding notes to a given file
pub struct AddNotesBuilder {
    client: Client,
    file: FileIdentifier,
    notes: HashMap<String, String>,
}

impl AddNotesBuilder {
    /// Creates a new notes builder for the given file id
    pub fn new(client: Client, file: FileIdentifier) -> Self {
        Self {
            client,
            file,
            notes: HashMap::new(),
        }
    }

    /// Adds a single note
    pub fn add_note<S1: ToString, S2: ToString>(mut self, name: S1, note: S2) -> Self {
        self.notes.insert(name.to_string(), note.to_string());

        self
    }

    /// Adds multiple notes to the builder
    pub fn add_notes<I: IntoIterator<Item = (S1, S2)>, S1: ToString, S2: ToString>(
        mut self,
        notes: I,
    ) -> Self {
        let notes_iter = notes
            .into_iter()
            .map(|(k, v): (S1, S2)| (k.to_string(), v.to_string()));
        self.notes.extend(notes_iter);

        self
    }

    /// Adds all notes mentioned in the builder to the given file
    pub async fn run(self) -> Result<()> {
        self.client.set_notes(self.file, self.notes).await
    }
}
