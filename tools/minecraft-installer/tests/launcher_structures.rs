use std::path::PathBuf;
use tokio::fs;
use serde_json::json;

use minecraft_installer::launcher_support::{LauncherType, LauncherManager};
use minecraft_installer::directories::DirectoryManager;
use minecraft_installer::error::Result;

/// Test launcher directory structures and instance management
#[tokio::test]
async fn test_launcher_structures() -> Result<()> {
    let test_dir = PathBuf::from("test-launchers");

    // Clean up any existing test directory
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir).await?;
    }

    // Create test launcher structures
    create_test_launcher_structures(&test_dir).await?;

    // Test each launcher type
    test_official_minecraft_launcher(&test_dir).await?;
    test_prism_launcher(&test_dir).await?;
    test_prism_launcher_cracked(&test_dir).await?;
    test_xmcl_launcher(&test_dir).await?;
    test_astral_rinth_app(&test_dir).await?;

    // Clean up
    fs::remove_dir_all(&test_dir).await?;

    Ok(())
}

/// Create test launcher directory structures
async fn create_test_launcher_structures(base_dir: &PathBuf) -> Result<()> {
    // Official Minecraft Launcher
    create_official_minecraft_structure(base_dir).await?;

    // PrismLauncher
    create_prism_launcher_structure(base_dir).await?;

    // PrismLauncher-Cracked
    create_prism_launcher_cracked_structure(base_dir).await?;

    // XMCL
    create_xmcl_structure(base_dir).await?;

    // AstralRinth App
    create_astral_rinth_structure(base_dir).await?;

    Ok(())
}

