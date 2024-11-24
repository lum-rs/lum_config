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
pub fn load<IntoString: Into<String>, FileConfig, EnvConfig>(
    app_name: IntoString,
    config_directory: Option<IntoString>,
    config_file_name: Option<IntoString>,
) -> Result<FileConfig, ConfigLoadError>
where
    FileConfig: Serialize + for<'de> Deserialize<'de> + MergeFrom<EnvConfig>,
    EnvConfig: Serialize + for<'de> Deserialize<'de>,
{
    let app_name = app_name.into();
    let config_directory = config_directory.map(Into::into);
    let config_file_name = config_file_name.map(Into::into);

    let env_handler = EnvHandler::new(app_name.clone());
    let file_handler = FileHandler::new(app_name, config_directory, config_file_name)?;

    let env_config = env_handler.load_config()?;
    let file_config = file_handler.load_config()?;

    let merged_config = merger::merge(env_config, file_config);

    Ok(merged_config)
}
