mod cmd;
mod operations;

use crate::cmd::{Command as CmdArgs, Commands};
use clap::Parser;
use libprovision::hello_world::ProvisionerClient;
use log::{Level, info, log};

use crate::operations::{handle_create, handle_delete, handle_pull, handle_restart};

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = CmdArgs::parse();
    log!(Level::Debug, "{:?}", args);

    info!("Creating client for connection to server");
    let mut client = ProvisionerClient::connect("http://[::1]:50051")
        .await
        .unwrap();

    info!("Sending request");
    match args.command {
        Commands::Create => handle_create(&mut client, args.name).await,
        Commands::Restart => handle_restart(&mut client).await,
        Commands::Pull => handle_pull(&mut client).await,
        Commands::Delete => handle_delete(&mut client).await,
    }
}
