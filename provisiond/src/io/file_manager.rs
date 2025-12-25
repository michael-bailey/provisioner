use mockall::automock;
use tonic::async_trait;
use std::io;
use std::path::PathBuf;

#[automock]
#[async_trait]
pub(crate) trait FileManager {
    fn new(provision_path: &PathBuf) -> io::Result<Self> where Self: Sized;

    fn service_folder_exists(&self, service_name: String) -> bool;
    fn unit_file_exists(&self, service_name: String) -> bool;
    fn env_file_exists(&self, service_name: String) -> bool;
    fn compose_file_exists(&self, service_name: String) -> bool;

    fn create_service_folder(&self, service_name: String) -> io::Result<()>;
    fn create_unit_file(&self, service_name: String) -> io::Result<()>;
    fn create_env_file(&self, service_name: String) -> io::Result<()>;
    fn create_compose_file(&self, service_name: String) -> io::Result<()>;
}
