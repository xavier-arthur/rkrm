use std::{
    default::Default, 
    path::PathBuf,
};

use serde::{Serialize, Deserialize};
use home::home_dir;

pub const CONFIG_FOLDER: &'static str = ".config/krm";
pub const CONFIG_FILE: &'static str = "config.toml";
pub const DBNAME: &'static str = "storage";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub database_path: String,
    pub private_key_path: String,
    pub public_key_path: String
}

impl Default for Config {
    fn default() -> Self {
        let home = home_dir()
            .unwrap()
            .as_os_str()
        .to_os_string();

        let mut private_key = PathBuf::from(&home);
        let mut public_key = PathBuf::from(&home);
        let mut path_db = private_key.clone(); 
        
        path_db.push(format!("{CONFIG_FOLDER}/{DBNAME}"));
        private_key.push(".ssh/id_rsa");
        public_key.push(".ssh/id_rsa.pub");

        Self {
            database_path:    format!("{}", path_db.display()),
            private_key_path: format!("{}", private_key.display()),
            public_key_path:  format!("{}", public_key.display())
        } 
    }

}

impl Config {
    pub fn get_path() -> String {
        format!("{}/{}/{}", 
            home_dir().unwrap().display(),
            CONFIG_FOLDER,
            CONFIG_FILE
        )
    }
}