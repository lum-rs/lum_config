use std::{fs, io, marker::PhantomData, path::PathBuf};

use lum_libs::{
    dirs,
    serde::{Deserialize, Serialize},
    serde_json,
};

use crate::{ConfigPathError, ConfigSaveError, FileConfigParseError};

/// A handler for loading and saving configuration from/to files.
///
/// The `FileHandler` struct is a generic type that takes a configuration type `Config`
/// which must implement the `Serialize` and `Deserialize` traits from `serde`.
///
/// # Type Parameters
///
/// * `Config` - The configuration type that implements `Serialize` and `Deserialize`. This is the type to which the configuration file will be deserialized.
///
/// # Fields
///
/// * `config_directory_path` - The path to the directory where the configuration file is stored.
/// * `config_file_path` - The path to the configuration file.
///
/// # Examples
///
/// ```
/// use lum_libs::{uuid::Uuid, serde::{Deserialize, Serialize}};
/// use lum_config::file_handler::FileHandler;
/// use std::{env, fs, path::PathBuf};
///
/// #[derive(Serialize, Deserialize)]
/// #[serde(crate = "lum_libs::serde")]
/// #[serde(default)]
/// struct Config {
///     key: String,
/// }
///
/// impl Default for Config {
///    fn default() -> Self {
///       Config {
///          key: "default_value".to_string(),
///      }
///   }
/// }
///
/// let uuid = Uuid::new_v4().to_string();
/// let temp_dir = env::temp_dir().join(uuid); // Using a temporary directory here just for the test. To use your OS-specific configuration directory, use `None` instead of `Some(temp_str)` below.
///
/// let temp_str = temp_dir.to_str().unwrap();
/// let file_handler: FileHandler<Config> =
///     FileHandler::new("MyApp", Some(temp_str), None::<&str>).unwrap();
///
/// let config = file_handler.load_config().unwrap();
/// fs::remove_dir_all(temp_dir).unwrap(); // To clean up the temporary directory when running the test
///
/// assert_eq!(config.key, "default_value");
/// ```
#[derive(Debug)]
pub struct FileHandler<Config>
where
    Config: Serialize + for<'de> Deserialize<'de>,
{
    pub config_directory_path: PathBuf,
    pub config_file_path: PathBuf,
    _phantom_data: PhantomData<Config>,
}

impl<Config> FileHandler<Config>
where
    Config: Serialize + for<'de> Deserialize<'de>,
{
    /// Creates a new `FileHandler`.
    ///
    /// # Arguments
    ///
    /// * `app_name` - The name of the application. This is used to construct the default configuration file path.
    /// * `config_directory` - An optional custom directory for the configuration file. Defaults to the OS-specific configuration directory.
    /// * `config_file_name` - An optional custom name for the configuration file. Defaults to `config.json`.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    /// * Success is indicated by an `Ok` value, containing the `FileHandler` instance.
    /// * Failure is indicated by an `Err` value, containing a `ConfigPathError`.
    pub fn new(
        app_name: impl Into<String>,
        config_directory: Option<impl Into<String>>,
        config_file_name: Option<impl Into<String>>,
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
            _phantom_data: PhantomData,
        })
    }

    /// Creates the configuration directory if it does not exist.
    ///
    /// **This does not need to be called manually** as it is called by `load_config` and `save_config`.
    /// However, it can be called manually, if you need it.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    /// * Success is indicated by an `Ok` value, containing the unit type `()`.
    /// * Failure is indicated by an `Err` value, containing an `io::Error`.
    pub fn create_config_directory(&self) -> Result<(), io::Error> {
        let path = &self.config_directory_path;
        fs::create_dir_all(path)?;

        Ok(())
    }

    /// Saves the configuration to the configuration file.
    ///
    /// If the configuration directory does not exist, it will be created.
    ///
    /// If the configuration file does not exist, it will be created.
    ///
    /// If the configuration file already exists, it will be overwritten.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration to be saved.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    /// * Success is indicated by an `Ok` value, containing the unit type `()`.
    /// * Failure is indicated by an `Err` value, containing a `ConfigSaveError`.
    pub fn save_config(&self, config: &Config) -> Result<(), ConfigSaveError> {
        self.create_config_directory()?;

        let config_json = serde_json::to_string_pretty(config)?;
        fs::write(&self.config_file_path, config_json)?;

        Ok(())
    }

    /// Loads the configuration from the configuration file.
    ///
    /// If the configuration directory does not exist, it will be created.
    ///
    /// If the configuration file does not exist, it will be created with an empty JSON object.
    ///
    /// **To be able to create a fresh config file, or insert missing attributes,
    /// make sure that your configuration type has a default implementation
    /// (either by deriving `Default` or implementing the Default trait),
    /// and that \#[serde(default)] is used on the struct.**
    /// Otherwise, you will get a `serde` error at runtime, complaining about missing fields.
    /// For an example, see the example in the documentation of the [FileHandler#examples] struct.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    /// * Success is indicated by an `Ok` value, containing the Config instance.
    /// * Failure is indicated by an `Err` value, containing a `FileConfigParseError`.
    pub fn load_config(&self) -> Result<Config, FileConfigParseError> {
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
