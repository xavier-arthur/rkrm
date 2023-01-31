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

fn main() -> ExitCode {
    let args = opts::Args::from_args();

    if  args.bootstrap {
        bootstrap();
        return ExitCode::SUCCESS;
    };

    todo!()
}