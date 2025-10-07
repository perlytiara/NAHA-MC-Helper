import { app, BrowserWindow, ipcMain, Menu } from "electron";
import { fileURLToPath } from "url";
import path from "path";
import fs from "fs/promises";
import { spawn } from "child_process";
import { createWriteStream } from "fs";
import https from "https";
import http from "http";
import crypto from "crypto";
import AdmZip from "adm-zip";
import os from "os";
import MinecraftInstallerService from "./src/shared/services/MinecraftInstallerService.js";
import AutoUpdaterService from "./src/shared/services/AutoUpdaterService.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Global state for debug mode
let debugMode = false;
let mainWindow = null;

// Initialize services
const minecraftInstaller = new MinecraftInstallerService();
let autoUpdater = null;

const createMenu = () => {
  const template = [
    {
      label: "File",
      submenu: [{ role: "quit" }],
    },
    {
      label: "Edit",
      submenu: [
        { role: "undo" },
        { role: "redo" },
        { type: "separator" },
        { role: "cut" },
        { role: "copy" },
        { role: "paste" },
      ],
    },
    {
      label: "View",
      submenu: [
        { role: "reload" },
        { role: "forceReload" },
        { role: "toggleDevTools" },
        { type: "separator" },
        { role: "resetZoom" },
        { role: "zoomIn" },
        { role: "zoomOut" },
        { role: "togglefullscreen" },
      ],
    },
    {
      label: "Window",
      submenu: [{ role: "minimize" }, { role: "close" }],
    },
    {
      label: "Help",
      submenu: [
        {
          label: "About NAHA MC Helper",
          click: async () => {
            if (mainWindow) {
              mainWindow.webContents.send("show-about-dialog");
            }
          },
        },
        { type: "separator" },
        {
          label: "Check for Updates",
          click: async () => {
            console.log('🚀 MAIN: Menu "Check for Updates" clicked!');
            // Send message to renderer to trigger update check
            if (mainWindow && !mainWindow.isDestroyed()) {
              console.log('🚀 MAIN: Sending menu:check-for-updates event to renderer');
              mainWindow.webContents.send('menu:check-for-updates');
            } else {
              console.error('🚀 MAIN: mainWindow is null or destroyed');
            }
          },
        },
        { type: "separator" },
        {
          label: "Debug Mode",
          type: "checkbox",
          checked: debugMode,
          click: () => {
            debugMode = !debugMode;
            if (mainWindow) {
              mainWindow.webContents.send("debug-changed", debugMode);
            }
            // Recreate menu to update checkbox state
            createMenu();
          },
        },
        {
          label: "Reset Data",
          click: async () => {
            if (mainWindow) {
              const { dialog } = await import("electron");
              const result = await dialog.showMessageBox(mainWindow, {
                type: "warning",
                title: "Reset Application Data",
                message:
                  "Are you sure you want to reset all application data to factory settings?",
                detail:
                  "This will clear all settings, cache, and user data. This action cannot be undone.",
                buttons: ["Cancel", "Reset Data"],
                defaultId: 0,
                cancelId: 0,
              });

              if (result.response === 1) {
                // User confirmed reset - trigger reset from frontend
                console.log("User confirmed reset - triggering frontend reset");
                if (mainWindow && mainWindow.webContents) {
                  // Execute the reset function in the renderer process
                  mainWindow.webContents.executeJavaScript(`
                    console.log('Menu reset triggered from main process');
                    if (window.manualReset) {
                      window.manualReset();
                    } else {
                      console.error('manualReset function not available');
                    }
                  `);
                } else {
                  console.error(
                    "Cannot trigger reset - mainWindow or webContents not available"
                  );
                }
              } else {
                console.log("User cancelled reset");
              }
            }
          },
        },
      ],
    },
  ];

  // macOS specific menu adjustments
  if (process.platform === "darwin") {
    template.unshift({
      label: app.getName(),
      submenu: [
        {
          label: `About ${app.getName()}`,
          click: async () => {
            if (mainWindow) {
              mainWindow.webContents.send("show-about-dialog");
            }
          },
        },
        { type: "separator" },
        { role: "services" },
        { type: "separator" },
        { role: "hide" },
        { role: "hideOthers" },
        { role: "unhide" },
        { type: "separator" },
        { role: "quit" },
      ],
    });

    // Update Help menu for macOS (remove About since it's in app menu)
    const helpMenu = template.find((menu) => menu.label === "Help");
    if (helpMenu) {
      helpMenu.submenu = helpMenu.submenu.filter(
        (item) => item.label !== "About NAHA MC Helper"
      );
    }

    // Window menu - more compact
    template[6].submenu = [
      { role: "close" },
      { role: "minimize" },
      { role: "zoom" },
      { type: "separator" },
      { role: "front" },
    ];
  }

  const menu = Menu.buildFromTemplate(template);
  Menu.setApplicationMenu(menu);
};

const createWindow = () => {
  mainWindow = new BrowserWindow({
    width: 1200,
    height: 800,
    titleBarStyle: "default",
    autoHideMenuBar: false,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      preload: path.join(__dirname, "preload.js"),
    },
  });

  // In development, load Vite dev server
  const isDev = !app.isPackaged;

  if (isDev) {
    // Try to load Vite dev server first, fallback to built files
    mainWindow.loadURL("http://localhost:5173").catch((err) => {
      console.error("Failed to load Vite dev server:", err);
      console.log("Falling back to built files...");
      // Fallback to built files if dev server is not available
      mainWindow.loadFile(path.join(__dirname, "dist", "index.html")).catch((fallbackErr) => {
        console.error("Failed to load built files:", fallbackErr);
        console.log("Please run 'npm run build' first to create the dist folder");
      });
    });
    // Only open DevTools if debug mode is enabled
    if (debugMode) {
      mainWindow.webContents.openDevTools();
    }
  } else {
    // In production, load the built index.html
    mainWindow.loadFile(path.join(__dirname, "dist", "index.html"));
  }

  // Add error handling for failed loads
  mainWindow.webContents.on(
    "did-fail-load",
    (event, errorCode, errorDescription, validatedURL) => {
      console.error("Failed to load:", validatedURL, errorDescription);
    }
  );

  // Try to make menu more compact on Windows
  if (process.platform === "win32") {
    mainWindow.setMenuBarVisibility(true);
  }

  return mainWindow;
};

