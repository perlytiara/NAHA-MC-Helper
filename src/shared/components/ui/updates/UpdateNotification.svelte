<script>
  import { onMount, onDestroy } from 'svelte';
  import { fly, fade } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';

  export let show = false;
  export let updateInfo = null;
  export let onDownload = () => {};
  export let onDismiss = () => {};

  let isVisible = false;
  let autoHideTimer = null;

  onMount(() => {
    if (show) {
      isVisible = true;
      // Auto-hide after 10 seconds
      autoHideTimer = setTimeout(() => {
        handleDismiss();
      }, 10000);
    }
  });

  onDestroy(() => {
    if (autoHideTimer) {
      clearTimeout(autoHideTimer);
    }
  });

  function handleDownload() {
    onDownload();
    handleDismiss();
  }

  function handleDismiss() {
    isVisible = false;
    setTimeout(() => {
      onDismiss();
    }, 300);
  }

  $: if (show && !isVisible) {
    isVisible = true;
    autoHideTimer = setTimeout(() => {
      handleDismiss();
    }, 10000);
  }
</script>

{#if isVisible}
  <div 
    class="fixed top-4 right-4 z-50 max-w-sm"
    transition:fly={{ x: 300, duration: 500, easing: quintOut }}
  >
    <div class="bg-white dark:bg-gray-800 rounded-xl shadow-2xl border border-gray-200 dark:border-gray-700 overflow-hidden">
      <!-- Header -->
      <div class="bg-gradient-to-r from-green-500 to-emerald-600 p-4 text-white">
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-3">
            <div class="text-2xl animate-bounce">🚀</div>
            <div>
              <h3 class="font-bold text-lg">Update Available!</h3>
              <p class="text-green-100 text-sm">Version {updateInfo?.version}</p>
            </div>
          </div>
          <button 
            on:click={handleDismiss}
            class="text-white/80 hover:text-white transition-colors"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
            </svg>
          </button>
        </div>
      </div>

      <!-- Content -->
      <div class="p-4">
        <div class="mb-4">
          <div class="flex items-center space-x-2 mb-2">
            <span class="bg-green-100 dark:bg-green-900/30 text-green-800 dark:text-green-200 px-2 py-1 rounded-full text-xs font-medium">
              NEW
            </span>
            <span class="text-gray-600 dark:text-gray-300 text-sm">
              Released {new Date(updateInfo?.releaseDate).toLocaleDateString()}
            </span>
          </div>
          
          {#if updateInfo?.releaseNotes}
            <p class="text-gray-700 dark:text-gray-300 text-sm line-clamp-2">
              {updateInfo.releaseNotes.substring(0, 100)}...
            </p>
          {/if}
        </div>

        <!-- Actions -->
        <div class="flex space-x-2">
          <button 
            on:click={handleDownload}
            class="flex-1 bg-gradient-to-r from-green-500 to-emerald-600 text-white py-2 px-3 rounded-lg font-medium hover:from-green-600 hover:to-emerald-700 transition-all transform hover:scale-105 text-sm shadow-lg"
          >
            Download Now
          </button>
          <button 
            on:click={handleDismiss}
            class="px-3 py-2 text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors text-sm"
          >
            Later
          </button>
        </div>

        <!-- Progress indicator -->
        <div class="mt-3 flex items-center space-x-2">
          <div class="flex-1 h-1 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
            <div class="h-full bg-gradient-to-r from-green-500 to-emerald-600 rounded-full w-0 animate-pulse"></div>
          </div>
          <span class="text-xs text-gray-500 dark:text-gray-400">Auto-hide in 10s</span>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .line-clamp-2 {
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .animate-bounce {
    animation: bounce 2s infinite;
  }

  @keyframes bounce {
    0%, 20%, 53%, 80%, 100% {
      transform: translate3d(0,0,0);
    }
    40%, 43% {
      transform: translate3d(0, -6px, 0);
    }
    70% {
      transform: translate3d(0, -3px, 0);
    }
    90% {
      transform: translate3d(0, -1px, 0);
    }
  }

  button {
    transition: all 0.2s ease;
  }
  
  button:active {
    transform: scale(0.98);
  }
</style>
