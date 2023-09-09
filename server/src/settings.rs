use std::process::exit;

use config::{Config, ConfigError};
use serde::Deserialize;
use tracing::{error, info, instrument, warn};

// Holds global configuration values from CLI flags
#[derive(Debug, Default, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub listen_address: String,
}

// Build config from environment (and maybe other files too, later)
// Will exit progam on errors
#[instrument]
pub fn build() -> Settings {
    info!("Starting building configuration");

    load_env_file();

    let conf = Config::builder()
        .add_source(config::Environment::with_prefix("LIFTLOG").try_parsing(true))
        .build();

    let conf_result = match conf {
        Ok(result) => result,
        Err(error) => {
            error!("Failed to build config: {}", error);
            exit(1);
        }
    };

    let app_configuration_result: Result<Settings, ConfigError> = conf_result.try_deserialize();

    match app_configuration_result {
        Ok(result) => result,
        Err(error) => {
            error!("Failed to deserialize config: {}", error);
            exit(1);
        }
    }
}

// Load .env files to other environment variables
#[instrument]
fn load_env_file() {
    info!("Loading .env file");
    match dotenvy::dotenv() {
        Ok(_) => info!("File loaded"),
        Err(error) => warn!("Failed to load .env file: {}", error),
    };
}
