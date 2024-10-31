use std::io;

use lum_libs::thiserror::Error;
use lum_libs::{serde_env, serde_json, thiserror};

#[derive(Debug, Error)]
pub enum ConfigPathError {
    #[error("Unable to get OS-specific config directory")]
    UnknownConfigDirectory,
}

#[derive(Debug, Error)]
pub enum ConfigInitError {
    #[error("Unable to get config path: {0}")]
    Path(#[from] ConfigPathError),

    #[error("I/O error: {0}")]
    IO(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum ConfigSaveError {
    #[error("Unable to get config path: {0}")]
    Path(#[from] ConfigPathError),

    #[error("Unable to init config: {0}")]
    Init(#[from] ConfigInitError),

    #[error("Unable to serialize config: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    IO(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum FileConfigParseError {
    #[error("Unable to get config path: {0}")]
    Path(#[from] ConfigPathError),

    #[error("Unable to initialize config: {0}")]
    Init(#[from] ConfigInitError),

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
    Envy(#[from] serde_env::Error),
}

#[derive(Debug, Error)]
pub enum ConfigParseError {
    #[error("Unable to parse config from file: {0}")]
    File(#[from] FileConfigParseError),

    #[error("Unable to parse config from environment: {0}")]
    Env(#[from] EnvironmentConfigParseError),
}
