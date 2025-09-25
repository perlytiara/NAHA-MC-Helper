use std::path::PathBuf;
use tracing::{info, error};

use crate::error::{MinecraftInstallerError, Result};
use crate::directories::DirectoryManager;
use crate::download::{DownloadManager, VersionManifest};
use crate::java::JavaManager;

/// Main Minecraft installer
pub struct MinecraftInstaller {
    dirs: DirectoryManager,
    download_manager: DownloadManager,
    java_manager: JavaManager,
}

impl MinecraftInstaller {
    /// Create a new Minecraft installer
    pub async fn new(install_dir: PathBuf) -> Result<Self> {
        let dirs = DirectoryManager::new(install_dir);

        // Initialize directories
        dirs.init().await?;

        let download_manager = DownloadManager::new(dirs.clone());
        let java_manager = JavaManager::new(dirs.clone());

        Ok(Self {
            dirs,
            download_manager,
            java_manager,
        })
    }

    /// Install Minecraft
    pub async fn install_minecraft(
        &self,
        version: &str,
        loader: &str,
        loader_version: &str,
        force: bool,
    ) -> Result<()> {
        info!("Starting Minecraft {} installation", version);

        // Check if already installed
        if !force && self.dirs.is_version_installed(version).await {
            info!("Minecraft {} is already installed", version);
            return Ok(());
        }

        // Get version manifest
        let manifest = self.download_manager.get_version_manifest().await?;

        // Find the requested version
        let version_info = manifest.versions.iter()
            .find(|v| v.id == version)
            .ok_or_else(|| MinecraftInstallerError::InvalidVersion(version.to_string()))?;

        // Get detailed version information
        let version_details = self.download_manager.get_version_details(version_info).await?;

        // Determine required Java version
        let required_java = version_details.java_version
            .as_ref()
            .map(|jv| jv.major_version)
            .unwrap_or(8); // Default to Java 8 for older versions

        info!("Minecraft {} requires Java {}", version, required_java);

        // Ensure Java is installed
        let _java_path = self.java_manager.ensure_java(required_java).await?;

        // Install mod loader if not vanilla
        if loader != "vanilla" {
            return Err(MinecraftInstallerError::InvalidLoader(format!(
                "Mod loader '{}' is not yet supported. Only 'vanilla' is currently supported.",
                loader
            )));
        }

        // Download Minecraft components
        info!("Downloading Minecraft components...");

        // Download client JAR and save version JSON
        self.download_manager.download_client(&version_details).await?;

        // Download libraries
        self.download_manager.download_libraries(&version_details).await?;

        // Download assets
        self.download_manager.download_assets(&version_details).await?;

        // Create launcher profile
        self.create_launcher_profile(version).await?;

        info!("✓ Minecraft {} installation completed successfully!", version);
        self.print_installation_summary(version).await?;

        Ok(())
    }

    /// Create launcher profile JSON
    async fn create_launcher_profile(&self, version: &str) -> Result<()> {
        use serde_json::json;

        let profile_id = format!("minecraft-installer-{}", version);
        let instance_dir = self.dirs.instance_dir(&profile_id);

        // Create instance directory
        tokio::fs::create_dir_all(&instance_dir).await?;

        // Create launcher_profiles.json
        let profiles_json = json!({
            "profiles": {
                &profile_id: {
                    "created": chrono::Utc::now().to_rfc3339(),
                    "icon": "Crafting_Table",
                    "lastUsed": chrono::Utc::now().to_rfc3339(),
                    "lastVersionId": version,
                    "name": format!("Minecraft {}", version),
                    "type": "custom",
                    "gameDir": instance_dir.to_string_lossy()
                }
            },
            "settings": {
                "enableAdvanced": false,
                "enableAnalytics": false,
                "enableHistorical": false,
                "enableReleases": true,
                "enableSnapshots": false,
                "keepLauncherOpen": false,
                "profileSorting": "ByLastPlayed",
                "showGameLog": false,
                "showMenu": false,
                "soundOn": false
            },
            "version": 3
        });

        let profiles_path = self.dirs.launcher_profiles();
        let profiles_content = serde_json::to_string_pretty(&profiles_json)?;
        tokio::fs::write(profiles_path, profiles_content).await?;

        info!("Created launcher profile: {}", profile_id);
        Ok(())
    }

    /// Print installation summary
    async fn print_installation_summary(&self, version: &str) -> Result<()> {
        let minecraft_dir = self.dirs.minecraft_dir();
        let version_dir = self.dirs.version_dir(version);

        // Calculate installation size
        let total_size = self.dirs.get_directory_size(&minecraft_dir).await?;
        let size_mb = total_size / 1024 / 1024;

        println!("\n🎮 Minecraft Installation Summary");
        println!("═══════════════════════════════════");
        println!("Version: {}", version);
        println!("Installation Size: {} MB", size_mb);
        println!("Installation Directory: {}", minecraft_dir.display());
        println!("Version Directory: {}", version_dir.display());

        // Check what was installed
        let client_jar = self.dirs.version_jar(version);
        let version_json = self.dirs.version_json(version);
        let launcher_profiles = self.dirs.launcher_profiles();

        println!("\nInstalled Components:");
        println!("✓ Client JAR: {}", client_jar.display());
        println!("✓ Version JSON: {}", version_json.display());
        println!("✓ Launcher Profile: {}", launcher_profiles.display());

        let libraries_dir = self.dirs.libraries_dir();
        if libraries_dir.exists() {
            println!("✓ Libraries: {}", libraries_dir.display());
        }

        let assets_dir = self.dirs.assets_dir();
        if assets_dir.exists() {
            println!("✓ Assets: {}", assets_dir.display());
        }

        println!("\n📋 Next Steps:");
        println!("1. Open your Minecraft launcher");
        println!("2. Import the launcher_profiles.json file");
        println!("3. Select the '{}' profile", format!("minecraft-installer-{}", version));
        println!("4. Launch and enjoy Minecraft {}!", version);

        Ok(())
    }

    /// List available Minecraft versions
    pub async fn list_versions(&self, version_type: Option<&str>) -> Result<()> {
        info!("Fetching available Minecraft versions...");

        let manifest = self.download_manager.get_version_manifest().await?;

        println!("\n🎮 Available Minecraft Versions");
        println!("═════════════════════════════════");
        println!("Latest Release: {}", manifest.latest.release);
        println!("Latest Snapshot: {}", manifest.latest.snapshot);

        // Filter versions by type
        let filtered_versions: Vec<_> = manifest.versions.iter()
            .filter(|v| {
                if let Some(filter_type) = version_type {
                    v.version_type == filter_type
                } else {
                    true
                }
            })
            .take(20) // Show only the latest 20 versions
            .collect();

        println!("\nRecent Versions ({}):",
            version_type.unwrap_or("all types"));
        println!("─────────────────────────────────");

        for version in filtered_versions {
            let status = if self.dirs.is_version_installed(&version.id).await {
                "✓ Installed"
            } else {
                ""
            };

            println!("{:15} {:10} {}",
                version.id,
                version.version_type,
                status
            );
        }

        println!("\nUse --version <version_id> to install a specific version");
        Ok(())
    }

    /// Get installation directory
    pub fn get_install_dir(&self) -> &PathBuf {
        &self.dirs.base_dir
    }

    /// Check if a version is installed
    pub async fn is_installed(&self, version: &str) -> bool {
        self.dirs.is_version_installed(version).await
    }
}






