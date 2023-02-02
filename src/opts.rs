use std::{str::FromStr, fmt::{Display}};

use structopt::StructOpt;

#[derive(Debug, PartialEq)]
pub enum Action { add, edit, rm }

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct ActionNotFoundError { }
impl FromStr for Action {
    type Err = ActionNotFoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase() {
            s if s == "add" => Ok(Action::add),
            s if s == "edit" => Ok(Action::edit),
            s if s == "rm" => Ok(Action::rm),

            _ => Err(ActionNotFoundError {})
        }
    }    
}

impl Display for ActionNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Action not found")
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    no_version,
    about = "A password manager with RSA encryption"
)]
pub struct Args {

    #[structopt(short, long)]
    pub action: Action,

    #[structopt(long)]
    pub bootstrap: bool,

    #[structopt(short, long)]
    pub private_key: Option<String>,

    #[structopt(short, long)]
    pub storage: Option<String>
}