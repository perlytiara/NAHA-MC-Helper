/**
 * Minecraft Updater Service
 * Provides integration with the minecraft-updater executable
 */

const { spawn } = require('child_process');
const path = require('path');

class MinecraftUpdaterService {
  constructor() {
    this.platform = process.platform;
    this.binDir = this.getBinDirectory();
    this.updaterPath = this.getUpdaterPath();
    this.installerPath = this.getInstallerPath();
  }

  getBinDirectory() {
    // In development, binaries are in tools/minecraft-installer-releases/
    // In production, they're in resources/installer/
    const isDev = process.env.NODE_ENV === 'development' || !process.resourcesPath;
    
    if (isDev) {
      return path.join(__dirname, '../../../tools/minecraft-installer-releases');
    } else {
      return path.join(process.resourcesPath, 'installer');
    }
  }

  getUpdaterPath() {
    const binaryName = this.platform === 'win32' 
      ? 'minecraft-updater-windows.exe'
      : 'minecraft-updater-linux';
    
    return path.join(this.binDir, binaryName);
  }

  getInstallerPath() {
    const binaryName = this.platform === 'win32'
      ? 'minecraft-installer-windows.exe'
      : 'minecraft-installer-linux';
    
    return path.join(this.binDir, binaryName);
  }

  /**
   * Scan for existing Minecraft instances
   * @param {string} format - Output format: 'json', 'compact', 'pretty'
   * @returns {Promise<Object>} Scan results
   */
  async scanInstances(format = 'json') {
    return new Promise((resolve, reject) => {
      console.log(`🔍 Scanning instances with: ${this.updaterPath}`);
      
      const proc = spawn(this.updaterPath, ['scan', '--format', format], {
        stdio: ['ignore', 'pipe', 'pipe']
      });

      let stdout = '';
      let stderr = '';

      proc.stdout.on('data', (data) => {
        stdout += data.toString();
      });

      proc.stderr.on('data', (data) => {
        stderr += data.toString();
      });

      proc.on('close', (code) => {
        if (code === 0) {
          try {
            if (format === 'json') {
              const result = JSON.parse(stdout);
              resolve({ success: true, instances: result });
            } else {
              resolve({ success: true, output: stdout });
            }
          } catch (error) {
            console.error('Failed to parse scan output:', error);
            resolve({ success: true, output: stdout });
          }
        } else {
          console.error('Scan failed:', stderr);
          reject(new Error(`Scanner exited with code ${code}: ${stderr}`));
        }
      });

      proc.on('error', (error) => {
        console.error('Failed to start scanner:', error);
        reject(new Error(`Failed to start scanner: ${error.message}`));
      });
    });
  }

  /**
   * Update a specific instance
   * @param {string} instancePath - Path to the instance directory
   * @param {string} modpackType - Type of modpack: 'neoforge' or 'fabric'
   * @param {string} version - Specific version to update to (optional)
   * @returns {Promise<Object>} Update results
   */
  async updateInstance(instancePath, modpackType, version = null) {
    return new Promise((resolve, reject) => {
      console.log(`🔄 Updating instance: ${instancePath}`);
      
      const args = [
        'update',
        '--instance-path', instancePath,
        '--modpack-type', modpackType,
        '--format', 'json'
      ];

      if (version) {
        args.push('--version', version);
      }

      const proc = spawn(this.updaterPath, args, {
        stdio: ['ignore', 'pipe', 'pipe']
      });

      let stdout = '';
      let stderr = '';

      proc.stdout.on('data', (data) => {
        stdout += data.toString();
        // Also emit progress events
        const line = data.toString().trim();
        if (line.startsWith('🔄') || line.startsWith('✅') || line.startsWith('📦')) {
          console.log('Update progress:', line);
        }
      });

      proc.stderr.on('data', (data) => {
        stderr += data.toString();
      });

      proc.on('close', (code) => {
        if (code === 0) {
          try {
            const result = JSON.parse(stdout);
            resolve({ success: true, ...result });
          } catch (error) {
            console.error('Failed to parse update output:', error);
            resolve({ success: true, output: stdout });
          }
        } else {
          console.error('Update failed:', stderr);
          reject(new Error(`Update failed with code ${code}: ${stderr}`));
        }
      });

      proc.on('error', (error) => {
        console.error('Failed to start updater:', error);
        reject(new Error(`Failed to start updater: ${error.message}`));
      });
    });
  }

