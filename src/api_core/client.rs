use crate::api_core::common::{
    FileIdentifier, FileRecord, FileSelection, FileServiceSelection, OptionalStringNumber,
};
use crate::api_core::endpoints::access_management::{
    ApiVersion, ApiVersionResponse, GetServices, GetServicesResponse, SessionKey,
    SessionKeyResponse, VerifyAccessKey, VerifyAccessKeyResponse,
};
use crate::api_core::endpoints::adding_files::{
    AddFile, AddFileRequest, AddFileResponse, ArchiveFiles, ArchiveFilesRequest, DeleteFiles,
    DeleteFilesRequest, UnarchiveFiles, UnarchiveFilesRequest, UndeleteFiles, UndeleteFilesRequest,
};
use crate::api_core::endpoints::adding_notes::{
    DeleteNotes, DeleteNotesRequest, SetNotes, SetNotesRequest,
};
use crate::api_core::endpoints::adding_tags::{
    AddTags, AddTagsRequest, CleanTags, CleanTagsResponse,
};
use crate::api_core::endpoints::adding_urls::{
    AddUrl, AddUrlRequest, AddUrlResponse, AssociateUrl, AssociateUrlRequest, GetUrlFiles,
    GetUrlFilesResponse, GetUrlInfo, GetUrlInfoResponse,
};
use crate::api_core::endpoints::client_builder::ClientBuilder;
use crate::api_core::endpoints::managing_cookies_and_http_headers::{
    GetCookies, GetCookiesResponse, SetCookies, SetCookiesRequest, SetUserAgent,
    SetUserAgentRequest,
};
use crate::api_core::endpoints::managing_pages::{
    AddFiles, AddFilesRequest, FocusPage, FocusPageRequest, GetPageInfo, GetPageInfoResponse,
    GetPages, GetPagesResponse,
};
use crate::api_core::endpoints::searching_and_fetching_files::{
    FileMetadata, FileMetadataResponse, FileMetadataType, FileSearchOptions, GetFile,
    SearchFileHashes, SearchFileHashesResponse, SearchFiles, SearchFilesResponse, SearchQueryEntry,
};
use crate::api_core::endpoints::Endpoint;
use crate::error::{Error, Result};
use bytes::Buf;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;

const ACCESS_KEY_HEADER: &str = "Hydrus-Client-API-Access-Key";
const CONTENT_TYPE_HEADER: &str = "Content-Type";
const ACCEPT_HEADER: &str = "Accept";

#[cfg(feature = "cbor")]
const CONTENT_TYPE_CBOR: &str = "application/cbor";
#[cfg(feature = "json")]
const CONTENT_TYPE_JSON: &str = "application/json";

#[derive(Clone)]
/// A low level Client for the hydrus API. It provides basic abstraction
/// over the REST api.
#[derive(Debug)]
pub struct Client {
    pub(crate) inner: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) access_key: String,
}

impl Client {
    /// Returns a builder for the client
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Creates a new client to start requests against the hydrus api.
    pub fn new<S: AsRef<str>>(url: S, access_key: S) -> Self {
        Self {
            inner: reqwest::Client::new(),
            access_key: access_key.as_ref().to_string(),
            base_url: url.as_ref().to_string(),
        }
    }
    /// Returns the current API version. It's being incremented every time the API changes.
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn api_version(&self) -> Result<ApiVersionResponse> {
        self.get_and_parse::<ApiVersion, ()>(&()).await
    }

    /// Creates a new session key
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn session_key(&self) -> Result<SessionKeyResponse> {
        self.get_and_parse::<SessionKey, ()>(&()).await
    }

    /// Verifies if the access key is valid and returns some information about its permissions
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn verify_access_key(&self) -> Result<VerifyAccessKeyResponse> {
        self.get_and_parse::<VerifyAccessKey, ()>(&()).await
    }

    /// Returns the list of tag and file services of the client
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn get_services(&self) -> Result<GetServicesResponse> {
        self.get_and_parse::<GetServices, ()>(&()).await
    }

