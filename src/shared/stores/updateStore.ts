import { writable, derived } from 'svelte/store';
import type { Writable } from 'svelte/store';

export interface UpdateInfo {
  version: string;
  releaseNotes?: string;
  releaseDate: string;
  downloadUrl?: string;
}

export interface UpdateProgress {
  percent: number;
  transferred: number;
  total: number;
  bytesPerSecond: number;
}

export interface UpdateError {
  message: string;
  details?: string;
  stack?: string;
}

export type UpdateState = 
  | 'idle'
  | 'checking'
  | 'available'
  | 'downloading'
  | 'downloaded'
  | 'installing'
  | 'error';

// Store for update state
export const updateState: Writable<UpdateState> = writable('idle');

// Store for update information
export const updateInfo: Writable<UpdateInfo | null> = writable(null);

// Store for download progress
export const updateProgress: Writable<UpdateProgress> = writable({
  percent: 0,
  transferred: 0,
  total: 0,
  bytesPerSecond: 0
});

// Store for error information
export const updateError: Writable<UpdateError | null> = writable(null);

// Store for notification visibility
export const showUpdateNotification: Writable<boolean> = writable(false);

// Store for dialog visibility
export const showUpdateDialog: Writable<boolean> = writable(false);

// Store for current app version
export const currentVersion: Writable<string> = writable('1.0.0');

// Derived stores for computed values
export const isUpdateAvailable = derived(
  [updateState, updateInfo],
  ([$state, $info]) => $state === 'available' && $info !== null
);

export const isDownloading = derived(
  updateState,
  ($state) => $state === 'downloading'
);

export const isDownloaded = derived(
  updateState,
  ($state) => $state === 'downloaded'
);

export const hasError = derived(
  [updateState, updateError],
  ([$state, $error]) => $state === 'error' && $error !== null
);

export const progressPercentage = derived(
  updateProgress,
  ($progress) => Math.min(100, Math.max(0, $progress.percent))
);

export const downloadSpeed = derived(
  updateProgress,
  ($progress) => {
    const mbps = ($progress.bytesPerSecond / (1024 * 1024)).toFixed(1);
    return `${mbps} MB/s`;
  }
);

export const downloadedSize = derived(
  updateProgress,
  ($progress) => {
    const mb = ($progress.transferred / (1024 * 1024)).toFixed(1);
    return `${mb} MB`;
  }
);

export const totalSize = derived(
  updateProgress,
  ($progress) => {
    const mb = ($progress.total / (1024 * 1024)).toFixed(1);
    return `${mb} MB`;
  }
);

// Store actions
export const updateActions = {
  // Check for updates
  checkForUpdates: () => {
    updateState.set('checking');
    updateError.set(null);
    showUpdateDialog.set(true);
  },

  // Set update available
  setUpdateAvailable: (info: UpdateInfo) => {
    updateInfo.set(info);
    updateState.set('available');
    showUpdateNotification.set(true);
  },

  // Start download
  startDownload: () => {
    updateState.set('downloading');
    updateProgress.set({
      percent: 0,
      transferred: 0,
      total: 0,
      bytesPerSecond: 0
    });
  },

  // Update progress
  updateProgress: (progress: UpdateProgress) => {
    updateProgress.set(progress);
  },

  // Download completed
  downloadCompleted: () => {
    updateState.set('downloaded');
  },

  // Start installation
  startInstallation: () => {
    updateState.set('installing');
  },

  // Set error
  setError: (error: UpdateError) => {
    updateError.set(error);
    updateState.set('error');
  },

  // Clear error
  clearError: () => {
    updateError.set(null);
    updateState.set('idle');
  },

  // Reset all
  reset: () => {
    updateState.set('idle');
    updateInfo.set(null);
    updateProgress.set({
      percent: 0,
      transferred: 0,
      total: 0,
      bytesPerSecond: 0
    });
    updateError.set(null);
    showUpdateNotification.set(false);
    showUpdateDialog.set(false);
  },

  // Hide notification
  hideNotification: () => {
    showUpdateNotification.set(false);
  },

  // Show dialog
  showDialog: () => {
    showUpdateDialog.set(true);
  },

  // Hide dialog
  hideDialog: () => {
    showUpdateDialog.set(false);
  }
};

// Auto-update settings store
export interface AutoUpdateSettings {
  enabled: boolean;
  checkOnStartup: boolean;
  checkInterval: number; // in hours
  autoDownload: boolean;
  autoInstall: boolean;
}

export const autoUpdateSettings: Writable<AutoUpdateSettings> = writable({
  enabled: true,
  checkOnStartup: true,
  checkInterval: 6,
  autoDownload: false,
  autoInstall: false
});

// Update settings actions
export const settingsActions = {
  updateSettings: (settings: Partial<AutoUpdateSettings>) => {
    autoUpdateSettings.update(current => ({ ...current, ...settings }));
  },

  toggleEnabled: () => {
    autoUpdateSettings.update(current => ({ ...current, enabled: !current.enabled }));
  },

  toggleCheckOnStartup: () => {
    autoUpdateSettings.update(current => ({ ...current, checkOnStartup: !current.checkOnStartup }));
  },

  toggleAutoDownload: () => {
    autoUpdateSettings.update(current => ({ ...current, autoDownload: !current.autoDownload }));
  },

  toggleAutoInstall: () => {
    autoUpdateSettings.update(current => ({ ...current, autoInstall: !current.autoInstall }));
  },

  setCheckInterval: (hours: number) => {
    autoUpdateSettings.update(current => ({ ...current, checkInterval: Math.max(1, Math.min(24, hours)) }));
  }
};
