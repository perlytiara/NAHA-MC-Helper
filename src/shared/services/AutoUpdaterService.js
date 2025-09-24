import electron from 'electron';
import electronUpdater from 'electron-updater';
import { writeFileSync, readFileSync, existsSync } from 'fs';
import { join } from 'path';

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
      await autoUpdater.checkForUpdates();
    } catch (error) {
      console.error('Error checking for updates:', error);
      this.sendToRenderer('update-error', {
        message: 'Failed to check for updates',
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
      updateDownloaded: this.updateDownloaded
    };
  }

  // Manual update check with user feedback
  async manualUpdateCheck() {
    try {
      await this.checkForUpdates();
      
      // If no update is available, show a dialog
      setTimeout(() => {
        if (!this.updateAvailable) {
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
