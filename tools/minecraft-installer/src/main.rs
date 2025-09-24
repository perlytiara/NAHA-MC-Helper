use clap::Parser;
use std::path::PathBuf;
use tracing::{error, info, warn};
use tokio::fs;

mod installer;
mod error;
mod directories;
mod download;
mod java;
mod launcher_support;

use crate::error::Result;
use crate::installer::MinecraftInstaller;
use crate::launcher_support::LauncherManager;

#[derive(Parser)]
#[command(name = "minecraft-installer")]
#[command(about = "A standalone Minecraft installer")]
#[command(version = "0.1.0")]
struct Args {
    /// Minecraft version to install (e.g., "1.20.1", "1.19.4")
    #[arg(short, long, required_unless_present_any = ["list_versions", "mrpack", "list_launchers", "download_neoforge", "download_fabric"])]
    version: Option<String>,

    /// Installation directory (defaults to system's games directory)
    #[arg(short, long)]
    install_dir: Option<PathBuf>,

    /// Mod loader to install (vanilla, forge, fabric, quilt, neoforge)
    #[arg(short, long, default_value = "vanilla")]
    loader: String,

    /// Loader version (latest, stable, or specific version)
    #[arg(long, default_value = "stable")]
    loader_version: String,

    /// Force reinstall even if already installed
    #[arg(short, long)]
    force: bool,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// List available Minecraft versions
    #[arg(long)]
    list_versions: bool,

    /// Filter versions by type (release, snapshot, alpha, beta)
    #[arg(long)]
    version_type: Option<String>,

    /// Install mrpack file (Modrinth modpack)
    #[arg(long)]
    mrpack: Option<PathBuf>,

    /// Target launcher for instance creation (auto-detect if not specified)
    #[arg(long)]
    target_launcher: Option<String>,

    /// Create instance in detected launchers
    #[arg(long)]
    create_instance: bool,

            /// List detected launchers
            #[arg(long)]
            list_launchers: bool,

            /// Download and install NeoForge modpack from NAHA API
            #[arg(long)]
            download_neoforge: bool,

    /// Download and install Fabric modpack from NAHA API
    #[arg(long)]
    download_fabric: bool,

    /// Custom installation path for Other launcher type
    #[arg(long)]
    custom_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    let log_level = if args.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(format!("minecraft_installer={}", log_level))
        .init();

    info!("Minecraft Installer v0.1.0");
    if let Some(ref version) = args.version {
        info!("Installing Minecraft {} with {} loader", version, args.loader);
    }

    // Determine installation directory
    let install_dir = args.install_dir.unwrap_or_else(|| {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("MinecraftInstaller")
    });

    info!("Installation directory: {}", install_dir.display());

    // Create installer instance
    let installer = MinecraftInstaller::new(install_dir).await?;
    let launcher_manager = LauncherManager::new();

    // Handle list launchers command
    if args.list_launchers {
        match launcher_manager.detect_launchers().await.is_empty() {
            false => {
                println!("\n🚀 Detected Launchers");
                println!("════════════════════");
                for (launcher_type, path) in launcher_manager.detect_launchers().await {
                    println!("{:15} {}", format!("{:?}", launcher_type), path.display());
                }
            }
            true => {
                println!("No compatible launchers detected.");
                println!("Supported launchers:");
                println!("  - Official Minecraft Launcher");
                println!("  - PrismLauncher");
                println!("  - PrismLauncher-Cracked");
                println!("  - XMCL (X Minecraft Launcher)");
                println!("  - AstralRinth App");
                println!("  - Modrinth App");
                println!("  - MultiMC");
                println!("  - Other (Custom Path)");
            }
        }
        return Ok(());
    }

    // Handle list versions command
    if args.list_versions {
        match installer.list_versions(args.version_type.as_deref()).await {
            Ok(_) => {}
            Err(e) => {
                error!("✗ Failed to list versions: {}", e);
                std::process::exit(1);
            }
        }
        return Ok(());
    }

