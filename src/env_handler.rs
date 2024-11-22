use std::marker::PhantomData;

use lum_libs::{
    serde::{Deserialize, Serialize},
    serde_env,
};

use crate::EnvironmentConfigParseError;

#[derive(Debug)]
pub struct EnvHandler<'app_name, CONFIG>
where
    CONFIG: Serialize + for<'de> Deserialize<'de>,
{
    pub app_name: &'app_name str,
    _phantom_file: PhantomData<CONFIG>,
}

impl<'app_name, CONFIG> EnvHandler<'app_name, CONFIG>
where
    CONFIG: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(app_name: &'app_name str) -> Self {
        EnvHandler {
            app_name,
            _phantom_file: PhantomData,
        }
    }

    pub fn load_config(&self) -> Result<CONFIG, EnvironmentConfigParseError> {
        let prefix = self.app_name.to_uppercase();
        let config = serde_env::from_env_with_prefix(&prefix)?;

        Ok(config)
    }
}
