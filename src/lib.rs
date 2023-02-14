use std::{
    thread,
    fs::{
        create_dir_all,
        self
    },
    path::Path
};

use config::Config;
use errors::FileNotFoundError;

mod config;
mod errors;

pub fn bootstrap() {
    let config = Config::default();

    let mut config_path = home::home_dir().unwrap();
    config_path.push(".config/krm");

    let config_toml = toml::to_string_pretty(&config).unwrap();

    if !Path::exists(&config_path) {
        create_dir_all(&config_path).expect("permission denied on creating config path");
    }

    let handle = thread::spawn(move || {
        let database = config.database_path.clone();

        if Path::new(&database).exists() {
            let dbOld = format!("{}.old", &database);
            std::fs::rename(database, dbOld);
        }

        let connection = match sqlite::open(&config.database_path) {
            Ok(v) => v,
            Err(e) => panic!("{:#?}\ncouldn't open connection to database at {}", e.message, config.database_path)
        };

        ddl(&connection).unwrap_or_else(|e| panic!("could't run database's DDL\n{:#?}", e));
    });

    config_path.push("config.toml");
    fs::write(&config_path, config_toml).unwrap();

    println!("created config file at {}", config_path.display());

    handle.join();
}

pub fn ddl(connection: &sqlite::Connection) -> Result<(), sqlite::Error> {
    let sql = "  
    DROP TABLE IF EXISTS services;
    CREATE TABLE services (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        access TEXT,
        username TEXT,
        password BLOB NOT NULL,
        active INT DEFAULT 1
    )";

    connection.execute(sql)
}

pub fn get_config() -> Result<Config, errors::FileNotFoundError>
{
    let toml_wd = Config::get_path();
    let toml_path = Path::new(&toml_wd);
    let config: Config;

    if toml_path.exists() {
        let file_content = std::fs::read_to_string(&toml_path).unwrap();
        config = toml::from_str(&file_content).unwrap();
        Ok(config)
    } else {
        let err = FileNotFoundError {
            message: None,
            path: Some(Box::new(toml_path.to_owned()))
        };

        Err(err)
    }

}