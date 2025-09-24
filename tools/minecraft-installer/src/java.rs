use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::Command;
use tokio::fs;
use reqwest::Client;
use serde::Deserialize;
use tracing::{debug, info, warn};
use indicatif::{ProgressBar, ProgressStyle};

use crate::error::{MinecraftInstallerError, Result};
use crate::directories::DirectoryManager;

#[derive(Deserialize, Debug)]
struct AdoptiumRelease {
    binaries: Vec<AdoptiumBinary>,
}

#[derive(Deserialize, Debug)]
struct AdoptiumBinary {
    architecture: String,
    image_type: String,
    os: String,
    package: AdoptiumPackage,
}

#[derive(Deserialize, Debug, Clone)]
struct AdoptiumPackage {
    name: String,
    link: String,
    size: u64,
}

/// Java installation manager
pub struct JavaManager {
    client: Client,
    dirs: DirectoryManager,
}

impl JavaManager {
    pub fn new(dirs: DirectoryManager) -> Self {
        let client = Client::builder()
            .user_agent("MinecraftInstaller/0.1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self { client, dirs }
    }

    /// Check if Java is installed and get its version
    pub async fn check_java(&self, java_path: Option<&Path>) -> Result<Option<(PathBuf, u32)>> {
        let java_executable = if let Some(path) = java_path {
            path.to_path_buf()
        } else {
            // Try to find Java in system PATH
            self.find_system_java().await?
        };

        if !java_executable.exists() {
            return Ok(None);
        }

        // Check Java version
        let output = Command::new(&java_executable)
            .args(["-version"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            return Ok(None);
        }

        // Parse version from output
        let version_output = String::from_utf8_lossy(&output.stderr);
        let version = self.parse_java_version(&version_output)?;

        Ok(Some((java_executable, version)))
    }

    /// Find Java in system PATH
    async fn find_system_java(&self) -> Result<PathBuf> {
        let java_executable = if cfg!(target_os = "windows") {
            "java.exe"
        } else {
            "java"
        };

        // Try common locations
        let common_paths = if cfg!(target_os = "windows") {
            vec![
                PathBuf::from("java.exe"),
                PathBuf::from(r"C:\Program Files\Java\jre\bin\java.exe"),
                PathBuf::from(r"C:\Program Files (x86)\Java\jre\bin\java.exe"),
            ]
        } else if cfg!(target_os = "macos") {
            vec![
                PathBuf::from("java"),
                PathBuf::from("/usr/bin/java"),
                PathBuf::from("/Library/Java/JavaVirtualMachines/jdk/bin/java"),
            ]
        } else {
            vec![
                PathBuf::from("java"),
                PathBuf::from("/usr/bin/java"),
                PathBuf::from("/usr/lib/jvm/default-java/bin/java"),
            ]
        };

        for path in common_paths {
            if let Ok(output) = Command::new(&path)
                .args(["-version"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .output()
                .await
            {
                if output.status.success() {
                    return Ok(path);
                }
            }
        }

        // Try to find via 'which' command
        if let Ok(output) = Command::new("which")
            .arg(java_executable)
            .output()
            .await
        {
            if output.status.success() {
                let path_str = String::from_utf8_lossy(&output.stdout);
                let trimmed = path_str.trim();
                if !trimmed.is_empty() {
                    return Ok(PathBuf::from(trimmed));
                }
            }
        }

        Err(MinecraftInstallerError::JavaInstallationFailed(
            "Java not found in system PATH".to_string(),
        ))
    }

    /// Parse Java version from version output
    fn parse_java_version(&self, version_output: &str) -> Result<u32> {
        // Look for version pattern like "1.8.0_XXX", "11.0.X", "17.0.X", etc.
        for line in version_output.lines() {
            if line.contains("version") {
                // Extract version string between quotes
                if let Some(start) = line.find('"') {
                    if let Some(end) = line[start + 1..].find('"') {
                        let version_str = &line[start + 1..start + 1 + end];

                        // Parse version number
                        if version_str.starts_with("1.") {
                            // Old format: 1.8.0_XXX -> version 8
                            if let Some(major) = version_str.chars().nth(2) {
                                if let Some(version) = major.to_digit(10) {
                                    return Ok(version);
                                }
                            }
                        } else {
                            // New format: 11.0.X, 17.0.X -> version 11, 17
                            if let Some(dot_pos) = version_str.find('.') {
                                if let Ok(version) = version_str[..dot_pos].parse::<u32>() {
                                    return Ok(version);
                                }
                            }
                        }
                    }
                }
            }
        }

        Err(MinecraftInstallerError::JavaInstallationFailed(
            "Could not parse Java version".to_string(),
        ))
    }

    /// Install Java if needed
    pub async fn ensure_java(&self, required_version: u32) -> Result<PathBuf> {
        info!("Checking Java installation...");

        // Check if we already have the right version installed
        let java_dir = self.dirs.java_version_dir(required_version);
        let java_executable = if cfg!(target_os = "windows") {
            java_dir.join("bin").join("java.exe")
        } else {
            java_dir.join("bin").join("java")
        };

        if let Ok(Some((path, version))) = self.check_java(Some(&java_executable)).await {
            if version >= required_version {
                info!("Java {} already installed at {}", version, path.display());
                return Ok(path);
            }
        }

        // Check system Java
        if let Ok(Some((path, version))) = self.check_java(None).await {
            if version >= required_version {
                info!("Using system Java {} at {}", version, path.display());
                return Ok(path);
            }
        }

        // Install Java
        info!("Installing Java {}...", required_version);
        self.install_java(required_version).await?;

        // Verify installation
        if let Ok(Some((path, version))) = self.check_java(Some(&java_executable)).await {
            info!("Java {} successfully installed at {}", version, path.display());
            Ok(path)
        } else {
            Err(MinecraftInstallerError::JavaInstallationFailed(
                "Failed to verify Java installation".to_string(),
            ))
        }
    }

    /// Install Java from Adoptium
    async fn install_java(&self, version: u32) -> Result<()> {
        info!("Downloading Java {} from Adoptium...", version);

        // Get download URL
        let download_info = self.get_java_download_url(version).await?;

        // Create installation directory
        let install_dir = self.dirs.java_version_dir(version);
        fs::create_dir_all(&install_dir).await?;

        // Download Java
        let temp_file = install_dir.join("java_installer.tmp");
        self.download_java(&download_info.link, &temp_file, download_info.size).await?;

        // Extract Java
        self.extract_java(&temp_file, &install_dir).await?;

        // Clean up temporary file
        fs::remove_file(&temp_file).await?;

        info!("Java {} installation completed", version);
        Ok(())
    }

    /// Get Java download URL from Adoptium API
    async fn get_java_download_url(&self, version: u32) -> Result<AdoptiumPackage> {
        let os = if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "macos") {
            "mac"
        } else {
            "linux"
        };

        let arch = if cfg!(target_arch = "x86_64") {
            "x64"
        } else if cfg!(target_arch = "aarch64") {
            "aarch64"
        } else {
            return Err(MinecraftInstallerError::JavaInstallationFailed(
                format!("Unsupported architecture: {}", std::env::consts::ARCH),
            ));
        };

        let url = format!(
            "https://api.adoptium.net/v3/assets/latest/{}/hotspot?architecture={}&image_type=jre&os={}",
            version, arch, os
        );

        debug!("Fetching Java download info from: {}", url);

        let response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            return Err(MinecraftInstallerError::Network(format!(
                "Failed to get Java download info: HTTP {}",
                response.status()
            )));
        }

        let releases: Vec<AdoptiumRelease> = response.json().await?;

        if releases.is_empty() {
            return Err(MinecraftInstallerError::JavaInstallationFailed(
                format!("No Java {} releases found for {} {}", version, os, arch),
            ));
        }

        let binary = releases[0].binaries.iter()
            .find(|b| b.architecture == arch && b.os == os && b.image_type == "jre")
            .ok_or_else(|| MinecraftInstallerError::JavaInstallationFailed(
                format!("No suitable Java {} binary found", version),
            ))?;

        Ok(binary.package.clone())
    }

    /// Download Java archive
    async fn download_java(&self, url: &str, path: &Path, size: u64) -> Result<()> {
        let progress_bar = ProgressBar::new(size);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .expect("Invalid progress bar template")
                .progress_chars("#>-"),
        );
        progress_bar.set_message("Java JRE");

        let response = self.client.get(url).send().await?;
        if !response.status().is_success() {
            return Err(MinecraftInstallerError::DownloadFailed(format!(
                "HTTP {} for Java download",
                response.status()
            )));
        }

        let mut file = fs::File::create(path).await?;
        let mut downloaded = 0u64;
        let mut stream = response.bytes_stream();

        use futures::StreamExt;
        use tokio::io::AsyncWriteExt;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;
            progress_bar.set_position(downloaded);
        }

        file.sync_all().await?;
        progress_bar.finish_with_message("✓ Java JRE downloaded");
        Ok(())
    }

