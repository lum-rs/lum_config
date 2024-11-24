use std::{fs, io, marker::PhantomData, path::PathBuf};

use lum_libs::{
    dirs,
    serde::{Deserialize, Serialize},
    serde_json,
};

use crate::{ConfigPathError, ConfigSaveError, FileConfigParseError};

#[derive(Debug)]
pub struct FileHandler<CONFIG>
where
    CONFIG: Serialize + for<'de> Deserialize<'de>,
{
    config_directory_path: PathBuf,
    config_file_path: PathBuf,
    _phantom_file: PhantomData<CONFIG>,
}

impl<CONFIG> FileHandler<CONFIG>
where
    CONFIG: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new<STRING: Into<String>>(
        app_name: STRING,
        config_directory: Option<STRING>,
        config_file_name: Option<STRING>,
    ) -> Result<Self, ConfigPathError> {
        let app_name = app_name.into();

        let mut config_directory_path = match config_directory {
            Some(config_directory) => PathBuf::from(config_directory.into()),
            None => match dirs::config_dir() {
                Some(path) => path,
                None => return Err(ConfigPathError::UnknownConfigDirectory),
            },
        };
        config_directory_path.push(app_name);

        let config_file_name = config_file_name
            .map(Into::into)
            .unwrap_or("config.json".into());
        let config_file_path = config_directory_path.join(config_file_name);

        Ok(FileHandler {
            config_directory_path,
            config_file_path,
            _phantom_file: PhantomData,
        })
    }

    pub fn create_config_directory(&self) -> Result<(), io::Error> {
        let path = &self.config_directory_path;
        fs::create_dir_all(path)?;

        Ok(())
    }

    pub fn save_config(&self, config: &CONFIG) -> Result<(), ConfigSaveError> {
        self.create_config_directory()?;

        let config_json = serde_json::to_string_pretty(config)?;
        fs::write(&self.config_file_path, config_json)?;

        Ok(())
    }

    pub fn load_config(&self) -> Result<CONFIG, FileConfigParseError> {
        self.create_config_directory()?;

        let path = &self.config_file_path;
        if !path.exists() {
            fs::write(path, "{}")?;
        }

        let config_json = fs::read_to_string(path)?;
        let config = serde_json::from_str(&config_json)?;
        self.save_config(&config)?; // In case the config file was missing some fields which serde used the defaults for

        Ok(config)
    }
}
