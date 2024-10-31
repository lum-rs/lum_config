use lum_libs::serde::{Deserialize, Serialize};

pub trait Merge<T> {
    fn merge(&self, other: &T) -> Self;
}

pub fn merge<FILE, ENV>(prioritized_config: &ENV, secondary_config: FILE) -> FILE
where
    FILE: Serialize + for<'de> Deserialize<'de> + Merge<ENV>,
    ENV: Serialize + for<'de> Deserialize<'de>,
{
    secondary_config.merge(prioritized_config)
}
