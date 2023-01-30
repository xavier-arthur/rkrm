use std::fs::{
    File,
    create_dir_all, self
};

use std::{
    path::Path
};

mod config;

use config::Config;

pub fn bootstrap() {
    let config = Config::default();

    let mut config_path = home::home_dir().unwrap();
    config_path.push(".config/krm");

    let config_toml = toml::to_string_pretty(&config).unwrap();

    if !Path::exists(&config_path) {
        create_dir_all(&config_path);
    }

    config_path.push("config.toml");
    fs::write(config_path, config_toml);

    todo!()
}