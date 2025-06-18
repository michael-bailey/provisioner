use std::sync::Arc;
use std::collections::VecDeque;

use log::info;
use tonic::{Code, Request, Response, Status};

use libprovision::hello_world::{
    CreateRequest, CreateResponse, DeleteRequest, DeleteResponse, Provisioner, PullRequest,
    PullResponse, RestartRequest, RestartResponse,
};

use crate::executors::RealCreateExecutor;
use crate::executors::RealDeleteExecutor;
use crate::executors::{CreateExecutor, DeleteExecutor};

pub struct ProvisionerImpl {
    create_executor: Arc<dyn CreateExecutor + Send + Sync>,
    delete_executor: Arc<dyn DeleteExecutor + Send + Sync>,
}

impl ProvisionerImpl {

    pub(crate) fn unwind(
        &self,
        queue: VecDeque<Box<dyn FnOnce() -> () + Send>>,
    ) -> Result<(), Status> {
        for f in queue.into_iter().rev() {
            f();
        }

        Ok(())
    }
}

impl Default for ProvisionerImpl {
    fn default() -> Self {
        Self {
            create_executor: Arc::new(RealCreateExecutor::default()),
            delete_executor: Arc::new(RealDeleteExecutor::default()),
        }
    }
}

#[tonic::async_trait]
impl Provisioner for ProvisionerImpl {
    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        let service_name = request.get_ref().clone().service_name;
        info!(
            "Got create request to make service with name: {}",
            service_name
        );

        info!("Creating undo queue for: {}", service_name);
        let mut undo_stack: VecDeque<Box<dyn FnOnce() -> () + Send>> = VecDeque::new();


        info!("Creating folder for: {}", service_name);
        let name = service_name.clone();
        let folder_res = self.create_executor.create_folder(name);

        if let Err(e) = folder_res {
            info!("Folder creation failed unwinding stack for {} got error {}", service_name, e.to_string());
            let _ = self.unwind(undo_stack);
            return Err(Status::new(Code::Internal, "Error creating service folder"));
        }

        info!("Appending undo folder for: {}", service_name);
        let name = service_name.clone();
        let delete_executor = self.delete_executor.clone();
        undo_stack.push_back(Box::new(move || {
            info!("Deleting folder for: {}", name);
            delete_executor.delete_folder(name).unwrap();
        }));


        info!("Creating compose file for: {}", service_name);
        let name = service_name.clone();
        let compose_file_res = self.create_executor.create_compose_file(name);

        if let Err(e) = compose_file_res {
            info!("Compose file creation failed unwinding stack for {} got error {}", service_name, e.to_string());
            let _ = self.unwind(undo_stack);
            return Err(Status::new(Code::Internal, "Error creating compose file"));
        }

        info!("Appending undo compose file for: {}", service_name);
        let name = service_name.clone();
        let delete_executor = self.delete_executor.clone();
        undo_stack.push_back(Box::new(move || {
            info!("Deleting compose file for: {}", name);
            delete_executor.delete_compose_file(name).unwrap();
        }));


        info!("Creating env file for: {}", service_name);
        let name = service_name.clone();
        let env_file_res = self.create_executor.create_env_file(name);

        if let Err(e) = env_file_res {
            info!("Env file creation failed unwinding stack for {} got error {}", service_name, e.to_string());
            let _ = self.unwind(undo_stack);
            return Err(Status::new(Code::Internal, "Error creating compose file"));
        }

        info!("Appending undo env file for: {}", service_name);
        let name = service_name.clone();
        let delete_executor = self.delete_executor.clone();
        undo_stack.push_back(Box::new(move || {
            info!("Deleting env file for: {}", name);
            delete_executor.delete_env_file(name).unwrap();
        }));


        info!("Creating unit file for: {}", service_name);
        let name = service_name.clone();
        let unit_file_res = self.create_executor.create_systemd_unit(name);

        if let Err(e) = unit_file_res {
            info!("Unit file creation failed unwinding stack for {} got error {}", service_name, e.to_string());
            let _ = self.unwind(undo_stack);
            return Err(Status::new(Code::Internal, "Error creating compose file"));
        }

        info!("Appending undo unit file for: {}", service_name);
        let name = service_name.clone();
        let delete_executor = self.delete_executor.clone();
        undo_stack.push_back(Box::new(move || {
            info!("deleting unit file for: {}", name);
            delete_executor.delete_systemd_unit(name).unwrap();
        }));


        info!("Created service folder for: {}", service_name);
        Ok(Response::new(CreateResponse {}))
    }

    async fn restart(
        &self,
        request: Request<RestartRequest>,
    ) -> Result<Response<RestartResponse>, Status> {
        info!("Got restart request: {:?}", request.get_ref());
        Ok(Response::new(RestartResponse {}))
    }

    async fn pull(&self, request: Request<PullRequest>) -> Result<Response<PullResponse>, Status> {
        info!("Got pull request: {:?}", request.get_ref());
        Ok(Response::new(PullResponse {}))
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        info!("Got delete request: {:?}", request.get_ref());
        Ok(Response::new(DeleteResponse {}))
    }
}
