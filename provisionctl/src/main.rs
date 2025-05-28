mod cmd;
mod operations;

use clap::Parser;
use log::{log, Level};
use tonic::Request;
use libprovision::hello_world::{GreeterClient, HelloRequest};
use crate::cmd::Command as CmdArgs;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = CmdArgs::parse();
    log!(Level::Debug, "{:?}", args);

    let mut client = GreeterClient::connect("http://[::1]:50051").await.unwrap();

    let res = client.say_hello(Request::new(HelloRequest::default())).await.unwrap();

    log!(Level::Info, "{:?}", res);


}
