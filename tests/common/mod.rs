use std::{env, path::PathBuf};

use lum_config::Merge;
use lum_libs::{
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

pub static ENV_CONFIG_VALUE_SET: &str = "Environment config";
pub static ENV_CONFIG_VALUE_NOT_SET: &str = "Environment config not set";
pub static FILE_CONFIG_VALUE_SET: &str = "File config";
pub static NESTED_CONFIG_VALUE_SET: &str = "Nested config";

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct EnvConfig {
    pub value: Option<String>,
}

impl Default for EnvConfig {
    fn default() -> Self {
        EnvConfig {
            value: Some(ENV_CONFIG_VALUE_SET.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct FileConfig {
    pub value: String,
    pub env_config_variable: String,
}

impl Default for FileConfig {
    fn default() -> Self {
        FileConfig {
            value: FILE_CONFIG_VALUE_SET.to_string(),
            env_config_variable: ENV_CONFIG_VALUE_NOT_SET.to_string(),
        }
    }
}

impl Merge<EnvConfig> for FileConfig {
    fn merge(self, other: EnvConfig) -> Self {
        FileConfig {
            value: self.value,
            env_config_variable: other.value.unwrap_or("Missing".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct NestedConfig {
    pub value: String,
    pub file_config: Option<FileConfig>,
}

impl Default for NestedConfig {
    fn default() -> Self {
        NestedConfig {
            value: NESTED_CONFIG_VALUE_SET.to_string(),
            file_config: None,
        }
    }
}

impl Merge<FileConfig> for NestedConfig {
    fn merge(self, other: FileConfig) -> Self {
        NestedConfig {
            value: self.value,
            file_config: Some(other),
        }
    }
}

impl AsRef<FileConfig> for NestedConfig {
    fn as_ref(&self) -> &FileConfig {
        self.file_config.as_ref().unwrap()
    }
}

impl AsMut<FileConfig> for NestedConfig {
    fn as_mut(&mut self) -> &mut FileConfig {
        self.file_config.as_mut().unwrap()
    }
}

impl TryFrom<NestedConfig> for FileConfig {
    type Error = &'static str;

    fn try_from(value: NestedConfig) -> Result<Self, Self::Error> {
        match value.file_config {
            Some(file_config) => Ok(file_config),
            None => Err("File config not set"),
        }
    }
}

pub fn get_temp_dir() -> PathBuf {
    let uuid = Uuid::new_v4().to_string();
    let temp_dir = env::temp_dir();

    temp_dir.join(uuid)
}
