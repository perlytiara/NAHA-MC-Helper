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

  // Additional page imports would go here as features are added
  // import UpdatePage from '../features/update/UpdatePage.svelte';

  let showAboutDialog = false;
  let showUpdateModal = false;
  let updateInfo = null;
  let downloadProgress = null;
  let isChecking = false;
  let isDownloading = false;
  let updateError = null;
  let currentVersion = '1.0.2';

  // Auto-updater event handlers
  function handleUpdateChecking() {
    isChecking = true;
    showUpdateModal = true;
    updateError = null;
  }

  function handleUpdateAvailable(event, data) {
    updateInfo = data;
    isChecking = false;
    showUpdateModal = true;
    updateError = null;
  }

  function handleUpdateNotAvailable(event, data) {
    isChecking = false;
    updateError = null;
    // Don't show modal for "no updates" unless manually checking
  }

  function handleDownloadProgress(event, data) {
    downloadProgress = data;
    isDownloading = true;
  }

  function handleUpdateDownloaded(event, data) {
    downloadProgress = null;
    isDownloading = false;
    updateInfo = { ...updateInfo, ...data };
    // Show install option
  }

  function handleUpdateError(event, data) {
    updateError = data;
    isChecking = false;
    isDownloading = false;
    showUpdateModal = true;
  }

  function handleDownload() {
    if (window.prism?.autoUpdater?.downloadUpdate) {
      window.prism.autoUpdater.downloadUpdate();
    }
  }

  function handleInstall() {
    if (window.prism?.autoUpdater?.installUpdate) {
      window.prism.autoUpdater.installUpdate();
    }
  }

  function handleViewReleaseNotes() {
    if (updateInfo?.version) {
      window.open(`https://github.com/perlytiara/NAHA-MC-Helper/releases/tag/v${updateInfo.version}`, '_blank');
    }
  }

  function handleUpdateModalClose() {
    showUpdateModal = false;
    updateInfo = null;
    downloadProgress = null;
    updateError = null;
    isChecking = false;
    isDownloading = false;
  }

  function handleUpdateLater() {
    showUpdateModal = false;
    updateInfo = null;
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

    // Setup auto-updater event listeners
    if (window.prism?.autoUpdater) {
      const removeListeners = [
        window.prism.autoUpdater.onUpdateChecking?.(handleUpdateChecking),
        window.prism.autoUpdater.onUpdateAvailable?.(handleUpdateAvailable),
        window.prism.autoUpdater.onUpdateNotAvailable?.(handleUpdateNotAvailable),
        window.prism.autoUpdater.onDownloadProgress?.(handleDownloadProgress),
        window.prism.autoUpdater.onUpdateDownloaded?.(handleUpdateDownloaded),
        window.prism.autoUpdater.onUpdateError?.(handleUpdateError)
      ];

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