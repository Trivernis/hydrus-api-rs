use crate::endpoints::access_management::{
    ApiVersion, ApiVersionResponse, GetServices, GetServicesResponse, SessionKey,
    SessionKeyResponse, VerifyAccessKey, VerifyAccessKeyResponse,
};
use crate::endpoints::adding_files::{
    AddFile, AddFileRequest, AddFileResponse, ArchiveFiles, ArchiveFilesRequest, DeleteFiles,
    DeleteFilesRequest, UnarchiveFiles, UnarchiveFilesRequest, UndeleteFiles, UndeleteFilesRequest,
};
use crate::endpoints::adding_tags::{AddTags, AddTagsRequest, CleanTags, CleanTagsResponse};
use crate::endpoints::adding_urls::{
    AddUrl, AddUrlRequest, AddUrlResponse, AssociateUrl, AssociateUrlRequest, GetUrlFiles,
    GetUrlFilesResponse, GetUrlInfo, GetUrlInfoResponse,
};
use crate::endpoints::common::{FileIdentifier, FileMetadataInfo, FileRecord};
use crate::endpoints::searching_and_fetching_files::{
    FileMetadata, FileMetadataResponse, FileSearchLocation, GetFile, SearchFiles,
    SearchFilesResponse,
};
use crate::endpoints::Endpoint;
use crate::error::{Error, Result};
use crate::utils::{number_list_to_json_array, string_list_to_json_array};
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::Serialize;

static ACCESS_KEY_HEADER: &str = "Hydrus-Client-API-Access-Key";

#[derive(Clone)]
pub struct Client {
    inner: reqwest::Client,
    base_url: String,
    access_key: String,
}

impl Client {
    /// Creates a new client to start requests against the hydrus api.
    pub fn new<S: AsRef<str>>(url: S, access_key: S) -> Result<Self> {
        Ok(Self {
            inner: reqwest::Client::new(),
            access_key: access_key.as_ref().to_string(),
            base_url: url.as_ref().to_string(),
        })
    }

    /// Starts a get request to the path
    async fn get<E: Endpoint, Q: Serialize + ?Sized>(&self, query: &Q) -> Result<Response> {
        let response = self
            .inner
            .get(format!("{}/{}", self.base_url, E::path()))
            .header(ACCESS_KEY_HEADER, &self.access_key)
            .query(query)
            .send()
            .await?;

        Self::extract_error(response).await
    }

    /// Starts a get request to the path associated with the Endpoint Type
    async fn get_and_parse<E: Endpoint, Q: Serialize + ?Sized>(
        &self,
        query: &Q,
    ) -> Result<E::Response> {
        let response = self.get::<E, Q>(query).await?;

        Self::extract_content(response).await
    }

    /// Stats a post request to the path associated with the Endpoint Type
    async fn post<E: Endpoint>(&self, body: E::Request) -> Result<Response> {
        let response = self
            .inner
            .post(format!("{}/{}", self.base_url, E::path()))
            .json(&body)
            .header(ACCESS_KEY_HEADER, &self.access_key)
            .send()
            .await?;
        let response = Self::extract_error(response).await?;
        Ok(response)
    }

    /// Stats a post request and parses the body as json
    async fn post_and_parse<E: Endpoint>(&self, body: E::Request) -> Result<E::Response> {
        let response = self.post::<E>(body).await?;

        Self::extract_content(response).await
    }

    /// Stats a post request to the path associated with the return type
    async fn post_binary<E: Endpoint>(&self, data: Vec<u8>) -> Result<E::Response> {
        let response = self
            .inner
            .post(format!("{}/{}", self.base_url, E::path()))
            .body(data)
            .header(ACCESS_KEY_HEADER, &self.access_key)
            .header("Content-Type", "application/octet-stream")
            .send()
            .await?;
        let response = Self::extract_error(response).await?;

        Self::extract_content(response).await
    }

    /// Returns an error with the response text content if the status doesn't indicate success
    async fn extract_error(response: Response) -> Result<Response> {
        if !response.status().is_success() {
            let msg = response.text().await?;
            Err(Error::Hydrus(msg))
        } else {
            Ok(response)
        }
    }

    /// Parses the response as JSOn
    async fn extract_content<T: DeserializeOwned>(response: Response) -> Result<T> {
        response.json::<T>().await.map_err(Error::from)
    }

    /// Returns the current API version. It's being incremented every time the API changes.
    pub async fn api_version(&self) -> Result<ApiVersionResponse> {
        self.get_and_parse::<ApiVersion, ()>(&()).await
    }

    /// Creates a new session key
    pub async fn session_key(&self) -> Result<SessionKeyResponse> {
        self.get_and_parse::<SessionKey, ()>(&()).await
    }

    /// Verifies if the access key is valid and returns some information about its permissions
    pub async fn verify_access_key(&self) -> Result<VerifyAccessKeyResponse> {
        self.get_and_parse::<VerifyAccessKey, ()>(&()).await
    }

