use thiserror::Error;

pub type Result<T> = std::result::Result<T, MinecraftInstallerError>;

#[derive(Error, Debug)]
pub enum MinecraftInstallerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("ZIP extraction error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Invalid Minecraft version: {0}")]
    InvalidVersion(String),

    #[error("Invalid mod loader: {0}")]
    InvalidLoader(String),

    #[error("Download failed: {0}")]
    DownloadFailed(String),

    #[error("Installation failed: {0}")]
    InstallationFailed(String),

    #[error("Java installation failed: {0}")]
    JavaInstallationFailed(String),

    #[error("File system error: {0}")]
    FileSystem(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Validation error: {0}")]
    Validation(String),
}










