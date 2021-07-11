use crate::endpoints::common::FileIdentifier;
use crate::Client;

#[derive(Clone)]
pub struct HydrusFile {
    pub(crate) client: Client,
    pub id: FileIdentifier,
}
