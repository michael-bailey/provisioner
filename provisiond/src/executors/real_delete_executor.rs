use crate::executors::delete_error_type::DeleteErrorType;
use crate::executors::{DeleteExecutor, DeleteExecutorError};
use log::info;
use tonic::async_trait;

#[derive(Default)]
pub struct RealDeleteExecutor;

#[async_trait]
impl DeleteExecutor for RealDeleteExecutor {
    fn delete_folder(&self, service_name: String) -> Result<(), DeleteExecutorError> {
        use std::fs::remove_dir_all;
        use std::path::PathBuf;

        let folder_path = PathBuf::from(format!("/mnt/srv/{}", service_name));
        info!("Deleting folder {}", folder_path.display());

        if !folder_path.exists() {
            info!("Folder {} already exists", folder_path.display());
            return Err(DeleteExecutorError::new(
                DeleteErrorType::FolderDoesNotExist,
                format!("the folder at '{}' does not exist", folder_path.display()).to_string(),
            ));
        }

        remove_dir_all(&folder_path).map_err(|e| {
            DeleteExecutorError::new(
                DeleteErrorType::FolderDeletionFailed,
                format!("Failed to delete folder '{}' with error: {}", folder_path.display(), e.to_string()),
            )
        })?;

        Ok(())
    }

    fn delete_compose_file(&self, service_name: String) -> Result<(), DeleteExecutorError> {
        use std::fs::remove_file;
        use std::path::PathBuf;

        let file_path = PathBuf::from(format!("/mnt/srv/{}/docker-compose.yaml", service_name));
        info!("Deleting compose file {}", file_path.display());

        if !file_path.exists() {
            return Err(DeleteExecutorError::new(
                DeleteErrorType::ComposeFileDoesNotExist,
                format!(
                    "the compose file at '{}' does not exist",
                    file_path.display()
                )
                .to_string(),
            ));
        }

        remove_file(&file_path).map_err(|e| {
            DeleteExecutorError::new(
                DeleteErrorType::ComposeFileDeletionFailed,
                format!("Failed to delete compose file at '{}' with error: {}", file_path.display(), e.to_string()),
            )
        })?;

        Ok(())
    }

    fn delete_env_file(&self, service_name: String) -> Result<(), DeleteExecutorError> {
        use std::fs::remove_file;
        use std::path::PathBuf;

        let file_path = PathBuf::from(format!("/mnt/srv/{}/.env", service_name));
        info!("Deleting env file {}", file_path.display());

        if !file_path.exists() {
            return Err(DeleteExecutorError::new(
                DeleteErrorType::EnvFileDoesNotExist,
                format!(
                    "the env file at '{}' does not exist",
                    file_path.display()
                ),
            ));
        }

        remove_file(&file_path).map_err(|e| {
            DeleteExecutorError::new(
                DeleteErrorType::EnvFileDeletionFailed,
                format!("Failed to delete compose file at '{}' with error: {}", file_path.display(), e.to_string()),
            )
        })?;

        Ok(())
    }

    fn delete_systemd_unit(&self, service_name: String) -> Result<(), DeleteExecutorError> {
        use std::fs::remove_file;
        use std::path::PathBuf;

        let file_path = PathBuf::from(format!(
            "/mnt/srv/{}/{}.service",
            service_name, service_name
        ));
        info!("Deleting systemd unit {}", file_path.display());

        if !file_path.exists() {
            return Err(DeleteExecutorError::new(
                DeleteErrorType::UnitFileDoesNotExist,
                format!(
                    "the unit file at '{}' does not exist",
                    file_path.display()
                ),
            ));
        }

        remove_file(&file_path).map_err(|e| {
            DeleteExecutorError::new(
                DeleteErrorType::UnitFileDeletionFailed,
                format!("Failed to delete unit file at '{}' with error: {}", file_path.display(), e.to_string()),
            )
        })?;

        Ok(())
    }
}
