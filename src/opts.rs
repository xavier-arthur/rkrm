use std::{fmt::{Display}};

use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq)]
#[structopt(no_version)]
pub enum Action { 
    Get {
        service: String
    },

    Add {
        service: String,
        username: String,
        password: Option<String>,

        #[structopt(long)]
        prompt: bool
    },

    Rm {
        service: String
    },

    Edit {
        service: String
    },

    Bootstrap 
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    no_version,
    about = "A password manager with RSA encryption"
)]
pub struct Args {

    #[structopt(long)]
    pub verbose: bool,

    #[structopt(subcommand)]
    pub action: Action,

    // #[structopt(short, long)]
    // pub private_key: Option<String>,

    #[structopt(short, long)]
    pub storage: Option<String>
}