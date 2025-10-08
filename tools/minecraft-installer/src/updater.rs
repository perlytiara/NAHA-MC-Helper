use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing::{info, debug};
use crate::error::{MinecraftInstallerError, Result};
use crate::launcher_support::{LauncherManager, LauncherType, MrpackIndex, MrpackFile, NahaModpackInfo};

/// Instance information for display in Electron app
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceInfo {
    pub name: String,
    pub launcher_type: String,
    pub launcher_path: String,
    pub instance_path: String,
    pub minecraft_version: String,
    pub mod_loader: String,
    pub mod_loader_version: Option<String>,
    pub mod_count: usize,
    pub mods: Vec<ModInfo>,
    pub has_automodpack: bool,
    pub server_info: Option<ServerInfo>,
    pub last_updated: Option<String>,
}

/// Mod information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModInfo {
    pub name: String,
    pub filename: String,
    pub version: Option<String>,
    pub mod_id: Option<String>,
    pub is_user_mod: bool, // true if added by user, false if from modpack
    pub file_size: u64,
    pub last_modified: String,
}

/// Server information from automodpack
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub server_ip: String,
    pub server_port: u16,
    pub fingerprint: String,
    pub server_name: String,
}

/// Update result for a specific instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResult {
    pub instance_name: String,
    pub success: bool,
    pub updated_mods: Vec<String>,
    pub new_mods: Vec<String>,
    pub preserved_mods: Vec<String>,
    pub errors: Vec<String>,
    pub message: String,
}

/// Main updater for Minecraft instances
pub struct MinecraftUpdater {
    launcher_manager: LauncherManager,
}

impl MinecraftUpdater {
    pub fn new() -> Self {
        Self {
            launcher_manager: LauncherManager::new(),
        }
    }

    /// Scan all launchers and return instance information
    pub async fn scan_instances(&self) -> Result<Vec<InstanceInfo>> {
        info!("🔍 Scanning for Minecraft instances...");

        let mut instances = Vec::new();
        let detected_launchers = self.launcher_manager.detect_launchers().await;

        for (launcher_type, launcher_path) in detected_launchers {
            match launcher_type {
                LauncherType::AstralRinth => {
                    info!("📱 Scanning AstralRinth database at: {}", launcher_path.display());
                    let profiles = self.scan_astralrinth_profiles(&launcher_path).await?;
                    info!("✅ Found {} AstralRinth instances", profiles.len());
                    instances.extend(profiles);
                }
                LauncherType::ModrinthApp => {
                    info!("📱 Scanning ModrinthApp database at: {}", launcher_path.display());
                    let profiles = self.scan_astralrinth_profiles(&launcher_path).await?;
                    info!("✅ Found {} ModrinthApp instances", profiles.len());
                    instances.extend(profiles);
                }
                LauncherType::XMCL => {
                    info!("📁 Scanning XMCL folders at: {}", launcher_path.display());
                    let xmcl_instances = self.scan_xmcl_instances(&launcher_path).await?;
                    info!("✅ Found {} XMCL instances", xmcl_instances.len());
                    instances.extend(xmcl_instances);
                }
                LauncherType::Prism | LauncherType::PrismCracked => {
                    info!("📁 Scanning PrismLauncher folders at: {}", launcher_path.display());
                    let prism_instances = self.scan_prism_instances(&launcher_path).await?;
                    info!("✅ Found {} PrismLauncher instances", prism_instances.len());
                    instances.extend(prism_instances);
                }
                LauncherType::Official => {
                    info!("📁 Scanning Official Minecraft profiles at: {}", launcher_path.display());
                    let official_instances = self.scan_official_instances(&launcher_path).await?;
                    info!("✅ Found {} Official Minecraft instances", official_instances.len());
                    instances.extend(official_instances);
                }
                _ => {
                    debug!("⏭️  Skipping unsupported launcher type: {:?}", launcher_type);
                }
            }
        }

        info!("🎯 Total instances found: {}", instances.len());
        Ok(instances)
    }

    /// Update mods for a specific instance
    pub async fn update_instance_mods(
        &self,
        instance_path: &Path,
        modpack_type: &str, // "neoforge" or "fabric"
    ) -> Result<UpdateResult> {
        self.update_instance_mods_version(instance_path, modpack_type, None).await
    }

