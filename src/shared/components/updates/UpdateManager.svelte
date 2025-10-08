<script>
  import { onMount, onDestroy } from 'svelte';
  import { updateActions, updateState, updateInfo, updateProgress, updateError, showUpdateDialog, showUpdateNotification, currentVersion } from '../stores/updateStore.js';
  import UpdateDialog from './ui/updates/UpdateDialog.svelte';
  import UpdateNotification from './ui/updates/UpdateNotification.svelte';

  let isInitialized = false;

  onMount(async () => {
    await initializeUpdateManager();
  });

  onDestroy(() => {
    // Cleanup if needed
  });

  async function initializeUpdateManager() {
    try {
      // Get current version from main process
      const versionResult = await window.electronAPI?.invoke('auto-updater:get-current-version');
      if (versionResult?.success) {
        currentVersion.set(versionResult.version);
      }

      // Set up IPC listeners for auto-updater events
      if (window.electronAPI) {
        window.electronAPI.on('auto-updater:update-checking', handleUpdateChecking);
        window.electronAPI.on('auto-updater:update-available', handleUpdateAvailable);
        window.electronAPI.on('auto-updater:update-not-available', handleUpdateNotAvailable);
        window.electronAPI.on('auto-updater:download-progress', handleDownloadProgress);
        window.electronAPI.on('auto-updater:update-downloaded', handleUpdateDownloaded);
        window.electronAPI.on('auto-updater:update-error', handleUpdateError);
      }

      isInitialized = true;
      console.log('Update manager initialized');
    } catch (error) {
      console.error('Failed to initialize update manager:', error);
    }
  }

  function handleUpdateChecking() {
    updateState.set('checking');
    showUpdateDialog.set(true);
  }

  function handleUpdateAvailable(event, info) {
    updateInfo.set(info);
    updateState.set('available');
    showUpdateNotification.set(true);
  }

  function handleUpdateNotAvailable(event, info) {
    updateState.set('idle');
    showUpdateNotification.set(false);
    showUpdateDialog.set(false);
  }

  function handleDownloadProgress(event, progress) {
    updateProgress.set(progress);
  }

  function handleUpdateDownloaded(event, info) {
    updateInfo.set(info);
    updateState.set('downloaded');
    showUpdateNotification.set(false);
    showUpdateDialog.set(true);
  }

  function handleUpdateError(event, error) {
    updateError.set(error);
    updateState.set('error');
    showUpdateDialog.set(true);
  }

  async function handleDownloadUpdate() {
    try {
      updateActions.startDownload();
      await window.electronAPI?.invoke('auto-updater:download-update');
    } catch (error) {
      console.error('Failed to download update:', error);
      updateActions.setError({
        message: 'Failed to download update',
        details: error.message
      });
    }
  }

  async function handleInstallUpdate() {
    try {
      updateActions.startInstallation();
      await window.electronAPI?.invoke('auto-updater:install-update');
    } catch (error) {
      console.error('Failed to install update:', error);
      updateActions.setError({
        message: 'Failed to install update',
        details: error.message
      });
    }
  }

  function handleRetryUpdate() {
    updateActions.clearError();
    updateActions.checkForUpdates();
  }

  function handleHideNotification() {
    updateActions.hideNotification();
  }

  function handleShowDialog() {
    updateActions.showDialog();
  }

  function handleHideDialog() {
    updateActions.hideDialog();
  }

  // Manual update check function (can be called from UI)
  export async function checkForUpdates() {
    if (!isInitialized) return;
    
    try {
      console.log('UpdateManager: Manual update check requested');
      await window.electronAPI?.invoke('auto-updater:manual-update-check');
    } catch (error) {
      console.error('Failed to check for updates:', error);
      updateActions.setError({
        message: 'Failed to check for updates',
        details: error.message
      });
    }
  }

  // Export functions for external use
  export { handleDownloadUpdate, handleInstallUpdate, handleRetryUpdate };
</script>

<!-- Update Dialog -->
<UpdateDialog 
  isOpen={$showUpdateDialog}
  updateInfo={$updateInfo}
  updateState={$updateState}
  progress={$updateProgress}
  error={$updateError}
  onDownload={handleDownloadUpdate}
  onInstall={handleInstallUpdate}
  onRetry={handleRetryUpdate}
  onClose={handleHideDialog}
/>

<!-- Update Notification -->
<UpdateNotification 
  show={$showUpdateNotification}
  updateInfo={$updateInfo}
  onDownload={handleDownloadUpdate}
  onDismiss={handleHideNotification}
/>

<!-- This component doesn't render any visible content itself -->
<!-- It just manages the update state and renders the update UI components -->
