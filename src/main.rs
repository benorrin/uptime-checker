use std::error::Error;
use std::fs::OpenOptions;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use csv::Writer;

const URL_TO_CHECK: &str = "https://example.com";
const CSV_FILE_PATH: &str = "log.csv";

async fn check_url_and_log() -> Result<(), Box<dyn Error>> {
    match reqwest::get(URL_TO_CHECK).await {
        Ok(response) => {
            let status_code = response.status();
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

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
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

            eprintln!(
                "Timestamp: {}, URL: {}, Error: {} (Not Accessible)",
                timestamp, URL_TO_CHECK, err
            );
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let current_time = SystemTime::now();
        let elapsed_time = match current_time.duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs() % 900,
            Err(err) => {
                eprintln!("Error getting duration since UNIX_EPOCH: {:?}", err);
                continue;
            }
        };

        let next_run_time = current_time + Duration::from_secs(900 - elapsed_time);
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

