use bakunin_config::{builder::{ConfigFile, ConfigFileFinder, OSFolder}, value_map, ConfigBuilder, Priority, Value};

#[derive(serde::Serialize, serde::Deserialize)]
/// Logger configuration
pub struct LoggerConfig {
    pub level: String,
    pub file: String,
}

fn main() {
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

    let config_file_name = "app-config"; // the name of the configuration file without extension

    // create a new configuration builder from the default value
    let mut builder = ConfigBuilder::from_base(default_value.clone()).unwrap();

    // Find the location of the configuration file, if no file is found, it will be /home/user/.app-config.toml
    let root_file = ConfigFileFinder::for_file(config_file_name.to_string()) // will search for files .app-config.{ext}
        .with_os_folder(OSFolder::UserHome) // will search in the user home directory: /home/user/
        .with_os_folder(OSFolder::Config) // will search in the user config directory: /home/user/.config/
        .with_os_folder(OSFolder::AppConfig("myapp".to_string())) // will search in the user config directory: /home/user/.config/myapp/
        .with_supported_extensions() // will search for files with the supported extensions: .json, .json5, .toml, .yaml, .yml (depending on the features enabled)
        .find_or_first() // will search for the first file that exists, if none is found, will return the first path
        .unwrap();

    // Find configuration files in the current working directory, returns a vector of paths
    let local_file = ConfigFileFinder::for_file(config_file_name.to_string()) // will search for files app-config.{ext}
        .with_os_folder(OSFolder::WorkingDirectory) // will search in the current working directory
        .with_supported_extensions() // will search for files with the supported extensions: .json, .json5, .toml, .yaml, .yml (depending on the features enabled)
        .find_all(); // find all files like app-config.toml, app-config.json, app-config.yaml, etc.

    let root_config_file = ConfigFile::new(root_file)
        .with_init(default_value.clone()); // will create a new ConfigFile with the default value if the file does not exist
    builder.add_config_file(root_config_file, Priority::FirstAvailable).unwrap(); // File is added with the next available priority

    if let Some(path) = local_file.first() {
        builder.add_config_file(ConfigFile::new(path.clone()), Priority::FirstAvailable).unwrap(); // File is added with the next available priority
    }

    // Will use environment variables to override the configuration values, ex: MY_APP_path will override the path value
    builder.environment("MY_APP").unwrap();

    // Read the configuration from the files and environment variables
    let config = builder.build();

    // Deserialize the configuration value into a LoggerConfig
    let log = config.get("log").deserialize::<LoggerConfig>().unwrap();
    println!("Logger Level: {}", log.level);
    println!("Logger File: {}", log.file);

    // Get the path value as a string
    let path = config.get("path").try_into_string().unwrap(); // Will panic if the value is not a string
    println!("Path: {}", path);

    // Get the delay value as a u64
    let delay: u64 = config.get("delay").try_into().unwrap(); // Will panic if the value is not a valid u64
    println!("Delay: {}", delay);

    // Try set the environment variable MY_APP_path to a new value or create a local app-config.toml file with the new value
}