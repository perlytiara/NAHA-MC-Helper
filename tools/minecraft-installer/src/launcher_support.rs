use std::path::{Path, PathBuf};
use tokio::fs;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{info, debug, warn};
use uuid::Uuid;
use rusqlite::{Connection, Result as SqliteResult};

use crate::error::{MinecraftInstallerError, Result};
use crate::directories::DirectoryManager;

/// API response structure for NAHA modpack information
#[derive(Debug, Deserialize, Serialize)]
pub struct NahaModpackInfo {
    pub server_name: String,
    pub server_type: String,
    pub latest_mrpack: String,
    pub fingerprint: String,
    pub version: String,
    pub last_updated: String,
    pub description: String,
    pub download_url: String,
    pub server_ip: String,
    pub server_port: u16,
}

/// Supported launcher types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LauncherType {
    Official,       // Official Minecraft Launcher
    Prism,         // PrismLauncher
    PrismCracked,  // PrismLauncher-Cracked
    XMCL,          // X Minecraft Launcher
    AstralRinth,   // AstralRinth App
    ModrinthApp,   // Modrinth App
    MultiMC,       // MultiMC (legacy)
    ATLauncher,    // ATLauncher
    Technic,       // Technic Launcher
    Other,         // Custom/Other launcher (custom path)
    Unknown,       // Unknown launcher type
}

