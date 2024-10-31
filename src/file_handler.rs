use std::{fs, marker::PhantomData, path::PathBuf};

use lum_libs::{
    dirs,
    serde::{Deserialize, Serialize},
    serde_json,
};

use crate::{ConfigInitError, ConfigPathError, ConfigSaveError, FileConfigParseError};

#[derive(Debug)]
pub struct FileHandler<CONFIG>
where
    CONFIG: Serialize + for<'de> Deserialize<'de>,
{
    pub app_name: String,
    pub config_file_name: String,
    _phantom_file: PhantomData<CONFIG>,
}

impl<CONFIG> FileHandler<CONFIG>
where
    CONFIG: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new<STRING>(app_name: STRING, config_file_name: Option<STRING>) -> Self
    where
        STRING: Into<String>,
    {
        FileHandler {
            app_name: app_name.into(),
            config_file_name: config_file_name
                .map(Into::into)
                .unwrap_or_else(|| String::from("config.json")),
            _phantom_file: PhantomData,
        }
    }

    pub fn get_config_dir_path(&self) -> Result<PathBuf, ConfigPathError> {
        let mut path = match dirs::config_dir() {
            Some(path) => path,
            None => return Err(ConfigPathError::UnknownConfigDirectory),
        };
        path.push(&self.app_name);

        Ok(path)
    }

    pub fn get_config_file_path(&self) -> Result<PathBuf, ConfigPathError> {
        let mut path = self.get_config_dir_path()?;
        path.push("config.json");

        Ok(path)
    }

    pub fn create_config_dir_path(&self) -> Result<(), ConfigInitError> {
        let path = self.get_config_dir_path()?;
        fs::create_dir_all(path)?;

        Ok(())
    }

    pub fn save_config(&self, config: &CONFIG) -> Result<(), ConfigSaveError> {
        let path = self.get_config_file_path()?;
        if !path.exists() {
            self.create_config_dir_path()?;
        }

        let config_json = serde_json::to_string_pretty(config)?;
        fs::write(path, config_json)?;

        Ok(())
    }

    pub fn load_config(&self) -> Result<CONFIG, FileConfigParseError> {
        let path = self.get_config_file_path()?;
        if !path.exists() {
            self.create_config_dir_path()?;
            fs::write(&path, "{}")?;
        }

        let config_json = fs::read_to_string(path)?;
        let config = serde_json::from_str(&config_json)?;
        self.save_config(&config)?; // In case the config file was missing some fields which serde used the defaults for

        Ok(config)
    }
}
