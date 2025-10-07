const { contextBridge, ipcRenderer } = require('electron');

console.log('Preload script loaded successfully');

// Expose protected methods that allow the renderer process to use
// the ipcRenderer without exposing the entire object
contextBridge.exposeInMainWorld('nahaAPI', {
  // Download functionality
  downloader: (options) => ipcRenderer.invoke('download-file', options),
  onDownloadProgress: (callback) => ipcRenderer.on('download-progress', callback),
  
  // File system operations
  ensureDir: (dirPath) => ipcRenderer.invoke('ensure-directory', dirPath),
  deleteFile: (filePath) => ipcRenderer.invoke('delete-file', filePath),
  getFileHash: (filePath, algorithm) => ipcRenderer.invoke('get-file-hash', filePath, algorithm),
  
  // Installation
  execInstaller: (options) => ipcRenderer.invoke('exec-installer', options),
  
  // Release fetching
  fetchLatest: (mode) => ipcRenderer.invoke('fetch-latest-release', mode),
  
  // Modpack functionality
  fetchLatestPack: (loader) => ipcRenderer.invoke('fetch-latest-pack', loader),
  onPacksDownload: (callback) => ipcRenderer.on('packs-download-progress', callback),
  
  // Theme functionality
  getTheme: () => ipcRenderer.invoke('get-theme'),
  onThemeChanged: (callback) => ipcRenderer.on('theme-changed', callback),
  
  // Task updates
  onTaskUpdate: (callback) => ipcRenderer.on('task-update', callback),
  
  // Utility functions
  getCacheDir: () => ipcRenderer.invoke('get-cache-dir'),
  getPlatform: () => process.platform,
  getArch: () => process.arch,
  
  // App info functions
  getAppInfo: () => ipcRenderer.invoke('get-app-info'),
  onShowAboutDialog: (callback) => ipcRenderer.on('show-about-dialog', callback),
  
  // Debug mode functions
  getDebugMode: () => ipcRenderer.invoke('get-debug-mode'),
  setDebugMode: (enabled) => ipcRenderer.invoke('set-debug-mode', enabled),
  onDebugChanged: (callback) => ipcRenderer.on('debug-changed', callback),
  
  // Data reset functions
  resetApplicationData: () => ipcRenderer.invoke('reset-application-data'),
  onResetDataConfirmed: (callback) => ipcRenderer.on('reset-data-confirmed', callback),
  
  // Instance creation functions
  showOpenDialog: (options) => ipcRenderer.invoke('show-open-dialog', options),
  validateMinecraftInstallation: (minecraftPath) => ipcRenderer.invoke('validate-minecraft-installation', minecraftPath),
  parseMrpack: (mrpackPath) => ipcRenderer.invoke('parse-mrpack', mrpackPath),
  extractMrpack: (options) => ipcRenderer.invoke('extract-mrpack', options),
  detectLaunchers: () => ipcRenderer.invoke('detect-launchers'),
  createXmclInstance: (options) => ipcRenderer.invoke('create-xmcl-instance', options),
  createPrismInstance: (options) => ipcRenderer.invoke('create-prism-instance', options),
  createAstralrinthInstance: (options) => ipcRenderer.invoke('create-astralrinth-instance', options),
  
  // Minecraft Installer functions
  installer: {
    isAvailable: () => ipcRenderer.invoke('installer-is-available'),
    getVersion: () => ipcRenderer.invoke('installer-get-version'),
    listLaunchers: () => ipcRenderer.invoke('installer-list-launchers'),
    downloadNeoForge: (targetLauncher, createInstance) => ipcRenderer.invoke('installer-download-neoforge', targetLauncher, createInstance),
    downloadFabric: (targetLauncher, createInstance) => ipcRenderer.invoke('installer-download-fabric', targetLauncher, createInstance),
    installMrpack: (options) => ipcRenderer.invoke('installer-install-mrpack', options),
    listVersions: (options) => ipcRenderer.invoke('installer-list-versions', options),
    installMinecraft: (options) => ipcRenderer.invoke('installer-install-minecraft', options),
    onProgress: (callback) => ipcRenderer.on('installer-progress', callback)
  },

  // Auto-Updater functions
  autoUpdater: {
    checkForUpdates: () => ipcRenderer.invoke('auto-updater:check-for-updates'),
    manualUpdateCheck: () => ipcRenderer.invoke('auto-updater:manual-update-check'),
    downloadUpdate: (updateInfo) => ipcRenderer.invoke('auto-updater:download-update', updateInfo),
    installUpdate: () => ipcRenderer.invoke('auto-updater:install-update'),
    getCurrentVersion: () => ipcRenderer.invoke('auto-updater:get-current-version'),
    getUpdateInfo: () => ipcRenderer.invoke('auto-updater:get-update-info'),
    startPeriodicCheck: () => ipcRenderer.invoke('auto-updater:start-periodic-check'),
    stopPeriodicCheck: () => ipcRenderer.invoke('auto-updater:stop-periodic-check'),
    onUpdateChecking: (callback) => {
      ipcRenderer.on('auto-updater:update-checking', callback);
      return () => ipcRenderer.off('auto-updater:update-checking', callback);
    },
    onUpdateAvailable: (callback) => {
      ipcRenderer.on('auto-updater:update-available', callback);
      return () => ipcRenderer.off('auto-updater:update-available', callback);
    },
    onUpdateNotAvailable: (callback) => {
      ipcRenderer.on('auto-updater:update-not-available', callback);
      return () => ipcRenderer.off('auto-updater:update-not-available', callback);
    },
    onDownloadProgress: (callback) => {
      ipcRenderer.on('auto-updater:download-progress', callback);
      return () => ipcRenderer.off('auto-updater:download-progress', callback);
    },
    onUpdateDownloaded: (callback) => {
      ipcRenderer.on('auto-updater:update-downloaded', callback);
      return () => ipcRenderer.off('auto-updater:update-downloaded', callback);
    },
    onUpdateError: (callback) => {
      ipcRenderer.on('auto-updater:update-error', callback);
      return () => ipcRenderer.off('auto-updater:update-error', callback);
    },
    on: (event, callback) => ipcRenderer.on(`auto-updater:${event}`, callback),
    off: (event, callback) => ipcRenderer.off(`auto-updater:${event}`, callback)
  },
  
  // Direct IPC access for progress events
  ipcRenderer: {
    on: (channel, callback) => ipcRenderer.on(channel, callback),
    off: (channel, callback) => ipcRenderer.off(channel, callback)
  }
});

// Expose menu event handling
contextBridge.exposeInMainWorld('electronAPI', {
  on: (event, callback) => {
    console.log('🔧 PRELOAD: Setting up listener for event:', event);
    ipcRenderer.on(event, callback);
  },
  off: (event, callback) => {
    console.log('🔧 PRELOAD: Removing listener for event:', event);
    ipcRenderer.off(event, callback);
  }
});
