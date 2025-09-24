use std::collections::HashMap;
use std::path::Path;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha1_smol::{Sha1, Digest};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tracing::{debug, info, warn};
use indicatif::{ProgressBar, ProgressStyle};

use crate::error::{MinecraftInstallerError, Result};
use crate::directories::DirectoryManager;

/// Minecraft version manifest from Mojang
#[derive(Deserialize, Debug)]
pub struct VersionManifest {
    pub latest: Latest,
    pub versions: Vec<VersionInfo>,
}

#[derive(Deserialize, Debug)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Deserialize, Debug)]
pub struct VersionInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
}

/// Detailed version information
#[derive(Deserialize, Serialize, Debug)]
pub struct VersionDetails {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub downloads: Downloads,
    pub libraries: Vec<Library>,
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndex,
    pub assets: String,
    #[serde(rename = "javaVersion")]
    pub java_version: Option<JavaVersion>,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    #[serde(rename = "minecraftArguments", default)]
    pub minecraft_arguments: Option<String>,
    #[serde(default)]
    pub arguments: Option<Arguments>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Downloads {
    pub client: Download,
    pub server: Option<Download>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Download {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Library {
    pub name: String,
    pub downloads: LibraryDownloads,
    pub rules: Option<Vec<Rule>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LibraryDownloads {
    pub artifact: Option<Artifact>,
    pub classifiers: Option<HashMap<String, Artifact>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Artifact {
    pub path: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rule {
    pub action: String,
    pub os: Option<OsRule>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OsRule {
    pub name: Option<String>,
    pub arch: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
    #[serde(rename = "totalSize")]
    pub total_size: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct JavaVersion {
    pub component: String,
    #[serde(rename = "majorVersion")]
    pub major_version: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Arguments {
    pub game: Option<Vec<serde_json::Value>>,
    pub jvm: Option<Vec<serde_json::Value>>,
}

#[derive(Deserialize, Debug)]
pub struct AssetIndexData {
    pub objects: HashMap<String, AssetObject>,
}

#[derive(Deserialize, Debug)]
pub struct AssetObject {
    pub hash: String,
    pub size: u64,
}

/// Download manager for Minecraft files
pub struct DownloadManager {
    client: Client,
    dirs: DirectoryManager,
}

impl DownloadManager {
    pub fn new(dirs: DirectoryManager) -> Self {
        let client = Client::builder()
            .user_agent("MinecraftInstaller/0.1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self { client, dirs }
    }

    /// Get the version manifest from Mojang
    pub async fn get_version_manifest(&self) -> Result<VersionManifest> {
        info!("Fetching Minecraft version manifest...");
        let url = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

        let response = self.client.get(url).send().await?;
        if !response.status().is_success() {
            return Err(MinecraftInstallerError::Network(format!(
                "Failed to fetch version manifest: HTTP {}",
                response.status()
            )));
        }

        let manifest: VersionManifest = response.json().await?;
        debug!("Found {} versions in manifest", manifest.versions.len());
        Ok(manifest)
    }

    /// Get detailed version information
    pub async fn get_version_details(&self, version_info: &VersionInfo) -> Result<VersionDetails> {
        info!("Fetching details for Minecraft {}...", version_info.id);

        let response = self.client.get(&version_info.url).send().await?;
        if !response.status().is_success() {
            return Err(MinecraftInstallerError::Network(format!(
                "Failed to fetch version details: HTTP {}",
                response.status()
            )));
        }

        let details: VersionDetails = response.json().await?;
        Ok(details)
    }

    /// Download a file with progress tracking
    async fn download_file_with_progress(
        &self,
        url: &str,
        path: &Path,
        expected_sha1: Option<&str>,
        progress_bar: Option<&ProgressBar>,
    ) -> Result<()> {
        // Check if file already exists and is valid
        if let Some(sha1) = expected_sha1 {
            if path.exists() {
                if let Ok(existing_hash) = self.calculate_sha1(path).await {
                    if existing_hash == sha1 {
                        debug!("File {} already exists with correct hash", path.display());
                        return Ok(());
                    }
                }
            }
        }

        debug!("Downloading {} to {}", url, path.display());

        // Create parent directories
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Download the file
        let response = self.client.get(url).send().await?;
        if !response.status().is_success() {
            return Err(MinecraftInstallerError::DownloadFailed(format!(
                "HTTP {} for {}",
                response.status(),
                url
            )));
        }

        let total_size = response.content_length().unwrap_or(0);
        if let Some(pb) = progress_bar {
            pb.set_length(total_size);
        }

        let mut file = fs::File::create(path).await?;
        let mut downloaded = 0u64;
        let mut stream = response.bytes_stream();

        use futures::StreamExt;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;

            if let Some(pb) = progress_bar {
                pb.set_position(downloaded);
            }
        }

        file.sync_all().await?;

        // Verify SHA1 if provided
        if let Some(expected_sha1) = expected_sha1 {
            let actual_sha1 = self.calculate_sha1(path).await?;
            if actual_sha1 != expected_sha1 {
                return Err(MinecraftInstallerError::Validation(format!(
                    "SHA1 mismatch for {}: expected {}, got {}",
                    path.display(),
                    expected_sha1,
                    actual_sha1
                )));
            }
        }

        Ok(())
    }

    /// Calculate SHA1 hash of a file
    async fn calculate_sha1(&self, path: &Path) -> Result<String> {
        let data = fs::read(path).await?;
        let mut hasher = Sha1::new();
        hasher.update(&data);
        Ok(hex::encode(hasher.digest().bytes()))
    }

    /// Download the Minecraft client
    pub async fn download_client(&self, version_details: &VersionDetails) -> Result<()> {
        info!("Downloading Minecraft client {}...", version_details.id);

        let client_download = &version_details.downloads.client;
        let jar_path = self.dirs.version_jar(&version_details.id);

        let progress_bar = ProgressBar::new(client_download.size);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .expect("Invalid progress bar template")
                .progress_chars("#>-"),
        );
        progress_bar.set_message(format!("Client {}", version_details.id));

        self.download_file_with_progress(
            &client_download.url,
            &jar_path,
            Some(&client_download.sha1),
            Some(&progress_bar),
        ).await?;

        progress_bar.finish_with_message(format!("✓ Client {} downloaded", version_details.id));

        // Save version JSON
        let version_json_path = self.dirs.version_json(&version_details.id);
        let version_json = serde_json::to_string_pretty(version_details)?;
        fs::write(version_json_path, version_json).await?;

        Ok(())
    }

    /// Download libraries
    pub async fn download_libraries(&self, version_details: &VersionDetails) -> Result<()> {
        info!("Downloading libraries for {}...", version_details.id);

        let mut valid_libraries = Vec::new();
        for library in &version_details.libraries {
            if self.should_include_library(library) {
                valid_libraries.push(library);
            }
        }

        if valid_libraries.is_empty() {
            info!("No libraries to download");
            return Ok(());
        }

        let progress_bar = ProgressBar::new(valid_libraries.len() as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} libraries")
                .expect("Invalid progress bar template")
                .progress_chars("#>-"),
        );
        progress_bar.set_message("Libraries");

        for library in valid_libraries {
            if let Some(artifact) = &library.downloads.artifact {
                let lib_path = self.dirs.libraries_dir().join(&artifact.path);

                match self.download_file_with_progress(
                    &artifact.url,
                    &lib_path,
                    Some(&artifact.sha1),
                    None,
                ).await {
                    Ok(_) => debug!("Downloaded library: {}", library.name),
                    Err(e) => warn!("Failed to download library {}: {}", library.name, e),
                }
            }

            // Download natives if present
            if let Some(classifiers) = &library.downloads.classifiers {
                let os_name = self.get_os_name();
                if let Some(native) = classifiers.get(&format!("natives-{}", os_name)) {
                    let natives_dir = self.dirs.natives_dir(&version_details.id);
                    let native_path = natives_dir.join(format!("{}.jar", library.name.replace(':', "_")));

                    match self.download_file_with_progress(
                        &native.url,
                        &native_path,
                        Some(&native.sha1),
                        None,
                    ).await {
                        Ok(_) => {
                            // Extract native library
                            if let Err(e) = self.extract_native(&native_path, &natives_dir).await {
                                warn!("Failed to extract native {}: {}", library.name, e);
                            }
                        }
                        Err(e) => warn!("Failed to download native {}: {}", library.name, e),
                    }
                }
            }

            progress_bar.inc(1);
        }

        progress_bar.finish_with_message("✓ Libraries downloaded");
        Ok(())
    }

    /// Download assets
    pub async fn download_assets(&self, version_details: &VersionDetails) -> Result<()> {
        info!("Downloading assets for {}...", version_details.id);

        // Download asset index
        let asset_index = &version_details.asset_index;
        let index_path = self.dirs.assets_index_dir().join(format!("{}.json", asset_index.id));

        self.download_file_with_progress(
            &asset_index.url,
            &index_path,
            Some(&asset_index.sha1),
            None,
        ).await?;

        // Parse asset index
        let index_data: AssetIndexData = serde_json::from_slice(&fs::read(index_path).await?)?;

        if index_data.objects.is_empty() {
            info!("No assets to download");
            return Ok(());
        }

        let progress_bar = ProgressBar::new(index_data.objects.len() as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} assets")
                .expect("Invalid progress bar template")
                .progress_chars("#>-"),
        );
        progress_bar.set_message("Assets");

        // Download assets
        for (_name, asset) in index_data.objects {
            let asset_path = self.dirs.asset_object_path(&asset.hash);
            let asset_url = format!(
                "https://resources.download.minecraft.net/{}/{}",
                &asset.hash[..2],
                asset.hash
            );

            match self.download_file_with_progress(
                &asset_url,
                &asset_path,
                Some(&asset.hash),
                None,
            ).await {
                Ok(_) => {}
                Err(e) => warn!("Failed to download asset {}: {}", asset.hash, e),
            }

            progress_bar.inc(1);
        }

        progress_bar.finish_with_message("✓ Assets downloaded");
        Ok(())
    }

    /// Check if a library should be included based on rules
    fn should_include_library(&self, library: &Library) -> bool {
        if let Some(rules) = &library.rules {
            for rule in rules {
                match rule.action.as_str() {
                    "allow" => {
                        if let Some(os) = &rule.os {
                            if let Some(name) = &os.name {
                                if name != self.get_os_name() {
                                    continue;
                                }
                            }
                        }
                        return true;
                    }
                    "disallow" => {
                        if let Some(os) = &rule.os {
                            if let Some(name) = &os.name {
                                if name == self.get_os_name() {
                                    return false;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            false
        } else {
            true
        }
    }

    /// Get the current OS name for library rules
    fn get_os_name(&self) -> &'static str {
        if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "macos") {
            "osx"
        } else {
            "linux"
        }
    }

    /// Extract a native library JAR
    async fn extract_native(&self, jar_path: &Path, extract_dir: &Path) -> Result<()> {
        use std::io::Read;

        let file = std::fs::File::open(jar_path)?;
        let mut archive = zip::ZipArchive::new(file)?;

        fs::create_dir_all(extract_dir).await?;

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
            }
        }

        // Remove the JAR file after extraction
        fs::remove_file(jar_path).await?;
        Ok(())
    }
}
