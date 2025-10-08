// Global type declarations for the application

interface UpdateInfo {
  version: string;
  releaseNotes?: string;
  releaseName?: string;
  releaseDate?: string;
  files?: Array<{
    url: string;
    sha512: string;
    size: number;
  }>;
}

interface DownloadProgress {
  bytesPerSecond: number;
  percent: number;
  transferred: number;
  total: number;
}

interface UpdateError {
  message: string;
  details?: string;
}

interface AutoUpdater {
  checkForUpdates: () => Promise<void>;
  manualUpdateCheck: () => Promise<void>;
  downloadUpdate: (updateInfo?: UpdateInfo | null) => Promise<void>;
  installUpdate: () => Promise<void>;
  getCurrentVersion: () => Promise<string>;
  getUpdateInfo: () => Promise<UpdateInfo | null>;
  startPeriodicCheck: () => Promise<void>;
  stopPeriodicCheck: () => Promise<void>;
  onUpdateChecking: (callback: () => void) => () => void;
  onUpdateAvailable: (callback: (event: any, updateInfo: UpdateInfo) => void) => () => void;
  onUpdateNotAvailable: (callback: (event: any, updateInfo: UpdateInfo) => void) => () => void;
  onDownloadProgress: (callback: (event: any, progress: DownloadProgress) => void) => () => void;
  onUpdateDownloaded: (callback: (event: any, updateInfo: UpdateInfo) => void) => () => void;
  onUpdateError: (callback: (event: any, error: string) => void) => () => void;
  on: (event: string, callback: (...args: any[]) => void) => () => void;
  off: (event: string, callback: (...args: any[]) => void) => void;
}

interface Window {
  nahaAPI: {
    detectLaunchers: () => Promise<{ success: boolean; launchers: any[]; error?: string }>;
    validateMinecraftInstallation: (path: string) => Promise<boolean>;
    openFileDialog: (options: { properties: string[] }) => Promise<string[]>;
    onShowAboutDialog: (callback: () => void) => () => void;
    getFileHash: (filePath: string, algorithm?: string) => Promise<string>;
    getAppVersion: () => Promise<string>;
    autoUpdater: AutoUpdater;
    minecraftUpdater: {
      scanInstances: (format: string, launcher?: string | null) => Promise<{ success: boolean; instances: any[] }>;
      updateInstance: (instancePath: string, modpackType: string, version: string | null) => Promise<any>;
      listLaunchers: () => Promise<any>;
      getBinaryStatus: () => Promise<any>;
    };
    minecraftInstaller: {
      installModpack: (modpackType: string, targetLauncher: string | null) => Promise<any>;
      listLaunchers: () => Promise<any>;
    };
    ipcRenderer: {
      on: (channel: string, callback: (...args: any[]) => void) => void;
      off: (channel: string, callback: (...args: any[]) => void) => void;
    };
  };
  electronAPI?: {
    on: (event: string, callback: (...args: any[]) => void) => void;
    off: (event: string, callback: (...args: any[]) => void) => void;
  };
  manualUpdateCheck?: () => Promise<void>;
}

// Node.js globals for renderer process
declare const process: NodeJS.Process;
declare const setTimeout: (callback: () => void, ms: number) => NodeJS.Timeout;
declare const clearTimeout: (timeoutId: NodeJS.Timeout) => void;
