use crate::config::{Backend, BackendType};

#[derive(Debug, Clone)]
pub struct Configuration {
    port: u32,
    admin_port: u32,
    log: String,
    backend: Backend,
    compression: Compression,
    metrics: Metrics,
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            port: 0,
            admin_port: 0,
            log: "".to_string(),
            backend: Backend {
                backend_type: BackendType::Memory,
                redis: None,
            },
            compression: Compression {},
            metrics: Metrics { metrics_type: MetricsType::None },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Compression {}

#[derive(Debug, Clone)]
pub enum MetricsType {
    None,
    Influx,
    Prometheus,
}

#[derive(Debug, Clone)]
pub struct Metrics {
    metrics_type: MetricsType,

}