/// Mrpack (Modrinth modpack) format
#[derive(Deserialize, Serialize, Debug)]
pub struct MrpackIndex {
    #[serde(rename = "formatVersion")]
    pub format_version: u32,
    pub game: String,
    #[serde(rename = "versionId")]
    pub version_id: String,
    pub name: String,
    pub summary: Option<String>,
    pub files: Vec<MrpackFile>,
    pub dependencies: std::collections::HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MrpackFile {
    pub path: String,
    pub hashes: std::collections::HashMap<String, String>,
    pub env: Option<MrpackEnv>,
    pub downloads: Vec<String>,
    #[serde(rename = "fileSize")]
    pub file_size: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MrpackEnv {
    pub client: String,
    pub server: String,
}

/// Launcher detection and management
pub struct LauncherManager {
    common_launcher_paths: Vec<PathBuf>,
}

impl LauncherManager {
    pub fn new() -> Self {
        let mut common_paths = Vec::new();

        // Windows paths
        if cfg!(target_os = "windows") {
            if let Some(appdata) = dirs::data_dir() {
                common_paths.push(appdata.join(".minecraft"));
                common_paths.push(appdata.join("PrismLauncher"));
                common_paths.push(appdata.join("PrismLauncher-Cracked"));
                common_paths.push(appdata.join("AstralRinthApp"));
                common_paths.push(appdata.join("ModrinthApp"));
            }
            if let Some(roaming) = dirs::config_dir() {
                common_paths.push(roaming.join(".xmcl"));
                common_paths.push(roaming.join("ATLauncher"));
            }
            if let Some(home) = dirs::home_dir() {
                common_paths.push(home.join(".xmcl"));
            }
        }

        // macOS paths
        if cfg!(target_os = "macos") {
            if let Some(home) = dirs::home_dir() {
                common_paths.push(home.join("Library/Application Support/minecraft"));
                common_paths.push(home.join("Library/Application Support/PrismLauncher"));
                common_paths.push(home.join("Library/Application Support/AstralRinthApp"));
                common_paths.push(home.join("Library/Application Support/ModrinthApp"));
                common_paths.push(home.join(".xmcl"));
            }
        }

        // Linux paths
        if cfg!(target_os = "linux") {
            if let Some(home) = dirs::home_dir() {
                common_paths.push(home.join(".minecraft"));
                common_paths.push(home.join(".local/share/PrismLauncher"));
                common_paths.push(home.join(".local/share/AstralRinthApp"));
                common_paths.push(home.join(".local/share/ModrinthApp"));
                common_paths.push(home.join(".xmcl"));
            }
        }

        Self {
            common_launcher_paths: common_paths,
        }
    }

    /// Detect all installed launchers
    pub async fn detect_launchers(&self) -> Vec<(LauncherType, PathBuf)> {
        let mut launchers = Vec::new();

        for path in &self.common_launcher_paths {
            if path.exists() {
                match self.detect_launcher_type(path).await {
                    Ok(launcher_type) if launcher_type != LauncherType::Unknown => {
                        launchers.push((launcher_type, path.clone()));
                    }
                    _ => {}
                }
            }
        }

        launchers
    }

    /// Detect launcher type from directory structure
    pub async fn detect_launcher_type(&self, path: &Path) -> Result<LauncherType> {
        if !path.exists() {
            return Ok(LauncherType::Unknown);
        }

        // Check for Modrinth App first (more specific check)
        if path.join("app-window-state.json").exists() && path.join("profiles").exists() && path.file_name().unwrap() == "ModrinthApp" {
            return Ok(LauncherType::ModrinthApp);
        }

        // Check for AstralRinth App
        if path.join("app-window-state.json").exists() && path.join("profiles").exists() {
            return Ok(LauncherType::AstralRinth);
        }

        // Check for PrismLauncher
        if path.join("prismlauncher.cfg").exists() && path.join("instances").exists() {
            // Check if it's the cracked version
            if let Ok(accounts_content) = fs::read_to_string(path.join("accounts.json")).await {
                if accounts_content.contains("Offline") {
                    return Ok(LauncherType::PrismCracked);
                }
            }
            return Ok(LauncherType::Prism);
        }

        // Check for XMCL
        if path.join("instances").exists() && path.join("launcher_profiles.json").exists() {
            return Ok(LauncherType::XMCL);
        }

        // Check for Official Minecraft Launcher
        if path.join("launcher_profiles.json").exists() &&
           (path.join("versions").exists() || path.file_name().and_then(|n| n.to_str()) == Some(".minecraft")) {
            return Ok(LauncherType::Official);
        }

        // Check for MultiMC
        if path.join("multimc.cfg").exists() && path.join("instances").exists() {
            return Ok(LauncherType::MultiMC);
        }

        // Check for ATLauncher
        if path.join("configs").exists() && path.join("instances").exists() && path.join("servers").exists() {
            return Ok(LauncherType::ATLauncher);
        }

        Ok(LauncherType::Unknown)
    }

    /// Create a new instance for the specified launcher
    pub async fn create_instance(
        &self,
        launcher_path: &Path,
        instance_name: &str,
        minecraft_version: &str,
        mod_loader: &str,
        mod_loader_version: Option<&str>,
    ) -> Result<PathBuf> {
        let launcher_type = self.detect_launcher_type(launcher_path).await?;

        match launcher_type {
            LauncherType::Official => {
                self.create_official_instance(launcher_path, instance_name, minecraft_version).await
            }
            LauncherType::Prism | LauncherType::PrismCracked => {
                self.create_prism_instance(launcher_path, instance_name, minecraft_version, mod_loader, mod_loader_version).await
            }
            LauncherType::XMCL => {
                self.create_xmcl_instance(launcher_path, instance_name, minecraft_version, mod_loader).await
            }
            LauncherType::AstralRinth => {
                self.create_astral_rinth_instance(launcher_path, instance_name, minecraft_version, mod_loader, mod_loader_version).await
            }
            LauncherType::ModrinthApp => {
                self.create_modrinth_app_instance(launcher_path, instance_name, minecraft_version, mod_loader, mod_loader_version).await
            }
            LauncherType::MultiMC => {
                self.create_mmc_instance(launcher_path, instance_name, minecraft_version, mod_loader).await
            }
            LauncherType::Other => {
                self.create_other_instance(launcher_path, instance_name, minecraft_version, mod_loader, mod_loader_version).await
            }
            _ => Err(MinecraftInstallerError::InstallationFailed(
                format!("Unsupported launcher type: {:?}", launcher_type)
            ))
        }
    }

    /// Create instance for Official Minecraft Launcher
    async fn create_official_instance(
        &self,
        launcher_path: &Path,
        instance_name: &str,
        minecraft_version: &str,
    ) -> Result<PathBuf> {
        let profiles_path = launcher_path.join("launcher_profiles.json");

        // Read existing profiles or create new
        let mut profiles_json = if profiles_path.exists() {
            let content = fs::read_to_string(&profiles_path).await?;
            serde_json::from_str(&content)?
        } else {
            json!({
                "profiles": {},
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
            })
        };

        // Create new profile
        let profile_id = format!("minecraft-installer-{}", instance_name);
        let instance_dir = launcher_path.join("instances").join(instance_name);
        fs::create_dir_all(&instance_dir).await?;

        let profile = json!({
            "created": chrono::Utc::now().to_rfc3339(),
            "icon": "Crafting_Table",
            "lastUsed": chrono::Utc::now().to_rfc3339(),
            "lastVersionId": minecraft_version,
            "name": instance_name,
            "type": "custom",
            "gameDir": instance_dir.to_string_lossy()
        });

        profiles_json["profiles"][&profile_id] = profile;

        // Write updated profiles
        fs::write(
            &profiles_path,
            serde_json::to_string_pretty(&profiles_json)?
        ).await?;

        // Create instance directory structure
        fs::create_dir_all(instance_dir.join("saves")).await?;
        fs::create_dir_all(instance_dir.join("resourcepacks")).await?;
        fs::create_dir_all(instance_dir.join("shaderpacks")).await?;
        fs::create_dir_all(instance_dir.join("mods")).await?;

        info!("Created Official Minecraft Launcher instance: {}", instance_name);
        Ok(instance_dir)
    }

    /// Create instance for PrismLauncher
    async fn create_prism_instance(
        &self,
        launcher_path: &Path,
        instance_name: &str,
        minecraft_version: &str,
        mod_loader: &str,
        mod_loader_version: Option<&str>,
    ) -> Result<PathBuf> {
        let instance_dir = launcher_path.join("instances").join(instance_name);
        fs::create_dir_all(&instance_dir).await?;

        // Create .minecraft directory
        let minecraft_dir = instance_dir.join(".minecraft");
        fs::create_dir_all(&minecraft_dir).await?;
        fs::create_dir_all(minecraft_dir.join("saves")).await?;
        fs::create_dir_all(minecraft_dir.join("resourcepacks")).await?;
        fs::create_dir_all(minecraft_dir.join("shaderpacks")).await?;
        fs::create_dir_all(minecraft_dir.join("mods")).await?;
        fs::create_dir_all(minecraft_dir.join("config")).await?;

        // Create instance.cfg with proper structure
        let instance_config = format!(r#"[General]
ConfigVersion=1.2
iconKey=default
name={}
AutomaticJava=true
InstanceType=OneSix
ExportAuthor=
ExportName=
ExportOptionalFiles=true
ExportSummary=
ExportVersion=1.0.0
IgnoreJavaCompatibility=false
JavaArchitecture=64
JavaPath=
JoinServerOnLaunch=false
JavaRealArchitecture=amd64
JavaSignature=
JavaVendor=
JavaVersion=
LogPrePostOutput=true
ManagedPack=false
ManagedPackID=
ManagedPackName=
ManagedPackType=
ManagedPackVersionID=
ManagedPackVersionName=
OverrideCommands=false
OverrideConsole=false
OverrideEnv=false
OverrideGameTime=false
OverrideJavaArgs=false
OverrideJavaLocation=false
OverrideLegacySettings=false
OverrideMemory=false
OverrideMiscellaneous=false
OverrideNativeWorkarounds=false
OverridePerformance=false
OverrideWindow=false
Profiler=
UseAccountForInstance=false
lastLaunchTime={}
lastTimePlayed=0
linkedInstances=[]
notes=Created by Minecraft Installer
totalTimePlayed=0

[UI]
mods_Page\Columns=@ByteArray(\0\0\0\xff\0\0\0\0\0\0\0\x1\0\0\0\0\0\0\0\x1\x1\0\0\0\0\0\0\0\0\0\0\0\v\x80\a\0\0\0\x4\0\0\0\a\0\0\0\x64\0\0\0\b\0\0\0\x64\0\0\0\n\0\0\0\x64\0\0\0\t\0\0\0\x64\0\0\x4L\0\0\0\v\x1\x1\0\0\0\0\0\0\x1\0\0\0\0\0\0\0\x64\xff\xff\xff\xff\0\0\0\x81\0\0\0\0\0\0\0\v\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\x1\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\x3\xe8\0\0\0\0\x64\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1)
resourcepacks_Page\Columns=@ByteArray(\0\0\0\xff\0\0\0\0\0\0\0\x1\0\0\0\0\0\0\0\x1\x1\0\0\0\0\0\0\0\0\0\0\0\a\x10\0\0\0\x1\0\0\0\x4\0\0\0\x64\0\0\x2\xbc\0\0\0\a\x1\x1\0\0\0\0\0\0\x1\0\0\0\0\0\0\0\x64\xff\xff\xff\xff\0\0\0\x81\0\0\0\0\0\0\0\a\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\x1\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\x3\xe8\0\0\0\0\x64\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1)
shaderpacks_Page\Columns=@ByteArray(\0\0\0\xff\0\0\0\0\0\0\0\x1\0\0\0\0\0\0\0\x1\x1\0\0\0\0\0\0\0\0\0\0\0\x5\x10\0\0\0\x1\0\0\0\x4\0\0\0\x64\0\0\x1\xf4\0\0\0\x5\x1\x1\0\0\0\0\0\0\x1\0\0\0\0\0\0\0\x64\xff\xff\xff\xff\0\0\0\x81\0\0\0\0\0\0\0\x5\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\x1\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\x3\xe8\0\0\0\0\x64\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1)
texturepacks_Page\Columns=@ByteArray(\0\0\0\xff\0\0\0\0\0\0\0\x1\0\0\0\0\0\0\0\x1\x1\0\0\0\0\0\0\0\0\0\0\0\x6 \0\0\0\x1\0\0\0\x5\0\0\0\x64\0\0\x2X\0\0\0\x6\x1\x1\0\0\0\0\0\0\x1\0\0\0\0\0\0\0\x64\xff\xff\xff\xff\0\0\0\x81\0\0\0\0\0\0\0\x6\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\x1\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\0\x64\0\0\0\x1\0\0\0\0\0\0\x3\xe8\0\0\0\0\x64\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1)
"#, instance_name, chrono::Utc::now().timestamp_millis());

        fs::write(instance_dir.join("instance.cfg"), instance_config).await?;

        // Create mmc-pack.json with proper structure
        let mut components = vec![
            json!({
                "cachedName": "LWJGL 3",
                "cachedVersion": "3.3.3",
                "cachedVolatile": true,
                "dependencyOnly": true,
                "uid": "org.lwjgl3",
                "version": "3.3.3"
            }),
            json!({
                "cachedName": "Minecraft",
                "cachedRequires": [
                    {
                        "suggests": "3.3.3",
                        "uid": "org.lwjgl3"
                    }
                ],
                "cachedVersion": minecraft_version,
                "important": true,
                "uid": "net.minecraft",
                "version": minecraft_version
            })
        ];

        // Add mod loader component if specified
        if mod_loader != "vanilla" {
            let loader_component = match mod_loader {
                "fabric" => json!({
                    "cachedName": "Fabric Loader",
                    "cachedRequires": [{"uid": "net.minecraft"}],
                    "cachedVersion": mod_loader_version.unwrap_or("stable"),
                    "uid": "net.fabricmc.fabric-loader",
                    "version": mod_loader_version.unwrap_or("stable")
                }),
                "forge" => json!({
                    "cachedName": "Minecraft Forge",
                    "cachedRequires": [{"uid": "net.minecraft"}],
                    "cachedVersion": mod_loader_version.unwrap_or("recommended"),
                    "uid": "net.minecraftforge",
                    "version": mod_loader_version.unwrap_or("recommended")
                }),
                "quilt" => json!({
                    "cachedName": "Quilt Loader",
                    "cachedRequires": [{"uid": "net.minecraft"}],
                    "cachedVersion": mod_loader_version.unwrap_or("stable"),
                    "uid": "org.quiltmc.quilt-loader",
                    "version": mod_loader_version.unwrap_or("stable")
                }),
                "neoforge" => {
                    // For NeoForge, use a specific version instead of "latest"
                    let neoforge_version = if let Some(version) = mod_loader_version {
                        if version == "latest" {
                            // Use a known working version for 1.21.1
                            "21.1.209"
                        } else {
                            version
                        }
                    } else {
                        "21.1.209" // Default fallback
                    };

                    json!({
                        "cachedName": "NeoForge",
                        "cachedRequires": [{"equals": minecraft_version, "uid": "net.minecraft"}],
                        "cachedVersion": neoforge_version,
                        "uid": "net.neoforged",
                        "version": neoforge_version
                    })
                },
                _ => return Err(MinecraftInstallerError::InvalidLoader(mod_loader.to_string()))
            };
            components.push(loader_component);
        }

        let mmc_pack = json!({
            "components": components,
            "formatVersion": 1
        });

        fs::write(
            instance_dir.join("mmc-pack.json"),
            serde_json::to_string_pretty(&mmc_pack)?
        ).await?;

        info!("Created PrismLauncher instance: {}", instance_name);
        Ok(instance_dir)
    }

    /// Create instance for XMCL
    async fn create_xmcl_instance(
        &self,
        launcher_path: &Path,
        instance_name: &str,
        minecraft_version: &str,
        mod_loader: &str,
    ) -> Result<PathBuf> {
        let instances_path = launcher_path.join("instances.json");

        // Read existing instances
        let mut instances_json = if instances_path.exists() {
            let content = fs::read_to_string(&instances_path).await?;
            serde_json::from_str(&content)?
        } else {
            json!({
                "instances": [],
                "selectedInstance": ""
            })
        };

        // Create instance directory
        let instance_dir = launcher_path.join("instances").join(instance_name);
        fs::create_dir_all(&instance_dir).await?;
        fs::create_dir_all(instance_dir.join("saves")).await?;
        fs::create_dir_all(instance_dir.join("resourcepacks")).await?;
        fs::create_dir_all(instance_dir.join("shaderpacks")).await?;
        fs::create_dir_all(instance_dir.join("mods")).await?;
        fs::create_dir_all(instance_dir.join("config")).await?;

        // Create instance configuration
        let instance_config = json!({
            "name": instance_name,
            "maxMemory": 4096,
            "url": "",
            "icon": "",
            "runtime": {
                "minecraft": minecraft_version,
                "forge": if mod_loader == "forge" { Some("latest") } else { None::<&str> },
                "liteloader": None::<&str>,
                "fabricLoader": if mod_loader == "fabric" { Some("latest") } else { None::<&str> },
                "yarn": None::<&str>,
                "optifine": None::<&str>,
                "quiltLoader": if mod_loader == "quilt" { Some("latest") } else { None::<&str> },
                "neoForged": if mod_loader == "neoforge" { Some("21.1.209") } else { None::<&str> },
                "labyMod": None::<&str>
            },
            "java": "",
            "version": format!("{}-{}", mod_loader, minecraft_version),
            "server": null,
            "author": "Minecraft Installer",
            "description": format!("Minecraft {} instance created by Minecraft Installer", minecraft_version),
            "lastAccessDate": chrono::Utc::now().timestamp_millis(),
            "creationDate": chrono::Utc::now().timestamp_millis(),
            "modpackVersion": "",
            "fileApi": "",
            "tags": [],
            "assignMemory": true,
            "lastPlayedDate": chrono::Utc::now().timestamp_millis(),
            "playtime": 0
        });

        // Write instance.json directly
        fs::write(
            instance_dir.join("instance.json"),
            serde_json::to_string_pretty(&instance_config)?
        ).await?;

        info!("Created XMCL instance: {}", instance_name);
        Ok(instance_dir)
    }

    /// Create instance for AstralRinth App
    async fn create_astral_rinth_instance(
        &self,
        launcher_path: &Path,
        instance_name: &str,
        minecraft_version: &str,
        mod_loader: &str,
        mod_loader_version: Option<&str>,
    ) -> Result<PathBuf> {
        let profile_name = instance_name.to_lowercase().replace(" ", "-");
        let profile_dir = launcher_path.join("profiles").join(&profile_name);
        fs::create_dir_all(&profile_dir).await?;

        // Create profile.json
        let profile = json!({
            "name": instance_name,
            "game_version": minecraft_version,
            "loader": mod_loader,
            "loader_version": mod_loader_version,
            "icon_path": null,
            "created": chrono::Utc::now().to_rfc3339(),
            "modified": chrono::Utc::now().to_rfc3339(),
            "last_played": null,
            "submitted_time_played": 0,
            "recent_time_played": 0,
            "java_path": null,
            "extra_launch_args": null,
            "memory": null,
            "game_resolution": null,
            "force_fullscreen": null,
            "install_stage": "installed",
            "path": profile_name,
            "metadata": {
                "name": instance_name,
                "version_id": minecraft_version
            }
        });

        fs::write(
            profile_dir.join("profile.json"),
            serde_json::to_string_pretty(&profile)?
        ).await?;

        // Create AstralRinth profile directory structure (matches existing structure)
        fs::create_dir_all(profile_dir.join("saves")).await?;
        fs::create_dir_all(profile_dir.join("resourcepacks")).await?;
        fs::create_dir_all(profile_dir.join("shaderpacks")).await?;
        fs::create_dir_all(profile_dir.join("mods")).await?;
        fs::create_dir_all(profile_dir.join("config")).await?;
        fs::create_dir_all(profile_dir.join("logs")).await?;
        fs::create_dir_all(profile_dir.join("crash-reports")).await?;
        fs::create_dir_all(profile_dir.join("datapacks")).await?;
        fs::create_dir_all(profile_dir.join("blueprints")).await?;
        fs::create_dir_all(profile_dir.join("CustomSkinLoader")).await?;
        fs::create_dir_all(profile_dir.join("data")).await?;
        fs::create_dir_all(profile_dir.join("defaultconfigs")).await?;
        fs::create_dir_all(profile_dir.join("downloads")).await?;
        fs::create_dir_all(profile_dir.join("journeymap")).await?;
        fs::create_dir_all(profile_dir.join("kubejs")).await?;
        fs::create_dir_all(profile_dir.join("local")).await?;
        fs::create_dir_all(profile_dir.join("moddata")).await?;
        fs::create_dir_all(profile_dir.join("schematics")).await?;
        fs::create_dir_all(profile_dir.join("scripts")).await?;
        fs::create_dir_all(profile_dir.join("waypoints")).await?;
        fs::create_dir_all(profile_dir.join(".cache")).await?;
        fs::create_dir_all(profile_dir.join(".mixin.out")).await?;

        // Create options.txt file (required by AstralRinth)
        let options_content = "version:3955\nao:true\nbiomeBlendRadius:2\nenableVsync:true\nentityDistanceScaling:1.0\nentityShadows:true\nforceUnicodeFont:false\njapaneseGlyphVariants:false\nfov:0.0\nfovEffectScale:1.0\ndarknessEffectScale:1.0\nglintSpeed:0.5\nglintStrength:0.75\nprioritizeChunkUpdates:0\nfullscreen:false\ngamma:0.5\ngraphicsMode:1\nguiScale:0\nmaxFps:120\nmipmapLevels:4\nrenderDistance:12\nsimulationDistance:12\nuseVbo:true\n";
        fs::write(profile_dir.join("options.txt"), options_content).await?;

        // Note: servers.dat will be copied from mrpack during file copying phase

        // Inject profile into AstralRinth database
        if let Err(e) = self.inject_astralrinth_profile(launcher_path, &profile_name, instance_name, minecraft_version, mod_loader).await {
            warn!("Failed to inject profile into AstralRinth database: {}", e);
            // Continue anyway - the profile directory structure is still created
        }

        info!("Created AstralRinth App instance: {}", instance_name);
        Ok(profile_dir)
    }

    /// Create instance for Modrinth App
    async fn create_modrinth_app_instance(
        &self,
        launcher_path: &Path,
        instance_name: &str,
        minecraft_version: &str,
        mod_loader: &str,
        mod_loader_version: Option<&str>,
    ) -> Result<PathBuf> {
        let profile_name = instance_name.to_lowercase().replace(" ", "-");
        let profile_dir = launcher_path.join("profiles").join(&profile_name);
        fs::create_dir_all(&profile_dir).await?;

        // Create profile.json
        let profile = json!({
            "name": instance_name,
            "game_version": minecraft_version,
            "loader": mod_loader,
            "loader_version": mod_loader_version,
            "icon_path": null,
            "created": chrono::Utc::now().to_rfc3339(),
            "modified": chrono::Utc::now().to_rfc3339(),
            "last_played": null,
            "submitted_time_played": 0,
            "recent_time_played": 0,
            "java_path": null,
            "extra_launch_args": null,
            "memory": null,
            "game_resolution": null,
            "force_fullscreen": null,
            "install_stage": "installed",
            "path": profile_name,
            "version_id": minecraft_version
        });

        fs::write(
            profile_dir.join("profile.json"),
            serde_json::to_string_pretty(&profile)?
        ).await?;

        // Create Modrinth App profile directory structure (same as AstralRinth)
        fs::create_dir_all(profile_dir.join("saves")).await?;
        fs::create_dir_all(profile_dir.join("resourcepacks")).await?;
        fs::create_dir_all(profile_dir.join("shaderpacks")).await?;
        fs::create_dir_all(profile_dir.join("mods")).await?;
        fs::create_dir_all(profile_dir.join("config")).await?;
        fs::create_dir_all(profile_dir.join("logs")).await?;
        fs::create_dir_all(profile_dir.join("crash-reports")).await?;
        fs::create_dir_all(profile_dir.join("datapacks")).await?;
        fs::create_dir_all(profile_dir.join("blueprints")).await?;
        fs::create_dir_all(profile_dir.join("CustomSkinLoader")).await?;
        fs::create_dir_all(profile_dir.join("data")).await?;
        fs::create_dir_all(profile_dir.join("defaultconfigs")).await?;
        fs::create_dir_all(profile_dir.join("downloads")).await?;
        fs::create_dir_all(profile_dir.join("journeymap")).await?;
        fs::create_dir_all(profile_dir.join("kubejs")).await?;
        fs::create_dir_all(profile_dir.join("local")).await?;
        fs::create_dir_all(profile_dir.join("moddata")).await?;
        fs::create_dir_all(profile_dir.join("schematics")).await?;
        fs::create_dir_all(profile_dir.join("scripts")).await?;
        fs::create_dir_all(profile_dir.join("waypoints")).await?;
        fs::create_dir_all(profile_dir.join(".cache")).await?;
        fs::create_dir_all(profile_dir.join(".mixin.out")).await?;

        // Create options.txt file (required by Modrinth App)
        let options_content = "version:3955\nao:true\nbiomeBlendRadius:2\nenableVsync:true\nentityDistanceScaling:1.0\nentityShadows:true\nforceUnicodeFont:false\njapaneseGlyphVariants:false\nfov:0.0\nfovEffectScale:1.0\ndarknessEffectScale:1.0\nglintSpeed:0.5\nglintStrength:0.75\nprioritizeChunkUpdates:0\nfullscreen:false\ngamma:0.5\ngraphicsMode:1\nguiScale:0\nmaxFps:120\nmipmapLevels:4\nrenderDistance:12\nsimulationDistance:12\nuseVbo:true\n";
        fs::write(profile_dir.join("options.txt"), options_content).await?;

        // Note: servers.dat will be copied from mrpack during file copying phase

        // Inject profile into Modrinth App database (same as AstralRinth)
        if let Err(e) = self.inject_modrinth_app_profile(launcher_path, &profile_name, instance_name, minecraft_version, mod_loader).await {
            warn!("Failed to inject profile into Modrinth App database: {}", e);
            // Continue anyway - the profile directory structure is still created
        }

        info!("Created Modrinth App instance: {}", instance_name);
        Ok(profile_dir)
    }

    /// Create instance for MultiMC
    async fn create_mmc_instance(
        &self,
        launcher_path: &Path,
        instance_name: &str,
        minecraft_version: &str,
        mod_loader: &str,
    ) -> Result<PathBuf> {
        let instance_dir = launcher_path.join("instances").join(instance_name);
        fs::create_dir_all(&instance_dir).await?;

        // Create .minecraft directory
        let minecraft_dir = instance_dir.join(".minecraft");
        fs::create_dir_all(&minecraft_dir).await?;
        fs::create_dir_all(minecraft_dir.join("saves")).await?;
        fs::create_dir_all(minecraft_dir.join("resourcepacks")).await?;
        fs::create_dir_all(minecraft_dir.join("mods")).await?;

        // Create instance.cfg (similar to PrismLauncher but with MultiMC format)
        let instance_config = format!(r#"InstanceType=OneSix
IntendedVersion={}
LogPrePostOutput=true
name={}
notes=Created by Minecraft Installer
"#, minecraft_version, instance_name);

        fs::write(instance_dir.join("instance.cfg"), instance_config).await?;

        // Create mmc-pack.json
        let mmc_pack = json!({
            "components": [
                {
                    "cachedName": "Minecraft",
                    "cachedVersion": minecraft_version,
                    "important": true,
                    "uid": "net.minecraft",
                    "version": minecraft_version
                }
            ],
            "formatVersion": 1
        });

        fs::write(
            instance_dir.join("mmc-pack.json"),
            serde_json::to_string_pretty(&mmc_pack)?
        ).await?;

        info!("Created MultiMC instance: {}", instance_name);
        Ok(instance_dir)
    }

    /// Inject profile into AstralRinth database
    async fn inject_astralrinth_profile(
        &self,
        launcher_path: &Path,
        profile_name: &str,
        instance_name: &str,
        minecraft_version: &str,
        mod_loader: &str,
    ) -> Result<()> {
        let db_path = launcher_path.join("app.db");

        if !db_path.exists() {
            return Err(MinecraftInstallerError::InstallationFailed(
                "AstralRinth database not found".to_string()
            ));
        }

        // Open database connection
        let conn = Connection::open(&db_path)
            .map_err(|e| MinecraftInstallerError::InstallationFailed(
                format!("Failed to open AstralRinth database: {}", e)
            ))?;

        // Get current timestamp
        let now = chrono::Utc::now().timestamp_millis();

        // Try to insert into profiles table with different possible table structures

        let mut success = false;

        // Insert with the minimal required fields that we discovered through testing
        match conn.execute(
            "INSERT OR REPLACE INTO profiles (path, name, game_version, mod_loader, install_stage, created, modified, groups, override_extra_launch_args, override_custom_env_vars) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                profile_name,           // path
                instance_name,          // name
                minecraft_version,      // game_version
                mod_loader,             // mod_loader
                "installed",            // install_stage
                now,                    // created
                now,                    // modified
                "[]",                   // groups (empty JSON array)
                "[]",                   // override_extra_launch_args (empty JSON array)
                "{}"                    // override_custom_env_vars (empty JSON object)
            ]
        ) {
            Ok(_) => {
                success = true;
                info!("Successfully injected profile into AstralRinth database");
            }
            Err(e) => {
                debug!("Failed to insert profile: {}", e);
            }
        }

        if !success {
            return Err(MinecraftInstallerError::InstallationFailed(
                "Failed to inject profile into AstralRinth database - unknown table structure".to_string()
            ));
        }

        Ok(())
    }

    /// Inject profile into Modrinth App database
    async fn inject_modrinth_app_profile(
        &self,
        launcher_path: &Path,
        profile_name: &str,
        instance_name: &str,
        minecraft_version: &str,
        mod_loader: &str,
    ) -> Result<()> {
        let db_path = launcher_path.join("app.db");

        if !db_path.exists() {
            return Err(MinecraftInstallerError::InstallationFailed(
                "Modrinth App database not found".to_string()
            ));
        }

        // Open database connection
        let conn = Connection::open(&db_path)
            .map_err(|e| MinecraftInstallerError::InstallationFailed(
                format!("Failed to open Modrinth App database: {}", e)
            ))?;

        // Get current timestamp
        let now = chrono::Utc::now().timestamp_millis();

        // Insert with the minimal required fields (same as AstralRinth)
        match conn.execute(
            "INSERT OR REPLACE INTO profiles (path, name, game_version, mod_loader, install_stage, created, modified, groups, override_extra_launch_args, override_custom_env_vars) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                profile_name,           // path
                instance_name,          // name
                minecraft_version,      // game_version
                mod_loader,             // mod_loader
                "installed",            // install_stage
                now,                    // created
                now,                    // modified
                "[]",                   // groups (empty JSON array)
                "[]",                   // override_extra_launch_args (empty JSON array)
                "{}"                    // override_custom_env_vars (empty JSON object)
            ]
        ) {
            Ok(_) => {
                info!("Successfully injected profile into Modrinth App database");
            }
            Err(e) => {
                debug!("Failed to insert profile: {}", e);
            }
        }

        Ok(())
    }

    /// Install mrpack (Modrinth modpack) file
    pub async fn install_mrpack(
        &self,
        mrpack_path: &Path,
        instance_dir: &Path,
        instance_name: &str,
    ) -> Result<(String, String)> {
        info!("Installing mrpack: {}", mrpack_path.display());

        // Extract mrpack file
        let file = std::fs::File::open(mrpack_path)?;
        let mut archive = zip::ZipArchive::new(file)?;

        // Read modrinth.index.json
        let index: MrpackIndex = {
            let mut index_file = archive.by_name("modrinth.index.json")?;
            let mut index_content = String::new();
            std::io::Read::read_to_string(&mut index_file, &mut index_content)?;
            serde_json::from_str(&index_content)?
        };

        info!("Installing modpack: {} v{}", index.name, index.version_id);

        // Create instance directory
        fs::create_dir_all(instance_dir).await?;
        fs::create_dir_all(instance_dir.join("mods")).await?;
        fs::create_dir_all(instance_dir.join("config")).await?;
        fs::create_dir_all(instance_dir.join("saves")).await?;
        fs::create_dir_all(instance_dir.join("resourcepacks")).await?;

        // Extract overrides
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_path = file.name();

            if file_path.starts_with("overrides/") {
                let relative_path = file_path.strip_prefix("overrides/").unwrap();
                let output_path = instance_dir.join(relative_path);

                if file.is_dir() {
                    fs::create_dir_all(&output_path).await?;
                } else {
                    if let Some(parent) = output_path.parent() {
                        fs::create_dir_all(parent).await?;
                    }

                    let mut buffer = Vec::new();
                    std::io::Read::read_to_end(&mut file, &mut buffer)?;
                    fs::write(&output_path, buffer).await?;
                }
            }
        }

        // Download mod files
        let client = reqwest::Client::new();
        let total_files = index.files.len();
        info!("Downloading {} mod files...", total_files);

        for (i, file) in index.files.iter().enumerate() {
            // Check if file should be installed on client
            if let Some(env) = &file.env {
                if env.client == "unsupported" {
                    continue;
                }
            }

            info!("[{}/{}] Downloading: {}", i + 1, total_files, file.path);

            let file_path = instance_dir.join(&file.path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).await?;
            }

            // Try each download URL until one succeeds
            let mut downloaded = false;
            for download_url in &file.downloads {
                match client.get(download_url).send().await {
                    Ok(response) if response.status().is_success() => {
                        let bytes = response.bytes().await?;

                        // Verify hash if available
                        if let Some(sha1_hash) = file.hashes.get("sha1") {
                            use sha1_smol::{Sha1, Digest};
                            let mut hasher = Sha1::new();
                            hasher.update(&bytes);
                            let calculated_hash = hex::encode(hasher.digest().bytes());

                            if calculated_hash != *sha1_hash {
                                warn!("Hash mismatch for {}: expected {}, got {}",
                                    file.path, sha1_hash, calculated_hash);
                                continue;
                            }
                        }

                        fs::write(&file_path, bytes).await?;
                        downloaded = true;
                        info!("✓ Downloaded: {}", file.path);
                        break;
                    }
                    Ok(response) => {
                        warn!("Failed to download {} from {}: HTTP {}",
                            file.path, download_url, response.status());
                    }
                    Err(e) => {
                        warn!("Failed to download {} from {}: {}",
                            file.path, download_url, e);
                    }
                }
            }

            if !downloaded {
                return Err(MinecraftInstallerError::DownloadFailed(
                    format!("Failed to download file: {}", file.path)
                ));
            }
        }

        // Create instance metadata
        let minecraft_version = index.dependencies.get("minecraft")
            .ok_or_else(|| MinecraftInstallerError::InstallationFailed(
                "No Minecraft version specified in mrpack".to_string()
            ))?;

        let mod_loader = if index.dependencies.contains_key("fabric-loader") {
            "fabric"
        } else if index.dependencies.contains_key("forge") {
            "forge"
        } else if index.dependencies.contains_key("quilt-loader") {
            "quilt"
        } else if index.dependencies.contains_key("neoforge") {
            "neoforge"
        } else {
            "vanilla"
        };

        info!("✓ Mrpack installation completed: {}", instance_name);
        Ok((minecraft_version.clone(), mod_loader.to_string()))
    }

