use std::{process::ExitCode};

use krm::{
    bootstrap,
    // run_ddl,
    // parse_configs
};
use structopt::StructOpt;

mod config;
mod opts;
mod macros;
mod crypto;

fn main() -> ExitCode {
    let args = opts::Args::from_args();

    if  args.bootstrap {
        bootstrap();
        return ExitCode::SUCCESS;
    };

    let private = std::fs::read_to_string("/home/arthurx/private_key").unwrap();
    let pubk = std::fs::read_to_string("/home/arthurx/public_key").unwrap();

    let encrypted = crypto::encrypt("a very cleverly hidden text", &pubk);

    let decrypt  = crypto::decrypt(&encrypted, &private, None);

    println!("{:?}", String::from_utf8(decrypt).unwrap());

    todo!()
}