    /// Adds a file to hydrus
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn add_file<S: ToString + Debug>(&self, path: S) -> Result<AddFileResponse> {
        let path = path.to_string();
        self.post_and_parse::<AddFile>(AddFileRequest { path })
            .await
    }

    /// Adds a file from binary data to hydrus
    #[tracing::instrument(skip(self, data), level = "debug")]
    pub async fn add_binary_file(&self, data: Vec<u8>) -> Result<AddFileResponse> {
        self.post_binary::<AddFile>(data).await
    }

    /// Moves files with matching hashes to the trash
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn delete_files(
        &self,
        files: FileSelection,
        service: FileServiceSelection,
        reason: Option<String>,
    ) -> Result<()> {
        self.post::<DeleteFiles>(DeleteFilesRequest {
            file_selection: files,
            service_selection: service,
            reason,
        })
        .await?;

        Ok(())
    }

    /// Pulls files out of the trash by hash
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn undelete_files(
        &self,
        files: FileSelection,
        service: FileServiceSelection,
    ) -> Result<()> {
        self.post::<UndeleteFiles>(UndeleteFilesRequest {
            file_selection: files,
            service_selection: service,
        })
        .await?;

        Ok(())
    }

    /// Moves files from the inbox into the archive
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn archive_files(
        &self,
        files: FileSelection,
        service: FileServiceSelection,
    ) -> Result<()> {
        self.post::<ArchiveFiles>(ArchiveFilesRequest {
            file_selection: files,
            service_selection: service,
        })
        .await?;

        Ok(())
    }

    /// Moves files from the archive into the inbox
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn unarchive_files(
        &self,
        files: FileSelection,
        service: FileServiceSelection,
    ) -> Result<()> {
        self.post::<UnarchiveFiles>(UnarchiveFilesRequest {
            file_selection: files,
            service_selection: service,
        })
        .await?;

        Ok(())
    }

    /// Returns the list of tags as the client would see them in a human friendly order
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn clean_tags(&self, tags: Vec<String>) -> Result<CleanTagsResponse> {
        self.get_and_parse::<CleanTags, [(&str, String)]>(&[(
            "tags",
            Self::serialize_query_object(tags)?,
        )])
        .await
    }

    /// Adds tags to files with the given hashes
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn add_tags(&self, request: AddTagsRequest) -> Result<()> {
        self.post::<AddTags>(request).await?;

        Ok(())
    }

    /// Searches for files
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn search_files(
        &self,
        query: Vec<SearchQueryEntry>,
        options: FileSearchOptions,
    ) -> Result<SearchFilesResponse> {
        let mut args = options.into_query_args();
        args.push(("tags", Self::serialize_query_object(query)?));
        self.get_and_parse::<SearchFiles, [(&str, String)]>(&args)
            .await
    }

    /// Searches for file hashes
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn search_file_hashes(
        &self,
        query: Vec<SearchQueryEntry>,
        options: FileSearchOptions,
    ) -> Result<SearchFileHashesResponse> {
        let mut args = options.into_query_args();
        args.push(("tags", Self::serialize_query_object(query)?));
        args.push(("return_hashes", Self::serialize_query_object(true)?));
        self.get_and_parse::<SearchFileHashes, [(&str, String)]>(&args)
            .await
    }

    /// Returns the metadata for a given list of file_ids or hashes
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn get_file_metadata<M: FileMetadataType>(
        &self,
        file_ids: Vec<u64>,
        hashes: Vec<String>,
    ) -> Result<FileMetadataResponse<M>> {
        let id_query = if file_ids.len() > 0 {
            ("file_ids", Self::serialize_query_object(file_ids)?)
        } else {
            ("hashes", Self::serialize_query_object(hashes)?)
        };
        let query = [
            id_query,
            (
                "only_return_identifiers",
                Self::serialize_query_object(M::only_identifiers())?,
            ),
            (
                "only_return_basic_information",
                Self::serialize_query_object(M::only_basic_information())?,
            ),
        ];
        self.get_and_parse::<FileMetadata<M>, [(&str, String)]>(&query)
            .await
    }