    /// Auto-detect and install to best available launcher
    pub async fn auto_install_instance(
        &self,
        instance_name: &str,
        minecraft_version: &str,
        mod_loader: &str,
        mod_loader_version: Option<&str>,
        target_launcher: Option<&str>,
        custom_path: Option<&Path>,
    ) -> Result<PathBuf> {
        let detected_launchers = self.detect_launchers().await;

        if detected_launchers.is_empty() {
            return Err(MinecraftInstallerError::InstallationFailed(
                "No compatible launchers found".to_string()
            ));
        }

        // If target launcher is specified, try to find it first
        if let Some(target) = target_launcher {
            let target_type = match target.to_lowercase().as_str() {
                "astralrinth" => LauncherType::AstralRinth,
                "modrinth" | "modrinthapp" => LauncherType::ModrinthApp,
                "prism" | "prismlauncher" => {
                    // Check if PrismCracked is available, otherwise use Prism
                    if detected_launchers.iter().any(|(t, _)| matches!(t, LauncherType::PrismCracked)) {
                        LauncherType::PrismCracked
                    } else {
                        LauncherType::Prism
                    }
                },
                "prismcracked" => LauncherType::PrismCracked,
                "xmcl" => LauncherType::XMCL,
                "official" => LauncherType::Official,
                "multimc" => LauncherType::MultiMC,
                "other" => LauncherType::Other,
                _ => {
                    return Err(MinecraftInstallerError::InstallationFailed(
                        format!("Unknown target launcher: {}", target)
                    ));
                }
            };

            // Handle custom path for Other launcher
            if target_type == LauncherType::Other {
                if let Some(path) = custom_path {
                    info!("Installing to custom path: {}", path.display());
                    return self.create_other_instance(path, instance_name, minecraft_version, mod_loader, mod_loader_version).await;
                } else {
                    return Err(MinecraftInstallerError::InstallationFailed(
                        "Custom path required for Other launcher type".to_string()
                    ));
                }
            }

            if let Some((_, path)) = detected_launchers.iter()
                .find(|(launcher_type, _)| launcher_type == &target_type) {
                info!("Installing to {:?} launcher at: {}", target_type, path.display());
                return self.create_instance(path, instance_name, minecraft_version, mod_loader, mod_loader_version).await;
            } else {
                return Err(MinecraftInstallerError::InstallationFailed(
                    format!("Target launcher '{}' not found or not compatible", target)
                ));
            }
        }

        // Prefer AstralRinth, then ModrinthApp, then PrismLauncher, then others
        let preferred_order = [
            LauncherType::AstralRinth,
            LauncherType::ModrinthApp,
            LauncherType::Prism,
            LauncherType::XMCL,
            LauncherType::Official,
            LauncherType::MultiMC,
            LauncherType::PrismCracked,
        ];

        for preferred_type in &preferred_order {
            if let Some((_, path)) = detected_launchers.iter()
                .find(|(launcher_type, _)| launcher_type == preferred_type) {
                info!("Installing to {:?} launcher at: {}", preferred_type, path.display());
                return self.create_instance(path, instance_name, minecraft_version, mod_loader, mod_loader_version).await;
            }
        }

        // Fall back to first available launcher
        let (launcher_type, path) = &detected_launchers[0];
        info!("Installing to {:?} launcher at: {}", launcher_type, path.display());
        self.create_instance(path, instance_name, minecraft_version, mod_loader, mod_loader_version).await
    }

