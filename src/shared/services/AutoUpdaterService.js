import electron from 'electron';
import electronUpdater from 'electron-updater';
import os from 'os';
import path from 'path';
import fs from 'fs';
import { spawn } from 'child_process';

const { app, dialog, shell } = electron;
const { autoUpdater } = electronUpdater;

class AutoUpdaterService {
  constructor(mainWindow) {
    this.mainWindow = mainWindow;
    this.updateAvailable = false;
    this.updateDownloaded = false;
    this.currentVersion = app.getVersion();
    this.updateCheckInterval = null;
    
    this.setupAutoUpdater();
  }

  setupAutoUpdater() {
    // Configure auto-updater
    autoUpdater.setFeedURL({
      provider: 'github',
      owner: 'perlytiara',
      repo: 'NAHA-MC-Helper'
    });

    // Disable auto-download to show custom UI
    autoUpdater.autoDownload = false;
    autoUpdater.autoInstallOnAppQuit = false;
    
    console.log('Auto-updater: Configured for GitHub repository perlytiara/NAHA-MC-Helper');

    // Event listeners
    autoUpdater.on('checking-for-update', () => {
      this.sendToRenderer('update-checking');
    });

    autoUpdater.on('update-available', (info) => {
      console.log('Update available:', info);
      this.updateAvailable = true;
      this.sendToRenderer('update-available', {
        version: info.version,
        releaseNotes: info.releaseNotes,
        releaseDate: info.releaseDate
      });
    });

    autoUpdater.on('update-not-available', (info) => {
      console.log('Update not available:', info);
      this.sendToRenderer('update-not-available', {
        currentVersion: this.currentVersion
      });
    });

    autoUpdater.on('download-progress', (progressObj) => {
      this.sendToRenderer('download-progress', {
        percent: Math.round(progressObj.percent),
        transferred: progressObj.transferred,
        total: progressObj.total,
        bytesPerSecond: progressObj.bytesPerSecond
      });
    });

    autoUpdater.on('update-downloaded', (info) => {
      console.log('Update downloaded:', info);
      this.updateDownloaded = true;
      this.sendToRenderer('update-downloaded', {
        version: info.version,
        releaseNotes: info.releaseNotes
      });
    });

    autoUpdater.on('error', (error) => {
      console.error('Auto-updater error:', error);
      this.sendToRenderer('update-error', {
        message: error.message,
        stack: error.stack
      });
    });
  }

  async checkForUpdates() {
    try {
      console.log('Auto-updater: Starting update check...');
      console.log('Auto-updater: App packaged:', app.isPackaged);
      console.log('Auto-updater: Current version:', this.currentVersion);
      
      // Always use GitHub API directly (more reliable than electron-updater)
      console.log('Auto-updater: Using GitHub API directly for update check');
      await this.checkGitHubAPI();
      console.log('Auto-updater: Update check completed');
    } catch (error) {
      console.error('Auto-updater: Error checking for updates:', error);
      this.sendToRenderer('update-error', {
        message: 'Failed to check for updates',
        details: error.message
      });
    }
  }

  // Direct GitHub API check as fallback
  async checkGitHubAPI() {
    try {
      console.log('Auto-updater: Checking GitHub API directly...');
      
      // Use fetch API which is more reliable than Node.js https
      // eslint-disable-next-line no-undef
      const response = await fetch('https://api.github.com/repos/perlytiara/NAHA-MC-Helper/releases/latest', {
        headers: {
          'User-Agent': 'NAHA-MC-Helper-Update-Checker',
          'Accept': 'application/vnd.github.v3+json'
        }
      });

      if (!response.ok) {
        throw new Error(`GitHub API error: ${response.status} ${response.statusText}`);
      }

      const data = await response.json();

      const latestVersion = data.tag_name.replace('v', '');
      console.log('Auto-updater: GitHub API - Latest version:', latestVersion);
      console.log('Auto-updater: GitHub API - Current version:', this.currentVersion);

      // Compare versions properly
      const isUpdateAvailable = this.isVersionNewer(latestVersion, this.currentVersion);
      console.log('Auto-updater: Is update available?', isUpdateAvailable);

      if (isUpdateAvailable) {
        console.log('Auto-updater: Update available via GitHub API');
        
        // Parse release notes to extract key features
        const releaseNotes = this.parseReleaseNotes(data.body || 'Update available');
        
        // Create update info object
        const updateInfo = {
          version: latestVersion,
          releaseNotes: releaseNotes,
          releaseName: data.name || data.tag_name,
          releaseDate: data.published_at
        };

        this.updateAvailable = true;
        this.updateInfo = updateInfo;
        
        console.log('Auto-updater: Sending update-available event to renderer:', updateInfo);
        
        // In production, also trigger electron-updater to check
        if (app.isPackaged) {
          console.log('Auto-updater: Production mode - also checking with electron-updater');
          try {
            await autoUpdater.checkForUpdates();
          } catch {
            console.log('Auto-updater: electron-updater check failed, continuing with GitHub API info');
          }
        }
        
        // Send events to renderer
        this.sendToRenderer('update-available', updateInfo);
      } else {
        console.log('Auto-updater: No updates available via GitHub API');
        this.sendToRenderer('update-not-available', {
          version: this.currentVersion,
          releaseNotes: 'You are running the latest version'
        });
      }

    } catch (error) {
      console.error('Auto-updater: GitHub API check failed:', error);
      this.sendToRenderer('update-error', {
        message: 'Failed to check for updates via GitHub API',
        details: error.message
      });
    }
  }