  /**
   * Install a new modpack
   * @param {string} modpackType - Type of modpack: 'neoforge' or 'fabric'
   * @param {string} targetLauncher - Specific launcher to target (optional)
   * @returns {Promise<Object>} Installation results
   */
  async installModpack(modpackType, targetLauncher = null) {
    return new Promise((resolve, reject) => {
      console.log(`📦 Installing ${modpackType} modpack...`);
      
      const args = [
        `--download-${modpackType}`,
        '--create-instance'
      ];

      if (targetLauncher) {
        args.push('--target-launcher', targetLauncher);
      }

      const proc = spawn(this.installerPath, args, {
        stdio: ['ignore', 'pipe', 'pipe']
      });

      let stdout = '';
      let stderr = '';

      proc.stdout.on('data', (data) => {
        stdout += data.toString();
        // Emit progress events
        const line = data.toString().trim();
        if (line.startsWith('📦') || line.startsWith('✅') || line.startsWith('🔍')) {
          console.log('Install progress:', line);
        }
      });

      proc.stderr.on('data', (data) => {
        stderr += data.toString();
      });

      proc.on('close', (code) => {
        if (code === 0) {
          resolve({ success: true, output: stdout });
        } else {
          console.error('Installation failed:', stderr);
          reject(new Error(`Installation failed with code ${code}: ${stderr}`));
        }
      });

      proc.on('error', (error) => {
        console.error('Failed to start installer:', error);
        reject(new Error(`Failed to start installer: ${error.message}`));
      });
    });
  }

  /**
   * Get available launchers
   * @returns {Promise<Object>} Launcher information
   */
  async listLaunchers() {
    return new Promise((resolve, reject) => {
      console.log('🔍 Listing available launchers...');
      
      const proc = spawn(this.installerPath, ['--list-launchers'], {
        stdio: ['ignore', 'pipe', 'pipe']
      });

      let stdout = '';
      let stderr = '';

      proc.stdout.on('data', (data) => {
        stdout += data.toString();
      });

      proc.stderr.on('data', (data) => {
        stderr += data.toString();
      });

      proc.on('close', (code) => {
        if (code === 0) {
          resolve({ success: true, output: stdout });
        } else {
          console.error('Failed to list launchers:', stderr);
          reject(new Error(`Failed to list launchers: ${stderr}`));
        }
      });

      proc.on('error', (error) => {
        console.error('Failed to start launcher lister:', error);
        reject(new Error(`Failed to start launcher lister: ${error.message}`));
      });
    });
  }

  /**
   * Check if the updater binary exists
   * @returns {boolean} True if binary exists
   */
  checkBinaryExists() {
    const fs = require('fs');
    return fs.existsSync(this.updaterPath);
  }

  /**
   * Check if the installer binary exists
   * @returns {boolean} True if binary exists
   */
  checkInstallerExists() {
    const fs = require('fs');
    return fs.existsSync(this.installerPath);
  }

  /**
   * Get binary status
   * @returns {Object} Status of both binaries
   */
  getBinaryStatus() {
    return {
      updater: {
        path: this.updaterPath,
        exists: this.checkBinaryExists()
      },
      installer: {
        path: this.installerPath,
        exists: this.checkInstallerExists()
      },
      platform: this.platform,
      binDir: this.binDir
    };
  }
}

// Export singleton instance
const minecraftUpdaterService = new MinecraftUpdaterService();
module.exports = minecraftUpdaterService;