    /// Download modpack info from NAHA API
    pub async fn fetch_modpack_info(&self, modpack_type: &str) -> Result<NahaModpackInfo> {
        let api_url = format!("https://perlytiara.github.io/NAHA-MC.IO/api/{}/", modpack_type);
        info!("Fetching modpack info from: {}", api_url);

        let client = reqwest::Client::new();
        let response = client.get(&api_url).send().await
            .map_err(|e| MinecraftInstallerError::InstallationFailed(
                format!("Failed to fetch modpack info: {}", e)
            ))?;

        if !response.status().is_success() {
            return Err(MinecraftInstallerError::InstallationFailed(
                format!("API request failed with status: {}", response.status())
            ));
        }

        let modpack_info: NahaModpackInfo = response.json().await
            .map_err(|e| MinecraftInstallerError::InstallationFailed(
                format!("Failed to parse modpack info: {}", e)
            ))?;

        info!("✓ Fetched modpack info: {} v{}", modpack_info.server_name, modpack_info.version);
        Ok(modpack_info)
    }

    /// Download and install modpack from NAHA API
    pub async fn download_and_install_from_api(
        &self,
        modpack_type: &str,
        target_launcher: Option<&str>,
        create_instance: bool,
        custom_path: Option<&Path>,
    ) -> Result<()> {
        // Fetch modpack info from API
        let modpack_info = self.fetch_modpack_info(modpack_type).await?;

        info!("Downloading {} modpack from: {}", modpack_info.server_name, modpack_info.download_url);

        // Download the mrpack file
        let client = reqwest::Client::new();
        let response = client.get(&modpack_info.download_url).send().await
            .map_err(|e| MinecraftInstallerError::InstallationFailed(
                format!("Failed to download modpack: {}", e)
            ))?;

        if !response.status().is_success() {
            return Err(MinecraftInstallerError::InstallationFailed(
                format!("Download failed with status: {}", response.status())
            ));
        }

        let bytes = response.bytes().await
            .map_err(|e| MinecraftInstallerError::InstallationFailed(
                format!("Failed to read download data: {}", e)
            ))?;

        // Create temporary file for the mrpack
        let temp_mrpack_path = std::env::temp_dir().join(format!("naha-{}-{}.mrpack", modpack_type, modpack_info.version));
        fs::write(&temp_mrpack_path, bytes).await?;
        info!("✓ Downloaded modpack to: {}", temp_mrpack_path.display());

        // Install the mrpack
        let temp_instance_dir = std::env::temp_dir().join(format!("temp-{}-instance", modpack_type));

        match self.install_mrpack(&temp_mrpack_path, &temp_instance_dir, "temp-instance").await {
            Ok((minecraft_version, mod_loader)) => {
                info!("✓ Modpack installed successfully!");

                if create_instance {
                    // Generate proper instance name based on modpack type
                    let instance_name = match modpack_type {
                        "neoforge" => "NAHA-NeoForge".to_string(),
                        "fabric" => "NAHA-Fabric".to_string(),
                        _ => format!("NAHA-{}", modpack_type),
                    };

                    match self.auto_install_instance(
                        &instance_name,
                        &minecraft_version,
                        &mod_loader,
                        None,
                        target_launcher,
                        custom_path
                    ).await {
                        Ok(instance_path) => {
                            info!("✓ Instance created at: {}", instance_path.display());

                            // Copy files from temp instance to launcher instance
                            if let Err(e) = self.copy_instance_files(&temp_instance_dir, &instance_path).await {
                                warn!("Failed to copy instance files: {}", e);
                            } else {
                                info!("✓ Files copied to launcher instance");
                            }

                            // Set up automodpack configuration
                            if let Err(e) = self.setup_automodpack(&instance_path, &modpack_info).await {
                                warn!("Failed to setup automodpack: {}", e);
                            } else {
                                info!("✓ Automodpack configured");
                            }
                        }
                        Err(e) => {
                            warn!("Failed to create launcher instance: {}", e);
                            info!("You can still launch Minecraft from your installation directory.");
                        }
                    }
                } else {
                    info!("You can now launch Minecraft from your installation directory.");
                    info!("Use --create-instance to automatically create launcher instances.");
                }
            }
            Err(e) => {
                return Err(MinecraftInstallerError::InstallationFailed(
                    format!("Modpack installation failed: {}", e)
                ));
            }
        }

        // Clean up temporary files
        if let Err(e) = fs::remove_file(&temp_mrpack_path).await {
            warn!("Failed to clean up temporary mrpack file: {}", e);
        }
        if let Err(e) = fs::remove_dir_all(&temp_instance_dir).await {
            warn!("Failed to clean up temporary instance directory: {}", e);
        }

        Ok(())
    }

