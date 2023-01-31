use structopt::StructOpt;

#[derive(Debug)]

#[derive(Debug, StructOpt)]
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