app.whenReady().then(() => {
  createWindow();
  createMenu();

  // Initialize auto-updater after window is created
  if (mainWindow) {
    autoUpdater = new AutoUpdaterService(mainWindow);
    
    // Start periodic update checks if enabled
    autoUpdater.startPeriodicUpdateCheck();
  }

  app.on("activate", () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
      createMenu();
      
      // Re-initialize auto-updater for new window
      if (mainWindow) {
        autoUpdater = new AutoUpdaterService(mainWindow);
        autoUpdater.startPeriodicUpdateCheck();
      }
    }
  });
});

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") app.quit();
});

// IPC Handlers for installer functionality
ipcMain.handle("download-file", async (event, options) => {
  try {
    const { url: originalUrl, outPath } = options;
    let url = originalUrl;
    console.log("Downloading:", url, "to:", outPath);

    // No special handling for AstralRinth - let it download normally with the correct URLs

    // Ensure directory exists
    const dir = path.dirname(outPath);
    await fs.mkdir(dir, { recursive: true });

    return new Promise((resolve, reject) => {
      const makeRequest = (requestUrl, redirectCount = 0) => {
        if (redirectCount > 5) {
          reject(new Error("Too many redirects"));
          return;
        }

        const currentProtocol = requestUrl.startsWith("https:") ? https : http;
        const request = currentProtocol.get(requestUrl, (response) => {
          // Handle redirects
          if (
            response.statusCode === 301 ||
            response.statusCode === 302 ||
            response.statusCode === 303 ||
            response.statusCode === 307 ||
            response.statusCode === 308
          ) {
            const redirectUrl = response.headers.location;
            if (redirectUrl) {
              console.log(
                `Following redirect ${response.statusCode} to:`,
                redirectUrl
              );
              response.destroy(); // Clean up current response
              makeRequest(redirectUrl, redirectCount + 1);
              return;
            } else {
              console.log(`Redirect ${response.statusCode} but no location header found`);
            }
          }

          if (response.statusCode !== 200) {
            reject(
              new Error(
                `Download failed: ${response.statusCode} ${response.statusMessage}`
              )
            );
            return;
          }

          // Create file stream for the final URL
          const file = createWriteStream(outPath);
          const totalSize = parseInt(
            response.headers["content-length"] || "0",
            10
          );
          let downloadedSize = 0;

          response.on("data", (chunk) => {
            downloadedSize += chunk.length;
            const progress =
              totalSize > 0 ? (downloadedSize / totalSize) * 100 : 0;

            // Send progress update
            event.sender.send("download-progress", {
              progress: {
                percent: progress,
                transferred: downloadedSize,
                total: totalSize,
              },
            });
          });

          response.pipe(file);

          file.on("finish", () => {
            file.close();
            console.log("Download completed:", outPath);
            resolve({ ok: true, path: outPath });
          });

          file.on("error", (err) => {
            console.error("File write error:", err);
            fs.unlink(outPath).catch(() => {}); // Clean up on error
            reject(err);
          });
        });

        request.on("error", (err) => {
          console.error("Request error:", err);
          reject(err);
        });
      };

      // Start the initial request
      makeRequest(url, 0);
    });
  } catch (error) {
    console.error("Download error:", error);
    return { ok: false, error: error.message };
  }
});

ipcMain.handle("ensure-directory", async (event, dirPath) => {
  try {
    await fs.mkdir(dirPath, { recursive: true });
    return { ok: true };
  } catch (error) {
    return { ok: false, error: error.message };
  }
});

ipcMain.handle("delete-file", async (event, filePath) => {
  try {
    await fs.unlink(filePath);
    return { ok: true };
  } catch (error) {
    return { ok: false, error: error.message };
  }
});

ipcMain.handle("exec-installer", async (event, options) => {
  try {
    const { command, args, requiresElevation, os } = options;
    console.log("Executing installer:", command, args);

    return new Promise((resolve, reject) => {
      let actualCommand = command;
      let actualArgs = args || [];

      // Special handling for .appinstaller files
      if (command.endsWith(".appinstaller")) {
        // Use PowerShell with Invoke-Item to open .appinstaller files
        actualCommand = "powershell";
        actualArgs = ["-Command", `Invoke-Item "${command}"`];
        console.log("Using Invoke-Item for .appinstaller file");
      }
      // Handle elevation on Windows for other file types
      else if (requiresElevation && os === "windows") {
        actualCommand = "powershell";
        if (args && args.length > 0) {
          actualArgs = [
            "-Command",
            `Start-Process "${command}" -ArgumentList "${args.join(" ")}" -Verb RunAs -Wait`,
          ];
        } else {
          // No arguments - don't include ArgumentList parameter
          actualArgs = [
            "-Command",
            `Start-Process "${command}" -Verb RunAs -Wait`,
          ];
        }
      }

      const child = spawn(actualCommand, actualArgs, {
        detached: false, // Don't detach so we can track completion
        stdio: "pipe",
      });

      console.log("Installer process started with PID:", child.pid);

      // Always wait for completion to provide feedback
      child.on("close", (code) => {
        console.log("Installer process completed with exit code:", code);

        // Special handling for .appinstaller files
        if (command.endsWith(".appinstaller")) {
          // .appinstaller files might return various exit codes but still work
          // Common exit codes: 0 = success, 1 = already installed, 5 = access denied but might still work
          if (code === 0 || code === 1) {
            resolve({ ok: true, exitCode: code });
          } else {
            console.warn(
              `AppInstaller returned code ${code}, but this might still be successful`
            );
            resolve({
              ok: true,
              exitCode: code,
              warning: `AppInstaller returned code ${code}`,
            });
          }
        } else {
          // Regular installers
          if (code === 0) {
            resolve({ ok: true, exitCode: code });
          } else {
            reject(new Error(`Installer exited with code ${code}`));
          }
        }
      });

      child.on("error", (error) => {
        console.error("Installer process error:", error);
        reject(error);
      });

      // Log stdout/stderr for debugging
      child.stdout?.on("data", (data) => {
        console.log("Installer stdout:", data.toString());
      });

      child.stderr?.on("data", (data) => {
        console.log("Installer stderr:", data.toString());
      });
    });
  } catch (error) {
    console.error("Exec installer error:", error);
    return { ok: false, error: error.message };
  }
});