    /// Set up automodpack configuration with server fingerprint
    async fn setup_automodpack(&self, instance_path: &Path, modpack_info: &NahaModpackInfo) -> Result<()> {
        // Determine the base directory for automodpack files
        let base_dir = if instance_path.join("profile.json").exists() {
            // AstralRinth and ModrinthApp
            instance_path.to_path_buf()
        } else if instance_path.join("mmc-pack.json").exists() {
            // PrismLauncher
            instance_path.join(".minecraft")
        } else if instance_path.join("instance.json").exists() {
            // XMCL
            instance_path.to_path_buf()
        } else {
            // Default
            instance_path.join(".minecraft")
        };

        let automodpack_dir = base_dir.join("automodpack");
        let automodpack_private_dir = automodpack_dir.join(".private");

        fs::create_dir_all(&automodpack_private_dir).await?;

        // Create automodpack-known-hosts.json with server fingerprint (matching real format)
        let known_hosts = json!({
            "hosts": {
                modpack_info.server_ip.clone(): modpack_info.fingerprint.clone()
            }
        });

        fs::write(
            automodpack_private_dir.join("automodpack-known-hosts.json"),
            serde_json::to_string_pretty(&known_hosts)?
        ).await?;

        // Note: automodpack-client.json and automodpack-server.json are created automatically by automodpack

        info!("✓ Automodpack configured for server {}:{}", modpack_info.server_ip, modpack_info.server_port);
        Ok(())
    }

