use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::logger;
use crate::models::{Config, Status};

pub async fn check_url_and_log(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut statuses = Vec::new();

    for url in &config.urls_to_check {
        match reqwest::get(url).await {
            Ok(response) => {
                let status_code = response.status();
                let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

                let status = determine_status(status_code);
                statuses.push(Status {
                    url: url.clone(),
                    status,
                    http_status_code: status_code.as_u16(),
                    last_ping_time: timestamp,
                });
            }
            Err(err) => {
                let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                eprintln!(
                    "Timestamp: {}, URL: {}, Error: {} (Not Accessible)",
                    timestamp, url, err
                );
            }
        }
    }

    match &config.output_format.to_lowercase()[..] {
        "csv" => logger::log_statuses_to_csv(config, &statuses)?,
        "json" => logger::log_statuses_to_json(&statuses, &config.json_file_path)?,
        _ => {
            eprintln!("Invalid output format specified in config.yaml");
            return Err("Invalid output format".into());
        }
    }

    Ok(())
}

fn determine_status(status_code: reqwest::StatusCode) -> String {
    if status_code.is_success() {
        String::from("Online")
    } else {
        String::from("Offline")
    }
}

pub fn get_current_time() -> SystemTime {
    SystemTime::now()
}

pub fn get_elapsed_time(current_time: SystemTime) -> Result<u64, Box<dyn std::error::Error>> {
    let elapsed_time = current_time.duration_since(UNIX_EPOCH)?;
    Ok(elapsed_time.as_secs())
}

pub fn calculate_next_run_time(
    current_time: SystemTime,
    ping_interval_seconds: u64,
    elapsed_time: u64,
) -> Result<SystemTime, Box<dyn std::error::Error>> {
    let next_run_time = current_time
        + Duration::from_secs(ping_interval_seconds - elapsed_time % ping_interval_seconds);
    Ok(next_run_time)
}

pub fn calculate_sleep_duration(
    current_time: SystemTime,
    next_run_time: SystemTime,
) -> Result<Duration, Box<dyn std::error::Error>> {
    let sleep_duration = next_run_time.duration_since(current_time)?;
    Ok(sleep_duration)
}
