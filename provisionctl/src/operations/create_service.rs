use libprovision::hello_world::{CreateRequest, ProvisionerClient};
use log::info;
use tonic::Request;
use tonic::transport::Channel;

pub async fn handle_create(client: &mut ProvisionerClient<Channel>, service_name: String) {
    info!("handling create request");
    let res = client
        .create(Request::new(CreateRequest { service_name }))
        .await
        .unwrap();
    
    info!("got create response {:?}", res.get_ref());
}