ipcMain.handle("get-cache-dir", async () => {
  try {
    const userDataPath = app.getPath("userData");
    const cacheDir = path.join(userDataPath, "cache", "installers");
    await fs.mkdir(cacheDir, { recursive: true });
    return cacheDir;
  } catch (err) {
    console.warn("Cache dir error:", err);
    return app.getPath("temp");
  }
});

ipcMain.handle("fetch-latest-release", async () => {
  // This would implement the GitHub/GitLab API calls
  // For now, return null to use the fallback logic in the renderer
  return null;
});

ipcMain.handle("fetch-latest-pack", async () => {
  // This would implement modpack fetching
  // For now, return null to use existing logic
  return null;
});

ipcMain.handle("get-theme", async () => {
  return "system";
});

ipcMain.handle("get-file-hash", async (event, filePath, algorithm) => {
  try {
    const fileBuffer = await fs.readFile(filePath);
    const hash = crypto.createHash(algorithm || "sha256");
    hash.update(fileBuffer);
    return hash.digest("hex");
  } catch (error) {
    throw new Error(`Failed to compute hash: ${error.message}`);
  }
});

// App info handler
ipcMain.handle("get-app-info", async () => {
  try {
    const packageJson = JSON.parse(await fs.readFile("./package.json", "utf8"));
    return {
      name: packageJson.name,
      version: packageJson.version,
      description: packageJson.description,
      author: packageJson.author,
      electron: process.versions.electron,
      chrome: process.versions.chrome,
      node: process.versions.node,
      platform: process.platform,
      arch: process.arch,
    };
  } catch (error) {
    console.error("Failed to read package.json:", error);
    return {
      name: "naha-mc-helper",
      version: "1.0.0",
      description:
        "Electron app with Svelte, Tailwind CSS, DaisyUI, and TypeScript",
      author: "",
      electron: process.versions.electron,
      chrome: process.versions.chrome,
      node: process.versions.node,
      platform: process.platform,
      arch: process.arch,
    };
  }
});

// Debug mode handlers
ipcMain.handle("get-debug-mode", async () => {
  return debugMode;
});

ipcMain.handle("set-debug-mode", async (event, enabled) => {
  debugMode = enabled;
  if (mainWindow) {
    mainWindow.webContents.send("debug-changed", debugMode);
    // Toggle dev tools based on debug mode
    if (debugMode) {
      mainWindow.webContents.openDevTools();
    } else {
      mainWindow.webContents.closeDevTools();
    }
  }
  // Update menu checkbox
  createMenu();
  return debugMode;
});

// Reset data handler
ipcMain.handle("reset-application-data", async () => {
  try {
    console.log("Starting application data reset...");
    console.log("User data path:", app.getPath("userData"));

    // Get user data directory
    const userDataPath = app.getPath("userData");

    // List of directories/files to clean up (avoiding locked files)
    const pathsToClean = [
      path.join(userDataPath, "cache"),
      path.join(userDataPath, "logs"),
      path.join(userDataPath, "IndexedDB"),
      path.join(userDataPath, "databases"),
      path.join(userDataPath, "blob_storage"),
      path.join(userDataPath, "Local Extension Settings"),
      path.join(userDataPath, "Preferences"),
      path.join(userDataPath, "Network Persistent State"),
      path.join(userDataPath, "Code Cache"),
      path.join(userDataPath, "GPUCache"),
    ];

    // Note: Skipping 'Local Storage' and 'Session Storage' as they're often locked during app use

    // Clean up each path
    for (const pathToClean of pathsToClean) {
      try {
        const stats = await fs.stat(pathToClean);
        if (stats.isDirectory()) {
          await fs.rm(pathToClean, { recursive: true, force: true });
          console.log(`Cleaned directory: ${pathToClean}`);
        } else {
          await fs.unlink(pathToClean);
          console.log(`Cleaned file: ${pathToClean}`);
        }
      } catch (error) {
        // Path doesn't exist or can't be cleaned - that's okay
        if (error.code === "ENOENT") {
          // Path doesn't exist - this is normal, don't log it
          continue;
        } else if (error.code === "EBUSY" || error.code === "EPERM") {
          // File is locked or permission denied - this is expected for some files
          console.log(`Skipped locked file: ${path.basename(pathToClean)}`);
        } else {
          // Other errors - log them
          console.log(
            `Skipped cleaning ${path.basename(pathToClean)}: ${error.message}`
          );
        }
      }
    }

    // Reset debug mode
    debugMode = false;

    console.log("Application data reset completed successfully");

    return {
      success: true,
      message:
        "Application data has been reset successfully. Please restart the application to see the onboarding flow again.",
    };
  } catch (error) {
    console.error("Error resetting application data:", error);
    return {
      success: false,
      message: `Failed to reset application data: ${error.message}`,
    };
  }
});

