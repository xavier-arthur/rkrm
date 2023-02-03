use std::{
    thread,
    fs::{
        read_to_string,
        create_dir_all, self
    }
};

use std::{
    path::Path
};

use config::Config;
use home::home_dir;

mod config;

pub fn bootstrap() {
    let config = Config::default();

    let mut config_path = home::home_dir().unwrap();
    config_path.push(".config/krm");

    let config_toml = toml::to_string_pretty(&config).unwrap();

    if !Path::exists(&config_path) {
        create_dir_all(&config_path).expect("permission denied on creating config path");
    }

    let handle = thread::spawn(move || {
        let connection = match sqlite::open(&config.database_path) {
            Ok(v) => v,
            Err(e) => panic!("{:#?}\ncouldn't open connection to database at {}", e.message, config.database_path)
        };

        run_ddl(&connection).unwrap_or_else(|e| panic!("could't run database's DDL\n{:#?}", e));
    });

    config_path.push("config.toml");
    fs::write(config_path, config_toml).unwrap();

    handle.join();
}

pub fn run_ddl(connection: &sqlite::Connection) -> Result<(), sqlite::Error>{
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

pub fn parse_configs<T>(path: T) -> Config
where 
    T: AsRef<Path>
{
    let toml = read_to_string(path).unwrap();
    let config: Config = toml::from_str(&toml).unwrap();

    config
}