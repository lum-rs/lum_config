use std::marker::PhantomData;

use lum_libs::{
    serde::{Deserialize, Serialize},
    serde_env,
};

use crate::EnvironmentConfigParseError;

#[derive(Debug)]
pub struct EnvHandler<CONFIG>
where
    CONFIG: Serialize + for<'de> Deserialize<'de>,
{
    pub app_name: String,
    _phantom_file: PhantomData<CONFIG>,
}

impl<CONFIG> EnvHandler<CONFIG>
where
    CONFIG: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new<STRING>(app_name: STRING) -> Self
    where
        STRING: Into<String>,
    {
        EnvHandler {
            app_name: app_name.into(),
            _phantom_file: PhantomData,
        }
    }

    pub fn load_config(&self) -> Result<CONFIG, EnvironmentConfigParseError> {
        let prefix = self.app_name.to_uppercase();
        let config = serde_env::from_env_with_prefix(&prefix)?;

        Ok(config)
    }
}
