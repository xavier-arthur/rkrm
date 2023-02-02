mod config;
mod opts;
mod macros;
mod crypto;
mod traits;

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
    let args = opts::Args::from_args();

    let conf = parse_configs("/home/arthurx/.config/krm/config.toml");

    match args.action {
        Action::Bootstrap => {
            bootstrap();
        },

        Action::Add { service, password } => {
            let connection = sqlite::open(conf.database_path).unwrap();
            connection.execute(format!("INSERT INTO services (access, username, password) VALUES ('{service}', 'garok', '{}')", password.unwrap()));
        },

        Action::Get { service } => {

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