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


    // println!("{conf:?}");
    // return ExitCode::SUCCESS;

    let args = opts::Args::from_args();

    match args.action {
        Action::Bootstrap => {
            bootstrap();
        },

        Action::Add { service, password, username } => {

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


            let mut elegant = elegant::Elegant::new(
                "/home/arthurx/.config/krm/storage"
            );

            elegant.insert("services", hashmap![
                "username" => Some("garok".to_string()),
                "password" => Some(passwd_bytes),
                "access"   => Some(service)
            ]).expect("could not insert");
        },

        Action::Get { service } => {
            let conf = parse_configs(config::Config::get_path());

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