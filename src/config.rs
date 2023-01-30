use serde::{Serialize, Deserialize};
use std::{
    default::Default, 
    path::PathBuf,
    ffi::{OsString}
};
use home::home_dir;

#[derive(Serialize, Deserialize)]
pub struct Config {
    database_path: String,
}

impl Default for Config {
    fn default() -> Self {
        let home = home_dir()
            .unwrap()
            .as_os_str()
        .to_os_string();

        let mut path_full = PathBuf::from(&home);
        path_full.push(".config/krm/storage.sqlite");

        Self {
            database_path: path_full.into_os_string().into_string().unwrap()
        } 
    }
}