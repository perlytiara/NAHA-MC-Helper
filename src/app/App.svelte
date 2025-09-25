<script lang="ts">
  import { onMount } from 'svelte';
  import { currentPage, notification, onboardingCompleted } from '../shared/stores/appStore';
  import { HomePage } from '../features/homepage';
  import { OnboardingFlow } from '../features/onboarding';
  import ServersPage from '../features/servers/components/ServersPage.svelte';
  import InstallPage from '../features/install/components/InstallPage.svelte';
  import StatusBanner from '../shared/components/ui/feedback/StatusBanner.svelte';
  import AboutDialog from '../shared/components/ui/dialogs/AboutDialog.svelte';
  import UpdateModal from '../components/UpdateModal.svelte';
  import { isOnboardingCompleted } from '../shared/utils/onboardingUtils';
  // Types are now available globally from src/types/global.d.ts

  // Additional page imports would go here as features are added
  // import UpdatePage from '../features/update/UpdatePage.svelte';

  let showAboutDialog: boolean = false;
  let showUpdateModal: boolean = false;
  let updateInfo: UpdateInfo | null = null;
  let downloadProgress: DownloadProgress | null = null;
  let isChecking: boolean = false;
  let isDownloading: boolean = false;
  let updateError: string | null = null;
  let currentVersion: string = '1.0.2';

  // Auto-updater event handlers
  function handleUpdateChecking(): void {
    isChecking = true;
    showUpdateModal = true;
    updateError = null;
  }

  function handleUpdateAvailable(event: any, data: UpdateInfo): void {
    console.log('App: handleUpdateAvailable called with:', data);
    updateInfo = data;
    isChecking = false;
    showUpdateModal = true;
    updateError = null;
    console.log('App: Update modal should now be visible:', showUpdateModal);
  }

  function handleUpdateNotAvailable(event: any, data: UpdateInfo): void {
    isChecking = false;
    updateError = null;
    // Don't show modal for "no updates" unless manually checking
  }

  function handleDownloadProgress(event: any, data: DownloadProgress): void {
    downloadProgress = data;
    isDownloading = true;
  }

  function handleUpdateDownloaded(event: any, data: UpdateInfo): void {
    downloadProgress = null;
    isDownloading = false;
    updateInfo = { ...updateInfo, ...data };
    // Show install option
  }

  function handleUpdateError(event: any, data: string): void {
    updateError = data;
    isChecking = false;
    isDownloading = false;
    showUpdateModal = true;
  }

  function handleDownload(): void {
    if (window.prism?.autoUpdater?.downloadUpdate) {
      window.prism.autoUpdater.downloadUpdate();
    }
  }

  function handleInstall(): void {
    if (window.prism?.autoUpdater?.installUpdate) {
      window.prism.autoUpdater.installUpdate();
    }
  }

  function handleViewReleaseNotes(): void {
    if (updateInfo?.version) {
      window.open(`https://github.com/perlytiara/NAHA-MC-Helper/releases/tag/v${updateInfo.version}`, '_blank');
    }
  }

  function handleUpdateModalClose(): void {
    showUpdateModal = false;
    updateInfo = null;
    downloadProgress = null;
    updateError = null;
    isChecking = false;
    isDownloading = false;
  }

  function handleUpdateLater(): void {
    showUpdateModal = false;
    updateInfo = null;
  }

  // Manual update check function that can be called from UI
  async function handleManualUpdateCheck(): Promise<void> {
    try {
      console.log('🎯 App: handleManualUpdateCheck function called!');
      console.log('App: Manual update check requested');
      
      // Set checking state immediately
      isChecking = true;
      showUpdateModal = true;
      console.log('App: Modal state set - showUpdateModal:', showUpdateModal, 'isChecking:', isChecking);
      
      // Try direct GitHub API call first (most reliable)
      console.log('App: Trying direct GitHub API call first...');
      await checkGitHubAPIDirectly();
      
      // If that fails, try the backend approach
      if (!updateInfo && !updateError) {
        console.log('App: Direct API failed, trying backend approach...');
        if (window.prism?.autoUpdater?.manualUpdateCheck) {
          await window.prism.autoUpdater.manualUpdateCheck();
        }
        
        // Fallback to IPC check
        setTimeout(() => {
          if (!updateInfo && !updateError) {
            console.log('App: Checking for updates via IPC fallback...');
            checkForUpdatesViaIPC();
          }
        }, 2000);
      }
      
    } catch (error) {
      console.error('Manual update check failed:', error);
      isChecking = false;
      updateError = error.message;
    }
  }

  // Fallback method to check for updates via IPC
  async function checkForUpdatesViaIPC(): Promise<void> {
    try {
      console.log('App: Checking for updates via IPC...');
      // Try to get update info directly
      const updateInfoResult = await window.prism?.autoUpdater?.getUpdateInfo?.();
      if (updateInfoResult?.success && updateInfoResult.updateInfo) {
        console.log('App: Found update via IPC:', updateInfoResult.updateInfo);
        updateInfo = updateInfoResult.updateInfo;
        showUpdateModal = true;
        isChecking = false;
        return;
      }
      
      // If IPC doesn't work, try direct GitHub API call
      console.log('App: IPC failed, trying direct GitHub API...');
      await checkGitHubAPIDirectly();
      
    } catch (error) {
      console.error('App: IPC update check failed:', error);
      // Try direct GitHub API as last resort
      await checkGitHubAPIDirectly();
    }
  }

  // Direct GitHub API call from renderer
  async function checkGitHubAPIDirectly(): Promise<void> {
    try {
      console.log('App: Making direct GitHub API call...');
      
      const response = await fetch('https://api.github.com/repos/perlytiara/NAHA-MC-Helper/releases/latest', {
        headers: {
          'User-Agent': 'NAHA-MC-Helper-Update-Checker',
          'Accept': 'application/vnd.github.v3+json'
        }
      });
      
      console.log('App: GitHub API response status:', response.status);
      
      if (!response.ok) {
        throw new Error(`GitHub API error: ${response.status} ${response.statusText}`);
      }
      
      const release = await response.json();
      const latestVersion = release.tag_name.replace('v', '');
      
      console.log('App: GitHub API - Latest version:', latestVersion);
      console.log('App: GitHub API - Current version:', currentVersion);
      console.log('App: GitHub API - Release tag:', release.tag_name);
      
      if (latestVersion !== currentVersion) {
        console.log('App: ✅ UPDATE AVAILABLE via direct GitHub API!');
        
        updateInfo = {
          version: latestVersion,
          releaseNotes: release.body || 'Update available',
          releaseName: release.name || release.tag_name,
          releaseDate: release.published_at
        };
        
        showUpdateModal = true;
        isChecking = false;
        
        console.log('App: 🎉 Update modal should now be visible with version', latestVersion);
        console.log('App: Final modal state - showUpdateModal:', showUpdateModal, 'isChecking:', isChecking, 'updateInfo:', updateInfo);
      } else {
        console.log('App: ℹ️ No updates available via direct GitHub API');
        isChecking = false;
        showUpdateModal = false;
      }
      
    } catch (error) {
      console.error('App: ❌ Direct GitHub API check failed:', error);
      console.error('App: Error details:', error.message);
      isChecking = false;
      updateError = `Network error: ${error.message}`;
    }
  }

  // Expose manual update check function globally for UI components
  if (typeof window !== 'undefined') {
    window.manualUpdateCheck = handleManualUpdateCheck;
    console.log('App: manualUpdateCheck function exposed to window object');
    console.log('App: window.manualUpdateCheck available:', typeof window.manualUpdateCheck);
  }

  onMount(() => {
    // Check if this is the first time the user is opening the app
    const completed = isOnboardingCompleted();
    onboardingCompleted.set(completed);
    
    // Listen for about dialog request from main process
    if (window.prism?.onShowAboutDialog) {
      const removeAboutListener = window.prism.onShowAboutDialog(() => {
        showAboutDialog = true;
      });
      
      // Cleanup function
      return removeAboutListener;
    }

    // Listen for menu check-for-updates request
    const handleMenuCheckForUpdates = () => {
      console.log('App: Menu check-for-updates requested');
      handleManualUpdateCheck();
    };

    // Add IPC listener for menu events
    if (window.electronAPI) {
      window.electronAPI.on('menu:check-for-updates', handleMenuCheckForUpdates);
      
      // Return cleanup function
      return () => {
        window.electronAPI.off('menu:check-for-updates', handleMenuCheckForUpdates);
      };
    }

    // Setup auto-updater event listeners
    if (window.prism?.autoUpdater) {
      console.log('App: Setting up auto-updater event listeners');
      const removeListeners = [
        window.prism.autoUpdater.onUpdateChecking?.(handleUpdateChecking),
        window.prism.autoUpdater.onUpdateAvailable?.(handleUpdateAvailable),
        window.prism.autoUpdater.onUpdateNotAvailable?.(handleUpdateNotAvailable),
        window.prism.autoUpdater.onDownloadProgress?.(handleDownloadProgress),
        window.prism.autoUpdater.onUpdateDownloaded?.(handleUpdateDownloaded),
        window.prism.autoUpdater.onUpdateError?.(handleUpdateError)
      ];
      console.log('App: Event listeners set up:', removeListeners.length);

      // Auto-check for updates on startup (but don't show modal unless update available)
      setTimeout(() => {
        if (window.prism?.autoUpdater?.checkForUpdates) {
          window.prism.autoUpdater.checkForUpdates();
        }
      }, 3000); // Wait 3 seconds after app starts

      // Cleanup function
      return () => {
        removeListeners.forEach(remove => remove?.());
      };
    }
  });