    /// Update mods for a specific instance with optional version selection
    pub async fn update_instance_mods_version(
        &self,
        instance_path: &Path,
        modpack_type: &str, // "neoforge" or "fabric"
        version: Option<&str>, // Optional specific version (e.g., "0.0.18")
    ) -> Result<UpdateResult> {
        println!("🔄 Starting update process for: {}", instance_path.display());
        println!("📦 Modpack type: {}", modpack_type);
        if let Some(v) = version {
            println!("🎯 Target version: {}", v);
        }
        
        info!("Updating mods for instance: {}", instance_path.display());

        // Get modpack info from GitHub
        println!("🌐 Fetching modpack info from GitHub...");
        let modpack_info = if let Some(target_version) = version {
            self.launcher_manager.fetch_modpack_info_version(modpack_type, target_version).await?
        } else {
            self.launcher_manager.fetch_modpack_info(modpack_type).await?
        };
        println!("✅ Found modpack: {} v{}", modpack_info.server_name, modpack_info.version);
        println!("📥 Download URL: {}", modpack_info.download_url);

        // Download and extract the latest mrpack
        println!("📁 Creating temporary directory...");
        let temp_dir = instance_path.join("temp_update");
        fs::create_dir_all(&temp_dir).await?;

        println!("⬇️  Downloading latest mrpack...");
        let mrpack_path = self.download_latest_mrpack(&modpack_info, &temp_dir).await?;
        println!("✅ Downloaded: {}", mrpack_path.display());
        
        println!("📦 Extracting mrpack contents...");
        let mrpack_index_json = self.extract_mrpack_index(&mrpack_path).await?;
        
        // Parse the mrpack index
        println!("🔍 Parsing mrpack index...");
        let mut mrpack_index: MrpackIndex = serde_json::from_str(&mrpack_index_json)
            .map_err(|e| {
                println!("❌ Failed to parse mrpack index: {}", e);
                println!("📄 Raw JSON (first 200 chars): {}", &mrpack_index_json[..200.min(mrpack_index_json.len())]);
                e
            })?;
        
        // Debug: Check for duplicate paths in the mrpack index
        let mut path_counts = std::collections::HashMap::new();
        for file in &mrpack_index.files {
            *path_counts.entry(&file.path).or_insert(0) += 1;
        }
        
        let duplicates: Vec<_> = path_counts.iter()
            .filter(|(_, &count)| count > 1)
            .collect();
        
        if !duplicates.is_empty() {
            println!("⚠️  Found {} duplicate paths in mrpack index, deduplicating...", duplicates.len());
            
            // Deduplicate files by path
            let mut seen_paths = std::collections::HashSet::new();
            let original_count = mrpack_index.files.len();
            mrpack_index.files.retain(|file| {
                if seen_paths.contains(&file.path) {
                    false
                } else {
                    seen_paths.insert(file.path.clone());
                    true
                }
            });
            println!("✅ Removed {} duplicate entries from mrpack", original_count - mrpack_index.files.len());
        }
        
        println!("✅ Parsed mrpack with {} unique files", mrpack_index.files.len());

        // Analyze existing mods
        println!("🔍 Analyzing existing mods in instance...");
        let mods_dir = self.find_mods_directory(instance_path).await?;
        println!("📁 Mods directory: {}", mods_dir.display());
        println!("📁 Mods directory exists: {}", mods_dir.exists());
        let existing_mods = self.analyze_existing_mods_simple(instance_path).await?;
        println!("📊 Found {} existing mods", existing_mods.len());

        // Update mods intelligently
        println!("🔄 Updating mods intelligently...");
        let update_result = self.update_mods_intelligently(
            instance_path,
            &mrpack_index,
            &existing_mods,
            &modpack_info,
        ).await?;

        // Clean up temp directory
        println!("🧹 Cleaning up temporary files...");
        let _ = fs::remove_dir_all(&temp_dir).await;

        println!("✅ Update completed successfully!");
        Ok(update_result)
    }

    /// Extract mrpack index from downloaded mrpack file
    async fn extract_mrpack_index(&self, mrpack_path: &Path) -> Result<String> {
        use zip::ZipArchive;
        use std::io::Read;
        
        let file = std::fs::File::open(mrpack_path)?;
        let mut archive = ZipArchive::new(file)?;
        
        // Look for modrinth.index.json in the mrpack
        let mut index_file = archive.by_name("modrinth.index.json")
            .map_err(|_| MinecraftInstallerError::InstallationFailed(
                "modrinth.index.json not found in mrpack".to_string()
            ))?;
        
        let mut contents = String::new();
        index_file.read_to_string(&mut contents)?;
        
        Ok(contents)
    }

    /// Get previous version mrpack for comparison
    async fn get_previous_version_mrpack(&self, modpack_info: &NahaModpackInfo) -> Result<Option<PathBuf>> {
        println!("🔍 Looking for previous version for comparison...");
        
        // Use GitHub API to get all releases
        let api_url = "https://api.github.com/repos/perlytiara/NAHA-Minecraft-Modpacks/releases";
        let client = reqwest::Client::new();
        let response = client.get(api_url)
            .header("User-Agent", "Minecraft-Installer/1.0")
            .send().await?;

        if !response.status().is_success() {
            println!("⚠️  Could not fetch previous releases, skipping comparison");
            return Ok(None);
        }

        let releases: Vec<serde_json::Value> = response.json().await?;
        
        // Find the previous release (not the latest)
        for release in releases.iter().skip(1) { // Skip latest release
            let empty_vec = vec![];
            let assets = release["assets"].as_array().unwrap_or(&empty_vec);
            let mrpack_asset = if modpack_info.server_type == "neoforge" {
                assets.iter().find(|asset| {
                    let name = asset["name"].as_str().unwrap_or("");
                    (name.contains("NeoForge") || name.contains("Neoforge")) && name.ends_with(".mrpack")
                })
            } else if modpack_info.server_type == "fabric" {
                assets.iter().find(|asset| {
                    let name = asset["name"].as_str().unwrap_or("");
                    name.contains("Fabric") && name.ends_with(".mrpack")
                })
            } else {
                None
            };

            if let Some(asset) = mrpack_asset {
                let download_url = asset["browser_download_url"].as_str().unwrap_or("");
                let filename = asset["name"].as_str().unwrap_or("previous.mrpack");
                
                println!("📥 Downloading previous version: {}", filename);
                
                // Download to temp directory
                let temp_dir = std::env::temp_dir().join("minecraft_updater");
                fs::create_dir_all(&temp_dir).await?;
                let mrpack_path = temp_dir.join(filename);
                
                let response = client.get(download_url).send().await?;
                let content = response.bytes().await?;
                fs::write(&mrpack_path, content).await?;
                
                println!("✅ Downloaded previous version: {}", filename);
                return Ok(Some(mrpack_path));
            }
        }
        
        println!("⚠️  No previous version found, skipping comparison");
        Ok(None)
    }

