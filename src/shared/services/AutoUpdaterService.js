import electron from 'electron';
import electronUpdater from 'electron-updater';
import { writeFileSync, readFileSync, existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

// ES module equivalent of __dirname
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

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
      
      // In development mode, always use GitHub API directly
      if (!app.isPackaged) {
        console.log('Auto-updater: Development mode detected, using GitHub API directly');
        await this.checkGitHubAPI();
        return;
      }
      
      // In production, use electron-updater
      await autoUpdater.checkForUpdates();
      console.log('Auto-updater: Update check completed');
    } catch (error) {
      console.error('Auto-updater: Error checking for updates:', error);
      
      // Fallback to GitHub API if electron-updater fails
      if (!app.isPackaged) {
        console.log('Auto-updater: Falling back to direct GitHub API check');
        await this.checkGitHubAPI();
      } else {
        this.sendToRenderer('update-error', {
          message: 'Failed to check for updates',
          details: error.message
        });
      }
    }
  }

  // Direct GitHub API check as fallback
  async checkGitHubAPI() {
    try {
      console.log('Auto-updater: Checking GitHub API directly...');
      
      // Use Node.js https module to make direct API call
      const https = await import('https');
      
      const options = {
        hostname: 'api.github.com',
        port: 443,
        path: '/repos/perlytiara/NAHA-MC-Helper/releases/latest',
        method: 'GET',
        headers: {
          'User-Agent': 'NAHA-MC-Helper-Update-Checker',
          'Accept': 'application/vnd.github.v3+json'
        }
      };

      const response = await new Promise((resolve, reject) => {
        const req = https.default.request(options, (res) => {
          let data = '';
          res.on('data', (chunk) => {
            data += chunk;
          });
          res.on('end', () => {
            try {
              resolve(JSON.parse(data));
            } catch (error) {
              reject(error);
            }
          });
        });

        req.on('error', reject);
        req.end();
      });

      const latestVersion = response.tag_name.replace('v', '');
      console.log('Auto-updater: GitHub API - Latest version:', latestVersion);
      console.log('Auto-updater: GitHub API - Current version:', this.currentVersion);

      if (latestVersion !== this.currentVersion) {
        console.log('Auto-updater: Update available via GitHub API');
        
        // Simulate electron-updater events
        const updateInfo = {
          version: latestVersion,
          releaseNotes: response.body || 'Update available',
          releaseName: response.name || response.tag_name,
          releaseDate: response.published_at
        };

        this.updateAvailable = true;
        this.updateInfo = updateInfo;
        
        console.log('Auto-updater: Sending update-available event to renderer:', updateInfo);
        
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
    if (this.updateAvailable) {
      try {
        await autoUpdater.downloadUpdate();
      } catch (error) {
        console.error('Error downloading update:', error);
        this.sendToRenderer('update-error', {
          message: 'Failed to download update',
          details: error.message
        });
      }
    }
  }

  async installUpdate() {
    if (this.updateDownloaded) {
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
    this.updateCheckInterval = setInterval(() => {
      this.checkForUpdates();
    }, 6 * 60 * 60 * 1000);
  }

  stopPeriodicUpdateCheck() {
    if (this.updateCheckInterval) {
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

  // Manual update check with user feedback
  async manualUpdateCheck() {
    try {
      console.log('Auto-updater: Manual update check requested');
      
      // Always use GitHub API for manual checks in development
      if (!app.isPackaged) {
        console.log('Auto-updater: Manual check - using GitHub API directly');
        await this.checkGitHubAPI();
      } else {
        await this.checkForUpdates();
      }
      
      // If no update is available, show a dialog
      setTimeout(() => {
        if (!this.updateAvailable) {
          console.log('Auto-updater: No updates available, showing dialog');
          dialog.showMessageBox(this.mainWindow, {
            type: 'info',
            title: 'No Updates Available',
            message: 'You are running the latest version of NAHA MC Helper.',
            detail: `Current version: ${this.currentVersion}`,
            buttons: ['OK']
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
