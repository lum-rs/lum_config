use lum_libs::serde::{Deserialize, Serialize};

/// A trait that defines a method for merging an instance of T into an instance of Self.
pub trait MergeFrom<T> {
    /// Merges an instance of T into an instance of Self.
    ///
    /// # Returns
    ///
    /// An instance of Self, which is the result of the merge operation.
    fn merge_from(self, other: T) -> Self;
}

/// A trait that defines a method for merging an instance of Self into an instance of T.
pub trait MergeInto<T> {
    /// Merges an instance of Self into an instance of T.
    ///
    /// # Returns
    ///
    /// An instance of T, which is the result of the merge operation.
    fn merge_into(self, other: T) -> T;
}

/// When a type implements `MergeFrom` for another type, automatically implement `MergeInto` for the other type.
impl<Config, PartialConfig> MergeInto<Config> for PartialConfig
where
    Config: MergeFrom<PartialConfig>,
{
    fn merge_into(self, other: Config) -> Config {
        other.merge_from(self)
    }
}

/// A trait that defines a method for merging an instance of T into an instance of Self, with the possibility of an error occurring.
/// The `Error` type is used to indicate the error that may occur during the merge operation.
pub trait TryMergeFrom<T>: Sized {
    type Error;

    /// Tries to merge an instance of T into an instance of Self.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    /// * Success is indicated by an `Ok` value, containing an instance of Self, which is the result of the merge operation.
    /// * Failure is indicated by an `Err` value, containing an instance of the error type.
    fn try_merge_from(self, other: T) -> Result<Self, Self::Error>;
}

/// A trait that defines a method for merging an instance of Self into an instance of T, with the possibility of an error occurring.
/// The `Error` type is used to indicate the error that may occur during the merge operation.
pub trait TryMergeInto<T> {
    type Error;

    /// Tries to merge an instance of Self into an instance of T.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    /// * Success is indicated by an `Ok` value, containing an instance of T, which is the result of the merge operation.
    /// * Failure is indicated by an `Err` value, containing an instance of the error type.
    fn try_merge_into(self, other: T) -> Result<T, Self::Error>;
}

/// When a type implements `TryMergeFrom` for another type, automatically implement `TryMergeInto` for the other type.
impl<Config, PartialConfig> TryMergeInto<Config> for PartialConfig
where
    Config: TryMergeFrom<PartialConfig>,
{
    type Error = Config::Error;

    fn try_merge_into(self, other: Config) -> Result<Config, Self::Error> {
        other.try_merge_from(self)
    }
}

/// When a type implements `MergeFrom` for another type, automatically implement `TryMergeFrom` on the same type for the other type.
impl<Config, PartialConfig> TryMergeFrom<PartialConfig> for Config
where
    Config: MergeFrom<PartialConfig>,
{
    type Error = ();

    fn try_merge_from(self, other: PartialConfig) -> Result<Config, Self::Error> {
        Ok(self.merge_from(other))
    }
}

/// Merges two instances of type `Config` and `PartialConfig` into a single instance of `Config`.
///
/// By using this function instead of calling `merge_from` directly, you can help the Rust compiler to better infer your config types.
/// This is because this function requires `Config` to implement `MergeFrom<PartialConfig>`.
///
/// The `PartialConfig` instance is merged into the `Config` instance.
///
/// `Config` must implement the `MergeFrom` trait for `PartialConfig`.
///
/// `Config` and `PartialConfig` must both implement the `Serialize` and `Deserialize` traits from `serde`.
///
/// # Parameters
///
/// * `config` - The instance of `Config` to merge `partial_config` into.
/// * `partial_config` - The instance of `PartialConfig` to be merged into `config`.
///
/// # Returns
///
/// An instance of `Config`, which is the result of the merge operation.
pub fn merge<Config, PartialConfig>(config: Config, partial_config: PartialConfig) -> Config
where
    Config: Serialize + for<'de> Deserialize<'de> + MergeFrom<PartialConfig>,
    PartialConfig: Serialize + for<'de> Deserialize<'de>,
{
    config.merge_from(partial_config)
}

/// Tries to merge two instances of type `Config` and `PartialConfig` into a single instance of `Config`.
///
/// By using this function instead of calling `try_merge_from` directly, you can help the Rust compiler to better infer your config types.
/// This is because this function requires `Config` to implement `TryMergeFrom<PartialConfig>`.
///
/// The `PartialConfig` instance is merged into the `Config` instance.
///
/// `Config` must implement the `TryMergeFrom` trait for `PartialConfig`.
///
/// `Config` and `PartialConfig` must both implement the `Serialize` and `Deserialize` traits from `serde`.
///
/// # Parameters
///
/// * `config` - The instance of `Config` to merge `partial_config` into.
/// * `partial_config` - The instance of `PartialConfig` to be merged into `config`.
///
/// # Returns
///
/// A `Result` indicating success or failure.
/// * Success is indicated by an `Ok` value, containing an instance of `Config`, which is the result of the merge operation.
/// * Failure is indicated by an `Err` value, containing an instance of the error type.
pub fn try_merge<Config, PartialConfig>(
    config: Config,
    partial_config: PartialConfig,
) -> Result<Config, Config::Error>
where
    Config: Serialize + for<'de> Deserialize<'de> + TryMergeFrom<PartialConfig>,
    PartialConfig: Serialize + for<'de> Deserialize<'de>,
{
    config.try_merge_from(partial_config)
}
