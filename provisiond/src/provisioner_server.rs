use std::collections::VecDeque;
use std::sync::Arc;

use log::info;
use tonic::{Code, Request, Response, Status};

use libprovision::hello_world::{
    CreateRequest, CreateResponse, DeleteRequest, DeleteResponse, Provisioner, PullRequest,
    PullResponse, RestartRequest, RestartResponse,
};

use crate::executors::{CreateExecutorError, DeleteExecutorError, RealCreateExecutor};
use crate::executors::RealDeleteExecutor;
use crate::executors::{CreateExecutor, DeleteExecutor};

pub struct ProvisionerImpl {
    create_executor: Arc<dyn CreateExecutor + Send + Sync>,
    delete_executor: Arc<dyn DeleteExecutor + Send + Sync>,
}

impl ProvisionerImpl {
    pub(crate) fn unwind(
        &self,
        service_name: String,
        queue: VecDeque<Box<dyn FnOnce(String) -> () + Send>>,
    ) -> Result<(), Status> {
        for f in queue.into_iter().rev() {
            f(service_name.clone());
        }

        Ok(())
    }

    fn run_step(
        &self,
        service_name: &String,
        step_name: &'static str,
        undo_stack: &mut VecDeque<Box<dyn FnOnce(String) + Send>>,
        step_fn: &dyn Fn(String) -> Result<(), CreateExecutorError>,
        inverse_fn: &dyn Fn(String) -> Result<(), DeleteExecutorError>,
    ) -> Option<Result<Response<CreateResponse>, Status>> {

        info!("Running step '{step_name}' for: {service_name}", step_name = step_name, service_name = service_name);

        let name = service_name.clone();
        let step_res = step_fn(name);

        if let Err(e) = step_res {
            info!(
                "Step '{step_name}' failed unwinding stack for {service_name} got error {err}",
                step_name=  step_name,
                service_name = service_name,
                err = e.to_string()
            );
            return Some(Err(Status::new(
                Code::Internal,
                format!("Error Running step '{step_name}' for {service_name} got error {err}", step_name = step_name, service_name = service_name,err = e.to_string()),

            )));
        }

        info!("Appending undo folder for: {}", service_name);

        let delete_executor = self.delete_executor.clone();
        undo_stack.push_back(Box::new(move |name| {
            info!("Deleting folder for: {}", name);
            delete_executor.delete_folder(name).unwrap();
        }));

        None
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
        let mut undo_stack: VecDeque<Box<dyn FnOnce(String) -> () + Send>> = VecDeque::new();

        // Create folder step
        if let Some(value) = self.run_step(
            &service_name,
            "Create Folder",
            &mut undo_stack,
            & |n| { self.create_executor.create_folder(n) },
            & |n| { self.delete_executor.delete_folder(n) }
        ) {
            self.unwind(service_name, undo_stack)?;
            return value;
        }

        // Create compose file step
        if let Some(value) = self.run_step(
            &service_name,
            "Create Compose File",
            &mut undo_stack,
            & |n| { self.create_executor.create_compose_file(n) },
            & |n| { self.delete_executor.delete_compose_file(n) }
        ) {
            self.unwind(service_name, undo_stack)?;
            return value;
        }

        // Create env file step
        if let Some(value) = self.run_step(
            &service_name,
            "Create Compose File",
            &mut undo_stack,
            & |n| { self.create_executor.create_env_file(n) },
            & |n| { self.delete_executor.delete_env_file(n) }
        ) {
            self.unwind(service_name, undo_stack)?;
            return value;
        }

        // Create unit file step
        if let Some(value) = self.run_step(
            &service_name,
            "Create Compose File",
            &mut undo_stack,
            & |n| { self.create_executor.create_systemd_unit(n) },
            & |n| { self.delete_executor.delete_systemd_unit(n) }
        ) {
            self.unwind(service_name, undo_stack)?;
            return value;
        }
        
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
