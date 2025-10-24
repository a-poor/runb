//! Defines the configuration file structs.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, path::PathBuf};
use validator::{Validate, ValidationError};

/// Environment variable that can optionally be
/// set pointing to the `runb` config file.
pub const CONFIG_FILE_PATH_VAR: &str = "RUNB_CONF";

/// The default file directory in which to look for
/// the configuration file.
///
/// Note: This is expected to be relative to `$HOME`.
///
/// See also: `DEFAULT_CONFIG_FILE_DIR`
pub const DEFAULT_CONFIG_FILE_DIR: &str = ".conf/runb";

/// The default configuration file name
pub const DEFAULT_CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ConfigFile {
    #[validate(custom(function = "is_v0"))]
    pub version: String,
}

impl ConfigFile {
    /// Get the path to the config file as set via
    /// an environment variable or fallback to
    /// the default location.
    pub fn get_config_path() -> Option<PathBuf> {
        // Check for an environment variable
        if let Ok(p) = env::var(CONFIG_FILE_PATH_VAR) {
            return Some(PathBuf::from(p));
        }

        // Get the home directory
        //
        // If it can't be found, returns None
        let home = match env::home_dir() {
            Some(p) => p,
            None => return None,
        };

        // Add in the key parts and return
        let conf_path = home
            .join(DEFAULT_CONFIG_FILE_DIR)
            .join(DEFAULT_CONFIG_FILE_NAME);
        Some(conf_path)
    }

    /// Load a config file from disk and validate it.
    pub async fn load_from_file(path: &PathBuf) -> Result<Self> {
        let raw = tokio::fs::read_to_string(path).await?;
        let conf: ConfigFile = toml::from_str(&raw)?;
        if let Err(e) = conf.validate() {
            return Err(e.into());
        }
        Ok(conf)
    }

    /// Write the current configuration to disk.
    pub async fn write_to_file(&self, path: &PathBuf) -> Result<()> {
        let raw = toml::to_string_pretty(self)?;
        tokio::fs::write(path, raw).await?;
        Ok(())
    }
}

fn is_v0(val: &str) -> core::result::Result<(), ValidationError> {
    if val != "v0" {
        return Err(ValidationError::new("bad_version"));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_load_conf_from_file() -> Result<()> {
        let cases = vec![
            (true, "version = \"v0\""),
            (false, ""),
            (false, "version = \"v1\""),
        ];
        for (expect_success, raw) in cases.iter() {
            // Create a temporary file
            let tf = NamedTempFile::new()?;
            let pb = tf.path().to_path_buf();

            // Write the raw config data to the temp file
            tokio::fs::write(&pb, *raw).await?;

            // Try to read and load-in the config file
            let res = ConfigFile::load_from_file(&pb).await;
            let success = res.is_ok();

            // Validate the file loaded successfully
            assert_eq!(*expect_success, success, "Oops! Err: {:?}", res.err());

            // Clean up the temp file
            tf.close()?;
        }
        Ok(())
    }

    #[test]
    fn test_conf_validate() {
        let cases = vec![
            (
                true,
                ConfigFile {
                    version: "v0".to_string(),
                },
            ),
            (
                false,
                ConfigFile {
                    version: "v".to_string(),
                },
            ),
            (
                false,
                ConfigFile {
                    version: "".to_string(),
                },
            ),
            (
                false,
                ConfigFile {
                    version: "V".to_string(),
                },
            ),
            (
                false,
                ConfigFile {
                    version: "v1".to_string(),
                },
            ),
        ];
        for (should_succeed, conf) in cases.iter() {
            let res = conf.validate();
            let success = res.is_ok();
            assert_eq!(*should_succeed, success, "version={:?}", conf.version,);
        }
    }
}
