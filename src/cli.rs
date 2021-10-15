use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    pub channel: String,

    // TODO: Take in username argument when authentication
    // has been implimented.
    //
    // pub user: String,
}
