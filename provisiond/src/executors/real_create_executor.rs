use crate::executors::create_error_type::CreateErrorType;
use crate::executors::{CreateExecutor, CreateExecutorError};
use std::fs::{File, create_dir};
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use log::info;
use tonic::async_trait;

#[derive(Default)]
pub struct RealCreateExecutor;

#[async_trait]
impl CreateExecutor for RealCreateExecutor {
    fn create_folder(&self, service_name: String) -> Result<(), CreateExecutorError> {
        let folder_path = PathBuf::from(format!("/mnt/srv/{}", service_name));
        info!("checking for folder at path {}", folder_path.display());

        if folder_path.exists() {
            info!("Folder {} already exists", folder_path.display());
            return Err(CreateExecutorError::new(
                CreateErrorType::FolderExists,
                format!("the folder at '{}' already exists", service_name).to_string(),
            ));
        }

        info!("Creating folder {}", folder_path.display());

        create_dir(&folder_path).map_err(|e| {
            CreateExecutorError::new(CreateErrorType::FolderCreateFailed, e.to_string())
        })?;

        info!("Created folder {}", folder_path.display());

        Ok(())
    }

    fn create_compose_file(&self, service_name: String) -> Result<(), CreateExecutorError> {
        let docker_compose_content = include_str!("../../res/template/docker-compose.yaml");
        let docker_compose_file =
            PathBuf::from(format!("/mnt/srv/{}/docker-compose.yaml", service_name));

        let mut docker_compose_file =
            File::create(docker_compose_file).map_err(|err| match err.kind() {
                ErrorKind::AlreadyExists => CreateExecutorError::new(
                    CreateErrorType::ComposeFileExists,
                    "docker compose file already exists?".to_owned(),
                ),
                _ => CreateExecutorError::new(CreateErrorType::OtherIO, err.to_string()),
            })?;

        docker_compose_file
            .write(docker_compose_content.as_bytes())
            .map_err(|err| match err.kind() {
                ErrorKind::PermissionDenied => CreateExecutorError::new(
                    CreateErrorType::PermissionError,
                    "failed to write to docker compose due to incorrect permissions".to_owned(),
                ),
                _ => CreateExecutorError::new(CreateErrorType::OtherIO, err.to_string()),
            })
            .map(|_| ())
    }

    fn create_env_file(&self, service_name: String) -> Result<(), CreateExecutorError> {
        let env_content = format!(
            include_str!("../../res/template/env"),
            format!("{}-db", service_name),
            format!("{}-service", service_name),
            format!("{}-password", service_name)
        );

        let env_file = PathBuf::from(format!("/mnt/srv/{}/.env", service_name));

        let mut env_file = File::create(env_file).map_err(|err| match err.kind() {
            ErrorKind::AlreadyExists => CreateExecutorError::new(
                CreateErrorType::EnvFileExists,
                "Env file already exists?".to_owned(),
            ),
            _ => CreateExecutorError::new(CreateErrorType::FileCreateFailed, err.to_string()),
        })?;

        env_file
            .write_all(env_content.as_bytes())
            .map_err(|err| match err.kind() {
                ErrorKind::PermissionDenied => CreateExecutorError::new(
                    CreateErrorType::PermissionError,
                    "failed to write to docker compose due to incorrect permissions".to_owned(),
                ),
                _ => CreateExecutorError::new(CreateErrorType::FileWriteFailed, err.to_string()),
            })
            .map(|_| ())
    }

    fn create_systemd_unit(&self, service_name: String) -> Result<(), CreateExecutorError> {
        let unit_file_content = format!(
            include_str!("../../res/template/unit.service"),
            service_name = service_name,
        );

        let unit_file_path = PathBuf::from(format!(
            "/mnt/srv/{}/{}.service",
            service_name, service_name
        ));

        if unit_file_path.exists() {
            return Err(CreateExecutorError::new(
                CreateErrorType::UnitFileExists,
                format!(
                    "The unit file at '{}' already exists",
                    unit_file_path.to_str().unwrap()
                ),
            ));
        }

        let mut unit_file = File::create(&unit_file_path).map_err(|err| match err.kind() {
            ErrorKind::AlreadyExists => CreateExecutorError::new(
                CreateErrorType::UnitFileExists,
                format!(
                    "The unit file at '{}' already exists",
                    unit_file_path.to_str().unwrap()
                ),
            ),
            _ => CreateExecutorError::new(CreateErrorType::OtherIO, err.to_string()),
        })?;

        unit_file
            .write(unit_file_content.as_bytes())
            .map_err(|err| match err.kind() {
                ErrorKind::PermissionDenied => CreateExecutorError::new(
                    CreateErrorType::PermissionError,
                    "failed to write to docker compose due to incorrect permissions".to_owned(),
                ),
                _ => CreateExecutorError::new(CreateErrorType::OtherIO, err.to_string()),
            })
            .map(|_| Ok(()))?
    }
}
