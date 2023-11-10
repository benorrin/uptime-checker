use csv::Writer;
use std::error::Error;
use std::fs::OpenOptions;

use crate::models::{Config, Status};

// Log statuses to a CSV file
pub fn log_statuses_to_csv(config: &Config, statuses: &[Status]) -> Result<(), Box<dyn Error>> {
    // Log that CSV logging is starting
    log::info!("Logging statuses to CSV...");

    // Open the CSV file for writing
    let mut csv_writer = Writer::from_writer(
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&config.csv_file_path)?,
    );

    // Iterate over each status and write it to the CSV file
    for status in statuses {
        csv_writer.serialize(status)?; // Serialize the entire status struct directly
    }

    // Log that CSV logging has completed
    log::info!("CSV logging completed successfully.");

    // Return Ok if the operation was successful
    Ok(())
}

/// Log statuses to a JSON file
pub fn log_statuses_to_json(
    config: &Config,
    statuses: &[Status],
) -> Result<(), Box<dyn Error>> {
    // Log that JSON logging is starting
    log::info!("Logging statuses to JSON...");

    // Open the JSON file for writing
    let json_writer = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&config.json_file_path)?;

    // Serialize statuses to JSON and write to the file
    serde_json::to_writer_pretty(json_writer, &statuses)?;

    // Log that JSON logging has completed
    log::info!("JSON logging completed successfully.");

    // Return Ok if the operation was successful
    Ok(())
}