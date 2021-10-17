use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    pub user: String,
    pub channel: String,
}
