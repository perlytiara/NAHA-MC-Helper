<script lang="ts">
  import { onMount } from 'svelte';
  import { currentPage, notification, onboardingCompleted } from '../shared/stores/appStore';
  import { HomePage } from '../features/homepage';
  import { OnboardingFlow } from '../features/onboarding';
  import ServersPage from '../features/servers/components/ServersPage.svelte';
  import InstallPage from '../features/install/components/InstallPage.svelte';
  import StatusBanner from '../shared/components/ui/feedback/StatusBanner.svelte';
  import AboutDialog from '../shared/components/ui/dialogs/AboutDialog.svelte';
  import { isOnboardingCompleted } from '../shared/utils/onboardingUtils';

  // Additional page imports would go here as features are added
  // import UpdatePage from '../features/update/UpdatePage.svelte';

  let showAboutDialog = false;

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