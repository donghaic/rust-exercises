use std::time::Duration;

pub struct Metrics {}

impl Metrics {
    pub fn new() -> Metrics {
        Metrics {}
    }

    pub fn record_get_backend_total(&self) {
        println!("record_get_backend_total")
    }

    pub fn record_get_backend_duration(&self, duration: Duration) {
        
    }

    pub fn record_key_not_found_error(&self) {
        
    }

    pub fn record_missing_key_error(&self) {
        
    }

    pub fn record_get_backend_error(&self) {
        
    }

    pub fn record_put_backend_error(&self) {
        
    }

    pub fn record_put_backend_duration(&self, duration: Duration) {
        
    }

    pub fn record_put_backend_size(&self, size: f64) {
        
    }

    pub fn record_put_total(&self) {
        
    }
}