    /// Extract mods from a mrpack file
    async fn extract_mods_from_mrpack(&self, mrpack_path: &Path) -> Result<Vec<ModInfo>> {
        use zip::ZipArchive;
        use std::io::Read;
        
        let file = std::fs::File::open(mrpack_path)?;
        let mut archive = ZipArchive::new(file)?;
        let mut mods = Vec::new();
        
        // Extract modrinth.index.json
        let mut index_file = archive.by_name("modrinth.index.json")?;
        let mut contents = String::new();
        index_file.read_to_string(&mut contents)?;
        
        let index_data: serde_json::Value = serde_json::from_str(&contents)?;
        if let Some(files) = index_data["files"].as_array() {
            for file in files {
                if let Some(path) = file["path"].as_str() {
                    if path.starts_with("mods/") && path.ends_with(".jar") {
                        let filename = Path::new(path).file_name()
                            .unwrap()
                            .to_string_lossy()
                            .to_string();
                        
                        let mod_info = ModInfo {
                            name: filename.clone(),
                            filename: filename,
                            version: None,
                            is_user_mod: false,
                            file_size: 0,
                            last_modified: "unknown".to_string(),
                            mod_id: None,
                        };
                        mods.push(mod_info);
                    }
                }
            }
        }
        
        Ok(mods)
    }

    /// Create normalized mod map for comparison
    fn create_mod_map(&self, mods: &[ModInfo]) -> HashMap<String, ModInfo> {
        let mut map = HashMap::new();
        for mod_info in mods {
            let normalized_name = self.normalize_mod_name(&mod_info.name);
            map.insert(normalized_name, mod_info.clone());
        }
        map
    }

    /// Create mod map from HashMap
    fn create_mod_map_from_hashmap(&self, mods: &HashMap<String, ModInfo>) -> HashMap<String, ModInfo> {
        let mut map = HashMap::new();
        for (_, mod_info) in mods {
            let normalized_name = self.normalize_mod_name(&mod_info.name);
            map.insert(normalized_name, mod_info.clone());
        }
        map
    }

    /// Create mod map from mrpack index
    fn create_mrpack_mod_map(&self, mrpack_index: &MrpackIndex) -> HashMap<String, ModInfo> {
        let mut map = HashMap::new();
        for file in &mrpack_index.files {
            if file.path.starts_with("mods/") && file.path.ends_with(".jar") {
                let filename = Path::new(&file.path).file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let normalized_name = self.normalize_mod_name(&filename);
                let mod_info = ModInfo {
                    name: filename.clone(),
                    filename: filename,
                    version: None, // We'll extract this from the file
                    is_user_mod: false,
                    file_size: 0,
                    last_modified: "unknown".to_string(),
                    mod_id: None,
                };
                map.insert(normalized_name, mod_info);
            }
        }
        map
    }

    /// Normalize mod name for comparison
    fn normalize_mod_name(&self, name: &str) -> String {
        let name = name.to_lowercase().replace(".jar", "").replace(".disabled", "");
        
        // Handle complex mod names with $ separators first
        let name = if name.contains('$') {
            name.split('$').next().unwrap_or(&name).to_string()
        } else {
            name
        };
        
        // Extract just the base mod name (first part before version patterns)
        let parts: Vec<&str> = name.split('-').collect();
        
        if parts.is_empty() {
            return name;
        }
        
        // Special handling for multi-word mod names
        // "sodium-extra" should stay as "sodium-extra", not just "sodium"
        if parts.len() >= 2 {
            let second_part = parts[1];
            // If second part is NOT a version number (doesn't start with digit), include it
            if !second_part.chars().next().unwrap_or('0').is_ascii_digit() {
                return format!("{}-{}", parts[0], parts[1]);
            }
        }
        
        // Just return the first part - the actual mod name
        // "badoptimizations-2.3.0-1.21.1" -> "badoptimizations"
        // "bocchud-0.4.0+mc1.21.1" -> "bocchud"
        // "chat_heads-0.14.0-neoforge-1.21" -> "chat_heads"
        // "sodium-extra-0.6.0" -> "sodium-extra"
        parts[0].to_string()
    }