// Instance Creation Handlers

// Show open dialog for file selection
ipcMain.handle("show-open-dialog", async (event, options) => {
  try {
    const { dialog } = await import("electron");
    const window = BrowserWindow.fromWebContents(event.sender);
    const result = await dialog.showOpenDialog(window, options);
    return result;
  } catch (error) {
    console.error("Error showing open dialog:", error);
    return { canceled: true, error: error.message };
  }
});

// Validate Minecraft installation directory
ipcMain.handle("validate-minecraft-installation", async (event, minecraftPath) => {
  try {
    const fs = await import("fs/promises");
    const path = await import("path");
    
    console.log("🔍 Validating Minecraft installation at:", minecraftPath);
    
    // Check if directory exists
    try {
      await fs.access(minecraftPath);
    } catch {
      console.log("❌ Directory does not exist:", minecraftPath);
      return false;
    }
    
    // Check for essential Minecraft files/folders
    const essentialItems = [
      'launcher_profiles.json', // Launcher profiles
      'versions',               // Versions folder
      'libraries',              // Libraries folder
      'assets'                  // Assets folder
    ];
    
    const validItems = [];
    for (const item of essentialItems) {
      try {
        const itemPath = path.join(minecraftPath, item);
        await fs.access(itemPath);
        validItems.push(item);
        console.log("✅ Found:", item);
      } catch {
        console.log("❌ Missing:", item);
      }
    }
    
    // Consider it valid if at least 3 out of 4 essential items are present
    const isValid = validItems.length >= 3;
    console.log("📊 Validation result:", isValid ? "✅ Valid" : "❌ Invalid", `(${validItems.length}/4 items found)`);
    
    return isValid;
  } catch (error) {
    console.error("Error validating Minecraft installation:", error);
    return false;
  }
});