    /// Copy files from temporary instance to launcher instance (moved from main.rs)
    pub async fn copy_instance_files(&self, temp_dir: &Path, target_dir: &Path) -> Result<()> {
        // Detect launcher type based on directory structure
        let is_astralrinth_or_modrinth = target_dir.join("profile.json").exists();
        let is_prism = target_dir.join("mmc-pack.json").exists();
        let is_xmcl = target_dir.join("instance.json").exists();
        let is_other_launcher = target_dir.join("launcher_profiles.json").exists() &&
                               !is_astralrinth_or_modrinth && !is_prism && !is_xmcl;

        // Determine the base directory for copying files
        let base_dir = if is_astralrinth_or_modrinth {
            // AstralRinth and ModrinthApp expect files directly in the profile directory
            target_dir.to_path_buf()
        } else if is_prism {
            // PrismLauncher expects files in .minecraft subdirectory
            target_dir.join(".minecraft")
        } else if is_xmcl {
            // XMCL expects files directly in the instance directory
            target_dir.to_path_buf()
        } else if is_other_launcher {
            // Other/Custom launcher expects files directly in the target directory
            target_dir.to_path_buf()
        } else {
            // Default to .minecraft for other launchers
            target_dir.join(".minecraft")
        };

        // Copy mods directory
        let temp_mods = temp_dir.join("mods");
        let target_mods = base_dir.join("mods");

        if temp_mods.exists() {
            fs::create_dir_all(&target_mods).await?;
            self.copy_dir_recursive(&temp_mods, &target_mods).await?;
        }

        // Copy config directory
        let temp_config = temp_dir.join("config");
        let target_config = base_dir.join("config");

        if temp_config.exists() {
            fs::create_dir_all(&target_config).await?;
            self.copy_dir_recursive(&temp_config, &target_config).await?;
        }

        // Copy resourcepacks directory
        let temp_resourcepacks = temp_dir.join("resourcepacks");
        let target_resourcepacks = base_dir.join("resourcepacks");

        if temp_resourcepacks.exists() {
            fs::create_dir_all(&target_resourcepacks).await?;
            self.copy_dir_recursive(&temp_resourcepacks, &target_resourcepacks).await?;
        }

        // Copy shaderpacks directory
        let temp_shaderpacks = temp_dir.join("shaderpacks");
        let target_shaderpacks = base_dir.join("shaderpacks");

        if temp_shaderpacks.exists() {
            fs::create_dir_all(&target_shaderpacks).await?;
            self.copy_dir_recursive(&temp_shaderpacks, &target_shaderpacks).await?;
        }

        // Copy saves directory
        let temp_saves = temp_dir.join("saves");
        let target_saves = base_dir.join("saves");

        if temp_saves.exists() {
            fs::create_dir_all(&target_saves).await?;
            self.copy_dir_recursive(&temp_saves, &target_saves).await?;
        }

        // Copy servers.dat file if it exists
        let temp_servers_dat = temp_dir.join("servers.dat");
        let target_servers_dat = base_dir.join("servers.dat");

        if temp_servers_dat.exists() {
            fs::copy(&temp_servers_dat, &target_servers_dat).await?;
            info!("✓ Copied servers.dat file");
        }

        Ok(())
    }

