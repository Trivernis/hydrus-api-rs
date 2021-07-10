#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicServiceInfo {
    pub name: String,
    pub service_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicHashList {
    pub hashes: Vec<String>,
}
