use libprovision::hello_world::{
    CreateRequest, CreateResponse, DeleteRequest, DeleteResponse, Provisioner, PullRequest,
    PullResponse, RestartRequest, RestartResponse,
};
use log::info;
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct ProvisionerImpl;

#[tonic::async_trait]
impl Provisioner for ProvisionerImpl {
    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        info!("got create request: {:?}", request.get_ref());
        Ok(Response::new(CreateResponse {}))
    }

    async fn restart(
        &self,
        request: Request<RestartRequest>,
    ) -> Result<Response<RestartResponse>, Status> {
        info!("got restart request: {:?}", request.get_ref());
        Ok(Response::new(RestartResponse {}))
    }

    async fn pull(&self, request: Request<PullRequest>) -> Result<Response<PullResponse>, Status> {
        info!("got pull request: {:?}", request.get_ref());
        Ok(Response::new(PullResponse {}))
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        info!("got delete request: {:?}", request.get_ref());
        Ok(Response::new(DeleteResponse {}))
    }
}