    /// Returns the list of tag and file services of the client
    pub async fn get_services(&self) -> Result<GetServicesResponse> {
        self.get_and_parse::<GetServices, ()>(&()).await
    }

    /// Adds a file to hydrus
    pub async fn add_file<S: AsRef<str>>(&self, path: S) -> Result<AddFileResponse> {
        self.post_and_parse::<AddFile>(AddFileRequest {
            path: path.as_ref().to_string(),
        })
        .await
    }

    /// Adds a file from binary data to hydrus
    pub async fn add_binary_file(&self, data: Vec<u8>) -> Result<AddFileResponse> {
        self.post_binary::<AddFile>(data).await
    }

    /// Moves files with matching hashes to the trash
    pub async fn delete_files(&self, hashes: Vec<String>) -> Result<()> {
        self.post::<DeleteFiles>(DeleteFilesRequest { hashes })
            .await?;

        Ok(())
    }

    /// Pulls files out of the trash by hash
    pub async fn undelete_files(&self, hashes: Vec<String>) -> Result<()> {
        self.post::<UndeleteFiles>(UndeleteFilesRequest { hashes })
            .await?;

        Ok(())
    }

    /// Moves files from the inbox into the archive
    pub async fn archive_files(&self, hashes: Vec<String>) -> Result<()> {
        self.post::<ArchiveFiles>(ArchiveFilesRequest { hashes })
            .await?;

        Ok(())
    }

    /// Moves files from the archive into the inbox
    pub async fn unarchive_files(&self, hashes: Vec<String>) -> Result<()> {
        self.post::<UnarchiveFiles>(UnarchiveFilesRequest { hashes })
            .await?;

        Ok(())
    }

    /// Returns the list of tags as the client would see them in a human friendly order
    pub async fn clean_tags(&self, tags: Vec<String>) -> Result<CleanTagsResponse> {
        self.get_and_parse::<CleanTags, [(&str, String)]>(&[(
            "tags",
            string_list_to_json_array(tags),
        )])
        .await
    }

    /// Adds tags to files with the given hashes
    pub async fn add_tags(&self, request: AddTagsRequest) -> Result<()> {
        self.post::<AddTags>(request).await?;

        Ok(())
    }

    /// Searches for files in the inbox, the archive or both
    pub async fn search_files(
        &self,
        tags: Vec<String>,
        location: FileSearchLocation,
    ) -> Result<SearchFilesResponse> {
        self.get_and_parse::<SearchFiles, [(&str, String)]>(&[
            ("tags", string_list_to_json_array(tags)),
            ("system_inbox", location.is_inbox().to_string()),
            ("system_archive", location.is_archive().to_string()),
        ])
        .await
    }

    /// Returns the metadata for a given list of file_ids or hashes
    pub async fn get_file_metadata(
        &self,
        file_ids: Vec<u64>,
        hashes: Vec<String>,
    ) -> Result<FileMetadataResponse> {
        let query = if file_ids.len() > 0 {
            ("file_ids", number_list_to_json_array(file_ids))
        } else {
            ("hashes", string_list_to_json_array(hashes))
        };
        self.get_and_parse::<FileMetadata, [(&str, String)]>(&[query])
            .await
    }

    /// Returns the metadata for a single file identifier
    pub async fn get_file_metadata_by_identifier(
        &self,
        identifier: FileIdentifier,
    ) -> Result<FileMetadataInfo> {
        let mut response = match identifier.clone() {
            FileIdentifier::ID(id) => self.get_file_metadata(vec![id], vec![]).await?,
            FileIdentifier::Hash(hash) => self.get_file_metadata(vec![], vec![hash]).await?,
        };

        response
            .metadata
            .pop()
            .ok_or_else(|| Error::FileNotFound(identifier))
    }

    /// Returns the bytes of a file from hydrus
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
    pub async fn get_url_files<S: AsRef<str>>(&self, url: S) -> Result<GetUrlFilesResponse> {
        self.get_and_parse::<GetUrlFiles, [(&str, &str)]>(&[("url", url.as_ref())])
            .await
    }

    /// Returns information about the given url
    pub async fn get_url_info<S: AsRef<str>>(&self, url: S) -> Result<GetUrlInfoResponse> {
        self.get_and_parse::<GetUrlInfo, [(&str, &str)]>(&[("url", url.as_ref())])
            .await
    }

    /// Adds an url to hydrus, optionally with additional tags and a destination page
    pub async fn add_url(&self, request: AddUrlRequest) -> Result<AddUrlResponse> {
        self.post_and_parse::<AddUrl>(request).await
    }

    /// Associates urls with the given file hashes
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
    pub async fn disassociate_urls(&self, urls: Vec<String>, hashes: Vec<String>) -> Result<()> {
        self.post::<AssociateUrl>(AssociateUrlRequest {
            hashes,
            urls_to_add: vec![],
            urls_to_delete: urls,
        })
        .await?;

        Ok(())
    }
}
