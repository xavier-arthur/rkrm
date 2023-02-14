use std::{
    default::Default, 
    path::PathBuf,
};

use serde::{Serialize, Deserialize};
use home::home_dir;

use crate::errors::FileNotFoundError;

pub const CONFIG_FOLDER: &'static str = ".config/krm";
pub const CONFIG_FILE  : &'static str = "config.toml";
pub const DBNAME       : &'static str = "storage";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub database_path    : String,
    pub private_key_path : String,
    pub public_key_path  : String,
    pub uses_password    : bool
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
            public_key_path:  format!("{}", public_key.display()),
            uses_password:    false
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

    pub fn read_keys(&self) -> Result<(String, String), FileNotFoundError> {
        let path_public = PathBuf::from(&self.public_key_path);
        let path_private = PathBuf::from(&self.private_key_path);

        if !(path_public.exists() && path_private.exists()) {
            return Err(FileNotFoundError::default());
        }

        let public_key = std::fs::read_to_string(path_public).unwrap();
        let private_key = std::fs::read_to_string(path_private).unwrap();

        Ok((public_key, private_key))
    }
}