    /// Extract Java archive
    async fn extract_java(&self, archive_path: &Path, extract_dir: &Path) -> Result<()> {
        info!("Extracting Java...");

        if archive_path.extension().and_then(|s| s.to_str()) == Some("zip") {
            // Windows ZIP file
            self.extract_zip(archive_path, extract_dir).await
        } else {
            // Unix tar.gz file
            self.extract_tar_gz(archive_path, extract_dir).await
        }
    }

    /// Extract ZIP file (Windows)
    async fn extract_zip(&self, archive_path: &Path, extract_dir: &Path) -> Result<()> {
        use std::io::Read;

        let file = std::fs::File::open(archive_path)?;
        let mut archive = zip::ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_path = extract_dir.join(file.name());

            if file.is_dir() {
                fs::create_dir_all(&file_path).await?;
            } else {
                if let Some(parent) = file_path.parent() {
                    fs::create_dir_all(parent).await?;
                }

                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;
                fs::write(&file_path, buffer).await?;

                // Set executable permissions on Unix
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if file_path.file_name().and_then(|n| n.to_str()) == Some("java") {
                        let metadata = fs::metadata(&file_path).await?;
                        let mut perms = metadata.permissions();
                        perms.set_mode(0o755);
                        fs::set_permissions(&file_path, perms).await?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Extract tar.gz file (Unix)
    async fn extract_tar_gz(&self, archive_path: &Path, extract_dir: &Path) -> Result<()> {
        // For simplicity, use system tar command
        let output = Command::new("tar")
            .args(["-xzf", archive_path.to_str().unwrap(), "-C", extract_dir.to_str().unwrap()])
            .output()
            .await?;

        if !output.status.success() {
            return Err(MinecraftInstallerError::JavaInstallationFailed(
                format!("Failed to extract Java: {}", String::from_utf8_lossy(&output.stderr)),
            ));
        }

        Ok(())
    }
}