/// Official Minecraft Launcher structure
async fn create_official_minecraft_structure(base_dir: &PathBuf) -> Result<()> {
    let minecraft_dir = base_dir.join(".minecraft");

    // Create directory structure
    fs::create_dir_all(&minecraft_dir).await?;
    fs::create_dir_all(minecraft_dir.join("versions")).await?;
    fs::create_dir_all(minecraft_dir.join("libraries")).await?;
    fs::create_dir_all(minecraft_dir.join("assets")).await?;
    fs::create_dir_all(minecraft_dir.join("saves")).await?;
    fs::create_dir_all(minecraft_dir.join("resourcepacks")).await?;
    fs::create_dir_all(minecraft_dir.join("shaderpacks")).await?;
    fs::create_dir_all(minecraft_dir.join("mods")).await?;

    // Create launcher_profiles.json
    let profiles = json!({
        "profiles": {
            "latest-release": {
                "created": "2023-01-01T00:00:00.000Z",
                "icon": "Grass_Block",
                "lastUsed": "2023-01-01T00:00:00.000Z",
                "lastVersionId": "1.20.1",
                "name": "Latest Release",
                "type": "latest-release"
            }
        },
        "settings": {
            "enableAdvanced": false,
            "enableAnalytics": true,
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

    fs::write(
        minecraft_dir.join("launcher_profiles.json"),
        serde_json::to_string_pretty(&profiles)?
    ).await?;

    // Create options.txt
    let options = r#"version:3120
autoJump:false
autoSuggestions:true
chatColors:true
chatLinks:true
chatLinksPrompt:true
enableVsync:true
entityShadows:true
forceUnicodeFont:false
discrete_mouse_scroll:false
invertYMouse:false
realmsNotifications:true
reducedDebugInfo:false
showSubtitles:false
touchscreen:false
fullscreen:false
bobView:true
toggleCrouch:false
toggleSprint:false
mouseSensitivity:0.5
fov:0.0
screenEffectScale:1.0
fovEffectScale:1.0
gamma:0.0
renderDistance:12
simulationDistance:12
entityDistanceScaling:1.0
guiScale:0
particles:0
maxFps:120
difficulty:2
graphicsMode:1
ao:2
prioritizeChunkUpdates:0
biomeBlendRadius:2
renderClouds:true
resourcePacks:[]
incompatibleResourcePacks:[]
lastServer:
lang:en_us
soundDevice:
chatVisibility:0
chatOpacity:1.0
chatLineSpacing:0.0
textBackgroundOpacity:0.5
backgroundForChatOnly:true
hideServerAddress:false
advancedItemTooltips:false
pauseOnLostFocus:true
overrideWidth:0
overrideHeight:0
heldItemTooltips:true
chatHeightFocused:1.0
chatDelay:0.0
chatHeightUnfocused:0.44366195797920227
chatScale:1.0
chatWidth:1.0
mipmapLevels:4
useNativeTransport:true
mainHand:right
attackIndicator:1
narrator:0
tutorialStep:none
mouseWheelSensitivity:1.0
rawMouseInput:true
glDebugVerbosity:1
skipMultiplayerWarning:false
skipRealms32bitWarning:false
hideMatchedNames:true
joinedFirstServer:false
hideBundleTutorial:false
syncChunkWrites:true
key_key.attack:key.mouse.left
key_key.use:key.mouse.right
key_key.forward:key.keyboard.w
key_key.left:key.keyboard.a
key_key.back:key.keyboard.s
key_key.right:key.keyboard.d
key_key.jump:key.keyboard.space
key_key.sneak:key.keyboard.left.shift
key_key.sprint:key.keyboard.left.control
key_key.drop:key.keyboard.q
key_key.inventory:key.keyboard.e
key_key.chat:key.keyboard.t
key_key.playerlist:key.keyboard.tab
key_key.pickItem:key.mouse.middle
key_key.command:key.keyboard.slash
key_key.socialInteractions:key.keyboard.p
key_key.screenshot:key.keyboard.f2
key_key.togglePerspective:key.keyboard.f5
key_key.smoothCamera:key.keyboard.unknown
key_key.fullscreen:key.keyboard.f11
key_key.spectatorOutlines:key.keyboard.unknown
key_key.swapOffhand:key.keyboard.f
key_key.saveToolbarActivator:key.keyboard.c
key_key.loadToolbarActivator:key.keyboard.x
key_key.advancements:key.keyboard.l
key_key.hotbar.1:key.keyboard.1
key_key.hotbar.2:key.keyboard.2
key_key.hotbar.3:key.keyboard.3
key_key.hotbar.4:key.keyboard.4
key_key.hotbar.5:key.keyboard.5
key_key.hotbar.6:key.keyboard.6
key_key.hotbar.7:key.keyboard.7
key_key.hotbar.8:key.keyboard.8
key_key.hotbar.9:key.keyboard.9
soundCategory_master:1.0
soundCategory_music:1.0
soundCategory_record:1.0
soundCategory_weather:1.0
soundCategory_block:1.0
soundCategory_hostile:1.0
soundCategory_neutral:1.0
soundCategory_player:1.0
soundCategory_ambient:1.0
soundCategory_voice:1.0
modelPart_cape:true
modelPart_jacket:true
modelPart_left_sleeve:true
modelPart_right_sleeve:true
modelPart_left_pants_leg:true
modelPart_right_pants_leg:true
modelPart_hat:true
"#;

    fs::write(minecraft_dir.join("options.txt"), options).await?;

    Ok(())
}

/// PrismLauncher structure
async fn create_prism_launcher_structure(base_dir: &PathBuf) -> Result<()> {
    let prism_dir = base_dir.join("PrismLauncher");

    // Create directory structure
    fs::create_dir_all(&prism_dir).await?;
    fs::create_dir_all(prism_dir.join("instances")).await?;
    fs::create_dir_all(prism_dir.join("libraries")).await?;
    fs::create_dir_all(prism_dir.join("assets")).await?;
    fs::create_dir_all(prism_dir.join("meta")).await?;
    fs::create_dir_all(prism_dir.join("translations")).await?;
    fs::create_dir_all(prism_dir.join("themes")).await?;

    // Create accounts.json
    let accounts = json!({
        "accounts": [],
        "formatVersion": 3
    });

    fs::write(
        prism_dir.join("accounts.json"),
        serde_json::to_string_pretty(&accounts)?
    ).await?;

    // Create prismlauncher.cfg
    let config = r#"Analytics=true
AutoCloseConsole=false
AutoUpdate=true
BaseMemMaxAlloc=2048
BaseMemMinAlloc=512
CentralModsDir=
ConsoleMaxLines=100000
ConsoleOverflowStop=true
IconTheme=pe_colored
InstanceDir=instances
JavaArchitecture=64
JavaPath=javaw
JavaTimestampCheck=true
JavaVersion=17.0.2
Language=en_US
LastUsedGroupForNewInstance=
LaunchMaximized=false
LogPrePostOutput=true
MaxMemAlloc=2048
MinMemAlloc=512
MinecraftWinHeight=480
MinecraftWinWidth=854
PostExitCommand=
PreLaunchCommand=
QuitAfterGameStop=false
ShowConsole=true
ShowConsoleOnError=true
ShowGameTime=true
ShowGlobalGameTime=true
UpdateDialogGeometry=
UseNativeGLFW=false
UseNativeOpenAL=false
WrapperCommand=
"#;

    fs::write(prism_dir.join("prismlauncher.cfg"), config).await?;

    // Create a sample instance
    create_prism_instance(&prism_dir, "Vanilla 1.20.1").await?;

    Ok(())
}

/// Create a PrismLauncher instance
async fn create_prism_instance(prism_dir: &PathBuf, instance_name: &str) -> Result<()> {
    let instance_dir = prism_dir.join("instances").join(instance_name);
    fs::create_dir_all(&instance_dir).await?;

    // Create .minecraft directory
    let minecraft_dir = instance_dir.join(".minecraft");
    fs::create_dir_all(&minecraft_dir).await?;
    fs::create_dir_all(minecraft_dir.join("saves")).await?;
    fs::create_dir_all(minecraft_dir.join("resourcepacks")).await?;
    fs::create_dir_all(minecraft_dir.join("shaderpacks")).await?;
    fs::create_dir_all(minecraft_dir.join("mods")).await?;

    // Create instance.cfg
    let instance_config = r#"InstanceType=OneSix
IntendedVersion=1.20.1
LogPrePostOutput=true
OverrideCommands=false
OverrideConsole=false
OverrideGameTime=false
OverrideGlobalGameTime=false
OverrideJavaArgs=false
OverrideJavaLocation=false
OverrideMemory=false
OverrideNativeWorkarounds=false
OverrideWindow=false
iconKey=default
lastLaunchTime=1640995200
name=Vanilla 1.20.1
notes=
totalTimePlayed=0
"#;

    fs::write(instance_dir.join("instance.cfg"), instance_config).await?;

    // Create mmc-pack.json
    let mmc_pack = json!({
        "components": [
            {
                "cachedName": "Minecraft",
                "cachedRequires": [],
                "cachedVersion": "1.20.1",
                "important": true,
                "uid": "net.minecraft",
                "version": "1.20.1"
            }
        ],
        "formatVersion": 1
    });

    fs::write(
        instance_dir.join("mmc-pack.json"),
        serde_json::to_string_pretty(&mmc_pack)?
    ).await?;

    Ok(())
}

/// PrismLauncher-Cracked structure (similar to PrismLauncher but with different auth)
async fn create_prism_launcher_cracked_structure(base_dir: &PathBuf) -> Result<()> {
    let prism_cracked_dir = base_dir.join("PrismLauncher-Cracked");

    // Similar structure to PrismLauncher
    fs::create_dir_all(&prism_cracked_dir).await?;
    fs::create_dir_all(prism_cracked_dir.join("instances")).await?;
    fs::create_dir_all(prism_cracked_dir.join("libraries")).await?;
    fs::create_dir_all(prism_cracked_dir.join("assets")).await?;

    // Different accounts.json for cracked version
    let accounts = json!({
        "accounts": [
            {
                "active": true,
                "profile": {
                    "id": "00000000-0000-0000-0000-000000000000",
                    "name": "Player",
                    "properties": []
                },
                "type": "Offline"
            }
        ],
        "formatVersion": 3
    });

    fs::write(
        prism_cracked_dir.join("accounts.json"),
        serde_json::to_string_pretty(&accounts)?
    ).await?;

    // Similar config but with offline mode settings
    let config = r#"Analytics=false
AutoCloseConsole=false
AutoUpdate=false
BaseMemMaxAlloc=2048
BaseMemMinAlloc=512
OfflineMode=true
"#;

    fs::write(prism_cracked_dir.join("prismlauncher.cfg"), config).await?;

    Ok(())
}

/// XMCL (X Minecraft Launcher) structure
async fn create_xmcl_structure(base_dir: &PathBuf) -> Result<()> {
    let xmcl_dir = base_dir.join(".xmcl");

    // Create directory structure
    fs::create_dir_all(&xmcl_dir).await?;
    fs::create_dir_all(xmcl_dir.join("instances")).await?;
    fs::create_dir_all(xmcl_dir.join("libraries")).await?;
    fs::create_dir_all(xmcl_dir.join("assets")).await?;
    fs::create_dir_all(xmcl_dir.join("versions")).await?;
    fs::create_dir_all(xmcl_dir.join("temp")).await?;
    fs::create_dir_all(xmcl_dir.join("logs")).await?;

    // Create config.json
    let config = json!({
        "locale": "en",
        "autoInstallOnAppQuit": false,
        "autoDownload": false,
        "allowPrerelease": false,
        "apiSetsPreference": "mojang",
        "theme": "dark",
        "maxSockets": 16,
        "globalMaxSockets": 64,
        "globalMinTlsVersion": "TLSv1.2",
        "httpProxy": "",
        "httpProxyEnabled": false,
        "maxMemory": 0,
        "minMemory": 0,
        "vmOptions": [],
        "mcOptions": [],
        "hideLauncher": true,
        "showLogAfterLaunch": false,
        "defaultUser": {
            "id": "",
            "username": "",
            "accessToken": "",
            "userType": "mojang",
            "properties": {}
        },
        "users": [],
        "selectedUser": {
            "id": ""
        },
        "clientToken": "",
        "authServices": {
            "mojang": {
                "hostName": "https://authserver.mojang.com",
                "authenticate": "/authenticate",
                "refresh": "/refresh",
                "validate": "/validate",
                "invalidate": "/invalidate",
                "signout": "/signout"
            }
        }
    });

    fs::write(
        xmcl_dir.join("config.json"),
        serde_json::to_string_pretty(&config)?
    ).await?;

    // Create instances.json
    let instances = json!({
        "instances": [],
        "selectedInstance": ""
    });

    fs::write(
        xmcl_dir.join("instances.json"),
        serde_json::to_string_pretty(&instances)?
    ).await?;

    Ok(())
}

/// AstralRinth App structure
async fn create_astral_rinth_structure(base_dir: &PathBuf) -> Result<()> {
    let astral_dir = base_dir.join("AstralRinthApp");

    // Create directory structure
    fs::create_dir_all(&astral_dir).await?;
    fs::create_dir_all(astral_dir.join("profiles")).await?;
    fs::create_dir_all(astral_dir.join("meta")).await?;
    fs::create_dir_all(astral_dir.join("meta/versions")).await?;
    fs::create_dir_all(astral_dir.join("meta/libraries")).await?;
    fs::create_dir_all(astral_dir.join("meta/assets")).await?;
    fs::create_dir_all(astral_dir.join("caches")).await?;
    fs::create_dir_all(astral_dir.join("launcher_logs")).await?;

    // Create settings.json
    let settings = json!({
        "theme": "dark",
        "collapsed_navigation": false,
        "advanced_rendering": false,
        "opt_out_analytics": false,
        "default_page": "Home",
        "native_decorations": false,
        "custom_dir": null,
        "max_concurrent_downloads": 10,
        "max_concurrent_writes": 10,
        "memory": {
            "maximum": 2048,
            "minimum": 512
        },
        "game_resolution": {
            "width": 854,
            "height": 480
        },
        "custom_java_args": [],
        "custom_env_args": [],
        "hook": {
            "pre_launch": null,
            "wrapper": null,
            "post_exit": null
        },
        "init_hooks": [],
        "disable_discord_rpc": false,
        "developer_mode": false
    });

    fs::write(
        astral_dir.join("settings.json"),
        serde_json::to_string_pretty(&settings)?
    ).await?;

    // Create a sample profile
    create_astral_rinth_profile(&astral_dir, "vanilla-1.20.1").await?;

    Ok(())
}

/// Create an AstralRinth profile
async fn create_astral_rinth_profile(astral_dir: &PathBuf, profile_name: &str) -> Result<()> {
    let profile_dir = astral_dir.join("profiles").join(profile_name);
    fs::create_dir_all(&profile_dir).await?;

    // Create profile.json
    let profile = json!({
        "name": "Vanilla 1.20.1",
        "game_version": "1.20.1",
        "loader": "vanilla",
        "loader_version": null,
        "icon_path": null,
        "created": "2023-01-01T00:00:00Z",
        "modified": "2023-01-01T00:00:00Z",
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
            "name": "Vanilla 1.20.1",
            "version_id": "1.20.1"
        }
    });

    fs::write(
        profile_dir.join("profile.json"),
        serde_json::to_string_pretty(&profile)?
    ).await?;

    // Create Minecraft directory structure
    let minecraft_dir = profile_dir;
    fs::create_dir_all(minecraft_dir.join("saves")).await?;
    fs::create_dir_all(minecraft_dir.join("resourcepacks")).await?;
    fs::create_dir_all(minecraft_dir.join("shaderpacks")).await?;
    fs::create_dir_all(minecraft_dir.join("mods")).await?;
    fs::create_dir_all(minecraft_dir.join("config")).await?;
    fs::create_dir_all(minecraft_dir.join("logs")).await?;
    fs::create_dir_all(minecraft_dir.join("crash-reports")).await?;

    Ok(())
}

/// Test Official Minecraft Launcher integration
async fn test_official_minecraft_launcher(base_dir: &PathBuf) -> Result<()> {
    let launcher_manager = LauncherManager::new();
    let minecraft_dir = base_dir.join(".minecraft");

    // Test detecting launcher
    assert!(launcher_manager.detect_launcher_type(&minecraft_dir).await? == LauncherType::Official);

    // Test creating instance
    launcher_manager.create_instance(
        &minecraft_dir,
        "test-instance",
        "1.20.1",
        "vanilla",
        None
    ).await?;

    // Verify instance was created
    let profiles_path = minecraft_dir.join("launcher_profiles.json");
    assert!(profiles_path.exists());

    let profiles_content = fs::read_to_string(profiles_path).await?;
    assert!(profiles_content.contains("test-instance"));

    println!("✓ Official Minecraft Launcher test passed");
    Ok(())
}

/// Test PrismLauncher integration
async fn test_prism_launcher(base_dir: &PathBuf) -> Result<()> {
    let launcher_manager = LauncherManager::new();
    let prism_dir = base_dir.join("PrismLauncher");

    // Test detecting launcher
    assert!(launcher_manager.detect_launcher_type(&prism_dir).await? == LauncherType::Prism);

    // Test creating instance
    launcher_manager.create_instance(
        &prism_dir,
        "test-prism-instance",
        "1.20.1",
        "vanilla",
        None
    ).await?;

    // Verify instance was created
    let instance_dir = prism_dir.join("instances").join("test-prism-instance");
    assert!(instance_dir.exists());
    assert!(instance_dir.join("instance.cfg").exists());
    assert!(instance_dir.join("mmc-pack.json").exists());

    println!("✓ PrismLauncher test passed");
    Ok(())
}

/// Test PrismLauncher-Cracked integration
async fn test_prism_launcher_cracked(base_dir: &PathBuf) -> Result<()> {
    let launcher_manager = LauncherManager::new();
    let prism_cracked_dir = base_dir.join("PrismLauncher-Cracked");

    // Test detecting launcher
    assert!(launcher_manager.detect_launcher_type(&prism_cracked_dir).await? == LauncherType::PrismCracked);

    println!("✓ PrismLauncher-Cracked test passed");
    Ok(())
}

/// Test XMCL integration
async fn test_xmcl_launcher(base_dir: &PathBuf) -> Result<()> {
    let launcher_manager = LauncherManager::new();
    let xmcl_dir = base_dir.join(".xmcl");

    // Test detecting launcher
    assert!(launcher_manager.detect_launcher_type(&xmcl_dir).await? == LauncherType::XMCL);

    println!("✓ XMCL test passed");
    Ok(())
}

/// Test AstralRinth App integration
async fn test_astral_rinth_app(base_dir: &PathBuf) -> Result<()> {
    let launcher_manager = LauncherManager::new();
    let astral_dir = base_dir.join("AstralRinthApp");

    // Test detecting launcher
    assert!(launcher_manager.detect_launcher_type(&astral_dir).await? == LauncherType::AstralRinth);

    println!("✓ AstralRinth App test passed");
    Ok(())
}

#[tokio::test]
async fn test_mrpack_installation() -> Result<()> {
    // Test mrpack file installation
    let test_dir = PathBuf::from("test-mrpack");

    // Clean up any existing test directory
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir).await?;
    }

    // Create test mrpack structure
    create_test_mrpack(&test_dir).await?;

    // Test installation
    let launcher_manager = LauncherManager::new();
    let mrpack_path = test_dir.join("test-modpack.mrpack");

    launcher_manager.install_mrpack(
        &mrpack_path,
        &test_dir.join("instance"),
        "Test Modpack Instance"
    ).await?;

    // Verify installation
    let instance_dir = test_dir.join("instance");
    assert!(instance_dir.exists());
    assert!(instance_dir.join("mods").exists());
    assert!(instance_dir.join("config").exists());

    // Clean up
    fs::remove_dir_all(&test_dir).await?;

    println!("✓ Mrpack installation test passed");
    Ok(())
}

