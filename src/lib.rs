pub mod env_handler;
pub mod error;
pub mod file_handler;
pub mod merger;

pub use env_handler::*;
pub use error::*;
pub use file_handler::*;
use lum_libs::serde::{Deserialize, Serialize};
pub use merger::*;

pub fn load_config<FILE, ENV, STRING>(
    app_name: impl Into<String>,
    config_file_name: Option<impl Into<String>>,
) -> Result<FILE, ConfigParseError>
where
    FILE: Serialize + for<'de> Deserialize<'de> + Merge<ENV>,
    ENV: Serialize + for<'de> Deserialize<'de>,
{
    let app_name: String = app_name.into();
    let config_file_name: Option<String> = config_file_name.map(Into::into);

    let file_handler = FileHandler::new(app_name.clone(), config_file_name);
    let env_handler = EnvHandler::new(app_name);

    let env_config = env_handler.load_config()?;
    let file_config = file_handler.load_config()?;

    let merged_config = merger::merge(&env_config, file_config);

    Ok(merged_config)
}
