use lum_libs::serde::{Deserialize, Serialize};

pub mod env_handler;
pub mod error;
pub mod file_handler;
pub mod merger;

pub use env_handler::EnvHandler;
pub use error::*;
pub use file_handler::FileHandler;
pub use merger::*;

pub fn load<'app_name, 'config_directory_name, 'config_file_name, FILE, ENV>(
    app_name: &'app_name str,
    config_directory: Option<&'config_directory_name str>,
    config_file_name: Option<&'config_file_name str>,
) -> Result<FILE, ConfigLoadError>
where
    FILE: Serialize + for<'de> Deserialize<'de> + Merge<ENV>,
    ENV: Serialize + for<'de> Deserialize<'de>,
{
    let env_handler = EnvHandler::new(app_name);
    let file_handler = FileHandler::new(app_name, config_directory, config_file_name)?;

    let env_config = env_handler.load_config()?;
    let file_config = file_handler.load_config()?;

    let merged_config = merger::merge(&env_config, file_config);

    Ok(merged_config)
}