    /// Returns the metadata for a single file identifier
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn get_file_metadata_by_identifier<M: FileMetadataType>(
        &self,
        id: FileIdentifier,
    ) -> Result<M::Response> {
        let mut response = match id.clone() {
            FileIdentifier::ID(id) => self.get_file_metadata::<M>(vec![id], vec![]).await?,
            FileIdentifier::Hash(hash) => self.get_file_metadata::<M>(vec![], vec![hash]).await?,
        };

        response
            .metadata
            .pop()
            .ok_or_else(|| Error::FileNotFound(id))
    }

    /// Returns the bytes of a file from hydrus
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn get_file(&self, id: FileIdentifier) -> Result<FileRecord> {
        let response = match id {
            FileIdentifier::ID(id) => {
                self.get::<GetFile, [(&str, u64)]>(&[("file_id", id)])
                    .await?
            }
            FileIdentifier::Hash(hash) => {
                self.get::<GetFile, [(&str, String)]>(&[("hash", hash)])
                    .await?
            }
        };
        let mime_type = response
            .headers()
            .get("mime-type")
            .cloned()
            .map(|h| h.to_str().unwrap().to_string())
            .unwrap_or("image/jpeg".into());

        let bytes = response.bytes().await?.to_vec();

        Ok(FileRecord { bytes, mime_type })
    }

    /// Returns all files associated with the given url
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn get_url_files<S: AsRef<str> + Debug>(
        &self,
        url: S,
    ) -> Result<GetUrlFilesResponse> {
        self.get_and_parse::<GetUrlFiles, [(&str, &str)]>(&[("url", url.as_ref())])
            .await
    }

    /// Returns information about the given url
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn get_url_info<S: AsRef<str> + Debug>(&self, url: S) -> Result<GetUrlInfoResponse> {
        self.get_and_parse::<GetUrlInfo, [(&str, &str)]>(&[("url", url.as_ref())])
            .await
    }

    /// Adds an url to hydrus, optionally with additional tags and a destination page
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn add_url(&self, request: AddUrlRequest) -> Result<AddUrlResponse> {
        self.post_and_parse::<AddUrl>(request).await
    }

    /// Associates urls with the given file hashes
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn associate_urls(&self, urls: Vec<String>, hashes: Vec<String>) -> Result<()> {
        self.post::<AssociateUrl>(AssociateUrlRequest {
            hashes,
            urls_to_add: urls,
            urls_to_delete: vec![],
        })
        .await?;

        Ok(())
    }

    /// Disassociates urls with the given file hashes
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn disassociate_urls(&self, urls: Vec<String>, hashes: Vec<String>) -> Result<()> {
        self.post::<AssociateUrl>(AssociateUrlRequest {
            hashes,
            urls_to_add: vec![],
            urls_to_delete: urls,
        })
        .await?;

        Ok(())
    }

    /// Sets the notes for the file
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn set_notes(
        &self,
        id: FileIdentifier,
        notes: HashMap<String, String>,
    ) -> Result<()> {
        self.post::<SetNotes>(SetNotesRequest::new(id, notes))
            .await?;

        Ok(())
    }

    /// Deletes the notes of a file
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn delete_notes(&self, id: FileIdentifier, note_names: Vec<String>) -> Result<()> {
        self.post::<DeleteNotes>(DeleteNotesRequest::new(id, note_names))
            .await?;

        Ok(())
    }

    /// Returns all pages of the client
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn get_pages(&self) -> Result<GetPagesResponse> {
        self.get_and_parse::<GetPages, ()>(&()).await
    }

    /// Returns information about a single page
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn get_page_info<S: AsRef<str> + Debug>(
        &self,
        page_key: S,
    ) -> Result<GetPageInfoResponse> {
        self.get_and_parse::<GetPageInfo, [(&str, &str)]>(&[("page_key", page_key.as_ref())])
            .await
    }

