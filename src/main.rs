use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Read};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use csv::Writer;
use reqwest;
use serde::{Deserialize, Serialize, ser::SerializeSeq};

#[derive(Debug, Deserialize)]
struct Config {
    urls_to_check: Vec<String>,  // Fix the field name
    csv_file_path: String,
    json_file_path: String,
    ping_interval_seconds: u64,
    output_format: String,
}

async fn check_url_and_log(config: &Config) -> Result<(), Box<dyn Error>> {
    for url in &config.urls_to_check {
        match reqwest::get(url).await {
            Ok(response) => {
                let status_code = response.status();
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)?
                    .as_secs();

                match &config.output_format.to_lowercase()[..] {
                    "csv" => {
                        println!(
                            "Timestamp: {}, URL: {}, Status Code: {} (Accessible)",
                            timestamp, url, status_code
                        );

                        let mut csv_writer = Writer::from_writer(
                            OpenOptions::new()
                                .create(true)
                                .write(true)
                                .append(true)
                                .open(&config.csv_file_path)?,
                        );

                        csv_writer.write_record(&[timestamp.to_string(), status_code.to_string()])?;
                    }
                    "json" => {
                        println!(
                            "Timestamp: {}, URL: {}, Status Code: {} (Accessible)",
                            timestamp, url, status_code
                        );

                        let mut json_writer =
                            serde_json::to_writer(
                                OpenOptions::new()
                                    .create(true)
                                    .write(true)
                                    .append(true)
                                    .open(&config.json_file_path)?,
                                &(timestamp, url, status_code.as_u16()),
                            )?;
                    }
                    _ => {
                        eprintln!("Invalid output format specified in config.yaml");
                        return Err("Invalid output format".into());
                    }
                }
            }
            Err(err) => {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)?
                    .as_secs();
                eprintln!("Timestamp: {}, URL: {}, Error: {} (Not Accessible)", timestamp, url, err);
            }
        }
    }

    Ok(())
}

fn load_config() -> Result<Config, Box<dyn Error>> {
    let file = match File::open("config.yaml") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening config file: {:?}", err);
            return Err(err.into());
        }
    };

    let mut contents = String::new();
    if let Err(err) = file.take(1024).read_to_string(&mut contents) {
        eprintln!("Error reading config file: {:?}", err);
        return Err(err.into());
    }

    let config: Config = match serde_yaml::from_str(&contents) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error parsing config file: {:?}", err);
            return Err(err.into());
        }
    };

    Ok(config)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match load_config() {
        Ok(config) => config,
        Err(_) => return Ok(()), // Terminate the program if there's an error with the config file
    };

    println!(
        "URL Checker started. Checking every {} seconds for the following URLs: {:?}",
        config.ping_interval_seconds, config.urls_to_check
    );

    let ping_interval = Duration::from_secs(config.ping_interval_seconds);

    loop {
        let current_time = SystemTime::now();
        let elapsed_time = match current_time.duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs() % config.ping_interval_seconds,
            Err(err) => {
                eprintln!("Error getting duration since UNIX_EPOCH: {:?}", err);
                continue;
            }
        };

        let next_run_time =
            current_time + Duration::from_secs(config.ping_interval_seconds - elapsed_time);
        let sleep_duration = match next_run_time.duration_since(current_time) {
            Ok(duration) => duration,
            Err(err) => {
                eprintln!("Error calculating sleep duration: {:?}", err);
                continue;
            }
        };

        thread::sleep(sleep_duration);

        match tokio::runtime::Runtime::new() {
            Ok(runtime) => match runtime.block_on(check_url_and_log(&config)) {
                Ok(_) => (),
                Err(err) => eprintln!("Error checking URL and logging: {:?}", err),
            },
            Err(e) => eprintln!("Error creating Tokio runtime: {:?}", e),
        }
    }
}
