use std::marker::PhantomData;

use lum_libs::{
    serde::{Deserialize, Serialize},
    serde_env,
};

use crate::EnvironmentConfigParseError;

/// A handler for loading configuration from environment variables.
///
/// The `EnvHandler` struct is a generic type that takes a configuration type `Config`
/// which must implement the `Serialize` and `Deserialize` traits from `serde`.
///
/// ## Type Parameters
///
/// * `Config` - The configuration type that implements `Serialize` and `Deserialize`. This is the type to which the environment variables will be deserialized.
///
/// ## Fields
///
/// * `app_name` - The name of the application.
///
/// # Examples
///
/// ```
/// use lum_libs::serde::{Deserialize, Serialize};
/// use lum_config::env_handler::EnvHandler;
/// use std::env;
///
/// #[derive(Serialize, Deserialize)]
/// struct Config {
///     key: String,
/// }
///
/// env::set_var("MYAPP_KEY", "value");
///
/// let handler = EnvHandler::<Config>::new("MyApp");
/// let config = handler.load_config().unwrap();
///
/// assert_eq!(config.key, "value");
/// ```
#[derive(Debug)]
pub struct EnvHandler<Config>
where
    Config: Serialize + for<'de> Deserialize<'de>,
{
    pub app_name: String,
    _phantom_file: PhantomData<Config>,
}

impl<Config> EnvHandler<Config>
where
    Config: Serialize + for<'de> Deserialize<'de>,
{
    /// Creates a new `EnvHandler` instance with the given application name.
    ///
    /// # Parameters
    ///
    /// * `app_name` - The name of the application. This will be used as a prefix for the environment variables.
    ///
    /// # Returns
    ///
    /// A new `EnvHandler` instance.
    pub fn new<IntoString: Into<String>>(app_name: IntoString) -> Self {
        EnvHandler {
            app_name: app_name.into(),
            _phantom_file: PhantomData,
        }
    }

    /// Loads the configuration from the environment variables.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    /// * Success is indicated by an `Ok` value, containing the Config instance.
    /// * Failure is indicated by an `Err` value, containing an `EnvironmentConfigParseError`.
    pub fn load_config(&self) -> Result<Config, EnvironmentConfigParseError> {
        let prefix = self.app_name.to_uppercase();
        let config = serde_env::from_env_with_prefix(&prefix)?;

        Ok(config)
    }
}
