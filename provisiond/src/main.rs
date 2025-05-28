use log::info;
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use libprovision::hello_world::{Greeter, GreeterClient, GreeterServer, HelloReply, HelloRequest};

#[derive(Debug, Default)]
struct GreeterServerImpl;

#[tonic::async_trait]
impl Greeter for GreeterServerImpl {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        info!("Got a request from {:?}", request.remote_addr());
        Ok(Response::new(HelloReply { message: "Hello world".into() }))
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> () {
    env_logger::init();

    let g = GreeterServerImpl::default();

   Server::builder()
       .add_service(GreeterServer::new(g))
       .serve(
           "[::1]:50051".parse().unwrap()
       ).await.expect("TODO: panic message");
}