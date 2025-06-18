use libprovision::hello_world::{ProvisionerClient, RestartRequest};
use log::info;
use tonic::Request;
use tonic::transport::Channel;

pub async fn handle_restart(client: &mut ProvisionerClient<Channel>) {
    info!("handling restart request");

    let res = client
        .restart(Request::new(RestartRequest::default()))
        .await
        .unwrap();
    info!("got restart response {:?}", res.get_ref());
}
