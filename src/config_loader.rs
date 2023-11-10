use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;

use serde::Deserialize;

use crate::models::Config;

// Load configuration from a YAML file
pub fn load_config() -> Result<Config, Box<dyn Error>> {
    // Open the config.yaml file
    let file = match File::open("config.yaml") {
        Ok(file) => file,
        Err(err) => {
            // Print an error message and return the error
            eprintln!("Error opening config file: {:?}", err);
            return Err(err.into());
        }
    };

    // Read the contents of the config.yaml file
    let mut contents = String::new();
    if let Err(err) = file.take(1024).read_to_string(&mut contents) {
        // Print an error message and return the error
        eprintln!("Error reading config file: {:?}", err);
        return Err(err.into());
    }

    // Deserialize the contents into a Config struct
    let config: Config = match serde_yaml::from_str(&contents) {
        Ok(config) => config,
        Err(err) => {
            // Print an error message and return the error
            eprintln!("Error parsing config file: {:?}", err);
            return Err(err.into());
        }
    };

    // Return the loaded configuration
    Ok(config)
}
