/**
 * Minecraft Binaries Utility
 * Handles paths and execution of minecraft-installer and minecraft-updater binaries
 */

import path from 'path';
import { spawn } from 'child_process';
import fs from 'fs';

/**
 * Get the path to the binaries directory based on environment
 */
export function getBinariesDir() {
  // In Electron development, resourcesPath points to electron's own resources
  // In production, it points to the packaged app's resources
  // Better check: see if we can find the tools directory
  const devPath = path.join(process.cwd(), 'tools', 'minecraft-installer-releases');
  const prodPath = process.resourcesPath ? path.join(process.resourcesPath, 'binaries') : devPath;
  
  // Check if development path exists
  if (fs.existsSync(devPath)) {
    console.log('🔍 [BINARIES] Using development path:', devPath);
    return devPath;
  } else {
    console.log('🔍 [BINARIES] Using production path:', prodPath);
    return prodPath;
  }
}

/**
 * Get the platform-specific binary name
 */
export function getBinaryName(baseName) {
  const platform = process.platform;
  
  switch (platform) {
    case 'win32':
      // Use MSVC version for Windows (recommended)
      return `${baseName}-windows-x86_64.exe`;
    case 'darwin': {
      // Detect Apple Silicon vs Intel
      const arch = process.arch;
      if (arch === 'arm64') {
        return `${baseName}-macos-apple-silicon-aarch64`;
      } else {
        return `${baseName}-macos-intel-x86_64`;
      }
    }
    case 'linux':
      return `${baseName}-linux-x86_64`;
    default:
      throw new Error(`Unsupported platform: ${platform}`);
  }
}

/**
 * Get the full path to a binary
 */
export function getBinaryPath(binaryType) {
  const binariesDir = getBinariesDir();
  const binaryName = getBinaryName(binaryType);
  return path.join(binariesDir, binaryName);
}

/**
 * Spawn a Minecraft binary with arguments
 */
export function spawnMinecraftBinary(binaryType, args) {
  const binaryPath = getBinaryPath(binaryType);
  console.log('🚀 [BINARIES] Spawning:', binaryPath);
  console.log('🚀 [BINARIES] Args:', args);
  
  // Verify file exists
  if (!fs.existsSync(binaryPath)) {
    const error = new Error(`Binary not found: ${binaryPath}`);
    console.error('❌ [BINARIES]', error.message);
    throw error;
  }
  
  // Check file stats
  try {
    const stats = fs.statSync(binaryPath);
    console.log('📊 [BINARIES] File size:', (stats.size / 1024 / 1024).toFixed(2), 'MB');
  } catch (e) {
    console.warn('⚠️ [BINARIES] Could not read file stats:', e.message);
  }
  
  // On Windows, spawn .exe files directly
  const spawnOptions = {
    windowsVerbatimArguments: false,
    shell: false
  };
  
  try {
    const proc = spawn(binaryPath, args, spawnOptions);
    
    proc.on('error', (err) => {
      console.error('❌ [BINARIES] Process error:', err);
      console.error('   Error code:', err.code);
      console.error('   Error message:', err.message);
      if (err.code === 'UNKNOWN' || err.errno === -4094) {
        console.error('   💡 TIP: The executable might be blocked by Windows. Try:');
        console.error('       1. Right-click the file → Properties → Unblock');
        console.error('       2. Or run: powershell -c "Unblock-File \'' + binaryPath + '\'"');
      }
    });
    
    console.log('✅ [BINARIES] Spawned successfully');
    return proc;
  } catch (error) {
    console.error('❌ [BINARIES] Spawn failed:', error);
    throw error;
  }
}

/**
 * Execute a Minecraft binary and return JSON output
 */
export function executeMinecraftBinary(binaryType, args) {
  return new Promise((resolve, reject) => {
    const proc = spawnMinecraftBinary(binaryType, args);
    let stdout = '';
    let stderr = '';

    proc.stdout?.on('data', (data) => {
      stdout += data.toString();
    });

    proc.stderr?.on('data', (data) => {
      stderr += data.toString();
    });

    proc.on('close', (code) => {
      if (code === 0) {
        try {
          // Try to parse as JSON
          const result = JSON.parse(stdout);
          resolve(result);
        } catch {
          // If not JSON, return raw output
          resolve({ success: true, output: stdout });
        }
      } else {
        reject(new Error(`Process exited with code ${code}: ${stderr}`));
      }
    });

    proc.on('error', (err) => {
      reject(err);
    });
  });
}

/**
 * Scan all Minecraft instances
 */
export async function scanMinecraftInstances() {
  return executeMinecraftBinary('minecraft-updater', ['scan', '--format', 'json']);
}

/**
 * Update a Minecraft instance
 */
export async function updateMinecraftInstance(instancePath, modpackType, version) {
  const args = [
    'update',
    '--instance-path', instancePath,
    '--modpack-type', modpackType,
    '--format', 'json'
  ];
  
  if (version) {
    args.push('--version', version);
  }
  
  return executeMinecraftBinary('minecraft-updater', args);
}

/**
 * Install a modpack from mrpack file
 */
export function installFromMrpack(mrpackPath, targetLauncher, customPath) {
  const args = ['--mrpack', mrpackPath, '--create-instance'];
  
  if (targetLauncher) {
    args.push('--target-launcher', targetLauncher);
  }
  
  if (customPath) {
    args.push('--custom-path', customPath);
  }
  
  return spawnMinecraftBinary('minecraft-installer', args);
}

/**
 * Download and install from GitHub API
 */
export function installFromGitHub(modpackType, targetLauncher, customPath) {
  const args = [`--download-${modpackType}`, '--create-instance'];
  
  if (targetLauncher) {
    args.push('--target-launcher', targetLauncher);
  }
  
  if (customPath) {
    args.push('--custom-path', customPath);
  }
  
  return spawnMinecraftBinary('minecraft-installer', args);
}

/**
 * List detected launchers
 */
export async function listLaunchers() {
  return new Promise((resolve, reject) => {
    const proc = spawnMinecraftBinary('minecraft-installer', ['--list-launchers']);
    let output = '';
    
    proc.stdout?.on('data', (data) => {
      output += data.toString();
    });
    
    proc.on('close', (code) => {
      if (code === 0) {
        resolve(output);
      } else {
        reject(new Error(`Failed to list launchers: ${code}`));
      }
    });
  });
}

/**
 * Check if binaries exist
 */
export function checkBinariesExist() {
  const installerPath = getBinaryPath('minecraft-installer');
  const updaterPath = getBinaryPath('minecraft-updater');
  
  console.log('🔍 [BINARIES] Checking paths:');
  console.log('   Installer:', installerPath, '→', fs.existsSync(installerPath) ? '✅' : '❌');
  console.log('   Updater:', updaterPath, '→', fs.existsSync(updaterPath) ? '✅' : '❌');
  
  return {
    installer: fs.existsSync(installerPath),
    updater: fs.existsSync(updaterPath)
  };
}

