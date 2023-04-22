use std::{
    thread,
    fs::{
        create_dir_all,
        self
    },
    path::Path,
    io::Write
};

pub mod config;
mod errors;

use errors::FileNotFoundError;
use config::Config;

pub fn bootstrap(verbose: bool) {
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
            let db_old = format!("{}.old", &database);
            if let Err(e) = std::fs::rename(database, db_old) {
                eprint!("could not rename old database");

                if verbose {
                    eprintln!("\nERROR:\n {}", e);
                } else {
                    eprint!(", run with --verbose to read the backtrace");
                    std::io::stdout().flush().unwrap();
                }

                std::process::exit(1);
            };
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

pub fn get_config() -> Result<Config, errors::FileNotFoundError> {
    let toml_wd = Config::get_path();
    let file_path = format!("{toml_wd}/{}", config::CONFIG_FILE);
    let toml_path = Path::new(&file_path);
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