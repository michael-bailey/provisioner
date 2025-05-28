mod provisioner_server;

use crate::provisioner_server::ProvisionerImpl;
use libprovision::hello_world::{
    Greeter, GreeterServer, HelloReply, HelloRequest, ProvisionerServer,
};
use log::info;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
struct GreeterServerImpl;

#[tonic::async_trait]
impl Greeter for GreeterServerImpl {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        info!("Got a request from {:?}", request.remote_addr());
        Ok(Response::new(HelloReply {
            message: "Hello world".into(),
        }))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> () {
    env_logger::init();

    let g = GreeterServerImpl::default();
    let provisioner_server = ProvisionerImpl::default();

    Server::builder()
        .add_service(GreeterServer::new(g))
        .add_service(ProvisionerServer::new(provisioner_server))
        .serve("[::1]:50051".parse().unwrap())
        .await
        .expect("TODO: panic message");
}
