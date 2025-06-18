use libprovision::hello_world::{ProvisionerClient, PullRequest};
use log::info;
use tonic::Request;
use tonic::transport::Channel;

pub async fn handle_pull(client: &mut ProvisionerClient<Channel>) {
    info!("handling pull request");

    let res = client
        .pull(Request::new(PullRequest::default()))
        .await
        .unwrap();
    info!("got pull response {:?}", res.get_ref());
}
