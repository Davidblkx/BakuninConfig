> WARNING: Since 0.3.0 ConfigBuilder struct has been deprecated in favor of BakuninConfig and create_config! macro.

# BakuninConfig

Layered configuration for Rust applications.

## About

BakuninConfig is a Rust library designed to provide a flexible and layered configuration system for applications. It allows you to define configuration values, load them from various sources (like files, environment variables, or in-memory). The library supports TOML and JSON formats. Values are aggregated from multiple layers, allowing for a clear hierarchy and overriding behavior. This is particularly useful for applications that need to manage configurations across different environments (development, testing, production) or user preferences.

## Overview

This library can be understood as 4 main domains:
- **Value**: The core value type that represents a configuration value. It can be a primitive type, a map, or a list. It's built in top of serde, and can be serialized and deserialized to/from any serde supported format.
- **FileFinder**: Contains logic to find configuration files in the filesystem. It supports searching in predefined OS folders or in custom folders, and it can search for files with specific extensions. Allowing users to add their own custom folders and extensions.
- **ConfigLayer**: Contains logic to load configuration values from a source (file, environment variables, memory, etc). It can be used to create a configuration layer that can be added to a configuration handler.
- **BakuninConfig**: Contains logic to read a configuration value from multiple layers. It can be used to create a configuration value that is a combination of multiple layers, such as files, environment variables, and default values.

## Value

This will be the main type you will work with. Can be created as any enum like:

```rust
use bakunin_config::Value;
use std::collections::HashMap;

let value = Value::Integer(42);
let value = Value::String("Hello, world!".into());
let value = Value::Array(vec![Value::Integer(1), Value::String("two".into())]);

let mut map = HashMap::new();
map.insert("key".to_string(), Value::String("value".into()));
let value = Value::Map(map);
```
When creating a map or array, you can use the `value_map!` and `value_vec!` macros for convenience:

```rust
use bakunin_config::{Value, value_map, value_vec};

let value = value_map! {
    prop1: value_map! {
        sub_prop1: "value1",
        sub_prop2: 2
    },
    prop2: value_vec!["item1", 10, true]
};
```

Values can be easily converted to and from other types using the `try_into` method. For example, to convert a `Value` to an `i64`, you can do:

```rust
use bakunin_config::Value;

let value = Value::Integer(42);
let int_value: i64 = value.try_into().unwrap();
assert_eq!(int_value, 42);
```

Map values can be accessed using the `get` method, which returns an `Value`:

```rust
use bakunin_config::{Value, value_map};

let value = value_map! {
    key1: "value1",
    key2: value_map! {
        sub_key1: "sub_value1",
        sub_key2: 42
    }
};

assert_eq!(value.get("key1").try_into_string().unwrap(), "value1");
assert_eq!(value.get("key2").get("sub_key1").try_into_string().unwrap(), "sub_value1");
assert_eq!(value.get("key2").get("sub_key2").try_into_i64().unwrap(), 42);
assert!(value.get("missing_key").get("other_key").is_none());
```

Vec values can be accessed using the `at` method, which returns an `Option<Value>`:

```rust
use bakunin_config::{Value, value_vec};

let value = value_vec!["item1", 42, true];
assert_eq!(value.at(0).try_into_string().unwrap(), "item1");
assert_eq!(value.at(1).try_into_i64().unwrap(), 42);
assert_eq!(value.at(2).try_into_bool().unwrap(), true);
assert!(value.at(3).is_none()); // Out of bounds
```

## FileFinder

The `FileFinder` is used to locate configuration files in the filesystem. It can search for files in predefined OS folders or in custom folders, and it supports searching for files with specific extensions.

```rust
use bakunin_config::file_finder::FileFinder;

let finder = FileFinder::new(".my-config") // will search for files named ".my-config.[extension]"
    .with_supported_extensions() // This depends on the flags enabled, by default it will search for ".toml"
    .with_working_directory() // This will search in the current working directory
    .with_user_home() // This will search in the user's home directory
    .with_path("/path/to/custom/folder".to_string()); // This will search in the specified path

let search_result = finder.find_first(true).unwrap(); // Find the first file that exists or the first file that matches the criteria
```

### There are 4 search algorithms available:

> Note: This examples assume you have flag toml (default) and json enabled, so it will search for files with `.toml` and `.json` extensions.

#### find_first(allow_missing: bool) -> Result<FindResult>

Looks for the first file that exists in the specified directories, using the example, it would search for the following files:

- "/<current_working_dir>/.my-config.toml"
- "/<current_working_dir>/.my-config.json"
- "/<user_home>/.my-config.toml"
- "/<user_home>/.my-config.json"
- "/path/to/custom/folder/.my-config.toml"
- "/path/to/custom/folder/.my-config.json"

It stops searching as soon as it finds the first file that exists. If no file is found, throws an error.

The method receives a boolean parameter "allow_missing", if true and no file is found, it will return "/<current_working_dir>/.my-config.toml"

#### find_last(allow_missing: bool) -> Result<FindResult>

Looks for the first file that exists in the specified directories, using the example, it would search for the following files:

- "/<current_working_dir>/.my-config.toml"
- "/<current_working_dir>/.my-config.json"
- "/<user_home>/.my-config.toml"
- "/<user_home>/.my-config.json"
- "/path/to/custom/folder/.my-config.toml"
- "/path/to/custom/folder/.my-config.json"

