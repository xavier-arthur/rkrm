mod config;
mod opts;
mod macros;
mod crypto;
mod traits;
mod elegant;

use std::{process::ExitCode};

use krm::{
    bootstrap,
    // run_ddl,
    parse_configs
};
use opts::Action;
use structopt::StructOpt;
use traits::IntoString;
use crypto::Crypto;

fn main() -> ExitCode {

    let conf = parse_configs("/home/arthurx/.config/krm/config.toml");


    let mut elegant = elegant::Elegant::new(
        "/home/arthurx/.config/krm/storage"
    );

    elegant.insert("services", hashmap![
        "service" => None,
        "password" => Some("123")
    ]);

    return ExitCode::SUCCESS;

    let args = opts::Args::from_args();

    match args.action {
        Action::Bootstrap => {
            bootstrap();
        },

        Action::Add { service, password } => {
            let connection = sqlite::open(conf.database_path).unwrap();

            let private = std::fs::read_to_string("/home/arthurx/private_key").unwrap();
            let pubk = std::fs::read_to_string("/home/arthurx/public_key").unwrap();
            let mut Crypto = Crypto::new_with_keys(
                Some(private),
                Some(pubk)
            );

            let passwd_buf = Crypto.encrypt(password.unwrap());
            let passwd_bytes: String = passwd_buf
                .into_iter()
                .map(|v| v.to_string() + " ")
                .collect();

            connection.execute(format!("INSERT INTO services (access, username, password) VALUES ('{service}', 'garok', '{passwd_bytes}')"));
        },

        Action::Get { service } => {
            let sql = format!("SELECT * FROM services WHERE id = {service}");

            let private = std::fs::read_to_string("/home/arthurx/private_key").unwrap();
            let pubk = std::fs::read_to_string("/home/arthurx/public_key").unwrap();
            let mut Crypto = Crypto::new_with_keys(
                Some(private),
                Some(pubk)
            );

            let connection = sqlite::open(conf.database_path).unwrap();
            let mut map: std::collections::HashMap<String, Option<String>> = hashmap![];

            connection.iterate(sql, |pairs| { 
                for &(k, v) in pairs {
                    map.insert(k.to_owned(), if v.is_some() { Some(v.unwrap().to_owned()) } else { None } );
                }
                true
            }).unwrap();

            let passwd: Vec<u8> = map["password"]
                .as_ref()
                .unwrap()
                .trim()
                .split(" ")
                .map(|v| v.parse().unwrap())
                .collect();

            let desem = Crypto.decrypt(&passwd)
                .into_string();

            println!("{} | {}", map["username"].as_ref().unwrap(), desem);
        },

        _ => { }
    };

    ExitCode::SUCCESS

    // let private = std::fs::read_to_string("/home/arthurx/private_key").unwrap();
    // let pubk = std::fs::read_to_string("/home/arthurx/public_key").unwrap();

    // let mut crypt = Crypto::new_with_keys(
    //     Some(private),
    //     Some(pubk)
    // );
    // todo!()
}