</script>

<main class="min-h-screen overflow-hidden">
  <!-- Global notification banner -->
  {#if $notification}
    <div class="notification-container">
      <StatusBanner 
        show={true}
        type={$notification.type}
        message={$notification.message}
        title={$notification.title || ''}
        dismissible={true}
        autoHide={$notification.type !== 'error'}
        autoHideDelay={$notification.type === 'success' ? 5000 : 8000}
        on:dismiss={() => notification.set(null)}
      />
    </div>
  {/if}

  {#if !$onboardingCompleted}
    <OnboardingFlow />
  {:else if $currentPage === 'homepage'}
    <HomePage />
  {:else if $currentPage === 'install'}
    <InstallPage />
  {:else if $currentPage === 'update'}
    <div class="p-8">
      <h1>Update Page - Coming Soon</h1>
      <button on:click={() => currentPage.set('homepage')} class="btn">Back to Home</button>
    </div>
  {:else if $currentPage === 'servers'}
    <ServersPage />
  {:else if $currentPage === 'packer'}
    <div class="p-8">
      <h1>Packer Tool - Coming Soon</h1>
      <button on:click={() => currentPage.set('homepage')} class="btn">Back to Home</button>
    </div>
  {:else if $currentPage === 'mrpack-editor'}
    <div class="p-8">
      <h1>MRPack Editor - Coming Soon</h1>
      <button on:click={() => currentPage.set('homepage')} class="btn">Back to Home</button>
    </div>
  {:else if $currentPage === 'presets'}
    <div class="p-8">
      <h1>Presets Creator - Coming Soon</h1>
      <button on:click={() => currentPage.set('homepage')} class="btn">Back to Home</button>
    </div>
  {:else}
    <!-- Fallback to homepage -->
    <HomePage />
  {/if}

  <!-- Global About Dialog -->
  <AboutDialog bind:show={showAboutDialog} on:close={() => showAboutDialog = false} />
  
  <!-- Update Modal -->
  <UpdateModal 
    bind:isVisible={showUpdateModal}
    {updateInfo}
    {downloadProgress}
    {isChecking}
    {isDownloading}
    error={updateError}
    {currentVersion}
    on:download={handleDownload}
    on:install={handleInstall}
    on:later={handleUpdateLater}
    on:viewReleaseNotes={handleViewReleaseNotes}
    on:close={handleUpdateModalClose}
  />

</main>

<style>
  .notification-container {
    position: fixed;
    top: 80px;
    left: 50%;
    transform: translateX(-50%);
    z-index: 1000;
    max-width: 600px;
    width: 90%;
    pointer-events: none;
  }

  .notification-container :global(.status-banner) {
    pointer-events: auto;
  }

  .btn {
    background: linear-gradient(135deg, #3b82f6, #8b5cf6);
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 8px;
    cursor: pointer;
    font-weight: 500;
    transition: all 0.2s ease;
  }
  
  .btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
  }
</style>