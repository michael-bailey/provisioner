use crate::io::file_manager::FileManager;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io};
use std::fs::File;

pub(crate) struct RealFileManager {
    root_path: PathBuf,
}

impl FileManager for RealFileManager {
    fn new(provision_path: &PathBuf) -> io::Result<Self> {
        if !provision_path.exists() {
            fs::create_dir_all(provision_path)?;
        }
        Ok(RealFileManager {
            root_path: provision_path.clone(),
        })
    }

    fn service_folder_exists(&self, service_name: String) -> bool {
        let service_folder = self.root_path.clone().join(service_name);
        service_folder.exists()
    }

    fn unit_file_exists(&self, service_name: String) -> bool {
        let unit_file = self
            .root_path
            .clone()
            .join(&service_name)
            .join(service_name + ".service");
        unit_file.exists()
    }

    fn env_file_exists(&self, service_name: String) -> bool {
        let env_file = self.root_path.clone().join(&service_name).join(".env");
        env_file.exists()
    }

    fn compose_file_exists(&self, service_name: String) -> bool {
        let compose_file = self
            .root_path
            .clone()
            .join(&service_name)
            .join("docker-compose.yaml");
        compose_file.exists()
    }

    fn create_service_folder(&self, service_name: String) -> io::Result<()> {
        let service_folder = self.root_path.clone().join(service_name);
        fs::create_dir_all(&service_folder)?;
        Ok(())
    }

    fn create_unit_file(&self, service_name: String) -> io::Result<()> {
        let unit_file = self
            .root_path
            .clone()
            .join(&service_name)
            .join(service_name + ".service");

        let mut file = fs::File::create(&unit_file)?;

        let contents = r#"
        [Servcie]
        Description=Hello world
        "#
        .trim()
        .as_bytes();

        file.write_all(contents)?;

        Ok(())
    }

    fn create_env_file(&self, service_name: String) -> io::Result<()> {
        let env_file = self.root_path.clone().join(&service_name).join(".env");
        let _ = fs::File::create(&env_file)?;
        Ok(())
    }

    fn create_compose_file(&self, service_name: String) -> io::Result<()> {
        let compose_file = self
            .root_path
            .clone()
            .join(&service_name)
            .join("docker-compose.yaml");
        let _ = fs::File::create(&compose_file)?;
        Ok(())
    }
}

impl Default for RealFileManager {
    fn default() -> Self {
        Self {
            root_path: PathBuf::from("/mnt/srv/"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn get_root_path(test_name: &str) -> PathBuf {
        let path = PathBuf::from(format!("/tmp/provisiond_tests/{}", test_name));
        path
    }

    #[test]
    pub fn test_real_file_manager_created() {
        let path = get_root_path("test_real_file_manager_created");

        let fm = RealFileManager::new(&path);

        assert!(
            fm.is_ok(),
            "Failed to create file manager with path at {} with error {:?}",
            path.display(),
            fm.err()
        );

        let _ = fm.expect("Should be ok");

        assert!(path.exists(), "Path at {} Does not exist", path.display());

        fs::remove_dir(&path)
            .expect(format!("Failed to delete test path at {}", path.display()).as_str());
    }

    #[test]
    // todo: try remove clones
    pub fn test_new_service_folder_created() {
        let path = get_root_path("test_new_service_folder_created");
        let service_name = "test_service".to_owned();

        let fm = RealFileManager::new(&path).expect("Failed to create file manager");

        let exists = fm.service_folder_exists(service_name.clone());

        assert!(!exists, "Service folder should not exist yet");

        fm.create_service_folder(service_name.clone())
            .expect("Failed to create service folder");

        let exists = fm.service_folder_exists(service_name.clone());

        assert!(exists, "Service folder should now exist");

        fs::remove_dir_all(&path)
            .expect(format!("Failed to delete test path at {}", path.display()).as_str());
    }

    #[test]
    pub fn test_systemd_unit_file_created() {
        let path = get_root_path("test_systemd_unit_file_created");
        let service_name = "test_service".to_owned();

        let fm = RealFileManager::new(&path).expect("Failed to create file manager");
        fm.create_service_folder(service_name.clone())
            .expect("Failed to create service folder");

        let exists = fm.unit_file_exists(service_name.clone());

        assert!(!exists, "Service folder should not exist yet");

        fm.create_unit_file(service_name.clone())
            .expect("Failed to create unit folder");

        let exists = fm.unit_file_exists(service_name.clone());

        assert!(exists, "Unit file should exist");

        fs::remove_dir_all(&path)
            .expect(format!("Failed to delete test path at {}", path.display()).as_str());
    }

    #[test]
    pub fn test_env_file_created() {
        let path = get_root_path("test_env_file_created");
        let service_name = "test_service".to_owned();

        let fm = RealFileManager::new(&path).expect("Failed to create file manager");
        fm.create_service_folder(service_name.clone())
            .expect("Failed to create service folder");

        let exists = fm.env_file_exists(service_name.clone());

        assert!(!exists, "Env file should not exist yet");

        fm.create_env_file(service_name.clone())
            .expect("Failed to create env file");

        let exists = fm.env_file_exists(service_name.clone());

        assert!(exists, "Env file should now exist");

        fs::remove_dir_all(&path)
            .expect(format!("Failed to delete test path at {}", path.display()).as_str());
    }

    #[test]
    pub fn test_compose_file_created() {
        let path = get_root_path("test_compose_file_created");
        let service_name = "test_service".to_owned();

        let fm = RealFileManager::new(&path).expect("Failed to create file manager");
        fm.create_service_folder(service_name.clone())
            .expect("Failed to create service folder");

        let exists = fm.compose_file_exists(service_name.clone());

        assert!(!exists, "Compose file should not exist yet");

        fm.create_compose_file(service_name.clone())
            .expect("Failed to create compose file");

        let exists = fm.compose_file_exists(service_name.clone());

        assert!(exists, "Compose file should exist");

        fs::remove_dir_all(&path)
            .expect(format!("Failed to delete test path at {}", path.display()).as_str());
    }
}
