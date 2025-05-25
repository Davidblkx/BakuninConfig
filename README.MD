# BakuninConfig

Layered configuration for Rust applications.

## About

It was created to provide a way to search and load configurations from different sources and merge them into a single configuration value. It uses serde to serialize and deserialize the configuration values, which means that you can use any format supported by serde to store your configuration. By default, it supports TOML, and support for JSON, JSON5 and YAML can be enabled by features. It employs a slot system to merge the configuration values, starting from 0 to u64::MAX, where the last one takes precedence

## Usage

```rust
// the default value for our configurations
let default_value = value_map! {
    path: "some/path/to/dir",
    delay: 1000,
    enabled: true,
    log: value_map! {
        level: "info",
        file: "log.txt"
    }
};

// create a new configuration builder from the default value
let mut builder = ConfigBuilder::from_base(default_value.clone())?;

// Create the root config file path
let root_file: PathBuf = ConfigFileFinder::for_file(".app-config".to_string()) // will search for files .app-config.{ext}
    .with_os_folder(OSFolder::UserHome) // will search in the user home directory: /home/user/
    .with_os_folder(OSFolder::Config) // will search in the user config directory: /home/user/.config/
    .with_os_folder(OSFolder::AppConfig("myapp".to_string())) // will search in the user config directory: /home/user/.config/myapp/
    .with_supported_extensions() // will search for files with the supported extensions: .json, .json5, .toml, .yaml, .yml
    .find_or_first(); // will search for the first file that exists, if none is found, will return the first path
    .unwrap();

let config_file = ConfigFile::new(root_file)
    .with_init(default_value) // Will create the file with the default value if it does not exist

// add the root config file to the builder, first available means that will be added in the first slot available
builder.add_config_file(config_file, Priority::FirstAvailable)?;

// Will use environment variables to override the configuration values, ex: MY_APP_path will override the path value
builder.environment("MY_APP")?;

let config: Value = builder.build();

#[derive(Serialize, Deserialize)]
/// Logger configuration
pub struct LoggerConfig {
    pub level: String,
    pub file: String,
}

// Deserialize the configuration value into a LoggerConfig
let log = config.get("log").deserialize::<LoggerConfig>().unwrap();

// Get the path value as a string
let path = config.get("path").try_into_String().unwrap(); // Will panic if the value is not a string

// Get the delay value as a u64
let delay: u64 = config.get("delay").try_into().unwrap(); // Will panic if the value is not a valid u64
```
