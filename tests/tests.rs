pub mod common;

#[cfg(test)]
mod tests {
    use std::fs;

    use lum_config::{FileHandler, merger};

    use crate::common::{self};

    #[test]
    fn construction_allows_different_string_types() {
        let app_name = common::APP_NAME;

        let _file_handler: FileHandler<common::FileConfig> =
            FileHandler::new(app_name, None::<&str>, None::<&String>).unwrap();
    }

    #[test]
    fn file_config_default() {
        let file_config = common::FileConfig::default();

        assert_eq!(file_config.value, common::FILE_CONFIG_VALUE_SET);
        assert_eq!(
            file_config.env_config_variable,
            common::ENV_CONFIG_VALUE_NOT_SET
        );
    }

    #[test]
    fn file_config_defaults_from_empty_file() {
        let temp_dir = common::get_temp_dir();
        let temp_str = temp_dir.to_str().unwrap();
        let file_handler: FileHandler<common::FileConfig> =
            FileHandler::new(common::APP_NAME, Some(temp_str), None::<&str>).unwrap();
        let file_config = file_handler.load().unwrap();

        assert_eq!(file_config.value, common::FILE_CONFIG_VALUE_SET);
        assert_eq!(
            file_config.env_config_variable,
            common::ENV_CONFIG_VALUE_NOT_SET
        );

        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn env_config_default() {
        let env_config = common::EnvConfig::default();

        assert!(env_config.value.is_some());
        assert_eq!(env_config.value.unwrap(), common::ENV_CONFIG_VALUE_SET);
    }

    #[test]
    fn merge() {
        let file_config = common::FileConfig::default();
        let env_config = common::EnvConfig::default();

        let merged_config = merger::merge(file_config, env_config);

        assert_eq!(merged_config.value, common::FILE_CONFIG_VALUE_SET);
        assert_eq!(
            merged_config.env_config_variable,
            common::ENV_CONFIG_VALUE_SET
        );
    }

    #[test]
    fn nested_config() {
        let nested_config = common::NestedConfig::default();
        assert_eq!(nested_config.value, common::NESTED_CONFIG_VALUE_SET);
        assert!(nested_config.file_config.is_none());

        let file_config = common::FileConfig::default();
        let merged_config = merger::merge(nested_config, file_config);
        assert_eq!(merged_config.value, common::NESTED_CONFIG_VALUE_SET);
        assert_eq!(
            merged_config.file_config.unwrap().value,
            common::FILE_CONFIG_VALUE_SET
        );
    }
}
