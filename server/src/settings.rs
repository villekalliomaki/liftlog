use std::process::exit;

use config::{Config, ConfigError};
use serde::Deserialize;

// Holds global configuration values from CLI flags
#[derive(Debug, Default, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub listen_address: String,
    pub debug: bool,
}

// Build config from environment (and maybe other files too, later)
// Will exit progam on errors
pub fn build() -> Settings {
    load_env_file();

    let settings_result = Config::builder()
        .add_source(config::Environment::with_prefix("LIFTLOG").try_parsing(true))
        .build();

    let settings = match settings_result {
        Ok(result) => result,
        Err(error) => {
            println!("Failed to build settings: {}", error);
            exit(1);
        }
    };

    let app_configuration_result: Result<Settings, ConfigError> = settings.try_deserialize();

    match app_configuration_result {
        Ok(app_configuration) => app_configuration,
        Err(error) => {
            println!("Failed to deserialize settings: {}", error);
            exit(1);
        }
    }
}

// Load .env files to other environment variables
fn load_env_file() {
    if let Err(error) = dotenvy::dotenv() {
        println!("Failed to load .env file: {}", error);
    }
}