/// Create a test mrpack file
async fn create_test_mrpack(test_dir: &PathBuf) -> Result<()> {
    use std::io::Write;

    fs::create_dir_all(test_dir).await?;

    // Create a simple ZIP file structure for mrpack
    let mrpack_path = test_dir.join("test-modpack.mrpack");
    let file = std::fs::File::create(&mrpack_path)?;
    let mut zip = zip::ZipWriter::new(file);

    // Add modrinth.index.json
    let index = json!({
        "formatVersion": 1,
        "game": "minecraft",
        "versionId": "1.0.0",
        "name": "Test Modpack",
        "summary": "A test modpack",
        "files": [
            {
                "path": "mods/example-mod.jar",
                "hashes": {
                    "sha1": "da39a3ee5e6b4b0d3255bfef95601890afd80709"
                },
                "env": {
                    "client": "required",
                    "server": "required"
                },
                "downloads": [
                    "https://example.com/example-mod.jar"
                ],
                "fileSize": 12345
            }
        ],
        "dependencies": {
            "minecraft": "1.20.1",
            "fabric-loader": "0.14.21"
        }
    });

    zip.start_file("modrinth.index.json", zip::write::FileOptions::default())?;
    zip.write_all(serde_json::to_string_pretty(&index)?.as_bytes())?;

    // Add overrides directory with config files
    zip.add_directory("overrides/", zip::write::FileOptions::default())?;
    zip.add_directory("overrides/config/", zip::write::FileOptions::default())?;

    zip.start_file("overrides/config/example.toml", zip::write::FileOptions::default())?;
    zip.write_all(b"# Example config file\nenabled = true\n")?;

    zip.finish()?;

    Ok(())
}






