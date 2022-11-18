pub enum BackendType {
    Memory,
    Redis,
}


pub struct Backend {
    backend_type: BackendType,
    redis: Redis,
}

pub struct Redis {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) password: String,
    pub(crate) db: u16,
    pub(crate) expiration: u16,
}