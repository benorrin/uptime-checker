use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;

use serde::Deserialize;

use crate::models::Config;

// Load configuration from a YAML file
pub fn load_config() -> Result<Config, Box<dyn Error>> {
    // Log that configuration loading is starting
    log::info!("Loading configuration...");

    // Open the config.yaml file
    let file = match File::open("config.yaml") {
        Ok(file) => file,
        Err(err) => {
            // Log error opening config file
            log::error!("Error opening config file: {:?}", err);
            return Err(err.into());
        }
    };

    // Read the contents of the config.yaml file
    let mut contents = String::new();
    if let Err(err) = file.take(1024).read_to_string(&mut contents) {
        // Log error reading config file
        log::error!("Error reading config file: {:?}", err);
        return Err(err.into());
    }

    // Deserialize the contents into a Config struct
    let config: Config = match serde_yaml::from_str(&contents) {
        Ok(config) => config,
        Err(err) => {
            // Log error parsing config file
            log::error!("Error parsing config file: {:?}", err);
            return Err(err.into());
        }
    };

    // Log successful configuration loading
    log::info!("Configuration loaded successfully: {:?}", config);

    // Return the loaded configuration
    Ok(config)
}
