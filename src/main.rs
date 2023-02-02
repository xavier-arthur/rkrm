mod config;
mod opts;
mod macros;
mod crypto;
mod traits;


use std::{process::ExitCode, io::Read};

use krm::{
    bootstrap,
    // run_ddl,
    // parse_configs
};
use structopt::StructOpt;
use traits::IntoString;
use crypto::Crypto;

fn main() -> ExitCode {
    // let args = opts::Args::from_args();

    // if  args.bootstrap {
    //     bootstrap();
    //     return ExitCode::SUCCESS;
    // };

    let private = std::fs::read_to_string("/home/arthurx/private_key").unwrap();
    let pubk = std::fs::read_to_string("/home/arthurx/public_key").unwrap();

    let mut crypt = Crypto::new_with_keys(
        Some(private),
        Some(pubk)
    );
    todo!()
}