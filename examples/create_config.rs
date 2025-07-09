extern crate bakunin_config;

/// Experiment with changing the .bajunin.toml file in home directory
/// or create a new one in the working directory.
///
/// You can also experiment with overring values with environment
/// variables like BAK_key1=value1
fn main() {
    use bakunin_config::{create_config, value_map};

    let config = create_config!(".bakunin" {
        default: {
            key1: "value1",
            key2: 42,
            key3: value_map! {
                sub_key1: "sub_value1",
                sub_key2: true
            }
        },
        env: "BAK_",
        "global": [UserHome, UserConfig] init: true,
        "local": [WorkingDirectory]
    });

    let config_value = config.build_value(true).unwrap();

    println!("Config Value: {:?}", config_value);
}
