use std::path::PathBuf;
use clap::{Parser, Subcommand};
use serde_json;
use tracing::{info, error};
use minecraft_installer::updater::{MinecraftUpdater, InstanceInfo, UpdateResult};

#[derive(Parser)]
#[command(name = "minecraft-updater")]
#[command(about = "Minecraft instance updater for Electron apps")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan all instances and return JSON for Electron app
    Scan {
        /// Output format (json, pretty, compact)
        #[arg(short, long, default_value = "compact")]
        format: String,
    },
    /// Update mods for a specific instance
    Update {
        /// Path to the instance directory
        #[arg(short, long)]
        instance_path: PathBuf,
        /// Modpack type (neoforge, fabric)
        #[arg(short, long)]
        modpack_type: String,
        /// Specific version to download (e.g., "0.0.18", default: latest)
        #[arg(short, long)]
        version: Option<String>,
        /// Output format (json, pretty)
        #[arg(short, long, default_value = "json")]
        format: String,
    },
    /// Interactive instance selection and update
    Interactive {
        /// Modpack type (neoforge, fabric)
        #[arg(short, long)]
        modpack_type: String,
        /// Specific launcher to use (optional)
        #[arg(long)]
        launcher: Option<String>,
    },
    /// Update all instances of a specific modpack type
    UpdateAll {
        /// Modpack type (neoforge, fabric)
        #[arg(short, long)]
        modpack_type: String,
        /// Output format (json, pretty)
        #[arg(short, long, default_value = "json")]
        format: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let updater = MinecraftUpdater::new();

    match cli.command {
        Commands::Scan { format } => {
            match updater.scan_instances().await {
                Ok(instances) => {
                    match format.as_str() {
                        "json" => {
                            println!("{}", serde_json::to_string_pretty(&instances)?);
                        }
                        "pretty" => {
                            print_instances_pretty(&instances);
                        }
                        "compact" => {
                            print_instances_compact(&instances);
                        }
                        _ => {
                            eprintln!("Invalid format: {}. Use 'json', 'pretty', or 'compact'", format);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to scan instances: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Interactive { modpack_type, launcher } => {
            match updater.scan_instances().await {
                Ok(instances) => {
                    let mut filtered_instances: Vec<_> = instances.iter()
                        .filter(|instance| should_update_instance(instance, &modpack_type))
                        .collect();
                    
                    // Filter by launcher if specified
                    if let Some(target_launcher) = &launcher {
                        filtered_instances.retain(|instance| instance.launcher_type == *target_launcher);
                    }
                    
                    if filtered_instances.is_empty() {
                        println!("❌ No {} instances found to update", modpack_type);
                        return Ok(());
                    }
                    
                    // Group instances by launcher
                    let mut by_launcher: std::collections::HashMap<String, Vec<&minecraft_installer::updater::InstanceInfo>> = std::collections::HashMap::new();
                    for instance in &filtered_instances {
                        by_launcher.entry(instance.launcher_type.clone()).or_insert_with(Vec::new).push(instance);
                    }
                    
                    println!("🎮 Select a launcher:");
                    println!("═══════════════════════════════════════");
                    
                    let launcher_names: Vec<String> = by_launcher.keys().cloned().collect();
                    for (i, launcher_name) in launcher_names.iter().enumerate() {
                        let count = by_launcher[launcher_name].len();
                        let icon = match launcher_name.as_str() {
                            "AstralRinth" => "📱",
                            "ModrinthApp" => "📱", 
                            "XMCL" => "📁",
                            "PrismLauncher" => "📁",
                            "Official Minecraft" => "📁",
                            _ => "📦",
                        };
                        println!("{}. {} {} ({})", i + 1, icon, launcher_name, count);
                    }
                    
                    println!("\nEnter launcher number (1-{}): ", launcher_names.len());
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    
                    if let Ok(choice) = input.trim().parse::<usize>() {
                        if choice > 0 && choice <= launcher_names.len() {
                            let selected_launcher = &launcher_names[choice - 1];
                            let launcher_instances = &by_launcher[selected_launcher];
                            
                            println!("\n🎮 Select an instance from {}:", selected_launcher);
                            println!("═══════════════════════════════════════");
                            
                            for (i, instance) in launcher_instances.iter().enumerate() {
                                println!("{}. {} - {}", 
                                    i + 1, 
                                    instance.name,
                                    instance.instance_path
                                );
                            }
                            
                            println!("\nEnter instance number (1-{}): ", launcher_instances.len());
                            let mut input2 = String::new();
                            std::io::stdin().read_line(&mut input2)?;
                            
                            if let Ok(choice2) = input2.trim().parse::<usize>() {
                                if choice2 > 0 && choice2 <= launcher_instances.len() {
                                    let selected_instance = launcher_instances[choice2 - 1];
                                    println!("\n🔄 Updating {}...", selected_instance.name);
                                    
                                    match updater.update_instance_mods(
                                        &PathBuf::from(&selected_instance.instance_path),
                                        &modpack_type
                                    ).await {
                                        Ok(result) => {
                                            print_update_result_pretty(&result);
                                        }
                                        Err(e) => {
                                            error!("Failed to update instance: {}", e);
                                        }
                                    }
                                } else {
                                    println!("❌ Invalid instance selection");
                                }
                            } else {
                                println!("❌ Invalid instance input");
                            }
                        } else {
                            println!("❌ Invalid launcher selection");
                        }
                    } else {
                        println!("❌ Invalid launcher input");
                    }
                }
                Err(e) => {
                    error!("Failed to scan instances: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Update { instance_path, modpack_type, version, format } => {
            match updater.update_instance_mods_version(&instance_path, &modpack_type, version.as_deref()).await {
                Ok(result) => {
                    match format.as_str() {
                        "json" => {
                            println!("{}", serde_json::to_string_pretty(&result)?);
                        }
                        "pretty" => {
                            print_update_result_pretty(&result);
                        }
                        _ => {
                            eprintln!("Invalid format: {}. Use 'json' or 'pretty'", format);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to update instance: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::UpdateAll { modpack_type, format } => {
            match updater.scan_instances().await {
                Ok(instances) => {
                    let mut results = Vec::new();

                    for instance in instances {
                        // Only update instances that match the modpack type
                        if should_update_instance(&instance, &modpack_type) {
                            match updater.update_instance_mods(
                                &PathBuf::from(&instance.instance_path),
                                &modpack_type
                            ).await {
                                Ok(result) => results.push(result),
                                Err(e) => {
                                    error!("Failed to update instance {}: {}", instance.name, e);
                                }
                            }
                        }
                    }

                    match format.as_str() {
                        "json" => {
                            println!("{}", serde_json::to_string_pretty(&results)?);
                        }
                        "pretty" => {
                            print_update_results_pretty(&results);
                        }
                        _ => {
                            eprintln!("Invalid format: {}. Use 'json' or 'pretty'", format);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to scan instances: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}

/// Determine if an instance should be updated based on modpack type
fn should_update_instance(instance: &InstanceInfo, modpack_type: &str) -> bool {
    match modpack_type {
        "neoforge" => {
            instance.mod_loader.to_lowercase().contains("neoforge") ||
            instance.mod_loader.to_lowercase().contains("forge")
        }
        "fabric" => {
            instance.mod_loader.to_lowercase().contains("fabric")
        }
        _ => false,
    }
}

/// Print instances in a compact format
fn print_instances_compact(instances: &[InstanceInfo]) {
    println!("🎮 Minecraft Instances");
    println!("═══════════════════════════════════════");
    
    // Group instances by launcher type
    let mut by_launcher: std::collections::HashMap<String, Vec<&InstanceInfo>> = std::collections::HashMap::new();
    for instance in instances {
        by_launcher.entry(instance.launcher_type.clone()).or_insert_with(Vec::new).push(instance);
    }

    // Sort launchers for consistent display
    let launcher_order = vec![
        ("AstralRinth", "📱"),
        ("ModrinthApp", "📱"), 
        ("XMCL", "📁"),
        ("PrismLauncher", "📁"),
        ("Official Minecraft", "📁"),
    ];
    
    // Print known launchers first
    for (launcher_name, icon) in &launcher_order {
        if let Some(launcher_instances) = by_launcher.get(*launcher_name) {
            println!("{} {} ({})", icon, launcher_name, launcher_instances.len());
            
            for instance in launcher_instances {
                let mod_loader_info = if let Some(version) = &instance.mod_loader_version {
                    format!("{} {}", instance.mod_loader, version)
                } else {
                    instance.mod_loader.clone()
                };
                
                println!("  • {} | {} | {} | {} mods", 
                    instance.name, 
                    instance.minecraft_version,
                    mod_loader_info,
                    instance.mod_count
                );
            }
            println!();
        }
    }
    
    // Print any remaining launchers
    for (launcher_type, launcher_instances) in &by_launcher {
        if !launcher_order.iter().any(|(name, _)| name == launcher_type) {
            println!("📦 {} ({})", launcher_type, launcher_instances.len());
            
            for instance in launcher_instances {
                let mod_loader_info = if let Some(version) = &instance.mod_loader_version {
                    format!("{} {}", instance.mod_loader, version)
                } else {
                    instance.mod_loader.clone()
                };
                
                println!("  • {} | {} | {} | {} mods", 
                    instance.name, 
                    instance.minecraft_version,
                    mod_loader_info,
                    instance.mod_count
                );
            }
            println!();
        }
    }
}

/// Print instances in a pretty format
fn print_instances_pretty(instances: &[InstanceInfo]) {
    println!("🎮 Minecraft Instances Found");
    println!("═══════════════════════════════════════");
    println!("Total instances: {}", instances.len());
    println!();

    // Group instances by launcher type
    let mut by_launcher: std::collections::HashMap<String, Vec<&InstanceInfo>> = std::collections::HashMap::new();
    for instance in instances {
        by_launcher.entry(instance.launcher_type.clone()).or_insert_with(Vec::new).push(instance);
    }

    // Print each launcher group
    for (launcher_type, launcher_instances) in by_launcher {
        let icon = match launcher_type.as_str() {
            "AstralRinth" => "📱",
            "ModrinthApp" => "📱", 
            "XMCL" => "📁",
            "PrismLauncher" => "📁",
            "Official Minecraft" => "📁",
            _ => "📦",
        };
        
        println!("{} {} ({})", icon, launcher_type, launcher_instances.len());
        println!("{}", "─".repeat(40));

        for instance in launcher_instances {
            println!("  📁 {}", instance.name);
            println!("     Path: {}", instance.instance_path);
            println!("     Minecraft: {}", instance.minecraft_version);
            println!("     Mod Loader: {}", instance.mod_loader);
            if let Some(version) = &instance.mod_loader_version {
                println!("     Loader Version: {}", version);
            }
            println!("     Mods: {} files", instance.mod_count);
            if instance.has_automodpack {
                println!("     🔗 Automodpack: Enabled");
                if let Some(server) = &instance.server_info {
                    println!("     🌐 Server: {}:{}", server.server_ip, server.server_port);
                }
            }
            println!();
        }
    }
}

/// Print update result in a pretty format
fn print_update_result_pretty(result: &UpdateResult) {
    println!("🔄 Update Result: {}", result.instance_name);
    println!("═══════════════════════════════════════");

    if result.success {
        println!("✅ Status: Success");
    } else {
        println!("❌ Status: Failed");
    }

    println!("📝 {}", result.message);

    if !result.updated_mods.is_empty() {
        println!("\n🔄 Updated Mods ({}):", result.updated_mods.len());
        for mod_name in &result.updated_mods {
            println!("   • {}", mod_name);
        }
    }

    if !result.new_mods.is_empty() {
        println!("\n➕ New Mods ({}):", result.new_mods.len());
        for mod_name in &result.new_mods {
            println!("   • {}", mod_name);
        }
    }

    if !result.preserved_mods.is_empty() {
        println!("\n🔒 Preserved User Mods ({}):", result.preserved_mods.len());
        for mod_name in &result.preserved_mods {
            println!("   • {}", mod_name);
        }
    }

    if !result.errors.is_empty() {
        println!("\n❌ Errors ({}):", result.errors.len());
        for error in &result.errors {
            println!("   • {}", error);
        }
    }

    println!();
}

/// Print multiple update results in a pretty format
fn print_update_results_pretty(results: &[UpdateResult]) {
    println!("🔄 Update Results Summary");
    println!("═══════════════════════════════════════");
    println!("Total instances updated: {}", results.len());

    let successful = results.iter().filter(|r| r.success).count();
    let failed = results.len() - successful;

    println!("✅ Successful: {}", successful);
    println!("❌ Failed: {}", failed);
    println!();

    for result in results {
        print_update_result_pretty(result);
    }
}
