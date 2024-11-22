use std::io;

use lum_libs::{serde_env, serde_json, thiserror::Error};

#[derive(Debug, Error)]
pub enum ConfigPathError {
    #[error("Unable to get OS-specific config directory")]
    UnknownConfigDirectory,
}

#[derive(Debug, Error)]
pub enum ConfigSaveError {
    #[error("Unable to serialize config: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    IO(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum FileConfigParseError {
    #[error("Unable to save config: {0}")]
    Save(#[from] ConfigSaveError),

    #[error("I/O error: {0}")]
    IO(#[from] io::Error),

    #[error("Unable to serialize or deserialize config: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum EnvironmentConfigParseError {
    #[error("Unable to parse environment variables: {0}")]
    SerdeEnv(#[from] serde_env::Error),
}

#[derive(Debug, Error)]
pub enum ConfigLoadError {
    #[error("Unable to handle config path: {0}")]
    Path(#[from] ConfigPathError),

    #[error("Unable to parse environment config: {0}")]
    ParseEnv(#[from] EnvironmentConfigParseError),

    #[error("Unable to parse file config: {0}")]
    ParseFile(#[from] FileConfigParseError),
}
