use std::error::Error;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use reqwest;
use csv::Writer;

const URL_TO_CHECK: &str = "https://example.com";
const CSV_FILE_PATH: &str = "log.csv";
const PING_INTERVAL_SECONDS: u64 = 900; // Change this value to set the interval in seconds

async fn check_url_and_log() -> Result<(), Box<dyn Error>> {
    match reqwest::get(URL_TO_CHECK).await {
        Ok(response) => {
            let status_code = response.status();
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)?
                .as_secs();

            println!(
                "Timestamp: {}, URL: {}, Status Code: {} (Accessible)",
                timestamp, URL_TO_CHECK, status_code
            );

            let mut csv_writer = Writer::from_writer(
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open(CSV_FILE_PATH)?,
            );

            csv_writer.write_record(&[timestamp.to_string(), status_code.to_string()])?;
        }
        Err(err) => {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)?
                .as_secs();

            eprintln!(
                "Timestamp: {}, URL: {}, Error: {} (Not Accessible)",
                timestamp, URL_TO_CHECK, err
            );
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("URL Checker started. Checking every {} seconds.", PING_INTERVAL_SECONDS);

    let ping_interval = Duration::from_secs(PING_INTERVAL_SECONDS);

    loop {
        let current_time = SystemTime::now();
        let elapsed_time = match current_time.duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs() % PING_INTERVAL_SECONDS,
            Err(err) => {
                eprintln!("Error getting duration since UNIX_EPOCH: {:?}", err);
                continue;
            }
        };

        let next_run_time = current_time + Duration::from_secs(PING_INTERVAL_SECONDS - elapsed_time);
        let sleep_duration = match next_run_time.duration_since(current_time) {
            Ok(duration) => duration,
            Err(err) => {
                eprintln!("Error calculating sleep duration: {:?}", err);
                continue;
            }
        };

        thread::sleep(sleep_duration);

        match tokio::runtime::Runtime::new() {
            Ok(runtime) => match runtime.block_on(check_url_and_log()) {
                Ok(_) => (),
                Err(err) => eprintln!("Error checking URL and logging: {:?}", err),
            },
            Err(e) => eprintln!("Error creating Tokio runtime: {:?}", e),
        }
    }
}
