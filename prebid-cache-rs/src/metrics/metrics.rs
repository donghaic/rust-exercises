use std::time::{Duration, Instant};

use axum_prom::PrometheusMetricsRegistry;
use prometheus::{Opts, Registry};
use tokio::time;
pub struct Metrics {
    registry: Registry,
}

impl Metrics {
    pub fn new(metric_handle: PrometheusMetricsRegistry) -> Metrics {
        let this = Metrics {
            registry: metric_handle.registry,
        };

        this.uptime_metric();

        this
    }

    /// Create a metric that measures the uptime from when this metric was constructed.
    fn uptime_metric(&self) {
        let uptime_opts = Opts::new(
            "uptime",
            "Total number of seconds since the server has started",
        )
        .namespace("prebid_cache");

        let uptime_metric =
            prometheus::register_int_counter_with_registry!(uptime_opts.clone(), self.registry)
                .unwrap();

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                uptime_metric.inc_by(1);
            }
        });
    }

    pub fn record_get_backend_total(&self) {
        println!("record_get_backend_total")
    }

    pub fn record_get_backend_duration(&self, duration: Duration) {}

    pub fn record_key_not_found_error(&self) {}

    pub fn record_missing_key_error(&self) {}

    pub fn record_get_backend_error(&self) {}

    pub fn record_put_backend_error(&self) {}

    pub fn record_put_backend_duration(&self, duration: Duration) {}

    pub fn record_put_backend_size(&self, size: f64) {}

    pub fn record_put_total(&self) {}
}