// Parse .mrpack file and extract metadata
ipcMain.handle("parse-mrpack", async (event, mrpackPath) => {
  try {
    console.log("[PARSE] Parsing .mrpack file:", mrpackPath);

    // Check if file exists
    await fs.access(mrpackPath);
    console.log("[PARSE] File exists, proceeding with parsing...");

    // Read the .mrpack file
    console.log("[PARSE] Opening .mrpack archive...");
    const zip = new AdmZip(mrpackPath);
    const entries = zip.getEntries();
    console.log(`[PARSE] Found ${entries.length} entries in archive`);

    // Find and read modrinth.index.json
    console.log("[PARSE] Looking for modrinth.index.json...");
    const indexEntry = entries.find(
      (entry) => entry.entryName === "modrinth.index.json"
    );
    if (!indexEntry) {
      throw new Error("modrinth.index.json not found in .mrpack file");
    }
    console.log("[PARSE] Found modrinth.index.json");

    const indexContent = indexEntry.getData().toString("utf8");
    const indexData = JSON.parse(indexContent);
    console.log("[PARSE] Parsed index data:", {
      name: indexData.name,
      version: indexData.versionId,
      gameVersion: indexData.gameVersion,
      formatVersion: indexData.formatVersion,
      loader: indexData.loader,
      dependencies: indexData.dependencies,
    });

    // Extract comprehensive information
    console.log("Extracting pack information...");
    
    // Extract game version using mrpack-cli approach - check dependencies first
    let gameVersion = indexData.gameVersion;
    console.log(`[PARSE] Game version from index: ${gameVersion}`);
    
    if (!gameVersion && indexData.dependencies && indexData.dependencies["minecraft"]) {
      gameVersion = indexData.dependencies["minecraft"];
      console.log(`[PARSE] Game version from dependencies: ${gameVersion}`);
    }
    
    // Fallback to filename extraction if not found in index or dependencies
    if (!gameVersion) {
      const filename = path.basename(mrpackPath, '.mrpack');
      const versionMatch = filename.match(/(\d+\.\d+\.\d+)/);
      if (versionMatch) {
        gameVersion = versionMatch[1];
        console.log(`[PARSE] Game version from filename: ${gameVersion}`);
      }
    }
    
    // Try to detect mod loader from pack name or files
    let modLoaderType = "unknown";
    let modLoaderVersion = "unknown";
    
    // Extract mod loader using mrpack-cli approach - check dependencies first
    console.log("[PARSE] Using mrpack-cli approach for mod loader detection...");
    
    if (indexData.dependencies) {
      console.log("[PARSE] Dependencies structure:", indexData.dependencies);
      
      // Check for each loader type in dependencies (mrpack-cli format)
      if (indexData.dependencies["fabric-loader"]) {
        modLoaderType = "fabric";
        modLoaderVersion = indexData.dependencies["fabric-loader"];
        console.log(`[PARSE] Found Fabric Loader: ${modLoaderVersion}`);
      } else if (indexData.dependencies["neoforge"]) {
        modLoaderType = "neoforge";
        modLoaderVersion = indexData.dependencies["neoforge"];
        console.log(`[PARSE] Found NeoForge: ${modLoaderVersion}`);
      } else if (indexData.dependencies["forge"]) {
        modLoaderType = "forge";
        modLoaderVersion = indexData.dependencies["forge"];
        console.log(`[PARSE] Found Forge: ${modLoaderVersion}`);
      } else if (indexData.dependencies["quilt-loader"]) {
        modLoaderType = "quilt";
        modLoaderVersion = indexData.dependencies["quilt-loader"];
        console.log(`[PARSE] Found Quilt Loader: ${modLoaderVersion}`);
      } else {
        console.log("[PARSE] No loader found in dependencies, checking minecraft version...");
        if (indexData.dependencies["minecraft"]) {
          console.log(`[PARSE] Minecraft version from dependencies: ${indexData.dependencies["minecraft"]}`);
          if (!gameVersion) {
            gameVersion = indexData.dependencies["minecraft"];
          }
        }
      }
    }
    
    // Fallback: check legacy loader format if not found in dependencies
    if (modLoaderType === "unknown" && indexData.loader) {
      console.log("[PARSE] Checking legacy loader format:", indexData.loader);
      modLoaderType = indexData.loader.type || "unknown";
      modLoaderVersion = indexData.loader.version || "unknown";
      console.log(`[PARSE] Found legacy loader: ${modLoaderType} ${modLoaderVersion}`);
    }
    
    // Final fallback: analyze files if still unknown
    if (modLoaderType === "unknown" && indexData.files) {
      console.log("[PARSE] Analyzing files as final fallback...");
      for (const file of indexData.files) {
        const filePath = file.path.toLowerCase();
        
        if (filePath.includes("fabric-loader")) {
          modLoaderType = "fabric";
          const versionMatch = filePath.match(/fabric-loader-(\d+\.\d+\.\d+)/);
          if (versionMatch) {
            modLoaderVersion = versionMatch[1];
          }
          console.log(`[PARSE] Found Fabric from file: ${modLoaderVersion}`);
          break;
        } else if (filePath.includes("neoforge") && filePath.endsWith('.jar')) {
          modLoaderType = "neoforge";
          const versionMatch = filePath.match(/(\d+\.\d+\.\d+)/);
          if (versionMatch) {
            modLoaderVersion = versionMatch[1];
          }
          console.log(`[PARSE] Found NeoForge from file: ${modLoaderVersion}`);
          break;
        } else if (filePath.includes("forge") && !filePath.includes("neoforge")) {
          modLoaderType = "forge";
          const versionMatch = filePath.match(/forge-(\d+\.\d+\.\d+)/);
          if (versionMatch) {
            modLoaderVersion = versionMatch[1];
          }
          console.log(`[PARSE] Found Forge from file: ${modLoaderVersion}`);
          break;
        }
      }
    }
    
    // Extract pack version - try index first, then filename
    let packVersion = indexData.versionId;
    if (!packVersion) {
      const filename = path.basename(mrpackPath, '.mrpack');
      // Look for version patterns in filename - try to get the last one (pack version)
      const versionMatches = filename.match(/(\d+\.\d+\.\d+)/g);
      if (versionMatches && versionMatches.length > 0) {
        // Use the last version found (likely the pack version)
        packVersion = versionMatches[versionMatches.length - 1];
        console.log(`[PARSE] Extracted pack version from filename: ${packVersion}`);
      }
    } else {
      console.log(`[PARSE] Using pack version from index: ${packVersion}`);
    }
    
    const packInfo = {
      name: indexData.name || "Unknown Pack",
      version: packVersion || "1.0.0",
      gameVersion: gameVersion || "1.20.1",
      loader: indexData.loader || { type: modLoaderType, version: modLoaderVersion },
      dependencies: indexData.dependencies || [],
      files: indexData.files || [],
      overrides: indexData.overrides || {},
      summary: indexData.summary || "",
      description: indexData.description || "",
      formatVersion: indexData.formatVersion || 1,
      environment: indexData.environment || {},
      // Additional metadata
      totalFiles: indexData.files ? indexData.files.length : 0,
      totalDependencies: indexData.dependencies
        ? indexData.dependencies.length
        : 0,
      // Check for mod loader specific info
      modLoader: {
        type: modLoaderType,
        version: modLoaderVersion,
      },
    };

    console.log(`[PARSE] Pack Statistics:`);
    console.log(`[PARSE]   - Total Files: ${packInfo.totalFiles}`);
    console.log(`[PARSE]   - Dependencies: ${packInfo.totalDependencies}`);
    console.log(
      `[PARSE]   - Mod Loader: ${packInfo.modLoader.type} ${packInfo.modLoader.version}`
    );
    console.log(`[PARSE]   - Game Version: ${packInfo.gameVersion}`);
    console.log(`[PARSE]   - Pack Name: ${packInfo.name}`);
    console.log(`[PARSE]   - Pack Version: ${packInfo.version}`);
    console.log(`[PARSE]   - Format Version: ${packInfo.formatVersion}`);
    
    // Log dependencies for debugging
    if (indexData.dependencies && indexData.dependencies.length > 0) {
      console.log(`[PARSE] Dependencies found:`);
      indexData.dependencies.forEach((dep, index) => {
        console.log(`[PARSE]   ${index + 1}. ${dep.id} (${dep.version})`);
      });
    }

    // Try to find additional metadata files
    console.log("[PARSE] Checking for overrides...");
    const overridesEntry = entries.find((entry) =>
      entry.entryName.startsWith("overrides/")
    );
    if (overridesEntry) {
      packInfo.hasOverrides = true;
      console.log("[PARSE] Found overrides directory");
    } else {
      console.log("[PARSE] No overrides found");
    }

    // Look for manifest files
    console.log("[PARSE] Looking for manifest.json...");
    const manifestEntry = entries.find(
      (entry) => entry.entryName === "manifest.json"
    );
    if (manifestEntry) {
      try {
        const manifestContent = manifestEntry.getData().toString("utf8");
        const manifestData = JSON.parse(manifestContent);
        packInfo.manifest = manifestData;
        console.log("[PARSE] Found and parsed manifest.json");
      } catch (err) {
        console.log("[PARSE] Could not parse manifest.json:", err.message);
      }
    } else {
      console.log("[PARSE] No manifest.json found");
    }

    // Analyze file types
    console.log("[PARSE] Analyzing file types...");
    const fileTypes = {};
    packInfo.files.forEach((file) => {
      const ext = path.extname(file.path).toLowerCase();
      fileTypes[ext] = (fileTypes[ext] || 0) + 1;
    });
    console.log("[PARSE] File type breakdown:", fileTypes);

    console.log("[PARSE] .mrpack parsing completed successfully");
    console.log("[PARSE] Final pack info:", {
      name: packInfo.name,
      version: packInfo.version,
      gameVersion: packInfo.gameVersion,
      modLoader: packInfo.modLoader,
      totalFiles: packInfo.totalFiles,
      hasOverrides: packInfo.hasOverrides,
    });

    return { success: true, data: packInfo };
  } catch (error) {
    console.error("Error parsing .mrpack:", error);
    return { success: false, error: error.message };
  }
});

