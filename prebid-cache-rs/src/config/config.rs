use crate::config::{Backend, BackendType};
use config::Config;
use config::ConfigError;
use config::File;
use serde::Deserialize;
use std::env;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering::SeqCst;

pub static SERVER_PORT: AtomicU16 = AtomicU16::new(0);

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct Configuration {
    pub port: u16,
    pub admin_port: u16,
    pub log: Logger,
    pub backend: Backend,
    pub compression: Compression,
    pub metrics: Metrics,
}

impl Configuration {
    pub fn load() -> Result<Self, ConfigError> {
        let profiles_raw_string = env::var("RUST_PROFILES_ACTIVE").unwrap_or_default();
        let active_profiles: Vec<&str> = profiles_raw_string
            .split(',')
            .into_iter()
            .map(|p| p.trim())
            .filter(|p| !(*p).is_empty())
            .collect();

        // Load always properties of application.yml
        let mut builder = Config::builder().add_source(File::with_name("./application.yaml"));

        // Load property files for profiles
        for profile in active_profiles {
            builder = builder.add_source(
                File::with_name(&format!("./application-{}.yml", profile)).required(false),
            );
        }

        let parsed_config: Result<Configuration, ConfigError> = builder.build()?.try_deserialize();

        // Set server port statically
        if let Ok(config) = &parsed_config {
            SERVER_PORT.store(config.port, SeqCst);
        }

        // Return config
        parsed_config
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Compression {}

#[derive(Debug, Clone, Deserialize)]
pub struct Logger {
    level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub enum MetricsType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "prometheus")]
    Prometheus,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Metrics {
    #[serde(rename = "type")]
    metrics_type: MetricsType,
}
