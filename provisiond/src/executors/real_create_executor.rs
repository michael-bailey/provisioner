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
        info!("Checking for folder at path {}", folder_path.display());

        let display_path = folder_path.display().to_string();

        if folder_path.exists() {

            info!("Folder {display_path} already exists", display_path = display_path);

            return Err(CreateExecutorError::new(
                CreateErrorType::FolderExists,
                format!("the folder at '{service_name}' already exists", service_name = service_name).to_string(),
            ));
        }

        info!("Creating folder at {display_path}", display_path = display_path);

        create_dir(&folder_path).map_err(|e| {
            CreateExecutorError::new(CreateErrorType::FolderCreateFailed, e.to_string())
        })?;

        info!("Created folder {display_path}", display_path = display_path);

        Ok(())
    }

    fn create_compose_file(&self, service_name: String) -> Result<(), CreateExecutorError> {
        let docker_compose_content = include_str!("../../res/template/docker-compose.yaml");
        let docker_compose_file =
            PathBuf::from(format!("/mnt/srv/{}/docker-compose.yaml", service_name));

        let display_path = docker_compose_file.display().to_string();

        info!("Checking for compose file at path {display_path}", display_path = display_path);

        let mut docker_compose_file =
            File::create(docker_compose_file).map_err(|err| match err.kind() {
                ErrorKind::AlreadyExists => {

                    info!("The docker compose at '{display_path}' already exists", display_path = display_path);

                    CreateExecutorError::new(
                        CreateErrorType::ComposeFileExists,
                        "Docker compose file already exists?".to_owned(),
                    )
                },
                _ => CreateExecutorError::new(CreateErrorType::OtherIO, err.to_string()),
            })?;

        docker_compose_file
            .write(docker_compose_content.as_bytes())
            .map_err(|err| match err.kind() {
                ErrorKind::PermissionDenied => {

                    info!("Permission checks failed for file '{display_path}' please check SUID", display_path = display_path);

                    CreateExecutorError::new(
                        CreateErrorType::PermissionError,
                        "Failed to write to docker compose due to incorrect permissions".to_owned(),
                    )
                }
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

        let display_path = env_file.display().to_string();

        info!("Checking for compose file at path {display_path}", display_path = display_path);

        let mut env_file = File::create(env_file).map_err(|err| match err.kind() {
            ErrorKind::AlreadyExists => {

                info!("The env file at '{display_path}' already exists", display_path = display_path);

                CreateExecutorError::new(
                    CreateErrorType::EnvFileExists,
                    "Env file already exists?".to_owned(),
                )
            },
            _ => CreateExecutorError::new(CreateErrorType::FileCreateFailed, err.to_string()),
        })?;

        env_file
            .write_all(env_content.as_bytes())
            .map_err(|err| match err.kind() {
                ErrorKind::PermissionDenied => {

                    info!("Permission checks failed for file '{display_path}' please check SUID", display_path = display_path);

                    CreateExecutorError::new(
                        CreateErrorType::PermissionError,
                        "Failed to write to docker compose due to incorrect permissions".to_owned(),
                    )
                },
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

        let display_path = unit_file_path.display().to_string();

        info!("Checking for compose file at path {display_path}", display_path = display_path);
        
        let mut unit_file = File::create(&unit_file_path).map_err(|err| match err.kind() {
            ErrorKind::AlreadyExists => {

                info!("The env file at '{display_path}' already exists", display_path = display_path);

                CreateExecutorError::new(
                    CreateErrorType::UnitFileExists,
                    format!(
                        "The unit file at '{}' already exists",
                        unit_file_path.to_str().unwrap()
                    ),
                )
            },
            _ => CreateExecutorError::new(CreateErrorType::OtherIO, err.to_string()),
        })?;

        unit_file
            .write(unit_file_content.as_bytes())
            .map_err(|err| match err.kind() {
                ErrorKind::PermissionDenied => {
                    
                    info!("Permission checks failed for file '{display_path}' please check SUID", display_path = display_path);
                    
                    CreateExecutorError::new(
                        CreateErrorType::PermissionError,
                        "Failed to write to docker compose due to incorrect permissions".to_owned(),
                    )
                },
                _ => CreateExecutorError::new(CreateErrorType::OtherIO, err.to_string()),
            })
            .map(|_| Ok(()))?
    }
}
