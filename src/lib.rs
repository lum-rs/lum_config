use lum_libs::serde::{Deserialize, Serialize};
/// Environment-related configuration handling.
pub mod env_handler;
/// Error types used across the crate.
pub mod error;
/// File-related configuration handling.
pub mod file_handler;
/// Traits and helper functions for merging configurations.
pub mod merger;

pub use env_handler::EnvHandler;
pub use error::*;
pub use file_handler::FileHandler;
pub use merger::*;

/// Loads configurations from environment variables and a file, and merges them together.
/// This function is a convenience function that combines the functionality of [EnvHandler], [FileHandler], and [merger].
///
/// The function takes the following generic type parameters:
/// * `FileConfig` - The configuration type that will be loaded from the file. This type must implement `Serialize`, `Deserialize`, and `MergeFrom<EnvConfig>`.
/// * `EnvConfig` - The configuration type that will be loaded from the environment variables. This type must implement `Serialize` and `Deserialize`.
///
/// # Parameters
///
/// * `app_name` - The name of the application, provided to [EnvHandler] and [FileHandler].
/// * `config_directory` - The configuration directory, provided to [FileHandler].
/// * `config_file_name` - The configuration file name, provided to [FileHandler].
///
/// # Returns
///
/// A `Result` indicating success or failure.
/// * Success is indicated by an `Ok` value, containing the merged `FileConfig`.
/// * Failure is indicated by an `Err` value, containing an instance of [ConfigLoadError].
pub fn load<FileConfig, EnvConfig>(
    app_name: impl Into<String>,
    config_directory: Option<impl Into<String>>,
    config_file_name: Option<impl Into<String>>,
) -> Result<FileConfig, ConfigLoadError>
where
    FileConfig: Serialize + for<'de> Deserialize<'de> + MergeFrom<EnvConfig>,
    EnvConfig: Serialize + for<'de> Deserialize<'de>,
{
    let app_name = app_name.into();
    let env_handler = EnvHandler::new(app_name.clone());
    let file_handler = FileHandler::new(app_name, config_directory, config_file_name)?;

    let env_config = env_handler.load()?;
    let file_config = file_handler.load()?;
    let merged_config = merger::merge(env_config, file_config);

    Ok(merged_config)
}
