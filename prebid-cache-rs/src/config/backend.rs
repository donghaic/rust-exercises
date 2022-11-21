#[derive(Debug, Clone)]
pub enum BackendType {
    Memory,
    Redis,
}

#[derive(Debug, Clone)]
pub struct Backend {
    pub(crate) backend_type: BackendType,
    pub(crate) redis: Option<Redis>,
}

#[derive(Debug, Clone)]
pub struct Redis {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) password: String,
    pub(crate) db: u16,
    pub(crate) expiration: u16,
}