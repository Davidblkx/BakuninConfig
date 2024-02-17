#[macro_export]
macro_rules! impl_file_handler {
    ($id:ty, $name:expr) => {
        impl $id {
            pub fn for_config_file(file: $crate::builder::files::ConfigFile) -> $crate::builder::files::FileConfigBuilder<Self> {
                $crate::builder::files::FileConfigBuilder::new(file, Self {})
            }
        }

        impl $crate::builder::files::ConfigFile {
            paste::paste! {
                pub fn [<as_ $name>](self) -> $crate::builder::files::FileConfigBuilder::<$id> {
                    <$id>::for_config_file(self)
                }
            }
        }

        impl $crate::builder::ConfigBuilder {
            paste::paste! {
                pub fn [<add_ $name _file>](&mut self, file: $crate::builder::files::ConfigFile, priority: $crate::Priority) -> Result<u64, $crate::ConfigError> {
                    let file_config = file.[<as_ $name>]().set_priority(priority);
                    self.add_config(&file_config)
                }

                pub fn [<append_ $name _file>](&mut self, file: $crate::builder::files::ConfigFile) -> Result<u64, $crate::ConfigError> {
                    self.[<add_ $name _file>](file, $crate::Priority::Any)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_file_extensions {
    ($($x:expr),*) => {
        fn extensions() -> Vec<String> {
            vec![$(stringify!($x).to_string()),*]
        }
    };
}

#[macro_export]
macro_rules! impl_file_name {
    ($name:expr) => {
        fn name(&self) -> String {
            stringify!($name).to_string()
        }
    };
}