    /// Focuses a page in the client
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn focus_page<S: ToString + Debug>(&self, page_key: S) -> Result<()> {
        let page_key = page_key.to_string();
        self.post::<FocusPage>(FocusPageRequest { page_key })
            .await?;

        Ok(())
    }

    /// Adds files to a page
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn add_files_to_page<S: ToString + Debug>(
        &self,
        page_key: S,
        file_ids: Vec<u64>,
        hashes: Vec<String>,
    ) -> Result<()> {
        let page_key = page_key.to_string();
        self.post::<AddFiles>(AddFilesRequest {
            page_key,
            file_ids,
            hashes,
        })
        .await?;

        Ok(())
    }

    /// Returns all cookies for the given domain
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn get_cookies<S: AsRef<str> + Debug>(
        &self,
        domain: S,
    ) -> Result<GetCookiesResponse> {
        self.get_and_parse::<GetCookies, [(&str, &str)]>(&[("domain", domain.as_ref())])
            .await
    }

    /// Sets some cookies for some websites.
    /// Each entry needs to be in the format `[<name>, <value>, <domain>, <path>, <expires>]`
    /// with the types `[String, String, String, String, u64]`
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn set_cookies(&self, cookies: Vec<[OptionalStringNumber; 5]>) -> Result<()> {
        self.post::<SetCookies>(SetCookiesRequest { cookies })
            .await?;

        Ok(())
    }

    /// Sets the user agent that is being used for every request hydrus starts
    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn set_user_agent<S: ToString + Debug>(&self, user_agent: S) -> Result<()> {
        let user_agent = user_agent.to_string();
        self.post::<SetUserAgent>(SetUserAgentRequest { user_agent })
            .await?;

        Ok(())
    }

    /// Starts a get request to the path
    #[tracing::instrument(skip(self), level = "trace")]
    async fn get<E: Endpoint, Q: Serialize + Debug + ?Sized>(&self, query: &Q) -> Result<Response> {
        tracing::trace!("GET request to {}", E::path());
        #[cfg(feature = "json")]
        let content_type = CONTENT_TYPE_JSON;
        #[cfg(feature = "cbor")]
        let content_type = CONTENT_TYPE_CBOR;
        #[cfg(feature = "json")]
        let params: [(&str, &str); 0] = [];
        #[cfg(feature = "cbor")]
        let params = [("cbor", true)];

        let response = self
            .inner
            .get(format!("{}/{}", self.base_url, E::path()))
            .header(ACCESS_KEY_HEADER, &self.access_key)
            .header(CONTENT_TYPE_HEADER, content_type)
            .header(ACCEPT_HEADER, content_type)
            .query(query)
            .query(&params)
            .send()
            .await?;

        Self::extract_error(response).await
    }

    /// Starts a get request to the path associated with the Endpoint Type
    #[tracing::instrument(skip(self), level = "trace")]
    async fn get_and_parse<E: Endpoint, Q: Serialize + Debug + ?Sized>(
        &self,
        query: &Q,
    ) -> Result<E::Response> {
        let response = self.get::<E, Q>(query).await?;

        Self::extract_content(response).await
    }

    /// Serializes a given object into a json or cbor query object
    #[tracing::instrument(skip(obj), level = "trace")]
    fn serialize_query_object<S: Serialize>(obj: S) -> Result<String> {
        #[cfg(feature = "json")]
        {
            tracing::trace!("Serializing query to JSON");
            serde_json::ser::to_string(&obj).map_err(|e| Error::Serialization(e.to_string()))
        }

        #[cfg(feature = "cbor")]
        {
            tracing::trace!("Serializing query to CBOR");
            let mut buf = Vec::new();
            ciborium::ser::into_writer(&obj, &mut buf)
                .map_err(|e| Error::Serialization(e.to_string()))?;
            Ok(base64::encode(buf))
        }
    }

