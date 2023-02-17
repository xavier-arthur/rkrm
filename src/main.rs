mod config;
mod opts;
mod macros;
mod crypto;
mod traits;
mod elegant;
mod errors;

use std::{process::ExitCode, collections::HashMap};

use krm::{
    bootstrap,
    get_config,
};

use opts::Action;
use structopt::StructOpt;
use traits::IntoString;
use crypto::Crypto;
use inquire::Select;
use elegant::Elegant;

fn main() -> ExitCode {

    let args = opts::Args::from_args();

    match args.action {
        Action::Bootstrap => {
            bootstrap(args.verbose);
            ExitCode::SUCCESS
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
            } else if let None = password {
                eprintln!("no password or --prompt provided");

                return ExitCode::FAILURE;
            }

            let passwd_buf = crypto.encrypt(password.unwrap());
            let passwd_bytes: String = passwd_buf
                .into_iter()
                .map(|v| v.to_string() + " ")
                .collect();


            let mut elegant = Elegant::new(configs.database_path);

            elegant.insert("services", hashmap![
                "username" => Some(username),
                "password" => Some(passwd_bytes.trim().to_owned()),
                "access"   => Some(service)
            ]).expect("could not insert service at storage");

            ExitCode::SUCCESS
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
            let rows = elegant.select(
                "services",
                &format!("access = '{}'", service),
                &cols
            );

            let row: HashMap<String, String> = match rows.len()  {
                v if v > 1 => {
                    let service_names: Vec<String> = rows.iter()
                        .map(|v| format!("{} | {}", v["id"], v["username"]))
                        .collect();

                    let ans = Select::new("multiple services found", service_names).prompt()
                        .expect("couldn't not read the input, try again");

                    let id = ans.split(" | ")
                        .nth(0)
                        .unwrap();

                    rows.into_iter().filter(|v| {
                        v["id"] == id
                    })
                    .nth(0)
                    .unwrap()
                }, 

                1 => rows.into_iter().nth(0).unwrap(),

                // 0
                _ => { 
                    eprintln!("no entry matches for service {service}");
                    std::process::exit(1);
                }
            };

            let passwd: Vec<u8> = row["password"]
                .split(" ")
                .map(|v| v.trim().parse().unwrap())
                .collect();

            let unencrypted = crypto.decrypt(&passwd)
                .into_string();

            println!("{unencrypted}");

            ExitCode::SUCCESS
        },

        _ => ExitCode::SUCCESS 
    }
}