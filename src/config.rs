use std::{
    default::Default, 
    path::PathBuf,
};

use serde::{Serialize, Deserialize};
use home::home_dir;

pub const CONFIG_FOLDER: &'static str = ".config/krm";
pub const DBNAME: &'static str = "storage";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub database_path: String,
    pub private_key_path: String,
}

impl Default for Config {
    fn default() -> Self {
        let home = home_dir()
            .unwrap()
            .as_os_str()
        .to_os_string();

        let mut path_key = PathBuf::from(&home);
        let mut path_db = path_key.clone(); 
        
        path_db.push(format!("{CONFIG_FOLDER}/{DBNAME}"));
        path_key.push(".ssh/id_rsa");

        Self {
            database_path:    format!("{}", path_db.display()),
            private_key_path: format!("{}", path_key.display())
        } 
    }
}