    // Handle API download commands
    if args.download_neoforge {
        info!("Downloading NeoForge modpack from NAHA API...");
        match launcher_manager.download_and_install_from_api(
            "neoforge",
            args.target_launcher.as_deref(),
            args.create_instance,
            args.custom_path.as_deref(),
        ).await {
            Ok(_) => {
                info!("✓ NeoForge modpack downloaded and installed successfully!");
            }
            Err(e) => {
                error!("✗ NeoForge modpack download failed: {}", e);
                std::process::exit(1);
            }
        }
        return Ok(());
    }

    if args.download_fabric {
        info!("Downloading Fabric modpack from NAHA API...");
        match launcher_manager.download_and_install_from_api(
            "fabric",
            args.target_launcher.as_deref(),
            args.create_instance,
            args.custom_path.as_deref(),
        ).await {
            Ok(_) => {
                info!("✓ Fabric modpack downloaded and installed successfully!");
            }
            Err(e) => {
                error!("✗ Fabric modpack download failed: {}", e);
                std::process::exit(1);
            }
        }
        return Ok(());
    }

    // Handle mrpack installation
    if let Some(mrpack_path) = args.mrpack {
        info!("Installing mrpack: {}", mrpack_path.display());

        // Create temporary instance directory
        let temp_instance_dir = installer.get_install_dir().join("temp-mrpack-instance");

        match launcher_manager.install_mrpack(&mrpack_path, &temp_instance_dir, "temp-instance").await {
            Ok((minecraft_version, mod_loader)) => {
                info!("✓ Mrpack installed successfully!");

                // If create_instance is specified, also create launcher instances
                if args.create_instance {
                    // Generate proper instance name based on mod loader
                    let instance_name = match mod_loader.as_str() {
                        "neoforge" => "NAHA-NeoForge".to_string(),
                        "fabric" => "NAHA-Fabric".to_string(),
                        "forge" => "NAHA-Forge".to_string(),
                        "quilt" => "NAHA-Quilt".to_string(),
                        _ => format!("NAHA-{}", mod_loader),
                    };

                    // Handle custom path for Other launcher
                    let target_launcher = if args.target_launcher.as_deref() == Some("other") && args.custom_path.is_some() {
                        Some("other")
                    } else {
                        args.target_launcher.as_deref()
                    };

                    match launcher_manager.auto_install_instance(
                        &instance_name,
                        &minecraft_version,
                        &mod_loader,
                        None, // Let the launcher support determine the appropriate version
                        target_launcher,
                        args.custom_path.as_deref()
                    ).await {
                        Ok(instance_path) => {
                            info!("✓ Instance created at: {}", instance_path.display());

                            // Copy files from temp instance to launcher instance
                            if let Err(e) = launcher_manager.copy_instance_files(&temp_instance_dir, &instance_path).await {
                                warn!("Failed to copy files to launcher instance: {}", e);
                            } else {
                                info!("✓ Files copied to launcher instance");

                                // Clean up temporary directory
                                if let Err(e) = tokio::fs::remove_dir_all(&temp_instance_dir).await {
                                    warn!("Failed to clean up temporary directory: {}", e);
                                } else {
                                    info!("✓ Temporary directory cleaned up");
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to create launcher instance: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                error!("✗ Mrpack installation failed: {}", e);
                std::process::exit(1);
            }
        }
        return Ok(());
    }

    // Install Minecraft
    if let Some(version) = args.version {
        match installer.install_minecraft(
            &version,
            &args.loader,
            &args.loader_version,
            args.force,
        ).await {
            Ok(_) => {
                info!("✓ Minecraft {} successfully installed!", version);

                // Create instance in detected launchers if requested
                if args.create_instance {
                    let instance_name = format!("Minecraft {}", version);
                    match launcher_manager.auto_install_instance(
                        &instance_name,
                        &version,
                        &args.loader,
                        Some(&args.loader_version),
                        args.target_launcher.as_deref(),
                        args.custom_path.as_deref()
                    ).await {
                        Ok(instance_path) => {
                            info!("✓ Instance '{}' created at: {}", instance_name, instance_path.display());
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
                error!("✗ Installation failed: {}", e);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

