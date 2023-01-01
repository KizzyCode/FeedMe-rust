//! Configuration

use crate::{error, error::Error};
use serde::Deserialize;
use std::{env, fs, path::Path};

/// A config object
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// The server base URL (e.g. "https://example.org")
    pub base_url: String,
    /// The webroot path (e.g. "/var/www")
    pub webroot: String,
}
impl Config {
    /// Loads the config
    pub fn load() -> Result<Self, Error> {
        // Prefer configs from the environment
        if let Some(config) = Self::from_env()? {
            return Ok(config);
        }

        // Prefer local configs
        if let Some(config) = Self::from_file_local()? {
            return Ok(config);
        }

        // Load user configs
        if let Some(config) = Self::from_file_user()? {
            return Ok(config);
        }

        // Raise an error
        Err(error!("missing obligatory config file"))
    }

    /// Loads the config from the environment
    fn from_env() -> Result<Option<Self>, Error> {
        // Load the required variables from the environment
        let Ok(server_url) = env::var("FEEDME_BASE_URL") else {
            return Ok(None);
        };
        let Ok(webroot) = env::var("FEEDME_WEBROOT") else {
            return Ok(None);
        };

        // Create the config
        let config = Self { base_url: server_url, webroot };
        Ok(Some(config))
    }
    /// Loads the config from a directory-local file
    fn from_file_local() -> Result<Option<Self>, Error> {
        // Check if the config file exists
        let path = Path::new("feedme.toml");
        Self::from_file(path)
    }
    /// Loads the config from a per-user file
    fn from_file_user() -> Result<Option<Self>, Error> {
        // Get the home directory
        let Ok(home_path) = env::var("HOME") else {
            return Ok(None);
        };

        // Check if the config file exists
        let path = Path::new(&home_path).join(".config").join("feedme.toml");
        Self::from_file(&path)
    }
    /// Loads the config from a file if the file exists
    fn from_file(path: &Path) -> Result<Option<Self>, Error> {
        // Test path
        if !path.exists() {
            return Ok(None);
        }

        // Load the config
        let config_string = fs::read_to_string("feedme.toml")?;
        let config: Self = toml::from_str(&config_string)?;
        Ok(Some(config))
    }
}