// Extract .mrpack to temporary directory
ipcMain.handle("extract-mrpack", async (event, options) => {
  try {
    const { mrpackPath, extractPath } = options;
    console.log("Extracting .mrpack:", mrpackPath, "to:", extractPath);

    // Ensure extract directory exists
    await fs.mkdir(extractPath, { recursive: true });

    // Extract using AdmZip
    const zip = new AdmZip(mrpackPath);
    zip.extractAllTo(extractPath, true);

    console.log("Extraction completed");
    return { success: true, path: extractPath };
  } catch (error) {
    console.error("Error extracting .mrpack:", error);
    return { success: false, error: error.message };
  }
});

// Detect installed launchers
ipcMain.handle("detect-launchers", async () => {
  try {
    const homeDir = os.homedir();
    console.log("🔍 Detecting launchers in home directory:", homeDir);
    const launchers = [];

    // Check for XMCL
    const xmclPaths = [
      path.join(homeDir, ".xmcl"),
      path.join(homeDir, "AppData", "Roaming", "xmcl"),
    ];
    console.log("🔍 Checking XMCL paths:", xmclPaths);

    for (const xmclPath of xmclPaths) {
      try {
        await fs.access(xmclPath);
        console.log("✅ Found XMCL at:", xmclPath);
        launchers.push({
          name: "XMCL",
          id: "xmcl",
          path: xmclPath,
          type: "official",
        });
        break;
      } catch (err) {
        console.log("XMCL path not found:", xmclPath, err.message);
        // Path doesn't exist, continue
      }
    }

    // Check for Prism Launcher (Windows first, then Linux paths)
    const prismPaths = [
      path.join(homeDir, "AppData", "Roaming", "PrismLauncher"),
      path.join(homeDir, ".local", "share", "PrismLauncher"),
      path.join(homeDir, ".minecraft", "PrismLauncher"),
    ];
    console.log("🔍 Checking Prism paths:", prismPaths);

    for (const prismPath of prismPaths) {
      try {
        await fs.access(prismPath);
        console.log("✅ Found Prism at:", prismPath);
        launchers.push({
          name: "Prism Launcher",
          id: "prism",
          path: prismPath,
          type: "cracked",
        });
        break;
      } catch (err) {
        console.log("Prism path not found:", prismPath, err.message);
        // Path doesn't exist, continue
      }
    }

    // Check for AstralRinth
    const astralrinthPaths = [
      path.join(homeDir, "AppData", "Roaming", "AstralRinthApp"),
      path.join(homeDir, ".astralrinth"),
      path.join(homeDir, ".config", "astralrinth"),
    ];
    console.log("🔍 Checking AstralRinth paths:", astralrinthPaths);

    for (const astralrinthPath of astralrinthPaths) {
      try {
        await fs.access(astralrinthPath);
        console.log("✅ Found AstralRinth at:", astralrinthPath);
        launchers.push({
          name: "AstralRinth",
          id: "astralrinth",
          path: astralrinthPath,
          type: "astralrinth",
        });
        break;
      } catch (err) {
        console.log(
          "AstralRinth path not found:",
          astralrinthPath,
          err.message
        );
        // Path doesn't exist, continue
      }
    }

    // Check for ModRinth
    const modrinthPaths = [
      path.join(homeDir, "AppData", "Roaming", "ModrinthApp"),
      path.join(homeDir, "AppData", "Roaming", "modrinth"),
      path.join(homeDir, ".modrinth"),
      path.join(homeDir, ".config", "modrinth"),
    ];
    console.log("🔍 Checking ModRinth paths:", modrinthPaths);

    for (const modrinthPath of modrinthPaths) {
      try {
        await fs.access(modrinthPath);
        console.log("✅ Found ModRinth at:", modrinthPath);
        launchers.push({
          name: "ModRinth",
          id: "modrinth",
          path: modrinthPath,
          type: "modrinth",
        });
        break;
      } catch {
        console.log("❌ ModRinth path not found:", modrinthPath);
        // Path doesn't exist, continue
      }
    }

    console.log("🎮 Detected launchers:", launchers);
    return { success: true, launchers };
  } catch (error) {
    console.error("Error detecting launchers:", error);
    return { success: false, error: error.message };
  }
});

// Create instance for XMCL
ipcMain.handle("create-xmcl-instance", async (event, options) => {
  try {
    const { mrpackPath, instanceName } = options;
    console.log("Creating XMCL instance:", instanceName);

    // Get XMCL instances directory
    const homeDir = os.homedir();
    const xmclInstancesPath = path.join(
      homeDir,
      ".xmcl",
      "instances",
      instanceName
    );

    // Ensure instances directory exists
    await fs.mkdir(xmclInstancesPath, { recursive: true });

    // Extract .mrpack to instance directory
    const zip = new AdmZip(mrpackPath);
    zip.extractAllTo(xmclInstancesPath, true);

    // Create XMCL instance configuration
    const instanceConfig = {
      name: instanceName,
      version: "1.20.1",
      loader: "fabric", // Default, will be updated from .mrpack
      created: new Date().toISOString(),
      lastPlayed: null,
      playtime: 0,
      notes: "Created by NAHA MC Helper",
    };

    const configPath = path.join(xmclInstancesPath, "instance.json");
    await fs.writeFile(configPath, JSON.stringify(instanceConfig, null, 2));

    console.log("XMCL instance created successfully:", xmclInstancesPath);
    return { success: true, path: xmclInstancesPath };
  } catch (error) {
    console.error("Error creating XMCL instance:", error);
    return { success: false, error: error.message };
  }
});