    /// Recursively copy directory contents
    fn copy_dir_recursive<'a>(&'a self, src: &'a Path, dst: &'a Path) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
        Box::pin(async move {
            let mut entries = fs::read_dir(src).await?;

            while let Some(entry) = entries.next_entry().await? {
                let entry_path = entry.path();
                let file_name = entry_path.file_name().unwrap();
                let dst_path = dst.join(file_name);

                if entry_path.is_dir() {
                    fs::create_dir_all(&dst_path).await?;
                    self.copy_dir_recursive(&entry_path, &dst_path).await?;
                } else {
                    fs::copy(&entry_path, &dst_path).await?;
                }
            }

            Ok(())
        })
    }

    /// Create instance for Other/Custom launcher (custom path)
    async fn create_other_instance(
        &self,
        custom_path: &Path,
        instance_name: &str,
        minecraft_version: &str,
        mod_loader: &str,
        mod_loader_version: Option<&str>,
    ) -> Result<PathBuf> {
        // Install directly into the custom path (no .minecraft subdirectory)
        // Create the directory if it doesn't exist
        let minecraft_dir = custom_path.to_path_buf();
        fs::create_dir_all(&minecraft_dir).await?;

        // Create essential directories
        let dirs = [
            "mods", "config", "saves", "resourcepacks", "shaderpacks",
            "logs", "crash-reports", "datapacks", "screenshots"
        ];

        for dir in &dirs {
            fs::create_dir_all(minecraft_dir.join(dir)).await?;
        }

        // Create options.txt with default settings
        let options_content = "version:3955\nao:true\nbiomeBlendRadius:2\nenableVsync:true\nentityDistanceScaling:1.0\nentityShadows:true\nforceUnicodeFont:false\njapaneseGlyphVariants:false\nfov:0.0\nfovEffectScale:1.0\ndarknessEffectScale:1.0\nglintSpeed:0.5\nglintStrength:0.75\nprioritizeChunkUpdates:0\nfullscreen:false\ngamma:0.5\ngraphicsMode:1\nguiScale:0\nmaxFps:120\nmipmapLevels:4\nrenderDistance:12\nsimulationDistance:12\nuseVbo:true\n";
        fs::write(minecraft_dir.join("options.txt"), options_content).await?;

        // Create launcher_profiles.json for mod loader support
        let profiles = json!({
            "profiles": {
                instance_name: {
                    "name": instance_name,
                    "type": "custom",
                    "created": chrono::Utc::now().to_rfc3339(),
                    "lastUsed": chrono::Utc::now().to_rfc3339(),
                    "icon": "Grass",
                    "javaArgs": "",
                    "logConfig": "",
                    "gameDir": minecraft_dir.to_string_lossy(),
                    "javaDir": "",
                    "resolution": {
                        "width": 854,
                        "height": 480
                    },
                    "launcherVisibilityOnGameClose": "hide launcher and re-open when game closes",
                    "mods": []
                }
            },
            "selectedProfile": instance_name,
            "clientToken": "",
            "authenticationDatabase": {},
            "selectedUser": {
                "account": "",
                "profile": ""
            },
            "launcherVersion": {
                "name": "minecraft-installer",
                "format": 21,
                "profilesFormat": 2
            }
        });

        fs::write(
            minecraft_dir.join("launcher_profiles.json"),
            serde_json::to_string_pretty(&profiles)?
        ).await?;

        // Create version-specific files based on mod loader
        match mod_loader {
            "neoforge" => {
                // Create NeoForge version file
                let neoforge_version = mod_loader_version.unwrap_or("21.1.209");
                let version_id = format!("{}-{}", minecraft_version, neoforge_version);

                let version_json = json!({
                    "id": version_id,
                    "inheritsFrom": minecraft_version,
                    "type": "release",
                    "mainClass": "net.neoforged.userdev.LaunchTesting",
                    "arguments": {
                        "game": [
                            "--username", "${auth_player_name}",
                            "--version", "${version_name}",
                            "--gameDir", "${game_directory}",
                            "--assetsDir", "${assets_root}",
                            "--assetIndex", "${assets_index_name}",
                            "--uuid", "${auth_uuid}",
                            "--accessToken", "${auth_access_token}",
                            "--userType", "${user_type}",
                            "--versionType", "${version_type}",
                            "--width", "${resolution_width}",
                            "--height", "${resolution_height}"
                        ],
                        "jvm": [
                            "-Djava.library.path=${natives_directory}",
                            "-Dminecraft.launcher.brand=${launcher_name}",
                            "-Dminecraft.launcher.version=${launcher_version}",
                            "-cp", "${classpath}"
                        ]
                    },
                    "libraries": [
                        {
                            "name": "net.neoforged:neoforge:21.1.209",
                            "url": "https://maven.neoforged.net/releases/"
                        }
                    ],
                    "assetIndex": {
                        "id": minecraft_version,
                        "sha1": "",
                        "size": 0,
                        "totalSize": 0,
                        "url": format!("https://piston-meta.mojang.com/v1/packages/{}/1.json", minecraft_version)
                    },
                    "assets": minecraft_version,
                    "downloads": {
                        "client": {
                            "sha1": "",
                            "size": 0,
                            "url": format!("https://piston-data.mojang.com/v1/objects/{}/client.jar", minecraft_version)
                        }
                    },
                    "logging": {},
                    "javaVersion": {
                        "component": "java-runtime-gamma",
                        "majorVersion": 21
                    }
                });

                let versions_dir = minecraft_dir.join("versions").join(&version_id);
                fs::create_dir_all(&versions_dir).await?;
                fs::write(
                    versions_dir.join(format!("{}.json", version_id)),
                    serde_json::to_string_pretty(&version_json)?
                ).await?;
            }
            "fabric" => {
                // Create Fabric version file
                let fabric_version = mod_loader_version.unwrap_or("0.15.11");
                let version_id = format!("{}-fabric-{}", minecraft_version, fabric_version);

                let version_json = json!({
                    "id": version_id,
                    "inheritsFrom": minecraft_version,
                    "type": "release",
                    "mainClass": "net.fabricmc.loader.impl.launch.knot.KnotClient",
                    "arguments": {
                        "game": [
                            "--username", "${auth_player_name}",
                            "--version", "${version_name}",
                            "--gameDir", "${game_directory}",
                            "--assetsDir", "${assets_root}",
                            "--assetIndex", "${assets_index_name}",
                            "--uuid", "${auth_uuid}",
                            "--accessToken", "${auth_access_token}",
                            "--userType", "${user_type}",
                            "--versionType", "${version_type}",
                            "--width", "${resolution_width}",
                            "--height", "${resolution_height}"
                        ],
                        "jvm": [
                            "-Djava.library.path=${natives_directory}",
                            "-Dminecraft.launcher.brand=${launcher_name}",
                            "-Dminecraft.launcher.version=${launcher_version}",
                            "-cp", "${classpath}"
                        ]
                    },
                    "libraries": [
                        {
                            "name": "net.fabricmc:fabric-loader:0.15.11",
                            "url": "https://maven.fabricmc.net/"
                        }
                    ],
                    "assetIndex": {
                        "id": minecraft_version,
                        "sha1": "",
                        "size": 0,
                        "totalSize": 0,
                        "url": format!("https://piston-meta.mojang.com/v1/packages/{}/1.json", minecraft_version)
                    },
                    "assets": minecraft_version,
                    "downloads": {
                        "client": {
                            "sha1": "",
                            "size": 0,
                            "url": format!("https://piston-data.mojang.com/v1/objects/{}/client.jar", minecraft_version)
                        }
                    },
                    "logging": {},
                    "javaVersion": {
                        "component": "java-runtime-gamma",
                        "majorVersion": 21
                    }
                });

                let versions_dir = minecraft_dir.join("versions").join(&version_id);
                fs::create_dir_all(&versions_dir).await?;
                fs::write(
                    versions_dir.join(format!("{}.json", version_id)),
                    serde_json::to_string_pretty(&version_json)?
                ).await?;
            }
            _ => {
                // For vanilla or other loaders, just create a basic version file
                let version_json = json!({
                    "id": minecraft_version,
                    "type": "release",
                    "mainClass": "net.minecraft.client.main.Main",
                    "arguments": {
                        "game": [
                            "--username", "${auth_player_name}",
                            "--version", "${version_name}",
                            "--gameDir", "${game_directory}",
                            "--assetsDir", "${assets_root}",
                            "--assetIndex", "${assets_index_name}",
                            "--uuid", "${auth_uuid}",
                            "--accessToken", "${auth_access_token}",
                            "--userType", "${user_type}",
                            "--versionType", "${version_type}",
                            "--width", "${resolution_width}",
                            "--height", "${resolution_height}"
                        ],
                        "jvm": [
                            "-Djava.library.path=${natives_directory}",
                            "-Dminecraft.launcher.brand=${launcher_name}",
                            "-Dminecraft.launcher.version=${launcher_version}",
                            "-cp", "${classpath}"
                        ]
                    },
                    "assetIndex": {
                        "id": minecraft_version,
                        "sha1": "",
                        "size": 0,
                        "totalSize": 0,
                        "url": format!("https://piston-meta.mojang.com/v1/packages/{}/1.json", minecraft_version)
                    },
                    "assets": minecraft_version,
                    "downloads": {
                        "client": {
                            "sha1": "",
                            "size": 0,
                            "url": format!("https://piston-data.mojang.com/v1/objects/{}/client.jar", minecraft_version)
                        }
                    },
                    "logging": {},
                    "javaVersion": {
                        "component": "java-runtime-gamma",
                        "majorVersion": 21
                    }
                });

                let versions_dir = minecraft_dir.join("versions").join(minecraft_version);
                fs::create_dir_all(&versions_dir).await?;
                fs::write(
                    versions_dir.join(format!("{}.json", minecraft_version)),
                    serde_json::to_string_pretty(&version_json)?
                ).await?;
            }
        }

        info!("Created Other/Custom launcher instance: {} at {}", instance_name, minecraft_dir.display());
        Ok(minecraft_dir)
    }
}