    /// Clean up duplicate mods
    async fn cleanup_duplicate_mods(&self, mods_dir: &Path) -> Result<()> {
        let mut entries = fs::read_dir(mods_dir).await?;
        let mut mod_groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

        // Group mods by normalized name
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "jar") {
                let filename = path.file_name().unwrap().to_string_lossy();
                let normalized = self.normalize_mod_name(&filename);
                mod_groups.entry(normalized).or_insert_with(Vec::new).push(path);
            }
        }

        // Remove duplicates, keeping the newest version
        for (normalized_name, mut paths) in mod_groups {
            if paths.len() > 1 {
                println!("🔍 Found {} duplicates for mod: {}", paths.len(), normalized_name);
                
                // Sort by modification time (newest first)
                paths.sort_by(|a, b| {
                    let a_time = a.metadata().unwrap().modified().unwrap();
                    let b_time = b.metadata().unwrap().modified().unwrap();
                    b_time.cmp(&a_time)
                });

                // Keep the newest, remove the rest
                let newest = &paths[0];
                println!("✅ Keeping newest: {}", newest.file_name().unwrap().to_string_lossy());
                
                for path in paths.iter().skip(1) {
                    println!("🗑️  Removing old version: {}", path.file_name().unwrap().to_string_lossy());
                    let _ = fs::remove_file(path).await;
                }
            }
        }

        Ok(())
    }

    /// Scan AstralRinth/ModrinthApp profiles
    async fn scan_astralrinth_profiles(&self, launcher_path: &Path) -> Result<Vec<InstanceInfo>> {
        let mut instances = Vec::new();
        let profiles_dir = launcher_path.join("profiles");

        if !profiles_dir.exists() {
            return Ok(instances);
        }

        let mut entries = fs::read_dir(&profiles_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let profile_path = entry.path();
            if profile_path.is_dir() {
                if let Some(instance_info) = self.analyze_astralrinth_profile(&profile_path, launcher_path).await? {
                    instances.push(instance_info);
                }
            }
        }

        Ok(instances)
    }

    /// Analyze a single AstralRinth profile
    async fn analyze_astralrinth_profile(
        &self,
        profile_path: &Path,
        launcher_path: &Path,
    ) -> Result<Option<InstanceInfo>> {
        // AstralRinth stores profile info in the database, not in profile.json files
        // For now, we'll extract basic info from the folder structure
        let profile_name = profile_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();
        
        // Try to determine mod loader from folder name
        let (mod_loader, mod_loader_version) = if profile_name.to_lowercase().contains("neoforge") {
            ("NeoForge".to_string(), None)
        } else if profile_name.to_lowercase().contains("fabric") {
            ("Fabric".to_string(), None)
        } else if profile_name.to_lowercase().contains("forge") {
            ("Forge".to_string(), None)
        } else {
            ("Unknown".to_string(), None)
        };
        
        // Try to extract Minecraft version from folder name
        let minecraft_version = if let Some(version_match) = profile_name.split('-').find(|part| {
            part.starts_with("1.") && part.len() >= 3
        }) {
            version_match.to_string()
        } else {
            "Unknown".to_string()
        };

        // Analyze mods
        let mods_dir = profile_path.join("mods");
        let (mods, mod_count) = if mods_dir.exists() {
            let mods = self.analyze_mods_directory(&mods_dir).await?;
            (mods.clone(), mods.len())
        } else {
            (Vec::new(), 0)
        };

        // Check for automodpack
        let has_automodpack = profile_path.join("automodpack-known-hosts.json").exists();
        let server_info = if has_automodpack {
            self.extract_server_info(profile_path).await.ok()
        } else {
            None
        };

        // Determine launcher type based on the launcher path
        let launcher_type = if launcher_path.to_string_lossy().contains("ModrinthApp") {
            "ModrinthApp"
        } else {
            "AstralRinth"
        };

        Ok(Some(InstanceInfo {
            name: profile_name,
            launcher_type: launcher_type.to_string(),
            launcher_path: launcher_path.to_string_lossy().to_string(),
            instance_path: profile_path.to_string_lossy().to_string(),
            minecraft_version,
            mod_loader,
            mod_loader_version,
            mod_count,
            mods,
            has_automodpack,
            server_info,
            last_updated: None, // TODO: Extract from profile metadata
        }))
    }

    /// Scan XMCL instances
    async fn scan_xmcl_instances(&self, launcher_path: &Path) -> Result<Vec<InstanceInfo>> {
        let mut instances = Vec::new();
        let instances_dir = launcher_path.join("instances");

        if !instances_dir.exists() {
            return Ok(instances);
        }

        let mut entries = fs::read_dir(&instances_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let instance_path = entry.path();
            if instance_path.is_dir() {
                if let Some(instance_info) = self.analyze_xmcl_instance(&instance_path, launcher_path).await? {
                    instances.push(instance_info);
                }
            }
        }

        Ok(instances)
    }

    /// Analyze a single XMCL instance
    async fn analyze_xmcl_instance(
        &self,
        instance_path: &Path,
        launcher_path: &Path,
    ) -> Result<Option<InstanceInfo>> {
        let instance_json_path = instance_path.join("instance.json");
        if !instance_json_path.exists() {
            return Ok(None);
        }

        let instance_content = fs::read_to_string(&instance_json_path).await?;
        let instance_data: serde_json::Value = serde_json::from_str(&instance_content)?;

        let name = instance_data["name"].as_str().unwrap_or("Unknown").to_string();
        let minecraft_version = instance_data["runtime"]["minecraft"].as_str().unwrap_or("Unknown").to_string();
        
        // Determine mod loader from XMCL runtime structure
        let (mod_loader, mod_loader_version) = if let Some(neo_forged) = instance_data["runtime"]["neoForged"].as_str() {
            if !neo_forged.is_empty() {
                ("NeoForge".to_string(), Some(neo_forged.to_string()))
            } else if let Some(fabric_loader) = instance_data["runtime"]["fabricLoader"].as_str() {
                if !fabric_loader.is_empty() {
                    ("Fabric".to_string(), Some(fabric_loader.to_string()))
                } else if let Some(forge) = instance_data["runtime"]["forge"].as_str() {
                    if !forge.is_empty() {
                        ("Forge".to_string(), Some(forge.to_string()))
                    } else {
                        ("Unknown".to_string(), None)
                    }
                } else {
                    ("Unknown".to_string(), None)
                }
            } else if let Some(forge) = instance_data["runtime"]["forge"].as_str() {
                if !forge.is_empty() {
                    ("Forge".to_string(), Some(forge.to_string()))
                } else {
                    ("Unknown".to_string(), None)
                }
            } else {
                ("Unknown".to_string(), None)
            }
        } else if let Some(fabric_loader) = instance_data["runtime"]["fabricLoader"].as_str() {
            if !fabric_loader.is_empty() {
                ("Fabric".to_string(), Some(fabric_loader.to_string()))
            } else {
                ("Unknown".to_string(), None)
            }
        } else if let Some(forge) = instance_data["runtime"]["forge"].as_str() {
            if !forge.is_empty() {
                ("Forge".to_string(), Some(forge.to_string()))
            } else {
                ("Unknown".to_string(), None)
            }
        } else {
            ("Unknown".to_string(), None)
        };

        // Analyze mods
        let mods_dir = instance_path.join("mods");
        let (mods, mod_count) = if mods_dir.exists() {
            let mods = self.analyze_mods_directory(&mods_dir).await?;
            (mods.clone(), mods.len())
        } else {
            (Vec::new(), 0)
        };

        // Check for automodpack
        let has_automodpack = instance_path.join("automodpack-known-hosts.json").exists();
        let server_info = if has_automodpack {
            self.extract_server_info(instance_path).await.ok()
        } else {
            None
        };

        Ok(Some(InstanceInfo {
            name,
            launcher_type: "XMCL".to_string(),
            launcher_path: launcher_path.to_string_lossy().to_string(),
            instance_path: instance_path.to_string_lossy().to_string(),
            minecraft_version,
            mod_loader,
            mod_loader_version,
            mod_count,
            mods,
            has_automodpack,
            server_info,
            last_updated: None,
        }))
    }

    /// Scan PrismLauncher instances
    async fn scan_prism_instances(&self, launcher_path: &Path) -> Result<Vec<InstanceInfo>> {
        let mut instances = Vec::new();
        let instances_dir = launcher_path.join("instances");

        if !instances_dir.exists() {
            return Ok(instances);
        }

        let mut entries = fs::read_dir(&instances_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let instance_path = entry.path();
            if instance_path.is_dir() {
                if let Some(instance_info) = self.analyze_prism_instance(&instance_path, launcher_path).await? {
                    instances.push(instance_info);
                }
            }
        }

        Ok(instances)
    }

    /// Analyze a single PrismLauncher instance
    async fn analyze_prism_instance(
        &self,
        instance_path: &Path,
        launcher_path: &Path,
    ) -> Result<Option<InstanceInfo>> {
        let instance_cfg_path = instance_path.join("instance.cfg");
        let mmc_pack_path = instance_path.join("mmc-pack.json");

        if !instance_cfg_path.exists() || !mmc_pack_path.exists() {
            return Ok(None);
        }

        // Read instance name from instance.cfg
        let instance_cfg_content = fs::read_to_string(&instance_cfg_path).await?;
        let name = instance_cfg_content
            .lines()
            .find(|line| line.starts_with("name="))
            .and_then(|line| line.split('=').nth(1))
            .unwrap_or("Unknown")
            .to_string();

        // Read pack info from mmc-pack.json
        let mmc_pack_content = fs::read_to_string(&mmc_pack_path).await?;
        let mmc_pack_data: serde_json::Value = serde_json::from_str(&mmc_pack_content)?;

        let minecraft_version = mmc_pack_data["components"]
            .as_array()
            .and_then(|components| {
                components.iter().find(|c| c["cachedName"].as_str() == Some("Minecraft"))
            })
            .and_then(|mc| mc["version"].as_str())
            .unwrap_or("Unknown")
            .to_string();

        // Determine mod loader from components
        let (mod_loader, mod_loader_version) = if let Some(components) = mmc_pack_data["components"].as_array() {
            if let Some(fabric_loader) = components.iter().find(|c| {
                let name = c["cachedName"].as_str().unwrap_or("");
                name.contains("Fabric") || name.contains("fabric")
            }) {
                let version = fabric_loader["cachedVersion"].as_str().unwrap_or("Unknown");
                ("Fabric".to_string(), Some(version.to_string()))
            } else if let Some(forge_loader) = components.iter().find(|c| {
                let name = c["cachedName"].as_str().unwrap_or("");
                name.contains("Forge") || name.contains("forge")
            }) {
                let version = forge_loader["cachedVersion"].as_str().unwrap_or("Unknown");
                ("Forge".to_string(), Some(version.to_string()))
            } else if let Some(neoforge_loader) = components.iter().find(|c| {
                let name = c["cachedName"].as_str().unwrap_or("");
                name.contains("NeoForge") || name.contains("neoforge")
            }) {
                let version = neoforge_loader["cachedVersion"].as_str().unwrap_or("Unknown");
                ("NeoForge".to_string(), Some(version.to_string()))
            } else {
                ("Unknown".to_string(), None)
            }
        } else {
            ("Unknown".to_string(), None)
        };

        // Analyze mods
        let mods_dir = instance_path.join("minecraft").join("mods");
        let (mods, mod_count) = if mods_dir.exists() {
            let mods = self.analyze_mods_directory(&mods_dir).await?;
            (mods.clone(), mods.len())
        } else {
            (Vec::new(), 0)
        };

        // Check for automodpack
        let minecraft_dir = instance_path.join(".minecraft");
        let has_automodpack = minecraft_dir.join("automodpack-known-hosts.json").exists();
        let server_info = if has_automodpack {
            self.extract_server_info(&minecraft_dir).await.ok()
        } else {
            None
        };

        Ok(Some(InstanceInfo {
            name,
            launcher_type: "PrismLauncher".to_string(),
            launcher_path: launcher_path.to_string_lossy().to_string(),
            instance_path: instance_path.to_string_lossy().to_string(),
            minecraft_version,
            mod_loader,
            mod_loader_version: None,
            mod_count,
            mods,
            has_automodpack,
            server_info,
            last_updated: None,
        }))
    }

    /// Scan Official Minecraft Launcher instances
    async fn scan_official_instances(&self, launcher_path: &Path) -> Result<Vec<InstanceInfo>> {
        let mut instances = Vec::new();
        let profiles_path = launcher_path.join("launcher_profiles.json");

        if !profiles_path.exists() {
            return Ok(instances);
        }

        let profiles_content = fs::read_to_string(&profiles_path).await?;
        let profiles_data: serde_json::Value = serde_json::from_str(&profiles_content)?;

        if let Some(profiles) = profiles_data["profiles"].as_object() {
            for (profile_id, profile) in profiles {
                if let Some(instance_info) = self.analyze_official_profile(profile_id, profile, launcher_path).await? {
                    instances.push(instance_info);
                }
            }
        }

        Ok(instances)
    }

    /// Analyze a single Official Minecraft profile
    async fn analyze_official_profile(
        &self,
        _profile_id: &str,
        profile: &serde_json::Value,
        launcher_path: &Path,
    ) -> Result<Option<InstanceInfo>> {
        let name = profile["name"].as_str().unwrap_or("Unknown").to_string();
        let minecraft_version = profile["lastVersionId"].as_str().unwrap_or("Unknown").to_string();

        // Official launcher doesn't have mod loaders by default
        let mod_loader = "Vanilla".to_string();

        // Check if this is a modded profile by looking for mods directory
        let game_dir = profile["gameDir"].as_str()
            .map(|s| PathBuf::from(s))
            .unwrap_or_else(|| launcher_path.to_path_buf());

        let mods_dir = game_dir.join("mods");
        let (mods, mod_count) = if mods_dir.exists() {
            let mods = self.analyze_mods_directory(&mods_dir).await?;
            (mods.clone(), mods.len())
        } else {
            (Vec::new(), 0)
        };

        // Check for automodpack
        let has_automodpack = game_dir.join("automodpack-known-hosts.json").exists();
        let server_info = if has_automodpack {
            self.extract_server_info(&game_dir).await.ok()
        } else {
            None
        };

        Ok(Some(InstanceInfo {
            name,
            launcher_type: "Official".to_string(),
            launcher_path: launcher_path.to_string_lossy().to_string(),
            instance_path: game_dir.to_string_lossy().to_string(),
            minecraft_version,
            mod_loader,
            mod_loader_version: None,
            mod_count,
            mods,
            has_automodpack,
            server_info,
            last_updated: None,
        }))
    }

    /// Analyze mods in a directory
    async fn analyze_mods_directory(&self, mods_dir: &Path) -> Result<Vec<ModInfo>> {
        let mut mods = Vec::new();
        let mut entries = fs::read_dir(mods_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let mod_path = entry.path();
            if mod_path.extension().and_then(|s| s.to_str()) == Some("jar") {
                if let Some(mod_info) = self.analyze_mod_file(&mod_path).await? {
                    mods.push(mod_info);
                }
            }
        }

        Ok(mods)
    }

    /// Analyze a single mod file
    async fn analyze_mod_file(&self, mod_path: &Path) -> Result<Option<ModInfo>> {
        let filename = mod_path.file_name().unwrap().to_string_lossy().to_string();
        let metadata = fs::metadata(mod_path).await?;
        let file_size = metadata.len();
        let last_modified = metadata.modified()?.duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Try to extract mod information from JAR
        let (name, version, mod_id) = self.extract_mod_metadata(mod_path).await?;

        // Determine if this is a user mod (not from a known modpack)
        let is_user_mod = self.is_user_mod(&filename, &name);

        Ok(Some(ModInfo {
            name,
            filename,
            version,
            mod_id,
            is_user_mod,
            file_size,
            last_modified: chrono::DateTime::from_timestamp(last_modified as i64, 0)
                .unwrap_or_default()
                .to_rfc3339(),
        }))
    }

    /// Extract mod metadata from JAR file
    async fn extract_mod_metadata(&self, mod_path: &Path) -> Result<(String, Option<String>, Option<String>)> {
        // This is a simplified version - in a real implementation, you'd parse the JAR's mods.toml or fabric.mod.json
        let filename = mod_path.file_name().unwrap().to_string_lossy();

        // Extract name from filename (remove version numbers)
        let name = filename
            .replace(".jar", "")
            .split('-')
            .next()
            .unwrap_or(&filename)
            .to_string();

        // Try to extract version from filename
        let version = self.extract_version_from_filename(&filename);

        Ok((name, version, None))
    }

    /// Extract version from filename
    fn extract_version_from_filename(&self, filename: &str) -> Option<String> {
        // Look for version patterns like -1.0.0, -1.0, etc.
        let parts: Vec<&str> = filename.split('-').collect();
        if parts.len() > 1 {
            let last_part = parts.last().unwrap().replace(".jar", "");
            if last_part.chars().any(|c| c.is_ascii_digit()) {
                return Some(last_part);
            }
        }
        None
    }

    /// Determine if a mod is user-added
    fn is_user_mod(&self, filename: &str, name: &str) -> bool {
        // Known modpack mods that should be updated
        let known_modpack_mods = [
            "sodium", "iris", "lithium", "phosphor", "fabric-api", "neoforge",
            "jei", "jade", "wthit", "modmenu", "cloth-config", "auto-config",
        ];

        let lowercase_name = name.to_lowercase();
        let lowercase_filename = filename.to_lowercase();

        // Check if it's a known modpack mod
        for known_mod in &known_modpack_mods {
            if lowercase_name.contains(known_mod) || lowercase_filename.contains(known_mod) {
                return false; // This is a modpack mod, not a user mod
            }
        }

        // If it doesn't match known modpack patterns, assume it's user-added
        true
    }

    /// Extract server information from automodpack files
    async fn extract_server_info(&self, instance_path: &Path) -> Result<ServerInfo> {
        let known_hosts_path = instance_path.join("automodpack-known-hosts.json");
        let servers_dat_path = instance_path.join("servers.dat");

        let mut server_ip = "Unknown".to_string();
        let server_port = 25565;
        let mut fingerprint = "Unknown".to_string();

        // Read from automodpack-known-hosts.json
        if known_hosts_path.exists() {
            let content = fs::read_to_string(&known_hosts_path).await?;
            if let Ok(hosts_data) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(hosts) = hosts_data["hosts"].as_object() {
                    for (ip, fp) in hosts {
                        server_ip = ip.clone();
                        fingerprint = fp.as_str().unwrap_or("Unknown").to_string();
                        break; // Take the first server
                    }
                }
            }
        }

        // Try to extract port from servers.dat if available
        if servers_dat_path.exists() {
            // This would require parsing NBT format - simplified for now
            debug!("servers.dat found but NBT parsing not implemented");
        }

        Ok(ServerInfo {
            server_ip,
            server_port,
            fingerprint,
            server_name: "NAHA Server".to_string(),
        })
    }

    /// Download latest mrpack from API
    async fn download_latest_mrpack(&self, modpack_info: &NahaModpackInfo, temp_dir: &Path) -> Result<PathBuf> {
        info!("Downloading latest mrpack: {}", modpack_info.latest_mrpack);

        let mrpack_path = temp_dir.join("latest.mrpack");

        // Download the mrpack file
        let client = reqwest::Client::new();
        let response = client.get(&modpack_info.download_url).send().await?;

        if !response.status().is_success() {
            return Err(MinecraftInstallerError::DownloadFailed(
                format!("Failed to download mrpack: HTTP {}", response.status())
            ));
        }

        let content = response.bytes().await?;
        fs::write(&mrpack_path, content).await?;

        Ok(mrpack_path)
    }

    /// Analyze existing mods in an instance
    async fn analyze_existing_mods_simple(&self, instance_path: &Path) -> Result<HashMap<String, ModInfo>> {
        let mut existing_mods = HashMap::new();

        // Find mods directory based on launcher type
        let mods_dir = self.find_mods_directory(instance_path).await?;

        if mods_dir.exists() {
            let mut entries = fs::read_dir(&mods_dir).await?;
            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("jar") {
                    let filename = path.file_name().unwrap().to_string_lossy().to_string();
                    let normalized_name = self.normalize_mod_name(&filename);
                    let metadata = fs::metadata(&path).await?;
                    
                    let mod_info = ModInfo {
                        name: normalized_name.clone(),
                        filename: filename.clone(),
                        version: None,
                        is_user_mod: false, // We'll determine this later based on mrpack
                        file_size: metadata.len(),
                        last_modified: format!("{:?}", metadata.modified().ok()),
                        mod_id: None,
                    };
                    
                    existing_mods.insert(normalized_name, mod_info);
                }
            }
        }

        Ok(existing_mods)
    }

    /// Find the mods directory for an instance
    async fn find_mods_directory(&self, instance_path: &Path) -> Result<PathBuf> {
        // Try different possible locations
        let possible_paths = [
            instance_path.join("mods"),
            instance_path.join(".minecraft").join("mods"),
            instance_path.join("minecraft").join("mods"),
        ];

        for path in &possible_paths {
            if path.exists() {
                return Ok(path.clone());
            }
        }

        // Default to instance_path/mods
        Ok(instance_path.join("mods"))
    }

    /// Update mods intelligently
    async fn update_mods_intelligently(
        &self,
        instance_path: &Path,
        mrpack_index: &MrpackIndex,
        existing_mods: &HashMap<String, ModInfo>,
        modpack_info: &NahaModpackInfo,
    ) -> Result<UpdateResult> {
        let mut updated_mods = Vec::new();
        let mut new_mods = Vec::new();
        let mut preserved_mods = Vec::new();
        let mut errors = Vec::new();

        let mods_dir = self.find_mods_directory(instance_path).await?;
        fs::create_dir_all(&mods_dir).await?;

        // Build a set of modpack mod names from the mrpack
        let mut modpack_mod_names = std::collections::HashSet::new();
        for mrpack_file in &mrpack_index.files {
            if mrpack_file.path.starts_with("mods/") {
                let filename = Path::new(&mrpack_file.path).file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let normalized = self.normalize_mod_name(&filename);
                modpack_mod_names.insert(normalized);
            }
        }

        // Process each file in the mrpack
        for mrpack_file in &mrpack_index.files {
            if !mrpack_file.path.starts_with("mods/") {
                continue; // Skip non-mod files
            }

            let mod_filename = Path::new(&mrpack_file.path).file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            let mod_name = self.normalize_mod_name(&mod_filename);
            let target_path = mods_dir.join(&mod_filename);

            // Check if this mod already exists
            if let Some(existing_mod) = existing_mods.get(&mod_name) {
                // Check if the filename is exactly the same (already up to date)
                if existing_mod.filename == mod_filename {
                    // Same file, no update needed - skip it completely
                    continue;
                }

                // This is a modpack mod with a different version, update it
                // Remove the old version first
                let old_path = mods_dir.join(&existing_mod.filename);
                if old_path.exists() {
                    let _ = fs::remove_file(&old_path).await;
                }
                
                // Download the new version
                match self.download_mod_file(&mrpack_file, &target_path).await {
                    Ok(_) => {
                        println!("🔄 Updated: {} → {}", existing_mod.filename, mod_filename);
                        updated_mods.push(format!("{} → {}", existing_mod.filename, mod_filename));
                        info!("Updated mod: {}", mod_filename);
                    }
                    Err(e) => {
                        errors.push(format!("Failed to update {}: {}", mod_filename, e));
                    }
                }
            } else {
                // New mod, download it
                match self.download_mod_file(&mrpack_file, &target_path).await {
                    Ok(_) => {
                        println!("➕ Added: {}", mod_filename);
                        new_mods.push(mod_filename.clone());
                        info!("Added new mod: {}", mod_filename);
                    }
                    Err(e) => {
                        errors.push(format!("Failed to add {}: {}", mod_filename, e));
                    }
                }
            }
        }
        
        // Check for user mods (mods not in the mrpack)
        for (mod_name, mod_info) in existing_mods {
            if !modpack_mod_names.contains(mod_name) {
                preserved_mods.push(mod_info.filename.clone());
            }
        }

        // Clean up duplicate mods
        println!("🧹 Cleaning up duplicate mods...");
        if let Err(e) = self.cleanup_duplicate_mods(&mods_dir).await {
            errors.push(format!("Failed to cleanup duplicates: {}", e));
        }

        // Update automodpack configuration
        if let Err(e) = self.update_automodpack_config(instance_path, modpack_info).await {
            errors.push(format!("Failed to update automodpack config: {}", e));
        }

        // Update launcher database if this is an AstralRinth/ModrinthApp instance
        println!("🔄 Updating launcher database...");
        if let Err(e) = self.update_launcher_database(instance_path, modpack_info).await {
            println!("⚠️  Warning: Failed to update launcher database: {}", e);
            // Don't add to errors - this is non-critical
        }

        let success = errors.is_empty();
        let message = if success {
            format!("Successfully updated {} mods, added {} new mods, preserved {} user mods",
                   updated_mods.len(), new_mods.len(), preserved_mods.len())
        } else {
            format!("Update completed with {} errors", errors.len())
        };

        Ok(UpdateResult {
            instance_name: instance_path.file_name().unwrap().to_string_lossy().to_string(),
            success,
            updated_mods,
            new_mods,
            preserved_mods,
            errors,
            message,
        })
    }

    /// Extract mod name from filename
    fn extract_mod_name_from_filename(&self, filename: &str) -> String {
        // Remove .jar extension and .disabled
        let name = filename.replace(".jar", "").replace(".disabled", "");
        
        // Handle complex mod names with $ separators
        if name.contains('$') {
            // For names like "reinforced-barrels-2.6.1+1.21.1$reinforced-core-4.0.2+1.21.1$cloth-config-fabric-15.0.130-fabric"
            // Extract just the first part before the first $
            return name.split('$').next().unwrap_or(&name).to_string();
        }
        
        // Handle special cases where mod names have different patterns
        let name_lower = name.to_lowercase();
        
        // Handle kotlinforforge (special case)
        if name_lower.starts_with("kotlinforforge") {
            return "kotlinforforge".to_string();
        }
        
        // Handle yet_another_config_lib (special case)
        if name_lower.starts_with("yet_another_config_lib") {
            return "yet_another_config_lib".to_string();
        }
        
        // Handle mods with version patterns like "modname-version" or "modname+version"
        // Try to extract just the mod name part
        let parts: Vec<&str> = name.split('-').collect();
        if parts.len() >= 2 {
            // Check if the second part looks like a version (contains numbers)
            let second_part = parts[1];
            if second_part.chars().any(|c| c.is_ascii_digit()) {
                // This looks like "modname-version", so just return the mod name
                return parts[0].to_string();
            } else {
                // This might be "modname-loader", so return both
                return format!("{}-{}", parts[0], parts[1]);
            }
        }
        
        // Fallback to just the first part
        parts[0].to_string()
    }

    /// Determine if a mod should be updated
    fn should_update_mod(&self, existing_mod: &ModInfo, _mrpack_file: &MrpackFile) -> bool {
        // For now, always update modpack mods
        // In a more sophisticated implementation, you'd compare versions
        !existing_mod.is_user_mod
    }

    /// Download a mod file from the mrpack
    async fn download_mod_file(&self, mrpack_file: &MrpackFile, target_path: &Path) -> Result<()> {
        if mrpack_file.downloads.is_empty() {
            return Err(MinecraftInstallerError::DownloadFailed(
                "No download URLs available for mod".to_string()
            ));
        }

        let client = reqwest::Client::new();
        let response = client.get(&mrpack_file.downloads[0]).send().await?;

        if !response.status().is_success() {
            return Err(MinecraftInstallerError::DownloadFailed(
                format!("HTTP {} for mod download", response.status())
            ));
        }

        let content = response.bytes().await?;
        fs::write(target_path, content).await?;

        Ok(())
    }

    /// Update launcher database for AstralRinth/ModrinthApp
    async fn update_launcher_database(&self, instance_path: &Path, modpack_info: &NahaModpackInfo) -> Result<()> {
        use rusqlite::Connection;
        
        // Check if this is an AstralRinth or ModrinthApp instance
        let instance_path_str = instance_path.to_string_lossy();
        let launcher_path = if instance_path_str.contains("AstralRinthApp") {
            Some(instance_path.parent().and_then(|p| p.parent()))
        } else if instance_path_str.contains("ModrinthApp") {
            Some(instance_path.parent().and_then(|p| p.parent()))
        } else {
            None
        };

        if let Some(Some(launcher_root)) = launcher_path {
            let db_path = launcher_root.join("app.db");
            
            if !db_path.exists() {
                return Ok(()); // No database to update
            }

            println!("💾 Updating database: {}", db_path.display());

            let conn = Connection::open(&db_path)
                .map_err(|e| MinecraftInstallerError::InstallationFailed(
                    format!("Failed to open database: {}", e)
                ))?;

            let profile_name = instance_path.file_name().unwrap().to_string_lossy().to_string();
            let now = chrono::Utc::now().timestamp_millis();

            // Update the modified timestamp and version info for the profile
            match conn.execute(
                "UPDATE profiles SET modified = ?, game_version = ? WHERE path = ?",
                rusqlite::params![now, modpack_info.version, profile_name]
            ) {
                Ok(rows) if rows > 0 => {
                    println!("✅ Updated database entry for profile: {}", profile_name);
                }
                Ok(_) => {
                    // Profile doesn't exist, try to create it
                    match conn.execute(
                        "INSERT OR REPLACE INTO profiles (path, name, game_version, mod_loader, install_stage, created, modified, groups, override_extra_launch_args, override_custom_env_vars) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                        rusqlite::params![
                            profile_name.clone(),
                            profile_name.clone(),
                            modpack_info.version.clone(),
                            modpack_info.server_type.clone(),
                            "installed",
                            now,
                            now,
                            "[]",
                            "[]",
                            "{}"
                        ]
                    ) {
                        Ok(_) => println!("✅ Created new database entry for profile: {}", profile_name),
                        Err(e) => println!("⚠️  Could not create database entry: {}", e),
                    }
                }
                Err(e) => {
                    println!("⚠️  Could not update database: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Update automodpack configuration
    async fn update_automodpack_config(&self, instance_path: &Path, modpack_info: &NahaModpackInfo) -> Result<()> {
        // Update automodpack-known-hosts.json
        let known_hosts_path = instance_path.join("automodpack-known-hosts.json");
          let hosts_data = serde_json::json!({
            "hosts": {
                modpack_info.server_ip.clone(): modpack_info.fingerprint.clone()
            }
        });
        fs::write(&known_hosts_path, serde_json::to_string_pretty(&hosts_data)?).await?;

        // Update servers.dat if it exists
        let servers_dat_path = instance_path.join("servers.dat");
        if servers_dat_path.exists() {
            // This would require NBT parsing/writing - simplified for now
            debug!("servers.dat update not implemented (requires NBT library)");
        }

        Ok(())
    }
}
