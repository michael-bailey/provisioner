use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CreateErrorType {
    FolderExists,
    ComposeFileExists,
    EnvFileExists,
    UnitFileExists,
    
    FolderCreateFailed,
    FileCreateFailed,
    FileWriteFailed,

    PermissionError,
    
    OtherIO,
}

impl Display for CreateErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use CreateErrorType::*;

        match self {
            FolderExists => write!(f, "Folder already exists"),
            ComposeFileExists => write!(f, "Compose file already exists"),
            EnvFileExists => write!(f, "Env file already exists"),
            UnitFileExists => write!(f, "Unit file already exists"),

            PermissionError => write!(f, "Permission Error"),

            FolderCreateFailed => write!(f, "Folder creation failed"),

            FileCreateFailed => write!(f, "File creation failed"),
            FileWriteFailed => write!(f, "File write failed"),

            OtherIO => write!(f, "IO operation failed"),
        }
    }
}