// Create instance for Prism Launcher
ipcMain.handle("create-prism-instance", async (event, options) => {
  try {
    const { mrpackPath, instanceName } = options;
    console.log("Creating Prism Launcher instance:", instanceName);

    // Get Prism instances directory
    const homeDir = os.homedir();
    const prismInstancesPath = path.join(
      homeDir,
      "AppData",
      "Roaming",
      "PrismLauncher",
      "instances",
      instanceName
    );
    const prismMinecraftPath = path.join(prismInstancesPath, "minecraft");

    // Ensure instances directory exists
    await fs.mkdir(prismMinecraftPath, { recursive: true });

    // Extract .mrpack to minecraft subdirectory
    const zip = new AdmZip(mrpackPath);
    zip.extractAllTo(prismMinecraftPath, true);

    // Create Prism instance configuration
    const instanceConfig = {
      name: instanceName,
      iconKey: "grass",
      icon: "default",
      created: new Date().toISOString(),
      lastPlayed: null,
      playtime: 0,
      notes: "Created by NAHA MC Helper",
    };

    const configPath = path.join(prismInstancesPath, "instance.cfg");
    const configContent = Object.entries(instanceConfig)
      .map(([key, value]) => `${key}=${value}`)
      .join("\n");

    await fs.writeFile(configPath, configContent);

    console.log("Prism instance created successfully:", prismInstancesPath);
    return { success: true, path: prismInstancesPath };
  } catch (error) {
    console.error("Error creating Prism instance:", error);
    return { success: false, error: error.message };
  }
});

// Create instance for AstralRinth
ipcMain.handle("create-astralrinth-instance", async (event, options) => {
  try {
    const { mrpackPath, instanceName } = options;
    console.log("Creating AstralRinth instance:", instanceName);

    // Get AstralRinth profiles directory
    const homeDir = os.homedir();
    const astralrinthProfilesPath = path.join(
      homeDir,
      "AppData",
      "Roaming",
      "AstralRinthApp",
      "profiles",
      instanceName
    );

    // Ensure profiles directory exists
    await fs.mkdir(astralrinthProfilesPath, { recursive: true });

    // Extract .mrpack to profile directory
    const zip = new AdmZip(mrpackPath);
    zip.extractAllTo(astralrinthProfilesPath, true);

    // Create AstralRinth profile configuration
    const profileConfig = {
      name: instanceName,
      created: new Date().toISOString(),
      lastPlayed: null,
      playtime: 0,
      notes: "Created by NAHA MC Helper",
    };

    const configPath = path.join(astralrinthProfilesPath, "profile.json");
    await fs.writeFile(configPath, JSON.stringify(profileConfig, null, 2));

    console.log(
      "AstralRinth instance created successfully:",
      astralrinthProfilesPath
    );
    return { success: true, path: astralrinthProfilesPath };
  } catch (error) {
    console.error("Error creating AstralRinth instance:", error);
    return { success: false, error: error.message };
  }
});

// Minecraft Installer IPC Handlers

// Check if installer is available
ipcMain.handle("installer-is-available", async () => {
  try {
    return await minecraftInstaller.isAvailable();
  } catch (error) {
    console.error("Error checking installer availability:", error);
    return false;
  }
});

// Get installer version
ipcMain.handle("installer-get-version", async () => {
  try {
    return await minecraftInstaller.getVersion();
  } catch (error) {
    console.error("Error getting installer version:", error);
    return { available: false, error: error.message };
  }
});

// List detected launchers
ipcMain.handle("installer-list-launchers", async () => {
  try {
    return await minecraftInstaller.listLaunchers();
  } catch (error) {
    console.error("Error listing launchers:", error);
    return { success: false, error: error.message };
  }
});