    /// Stats a post request to the path associated with the Endpoint Type
    #[tracing::instrument(skip(self), level = "trace")]
    async fn post<E: Endpoint>(&self, body: E::Request) -> Result<Response> {
        tracing::trace!("POST request to {}", E::path());
        let body = Self::serialize_body(body)?;

        #[cfg(feature = "cbor")]
        let content_type = CONTENT_TYPE_CBOR;
        #[cfg(feature = "json")]
        let content_type = CONTENT_TYPE_JSON;

        let response = self
            .inner
            .post(format!("{}/{}", self.base_url, E::path()))
            .body(body)
            .header(ACCESS_KEY_HEADER, &self.access_key)
            .header(CONTENT_TYPE_HEADER, content_type)
            .header(ACCEPT_HEADER, content_type)
            .send()
            .await?;
        let response = Self::extract_error(response).await?;
        Ok(response)
    }

    /// Serializes a body into either CBOR or JSON
    #[tracing::instrument(skip(body), level = "trace")]
    fn serialize_body<S: Serialize>(body: S) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        #[cfg(feature = "json")]
        {
            tracing::trace!("Serializing body to JSON");
            serde_json::to_writer(&mut buf, &body)
                .map_err(|e| Error::Serialization(e.to_string()))?;
        }
        #[cfg(feature = "cbor")]
        {
            tracing::trace!("Serializing body to CBOR");
            ciborium::ser::into_writer(&body, &mut buf)
                .map_err(|e| Error::Serialization(e.to_string()))?;
        }

        Ok(buf)
    }

    /// Stats a post request and parses the body as json
    #[tracing::instrument(skip(self), level = "trace")]
    async fn post_and_parse<E: Endpoint>(&self, body: E::Request) -> Result<E::Response> {
        let response = self.post::<E>(body).await?;

        Self::extract_content(response).await
    }

    /// Stats a post request to the path associated with the return type
    /// This currently only supports JSON because of a limitation of the
    /// hydrus client api.
    #[tracing::instrument(skip(self, data), level = "trace")]
    async fn post_binary<E: Endpoint>(&self, data: Vec<u8>) -> Result<E::Response> {
        tracing::trace!("Binary POST request to {}", E::path());

        #[cfg(feature = "cbor")]
        let content_type = CONTENT_TYPE_CBOR;
        #[cfg(feature = "json")]
        let content_type = CONTENT_TYPE_JSON;

        let response = self
            .inner
            .post(format!("{}/{}", self.base_url, E::path()))
            .body(data)
            .header(ACCESS_KEY_HEADER, &self.access_key)
            .header(CONTENT_TYPE_HEADER, "application/octet-stream")
            .header(ACCEPT_HEADER, content_type)
            .send()
            .await?;
        let response = Self::extract_error(response).await?;

        Self::extract_content(response).await
    }

    /// Returns an error with the response text content if the status doesn't indicate success
    #[tracing::instrument(level = "trace")]
    async fn extract_error(response: Response) -> Result<Response> {
        if !response.status().is_success() {
            let msg = response.text().await?;
            tracing::error!("API returned error '{}'", msg);
            Err(Error::Hydrus(msg))
        } else {
            Ok(response)
        }
    }

    /// Parses the response as JSOn
    #[tracing::instrument(level = "trace")]
    async fn extract_content<T: DeserializeOwned + Debug>(response: Response) -> Result<T> {
        let bytes = response.bytes().await?;
        let reader = bytes.reader();
        #[cfg(feature = "json")]
        let content = {
            tracing::trace!("Deserializing content from JSON");
            serde_json::from_reader::<_, T>(reader)
                .map_err(|e| Error::Deserialization(e.to_string()))?
        };
        #[cfg(feature = "cbor")]
        let content = {
            tracing::trace!("Deserializing content from CBOR");
            ciborium::de::from_reader(reader).map_err(|e| Error::Deserialization(e.to_string()))?
        };
        tracing::trace!("response content: {:?}", content);

        Ok(content)
    }
}