  async downloadUpdate() {
    if (this.updateAvailable && this.updateInfo) {
      try {
        console.log('🚀 Auto-updater: Download update requested');
        console.log('🚀 Auto-updater: Update info:', this.updateInfo);
        
        // Always use direct download for reliability
        console.log('🚀 Auto-updater: Downloading installer directly');
        await this.downloadInstallerInDev();
        console.log('🚀 Auto-updater: downloadInstallerInDev completed!');
        
      } catch (error) {
        console.error('❌ Auto-updater: Error downloading update:', error);
        this.sendToRenderer('update-error', {
          message: 'Failed to download update',
          details: error.message
        });
      }
    } else {
      console.error('❌ Auto-updater: Cannot download - no update available or updateInfo missing');
      console.log('updateAvailable:', this.updateAvailable);
      console.log('updateInfo:', this.updateInfo);
    }
  }

  async downloadInstallerInDev() {
    try {
      const platform = process.platform;
      const arch = process.arch;
      let fileName = '';
      
      // Determine the correct installer file based on platform
      if (platform === 'win32') {
        fileName = `NAHA.MC.Helper.Setup.${this.updateInfo.version}.exe`;
      } else if (platform === 'darwin') {
        if (arch === 'arm64') {
          fileName = `NAHA.MC.Helper-${this.updateInfo.version}-arm64.dmg`;
        } else {
          fileName = `NAHA.MC.Helper-${this.updateInfo.version}.dmg`;
        }
      } else if (platform === 'linux') {
        fileName = `NAHA.MC.Helper-${this.updateInfo.version}.AppImage`;
      }
      
      if (!fileName) {
        throw new Error(`Unsupported platform: ${platform} ${arch}`);
      }
      
      const downloadUrl = `https://github.com/perlytiara/NAHA-MC-Helper/releases/download/v${this.updateInfo.version}/${fileName}`;
      console.log('📥 Auto-updater: Downloading installer from:', downloadUrl);
      
      // Send download started event
      console.log('📤 Auto-updater: Sending download-progress (0%)');
      this.sendToRenderer('download-progress', {
        percent: 0,
        transferred: 0,
        total: 0,
        bytesPerSecond: 0,
        version: this.updateInfo.version
      });
      
      // Download the installer
      // eslint-disable-next-line no-undef
      const response = await fetch(downloadUrl);
      if (!response.ok) {
        throw new Error(`Failed to download installer: ${response.status} ${response.statusText}`);
      }
      
      const contentLength = response.headers.get('content-length');
      const total = parseInt(contentLength, 10);
      let downloaded = 0;
      
      const reader = response.body.getReader();
      const chunks = [];
      const startTime = Date.now();
      
      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        
        chunks.push(value);
        downloaded += value.length;
        
        const currentTime = Date.now();
        const elapsedTime = (currentTime - startTime) / 1000; // in seconds
        const bytesPerSecond = elapsedTime > 0 ? downloaded / elapsedTime : 0;
        
        const percent = Math.round((downloaded / total) * 100);
        this.sendToRenderer('download-progress', {
          percent: percent,
          transferred: downloaded,
          total: total,
          bytesPerSecond: bytesPerSecond,
          version: this.updateInfo.version
        });
      }
      
      // Combine chunks into a single buffer
      const buffer = new Uint8Array(downloaded);
      let position = 0;
      for (const chunk of chunks) {
        buffer.set(chunk, position);
        position += chunk.length;
      }
      
      // Save the installer to a temporary file with unique name
      const tempDir = os.tmpdir();
      const timestamp = Date.now();
      const uniqueFileName = fileName.replace(/\.exe$/, `-${timestamp}.exe`);
      const tempPath = path.join(tempDir, uniqueFileName);
      console.log('💾 Auto-updater: Saving installer to:', tempPath);
      
      fs.writeFileSync(tempPath, buffer);
      
      console.log('✅ Auto-updater: Installer downloaded successfully!');
      console.log('📤 Auto-updater: Sending update-downloaded event');
      
      // Send download completed event
      this.sendToRenderer('update-downloaded', {
        version: this.updateInfo.version,
        installerPath: tempPath,
        releaseNotes: `Update ${this.updateInfo.version} downloaded successfully. The installer will open automatically.`
      });
      
      // Install silently in background
      console.log('🚀 Auto-updater: Installing update silently in background...');
      
      if (platform === 'win32') {
        // Windows: Run NSIS installer with silent flag
        // The installer will wait for this app to close before updating
        this.sendToRenderer('update-installing', {
          version: this.updateInfo.version,
          message: 'Installing update in background...'
        });
        
        // Start the installer in detached mode so it survives after we quit
        const installer = spawn(tempPath, ['/S'], {
          detached: true,
          stdio: 'ignore'
        });
        
        // Unref so it doesn't keep the parent process alive
        installer.unref();
        
        // Wait a moment to ensure the installer starts, then prompt user to restart
        setTimeout(() => {
          console.log('✅ Auto-updater: Installer started, prompting user to restart');
          this.sendToRenderer('update-ready-to-restart', {
            version: this.updateInfo.version,
            message: 'Update ready! Restart to complete installation.'
          });
        }, 2000);
      } else {
        // macOS/Linux: Open installer (user needs to complete installation)
        const result = await shell.openPath(tempPath);
        if (result) {
          console.error('❌ Auto-updater: Failed to open installer:', result);
          this.sendToRenderer('update-error', {
            message: 'Failed to open installer',
            details: result
          });
        } else {
          console.log('✅ Auto-updater: Installer opened successfully!');
          this.sendToRenderer('update-ready-to-restart', {
            version: this.updateInfo.version,
            message: 'Complete the installation, then restart the app.'
          });
        }
      }
      
    } catch (error) {
      console.error('Auto-updater: Error downloading installer:', error);
      
      let errorMessage = 'Failed to download installer';
      if (error.code === 'EBUSY') {
        errorMessage = 'Installer file is locked. Please close any open installers and try again.';
      } else if (error.message) {
        errorMessage = error.message;
      }
      
      this.sendToRenderer('update-error', {
        message: errorMessage,
        details: error.stack || error.toString()
      });
    }
  }

  async installUpdate() {
    console.log('Auto-updater: Install update requested');
    
    if (!app.isPackaged) {
      console.log('Auto-updater: Development mode - installer should already be running');
      // In development, the installer should already be opened by downloadInstallerInDev
      this.sendToRenderer('update-installed', {
        version: this.updateInfo.version,
        message: 'The installer has been opened. Please follow the installation wizard to complete the update.'
      });
    } else {
      console.log('Auto-updater: Production mode - using electron-updater to install');
      // In production, use electron-updater to quit and install
      try {
        autoUpdater.quitAndInstall();
      } catch (error) {
        console.error('Error installing update:', error);
        this.sendToRenderer('update-error', {
          message: 'Failed to install update',
          details: error.message
        });
      }
    }
  }

  startPeriodicUpdateCheck() {
    // Check for updates every 6 hours
    // eslint-disable-next-line no-undef
    this.updateCheckInterval = setInterval(() => {
      this.checkForUpdates();
    }, 6 * 60 * 60 * 1000);
  }

  stopPeriodicUpdateCheck() {
    if (this.updateCheckInterval) {
      // eslint-disable-next-line no-undef
      clearInterval(this.updateCheckInterval);
      this.updateCheckInterval = null;
    }
  }

  sendToRenderer(channel, data) {
    if (this.mainWindow && !this.mainWindow.isDestroyed()) {
      this.mainWindow.webContents.send(`auto-updater:${channel}`, data);
    }
  }

  getCurrentVersion() {
    return this.currentVersion;
  }

  getUpdateInfo() {
    return {
      currentVersion: this.currentVersion,
      updateAvailable: this.updateAvailable,
      updateDownloaded: this.updateDownloaded,
      updateInfo: this.updateInfo
    };
  }

  // Parse release notes markdown to extract key features
  parseReleaseNotes(markdown) {
    try {
      // Extract the "What's New" section
      const whatsNewMatch = markdown.match(/##\s*✨\s*What's New[^#]*((?:- \*\*[^*]+\*\*[^\n]*\n?)+)/);
      
      if (whatsNewMatch && whatsNewMatch[1]) {
        // Extract bullet points and clean them up
        const bullets = whatsNewMatch[1]
          .split('\n')
          .filter(line => line.trim().startsWith('-'))
          .map(line => {
            // Remove markdown formatting and emoji, keep just the feature name
            return line
              .replace(/^-\s*\*\*/, '• ')  // Replace - ** with bullet
              .replace(/\*\*:.*$/, '')      // Remove **: and everything after
              .trim();
          })
          .filter(line => line.length > 0)
          .join('\n');
        
        return bullets || 'Bug fixes and improvements';
      }
      
      // Fallback: try to find any bullet points
      const bulletPoints = markdown.match(/^-\s+\*\*([^*]+)\*\*/gm);
      if (bulletPoints) {
        return bulletPoints
          .map(point => point.replace(/^-\s+\*\*/, '• ').replace(/\*\*.*$/, ''))
          .join('\n');
      }
      
      // Last resort: return first paragraph
      const firstPara = markdown.split('\n\n')[0];
      if (firstPara && firstPara.length < 200) {
        return firstPara.replace(/[#*]/g, '').trim();
      }
      
      return 'Bug fixes and improvements';
    } catch (error) {
      console.error('Error parsing release notes:', error);
      return 'Bug fixes and improvements';
    }
  }

  // Compare version strings to determine if one is newer
  isVersionNewer(version1, version2) {
    const v1Parts = version1.split('.').map(Number);
    const v2Parts = version2.split('.').map(Number);
    
    // Ensure both arrays have the same length
    const maxLength = Math.max(v1Parts.length, v2Parts.length);
    while (v1Parts.length < maxLength) v1Parts.push(0);
    while (v2Parts.length < maxLength) v2Parts.push(0);
    
    for (let i = 0; i < maxLength; i++) {
      if (v1Parts[i] > v2Parts[i]) return true;
      if (v1Parts[i] < v2Parts[i]) return false;
    }
    
    return false; // Versions are equal
  }

  // Manual update check with user feedback
  async manualUpdateCheck() {
    try {
      console.log('Auto-updater: Manual update check requested');
      
      // Always use GitHub API for manual checks (more reliable)
      console.log('Auto-updater: Manual check - using GitHub API directly');
      await this.checkGitHubAPI();
      
      // If no update is available after a delay, send to renderer
      setTimeout(() => {
        if (!this.updateAvailable) {
          console.log('Auto-updater: No updates available, sending to renderer');
          this.sendToRenderer('update-not-available', {
            message: 'You are already running the latest version.',
            currentVersion: this.currentVersion
          });
        }
      }, 2000);
      
    } catch (error) {
      console.error('Auto-updater: Manual update check failed:', error);
      dialog.showErrorBox(
        'Update Check Failed',
        `Unable to check for updates: ${error.message}`
      );
    }
  }

  // Show update dialog for manual updates
  showUpdateDialog(updateInfo) {
    const response = dialog.showMessageBoxSync(this.mainWindow, {
      type: 'info',
      title: 'Update Available',
      message: `NAHA MC Helper ${updateInfo.version} is available!`,
      detail: `You are currently running version ${this.currentVersion}.\n\nWould you like to download and install the update?`,
      buttons: ['Download Now', 'Later', 'View Release Notes'],
      defaultId: 0,
      cancelId: 1
    });

    switch (response) {
      case 0: // Download Now
        this.downloadUpdate();
        break;
      case 2: // View Release Notes
        shell.openExternal(`https://github.com/perlytiara/NAHA-MC-Helper/releases/tag/${updateInfo.version}`);
        break;
    }
  }
}

export default AutoUpdaterService;
