mod config;
mod opts;
mod macros;
mod crypto;
mod traits;
mod elegant;
mod errors;

use std::{process::ExitCode};
use elegant::Elegant;

use krm::{
    bootstrap, get_config,
    // run_ddl,
    // parse_configs
};
use opts::Action;
use structopt::StructOpt;
use traits::IntoString;
use crypto::Crypto;

fn main() -> ExitCode {

    let args = opts::Args::from_args();

    match args.action {
        Action::Bootstrap => {
            bootstrap(args.verbose);
        },

        Action::Add { service, mut password, username, prompt} => {
            let configs = match get_config() {
                Ok(v) => v,
                Err(e) => {
                    println!("Configuration file at {} not found", e.path.unwrap().display());
                    std::process::exit(1);
                }
            };

            let (public, private) = match configs.read_keys() {
                Ok((publ, prv)) => (publ, prv),
                Err(_) => {
                    println!("one or more ssh keys don't exist");
                    std::process::exit(1);
                }
            };

            let mut crypto = Crypto::new_with_keys(
                Some(public),
                Some(private)
            );

            if prompt {
                let input = rpassword::prompt_password("Enter secret: ").unwrap();
                password = Some(input);
            }

            let passwd_buf = crypto.encrypt(password.unwrap());
            let passwd_bytes: String = passwd_buf
                .into_iter()
                .map(|v| v.to_string() + " ")
                .collect();


            let mut elegant = Elegant::new(
                "/home/arthurx/.config/krm/storage"
            );

            elegant.insert("services", hashmap![
                "username" => Some(username),
                "password" => Some(passwd_bytes.trim().to_owned()),
                "access"   => Some(service)
            ]).expect("could not insert");
        },

        Action::Get { service } => {
            let configs = match get_config() {
                Ok(v) => v,
                Err(e) => {
                    println!("Configuration file at {} not found", e.path.unwrap().display());
                    std::process::exit(1);
                }
            };

            let (public, private) = match configs.read_keys() {
                Ok((publ, prv)) => (publ, prv),
                Err(_) => {
                    println!("one or more ssh keys don't exist");
                    std::process::exit(1);
                }
            };

            let mut crypto = Crypto::new_with_keys(
                Some(public),
                Some(private)
            );

            let elegant = Elegant::new(configs.database_path);

            let cols: [&str; 3] = ["id", "username", "password"];
            let map = elegant.select(
                "services",
                &format!("access = '{}'", service),
                &cols
            );

            let passwd: Vec<u8> = map["password"]
                .split(" ")
                .map(|v| v.trim().parse().unwrap())
                .collect();

            let unencrypted = crypto.decrypt(&passwd)
                .into_string();

            println!("{unencrypted}");
        },

        _ => { }
    };

    ExitCode::SUCCESS
}