It stops searching as soon as it finds the first file that exists. If no file is found, throws an error.

The method receives a boolean parameter "allow_missing", if true and no file is found, it will return "/path/to/custom/folder/.my-config.json"

#### find_all(allow_missing: bool) -> Result<Vec<FindResult>>

Looks for the all files in the specified directories, using the example, it would search for the following files:

- "/<current_working_dir>/.my-config.toml"
- "/<current_working_dir>/.my-config.json"
- "/<user_home>/.my-config.toml"
- "/<user_home>/.my-config.json"
- "/path/to/custom/folder/.my-config.toml"
- "/path/to/custom/folder/.my-config.json"

If no file is found, throws an error.

The method receives a boolean parameter "allow_missing", if true, returns all paths that match the criteria, even if they do not exist.

#### find_all_or_first(&self) -> Result<Vec<FindResult>>

Looks for all files in the specified directories, using the example, it would search for the following files:

- "/<current_working_dir>/.my-config.toml"
- "/<current_working_dir>/.my-config.json"
- "/<user_home>/.my-config.toml"
- "/<user_home>/.my-config.json"
- "/path/to/custom/folder/.my-config.toml"
- "/path/to/custom/folder/.my-config.json"

If no file is found, it will return the first file that matches the criteria.

## ConfigLayer

It's a trait that defines how to read or load a configration value. It can be added to a `BakuninConfig` instance to provide a source of configuration values. 

Three main implementations are provided:

- **MemoryConfigLayer** A simple in-memory configuration layer that allows you to define configuration values directly in code. It can be used to provide default values or override values from other layers.

```rust
use bakunin_config::{BakuninConfig, config_layer::MemoryConfigLayer, Value, value_map};

MemoryConfigLayer::new(value_map! {
    key1: "value1",
    key3: value_map! {
        sub_key1: "sub_value1",
        sub_key2: true
    }
});
```

- **EnvironmentConfigLayer** Reads configuration values from environment variables. It allows you to define environment variables that can be used to override values from other layers. 

```rust
use bakunin_config::{BakuninConfig, config_layer::EnvironmentConfigLayer};

EnvironmentConfigLayer::new("MY_APP_"); // env var MY_APP_key1 will be used to override key1
```

- **FileConfigLayer<T> where T: FileHandler** Reads configuration values from files. It allows you to define a file handler that can read configuration files in different formats (TOML, JSON, etc.). TOML and JSON formats are supported are built-in, but you can implement your own file handler to support other formats.

```rust
use bakunin_config::{BakuninConfig, config_layer::FileConfigLayer, config_layer::handlers::TomlFileHandler};
use std::path::PathBuf;

FileConfigLayer::<TomlFileHandler>::new(PathBuf::from("path/to/config.toml"));
```

## BakuninConfig

It's the main struct, allows you to create a configuration handler that can read configuration values from multiple layers. It can be used to create a configuration value that is a combination of multiple layers, such as files, environment variables, and default values. Layers are added with a name, and can later be accessed by it.

**Example of a full configuration handler:**

Looks for a global configuration file in the user's home directory, a local configuration file in the current working directory, and environment variables prefixed with "MY_APP_". If the global configuration file does not exist, it creates it with default values.

```rust
use bakunin_config::{BakuninConfig, Value, value_map, file_finder::FileFinder};

let default_value = value_map! {
        key1: "value1",
        key2: 42,
        key3: value_map! {
            sub_key1: "sub_value1",
            sub_key2: true
        }
    };

let mut config = BakuninConfig::new()
    .with_memory_layer("default", default_value.clone());

let global_config = FileFinder::new(".my-config")
    .with_supported_extensions()
    .with_user_home()
    .with_user_config()
    .find_first(true);

match global_config {
    Ok(result) => {
        config.add_file_layer("global", result.path.clone());
        if !result.path.exists() {
            // Create the file if it does not exist
            if let Some(cfg) = config.get_layer("global") {
                cfg.write_value(&default_value).unwrap();
            }
        }
    }
    _=> {
        eprintln!("No global config found, using defaults");
    }
}

let local_config = FileFinder::new(".my-config")
    .with_supported_extensions()
    .with_working_directory()
    .find_first(false);

match local_config {
    Ok(result) => {
        config.add_file_layer("local", result.path.clone());
    }
    _=> {
        eprintln!("No local config found, using defaults");
    }
}

// add at end to override any previous values
config.add_environment_layer("env", "MY_APP_");

let config_value = config.build_value(true).unwrap();
```

previous example can be simplified using the `create_config!` macro:

```rust
use bakunin_config::{create_config, value_map};

let config = create_config!(".my-config" {
    default: {
        key1: "value1",
        key2: 42,
        key3: value_map! {
            sub_key1: "sub_value1",
            sub_key2: true
        }
    },
    env: "MY_APP_",
    "global": [UserHome, UserConfig] init: true,
    "local": [WorkingDirectory]
});

let config_value = config.build_value(true).unwrap();
```

local and global are arbitarily named, and can be any name you want, they are just used to identify the layers. The `create_config!` macro will automatically create the layers and add them to the configuration handler ny prder. The values in [] match enum OSDirectory.