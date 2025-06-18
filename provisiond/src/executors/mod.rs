mod create_error_type;
mod delete_error_type;
mod executor_error;
mod real_create_executor;
mod real_delete_executor;

use executor_error::ExecutorError;
use tonic::async_trait;

use crate::executors::create_error_type::CreateErrorType;
use crate::executors::delete_error_type::DeleteErrorType;
pub use real_create_executor::RealCreateExecutor;
pub use real_delete_executor::RealDeleteExecutor;

pub type CreateExecutorError = ExecutorError<CreateErrorType>;
pub type DeleteExecutorError = ExecutorError<DeleteErrorType>;

#[async_trait]
pub trait CreateExecutor {
    fn create_folder(&self, service_name: String) -> Result<(), CreateExecutorError>;
    fn create_compose_file(&self, service_name: String) -> Result<(), CreateExecutorError>;
    fn create_env_file(&self, service_name: String) -> Result<(), CreateExecutorError>;
    fn create_systemd_unit(&self, service_name: String) -> Result<(), CreateExecutorError>;
}

#[async_trait]
pub trait DeleteExecutor {
    fn delete_folder(&self, service_name: String) -> Result<(), DeleteExecutorError>;
    fn delete_compose_file(&self, service_name: String) -> Result<(), DeleteExecutorError>;
    fn delete_env_file(&self, service_name: String) -> Result<(), DeleteExecutorError>;
    fn delete_systemd_unit(&self, service_name: String) -> Result<(), DeleteExecutorError>;
}
