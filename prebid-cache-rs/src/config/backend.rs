use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum BackendType {
    #[serde(rename = "memory")]
    Memory,
    #[serde(rename = "redis")]
    Redis,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Backend {
    #[serde(rename(deserialize = "type"))]
    pub(crate) backend_type: BackendType,
    pub(crate) redis: Option<Redis>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Redis {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) password: String,
    pub(crate) db: u16,
    pub(crate) expiration: u16,
}
