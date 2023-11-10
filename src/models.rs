use serde::{Serialize, Deserialize};

// Struct representing the status of a URL
#[derive(Debug, Serialize)]
pub struct Status {
    pub url: String,
    pub status: String,
    pub http_status_code: u16,
    pub last_ping_time: u64,
}

// Struct representing the configuration loaded from the YAML file
#[derive(Debug, Deserialize)]
pub struct Config {
    pub urls_to_check: Vec<String>,
    pub csv_file_path: String,
    pub json_file_path: String,
    pub ping_interval_seconds: u64,
    pub output_format: String,
}
