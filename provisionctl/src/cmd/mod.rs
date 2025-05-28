use clap::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Command {

    #[arg()]
    pub name: String,

    #[command(subcommand)]
    pub command: Commands,

}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Create,
    Restart,
    Pull,
    Delete,
}