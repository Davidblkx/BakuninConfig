#[macro_export]
macro_rules! create_config {
    ($file_name:literal {
        default: { $($key:ident: $value:expr),* $(,)? },
        $(env: $env_prefix:literal,)?
        $($layer:literal: [$($layer_dir_name:ident$(($layer_name_or_path:literal))?),*] $(init: $layer_init:literal)?),*
    }) => {
        {
            let default_value = $crate::value_map! {
                $($key: $value),*
            };

            #[allow(unused_mut)]
            let mut config = $crate::BakuninConfig::new()
                .with_memory_layer("default", default_value.clone());

            $(
                let layer_finder = $crate::file_finder::FileFinder::new($file_name)
                    .with_supported_extensions()
                    $(.with_os_directory($crate::file_finder::OSDirectory::$layer_dir_name$(($layer_name_or_path.to_string()))?))*
                    .find_first(true);

                if let Ok(layer_path) = layer_finder {
                    let path = layer_path.path;

                    let added = config.add_file_layer(
                        $layer,
                        path.clone()
                    ).is_ok();

                    $(
                        if added && !path.exists() && $layer_init {
                            if let Some(config_layer) = config.get_layer($layer) {
                                config_layer.write_value(
                                    &default_value
                                ).unwrap_or_else(|e| {
                                    log::warn!("Error writing default value to {} layer: {:?}", $layer, e);
                                });
                            }
                        }
                    )?
                } else if let Err(e) = layer_finder {
                    log::warn!("Error loading global config: {:?}", e);
                }
            )*

            $(
                config.add_environment_layer("environment", $env_prefix);
            )?

            config
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::value_map;

    #[test]
    fn creates_default_value() {
        let config = create_config!(".app-config" {
            default: {
                path: "/default/path",
                delay: 1000,
                log: value_map! {
                    level: "info",
                    file: "default.log"
                }
            },
        });

        let v = config.build_value(false).unwrap();

        assert_eq!(v.get("path").try_into_string().unwrap(), "/default/path");
        assert_eq!(v.get("delay").try_into_i64().unwrap(), 1000);
        assert_eq!(v.get("log").get("level").try_into_string().unwrap(), "info");
        assert_eq!(
            v.get("log").get("file").try_into_string().unwrap(),
            "default.log"
        );
    }

    #[test]
    fn add_environment() {
        std::env::set_var("MY_APP_data", "11");

        let config = create_config!(".app-config" {
            default: { data: 10 },
            env: "MY_APP_",
        });

        let v = config.build_value(false).unwrap();

        assert_eq!(v.get("data").try_into_i64().unwrap(), 11);
    }

    #[test]
    fn initiliaze_file() {
        std::env::set_var("MY_APP_data", "11");

        let expected_path = std::path::PathBuf::from("./target/.app-config.toml");
        if expected_path.exists() {
            std::fs::remove_file(&expected_path).unwrap();
        }

        let config = create_config!(".app-config" {
            default: { data: 10 },
            env: "MY_APP_",
            "global": [Path("./target")] init: true
        });

        config.build_value(false).unwrap();

        assert!(
            expected_path.exists(),
            "Expected file to be created at {:?}",
            expected_path
        );
    }
}
