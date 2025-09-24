use std::path::{Path, PathBuf};
use tokio::fs;
use crate::error::{MinecraftInstallerError, Result};

/// Directory structure manager for Minecraft installation
#[derive(Debug, Clone)]
pub struct DirectoryManager {
    pub base_dir: PathBuf,
}

impl DirectoryManager {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    /// Initialize all required directories
    pub async fn init(&self) -> Result<()> {
        let dirs_to_create = [
            self.base_dir.clone(),
            self.minecraft_dir(),
            self.versions_dir(),
            self.libraries_dir(),
            self.assets_dir(),
            self.assets_index_dir(),
            self.assets_objects_dir(),
            self.java_dir(),
            self.logs_dir(),
            self.instances_dir(),
        ];

        for dir in &dirs_to_create {
            fs::create_dir_all(dir).await.map_err(|e| {
                MinecraftInstallerError::FileSystem(format!(
                    "Failed to create directory {}: {}",
                    dir.display(),
                    e
                ))
            })?;
        }

        Ok(())
    }

    /// Get the main Minecraft directory
    pub fn minecraft_dir(&self) -> PathBuf {
        self.base_dir.join("minecraft")
    }

    /// Get the versions directory
    pub fn versions_dir(&self) -> PathBuf {
        self.minecraft_dir().join("versions")
    }

    /// Get the directory for a specific version
    pub fn version_dir(&self, version: &str) -> PathBuf {
        self.versions_dir().join(version)
    }

    /// Get the version JAR file path
    pub fn version_jar(&self, version: &str) -> PathBuf {
        self.version_dir(version).join(format!("{}.jar", version))
    }

    /// Get the version JSON file path
    pub fn version_json(&self, version: &str) -> PathBuf {
        self.version_dir(version).join(format!("{}.json", version))
    }

    /// Get the libraries directory
    pub fn libraries_dir(&self) -> PathBuf {
        self.minecraft_dir().join("libraries")
    }

    /// Get the assets directory
    pub fn assets_dir(&self) -> PathBuf {
        self.minecraft_dir().join("assets")
    }

    /// Get the assets index directory
    pub fn assets_index_dir(&self) -> PathBuf {
        self.assets_dir().join("indexes")
    }

    /// Get the assets objects directory
    pub fn assets_objects_dir(&self) -> PathBuf {
        self.assets_dir().join("objects")
    }

    /// Get the path for a specific asset object
    pub fn asset_object_path(&self, hash: &str) -> PathBuf {
        let prefix = &hash[..2];
        self.assets_objects_dir().join(prefix).join(hash)
    }

    /// Get the Java installations directory
    pub fn java_dir(&self) -> PathBuf {
        self.base_dir.join("java")
    }

    /// Get the Java installation directory for a specific version
    pub fn java_version_dir(&self, version: u32) -> PathBuf {
        self.java_dir().join(format!("java-{}", version))
    }

    /// Get the logs directory
    pub fn logs_dir(&self) -> PathBuf {
        self.base_dir.join("logs")
    }

    /// Get the instances directory (for game instances/profiles)
    pub fn instances_dir(&self) -> PathBuf {
        self.base_dir.join("instances")
    }

    /// Get the instance directory for a specific instance
    pub fn instance_dir(&self, instance_name: &str) -> PathBuf {
        self.instances_dir().join(instance_name)
    }

    /// Get the natives directory for a version
    pub fn natives_dir(&self, version: &str) -> PathBuf {
        self.version_dir(version).join("natives")
    }

    /// Get the launcher profiles file path
    pub fn launcher_profiles(&self) -> PathBuf {
        self.minecraft_dir().join("launcher_profiles.json")
    }

    /// Check if a version is already installed
    pub async fn is_version_installed(&self, version: &str) -> bool {
        let version_jar = self.version_jar(version);
        let version_json = self.version_json(version);

        version_jar.exists() && version_json.exists()
    }

    /// Get the size of a directory recursively
    pub async fn get_directory_size(&self, path: &Path) -> Result<u64> {
        self.get_directory_size_impl(path).await
    }

    fn get_directory_size_impl<'a>(&'a self, path: &'a Path) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<u64>> + 'a>> {
        Box::pin(async move {
            let mut size = 0;
            let mut entries = fs::read_dir(path).await.map_err(|e| {
                MinecraftInstallerError::FileSystem(format!(
                    "Failed to read directory {}: {}",
                    path.display(),
                    e
                ))
            })?;

            while let Some(entry) = entries.next_entry().await.map_err(|e| {
                MinecraftInstallerError::FileSystem(format!(
                    "Failed to read directory entry: {}",
                    e
                ))
            })? {
                let metadata = entry.metadata().await.map_err(|e| {
                    MinecraftInstallerError::FileSystem(format!(
                        "Failed to read metadata: {}",
                        e
                    ))
                })?;

                if metadata.is_dir() {
                    size += self.get_directory_size_impl(&entry.path()).await?;
                } else {
                    size += metadata.len();
                }
            }

            Ok(size)
        })
    }
}
