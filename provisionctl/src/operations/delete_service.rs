use libprovision::hello_world::{DeleteRequest, ProvisionerClient};
use log::info;
use tonic::Request;
use tonic::transport::Channel;

pub async fn handle_delete(client: &mut ProvisionerClient<Channel>) {
    info!("handling delete request");

    let res = client
        .delete(Request::new(DeleteRequest::default()))
        .await
        .unwrap();
    info!("got delete response {:?}", res.get_ref());
}
