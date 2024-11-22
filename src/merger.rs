use lum_libs::serde::{Deserialize, Serialize};

pub trait Merge<T> {
    fn merge(self, other: T) -> Self;
}

pub fn merge<FILE, ENV>(env_config: ENV, file_config: FILE) -> FILE
where
    FILE: Serialize + for<'de> Deserialize<'de> + Merge<ENV>,
    ENV: Serialize + for<'de> Deserialize<'de>,
{
    file_config.merge(env_config)
}