// Download and install NeoForge modpack
ipcMain.handle("installer-download-neoforge", async (event, targetLauncher, createInstance, onProgress, customPath) => {
  try {
    // Set up progress callback
    const progressCallback = (progressData) => {
      // Clean up the data for prettier logging
      let cleanData = progressData.data;
      if (cleanData) {
        // Remove ANSI escape sequences
        // eslint-disable-next-line no-control-regex
        cleanData = cleanData.replace(/\x1b\[[0-9;]*[mK]/g, ''); // Remove ANSI color codes
        // eslint-disable-next-line no-control-regex
        cleanData = cleanData.replace(/\x1b\[[0-9;]*[A-Za-z]/g, ''); // Remove other ANSI sequences
        cleanData = cleanData.replace(/^\s*[\d-]+\s+[\d:]+\s+Z\s+INFO\s+\w+::\w+\s*:\s*/, ''); // Remove timestamp and log level
        cleanData = cleanData.trim();
        
        console.log(`📦 [INSTALLER] ${cleanData}`);
      }
      event.sender.send('installer-progress', progressData);
    };

    return await minecraftInstaller.downloadNeoForge(
      targetLauncher, 
      createInstance !== false,
      progressCallback,
      customPath
    );
  } catch (error) {
    console.error("Error downloading NeoForge:", error);
    return { success: false, error: error.message };
  }
});

// Download and install Fabric modpack
ipcMain.handle("installer-download-fabric", async (event, targetLauncher, createInstance, onProgress, customPath) => {
  try {
    // Set up progress callback
    const progressCallback = (progressData) => {
      // Clean up the data for prettier logging
      let cleanData = progressData.data;
      if (cleanData) {
        // Remove ANSI escape sequences
        // eslint-disable-next-line no-control-regex
        cleanData = cleanData.replace(/\x1b\[[0-9;]*[mK]/g, ''); // Remove ANSI color codes
        // eslint-disable-next-line no-control-regex
        cleanData = cleanData.replace(/\x1b\[[0-9;]*[A-Za-z]/g, ''); // Remove other ANSI sequences
        cleanData = cleanData.replace(/^\s*[\d-]+\s+[\d:]+\s+Z\s+INFO\s+\w+::\w+\s*:\s*/, ''); // Remove timestamp and log level
        cleanData = cleanData.trim();
        
        console.log(`🧵 [INSTALLER] ${cleanData}`);
      }
      event.sender.send('installer-progress', progressData);
    };

    return await minecraftInstaller.downloadFabric(
      targetLauncher, 
      createInstance !== false,
      progressCallback,
      customPath
    );
  } catch (error) {
    console.error("Error downloading Fabric:", error);
    return { success: false, error: error.message };
  }
});

// Install local mrpack file
ipcMain.handle("installer-install-mrpack", async (event, options) => {
  try {
    const { mrpackPath, targetLauncher, createInstance } = options;

    return await minecraftInstaller.installMrpack(
      mrpackPath,
      targetLauncher, 
      createInstance !== false
    );
  } catch (error) {
    console.error("Error installing mrpack:", error);
    return { success: false, error: error.message };
  }
});

// List available Minecraft versions
ipcMain.handle("installer-list-versions", async (event, options) => {
  try {
    const { versionType } = options || {};
    return await minecraftInstaller.listVersions(versionType);
  } catch (error) {
    console.error("Error listing versions:", error);
    return { success: false, error: error.message };
  }
});

// Install specific Minecraft version
ipcMain.handle("installer-install-minecraft", async (event, options) => {
  try {
    const { version, loader, loaderVersion, force } = options;

    return await minecraftInstaller.installMinecraft(
      version,
      loader || 'vanilla',
      loaderVersion || 'stable',
      force || false
    );
  } catch (error) {
    console.error("Error installing Minecraft:", error);
    return { success: false, error: error.message };
  }
});

// Auto-Updater IPC Handlers

// Check for updates
ipcMain.handle("auto-updater:check-for-updates", async () => {
  try {
    if (autoUpdater) {
      await autoUpdater.checkForUpdates();
      return { success: true };
    }
    return { success: false, error: "Auto-updater not initialized" };
  } catch (error) {
    console.error("Error checking for updates:", error);
    return { success: false, error: error.message };
  }
});

// Manual update check with user feedback
ipcMain.handle("auto-updater:manual-update-check", async () => {
  try {
    if (autoUpdater) {
      await autoUpdater.manualUpdateCheck();
      return { success: true };
    }
    return { success: false, error: "Auto-updater not initialized" };
  } catch (error) {
    console.error("Error in manual update check:", error);
    return { success: false, error: error.message };
  }
});

// Download update
ipcMain.handle("auto-updater:download-update", async (event, updateInfo) => {
  try {
    if (autoUpdater) {
      // If updateInfo is provided, set it on the service
      if (updateInfo) {
        console.log('Main: Setting updateInfo on service:', updateInfo);
        autoUpdater.updateAvailable = true;
        autoUpdater.updateInfo = updateInfo;
      }
      await autoUpdater.downloadUpdate();
      return { success: true };
    }
    return { success: false, error: "Auto-updater not initialized" };
  } catch (error) {
    console.error("Error downloading update:", error);
    return { success: false, error: error.message };
  }
});

// Install update
ipcMain.handle("auto-updater:install-update", async () => {
  try {
    if (autoUpdater) {
      await autoUpdater.installUpdate();
      return { success: true };
    }
    return { success: false, error: "Auto-updater not initialized" };
  } catch (error) {
    console.error("Error installing update:", error);
    return { success: false, error: error.message };
  }
});

// Get current version
ipcMain.handle("auto-updater:get-current-version", async () => {
  try {
    if (autoUpdater) {
      return { success: true, version: autoUpdater.getCurrentVersion() };
    }
    return { success: false, error: "Auto-updater not initialized" };
  } catch (error) {
    console.error("Error getting current version:", error);
    return { success: false, error: error.message };
  }
});

// Get update info
ipcMain.handle("auto-updater:get-update-info", async () => {
  try {
    if (autoUpdater) {
      const info = autoUpdater.getUpdateInfo();
      return { success: true, updateInfo: info.updateInfo, ...info };
    }
    return { success: false, error: "Auto-updater not initialized" };
  } catch (error) {
    console.error("Error getting update info:", error);
    return { success: false, error: error.message };
  }
});

// Start periodic update checks
ipcMain.handle("auto-updater:start-periodic-check", async () => {
  try {
    if (autoUpdater) {
      autoUpdater.startPeriodicUpdateCheck();
      return { success: true };
    }
    return { success: false, error: "Auto-updater not initialized" };
  } catch (error) {
    console.error("Error starting periodic check:", error);
    return { success: false, error: error.message };
  }
});

// Stop periodic update checks
ipcMain.handle("auto-updater:stop-periodic-check", async () => {
  try {
    if (autoUpdater) {
      autoUpdater.stopPeriodicUpdateCheck();
      return { success: true };
    }
    return { success: false, error: "Auto-updater not initialized" };
  } catch (error) {
    console.error("Error stopping periodic check:", error);
    return { success: false, error: error.message };
  }
});
