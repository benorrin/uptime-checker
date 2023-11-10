use std::thread;
use std::time::Duration;

mod config_loader;
mod logger;
mod models;
mod url_checker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    // Load the configuration from the config file
    let config = match config_loader::load_config() {
        Ok(config) => config,
        Err(_) => return Ok(()), // Terminate the program if there's an error with the config file
    };

    // Print a startup message with the configured parameters
    println!(
        "URL Checker started. Checking every {} seconds for the following URLs: {:?}",
        config.ping_interval_seconds, config.urls_to_check
    );

    // Log the startup message
    log::info!(
        "URL Checker started. Checking every {} seconds for the following URLs: {:?}",
        config.ping_interval_seconds, config.urls_to_check
    );

    // Create a Duration representing the ping interval
    let _ping_interval = Duration::from_secs(config.ping_interval_seconds);

    // Enter the main loop for periodic URL checking
    loop {
        // Get the current time
        let current_time = url_checker::get_current_time();

        // Calculate the time elapsed since the last run
        let elapsed_time = url_checker::get_elapsed_time(current_time)?;

        // Calculate the next run time based on the ping interval and elapsed time
        let next_run_time = url_checker::calculate_next_run_time(
            current_time,
            config.ping_interval_seconds,
            elapsed_time,
        )?;

        // Calculate the duration to sleep until the next run
        let sleep_duration = url_checker::calculate_sleep_duration(current_time, next_run_time)?;

        // Sleep until the next run time
        thread::sleep(sleep_duration);

        // Use Tokio runtime to asynchronously check URLs and log results
        match tokio::runtime::Runtime::new() {
            Ok(runtime) => match runtime.block_on(url_checker::check_url_and_log(&config)) {
                Ok(_) => (),
                Err(err) => {
                    // Log error checking URL and logging
                    log::error!("Error checking URL and logging: {:?}", err);
                }
            },
            Err(e) => {
                // Log error creating Tokio runtime
                log::error!("Error creating Tokio runtime: {:?}", e);
            }
        }
    }
}
