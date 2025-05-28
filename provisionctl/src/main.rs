mod cmd;
mod operations;

use crate::cmd::{Command as CmdArgs, Commands};
use clap::Parser;
use libprovision::hello_world::{
    CreateRequest, DeleteRequest, ProvisionerClient, PullRequest,
    RestartRequest,
};
use log::{Level, info, log};
use tonic::Request;
use tonic::transport::Channel;

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
        Commands::Create => handle_create(&mut client).await,
        Commands::Restart => handle_restart(&mut client).await,
        Commands::Pull => handle_pull(&mut client).await,
        Commands::Delete => handle_delete(&mut client).await,
    }
}

async fn handle_create(client: &mut ProvisionerClient<Channel>) {
    info!("handling create request");
    let res = client
        .create(Request::new(CreateRequest::default()))
        .await
        .unwrap();

    info!("got create response {:?}", res.get_ref());
}

async fn handle_restart(client: &mut ProvisionerClient<Channel>) {
    info!("handling restart request");

    let res = client
        .restart(Request::new(RestartRequest::default()))
        .await
        .unwrap();
    info!("got restart response {:?}", res.get_ref());
}

async fn handle_pull(client: &mut ProvisionerClient<Channel>) {
    info!("handling pull request");

    let res = client
        .pull(Request::new(PullRequest::default()))
        .await
        .unwrap();
    info!("got pull response {:?}", res.get_ref());
}

async fn handle_delete(client: &mut ProvisionerClient<Channel>) {
    info!("handling delete request");

    let res = client
        .delete(Request::new(DeleteRequest::default()))
        .await
        .unwrap();
    info!("got delete response {:?}", res.get_ref());
}
