use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum DeleteErrorType {
    FolderDoesNotExist,
    ComposeFileDoesNotExist,
    EnvFileDoesNotExist,
    UnitFileDoesNotExist,
    
    FolderDeletionFailed,
    ComposeFileDeletionFailed,
    EnvFileDeletionFailed,
    UnitFileDeletionFailed,
    
}

impl Display for DeleteErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            DeleteErrorType::FolderDoesNotExist => String::from("Folder does not exist"),
            DeleteErrorType::ComposeFileDoesNotExist => String::from("Environment file does not exist"),
            DeleteErrorType::EnvFileDoesNotExist => String::from("Environment file does not exist"),
            DeleteErrorType::UnitFileDoesNotExist => String::from("Unit file does not exist"),

            DeleteErrorType::FolderDeletionFailed => String::from("Folder deletion failed"),
            DeleteErrorType::ComposeFileDeletionFailed => String::from("Compose file deletion failed"),
            DeleteErrorType::EnvFileDeletionFailed => String::from("Environment file deletion failed"),
            DeleteErrorType::UnitFileDeletionFailed => String::from("Unit file deletion failed"),
        };
        write!(f, "{msg}")
    }
}