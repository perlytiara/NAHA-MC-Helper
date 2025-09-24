<script>
  import { onMount, onDestroy } from 'svelte';
  import { fade, fly } from 'svelte/transition';
  import { scale } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';
  import UpdateProgress from './UpdateProgress.svelte';
  import UpdateNotification from './UpdateNotification.svelte';
  import ReleaseNotes from './ReleaseNotes.svelte';

  export let isOpen = false;
  export let updateInfo = null;
  export let updateState = 'checking'; // checking, available, downloading, downloaded, installing, error
  export let progress = 0;
  export let error = null;

  let showReleaseNotes = false;

  // Update states
  const states = {
    checking: {
      title: 'Checking for Updates',
      icon: '🔄',
      description: 'Looking for the latest version...',
      showProgress: false
    },
    available: {
      title: 'Update Available',
      icon: '✨',
      description: `Version ${updateInfo?.version} is ready to download`,
      showProgress: false
    },
    downloading: {
      title: 'Downloading Update',
      icon: '⬇️',
      description: 'Getting the latest version...',
      showProgress: true
    },
    downloaded: {
      title: 'Update Ready',
      icon: '✅',
      description: 'Update downloaded successfully!',
      showProgress: false
    },
    installing: {
      title: 'Installing Update',
      icon: '⚙️',
      description: 'Setting up the new version...',
      showProgress: true
    },
    error: {
      title: 'Update Failed',
      icon: '❌',
      description: error?.message || 'Something went wrong',
      showProgress: false
    }
  };

  const currentState = states[updateState];

  function handleDownload() {
    updateState = 'downloading';
    // Emit download event
  }

  function handleInstall() {
    updateState = 'installing';
    // Emit install event
  }

  function handleLater() {
    isOpen = false;
  }

  function handleViewReleaseNotes() {
    showReleaseNotes = !showReleaseNotes;
  }

  function handleRetry() {
    updateState = 'checking';
    // Emit retry event
  }

  function handleClose() {
    isOpen = false;
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4" transition:fade={{ duration: 300 }}>
    <div 
      class="bg-white dark:bg-gray-800 rounded-2xl shadow-2xl max-w-md w-full mx-4 overflow-hidden"
      transition:scale={{ duration: 400, easing: quintOut }}
    >
      <!-- Header -->
      <div class="bg-gradient-to-r from-blue-500 to-purple-600 p-6 text-white">
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-3">
            <div class="text-3xl animate-pulse">{currentState.icon}</div>
            <div>
              <h2 class="text-xl font-bold">{currentState.title}</h2>
              <p class="text-blue-100 text-sm">{currentState.description}</p>
            </div>
          </div>
          {#if updateState !== 'downloading' && updateState !== 'installing'}
            <button 
              on:click={handleClose}
              class="text-white/80 hover:text-white transition-colors"
            >
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
              </svg>
            </button>
          {/if}
        </div>
      </div>

      <!-- Content -->
      <div class="p-6">
        {#if updateState === 'available'}
          <div class="space-y-4">
            <div class="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-4">
              <div class="flex items-center space-x-2 mb-2">
                <span class="text-blue-600 dark:text-blue-400 font-medium">New Version:</span>
                <span class="bg-blue-100 dark:bg-blue-800 text-blue-800 dark:text-blue-200 px-2 py-1 rounded-full text-sm font-mono">
                  {updateInfo?.version}
                </span>
              </div>
              <p class="text-gray-600 dark:text-gray-300 text-sm">
                Released on {new Date(updateInfo?.releaseDate).toLocaleDateString()}
              </p>
            </div>

            <div class="flex space-x-3">
              <button 
                on:click={handleDownload}
                class="flex-1 bg-gradient-to-r from-blue-500 to-purple-600 text-white py-3 px-4 rounded-lg font-medium hover:from-blue-600 hover:to-purple-700 transition-all transform hover:scale-105 shadow-lg"
              >
                Download Update
              </button>
              <button 
                on:click={handleViewReleaseNotes}
                class="px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
              >
                📝
              </button>
            </div>

            <button 
              on:click={handleLater}
              class="w-full text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors py-2"
            >
              Remind me later
            </button>
          </div>
        {:else if updateState === 'downloading'}
          <UpdateProgress {progress} />
        {:else if updateState === 'downloaded'}
          <div class="space-y-4">
            <div class="bg-green-50 dark:bg-green-900/20 rounded-lg p-4 text-center">
              <div class="text-4xl mb-2">🎉</div>
              <h3 class="font-semibold text-green-800 dark:text-green-200 mb-1">
                Download Complete!
              </h3>
              <p class="text-green-600 dark:text-green-300 text-sm">
                The update is ready to install. The application will restart automatically.
              </p>
            </div>

            <div class="flex space-x-3">
              <button 
                on:click={handleInstall}
                class="flex-1 bg-gradient-to-r from-green-500 to-emerald-600 text-white py-3 px-4 rounded-lg font-medium hover:from-green-600 hover:to-emerald-700 transition-all transform hover:scale-105 shadow-lg"
              >
                Install & Restart
              </button>
              <button 
                on:click={handleLater}
                class="px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
              >
                Install Later
              </button>
            </div>
          </div>
        {:else if updateState === 'error'}
          <div class="space-y-4">
            <div class="bg-red-50 dark:bg-red-900/20 rounded-lg p-4">
              <div class="flex items-center space-x-2 mb-2">
                <span class="text-red-600 dark:text-red-400">⚠️</span>
                <span class="font-medium text-red-800 dark:text-red-200">Update Failed</span>
              </div>
              <p class="text-red-600 dark:text-red-300 text-sm">
                {error?.message || 'An unexpected error occurred'}
              </p>
            </div>

            <div class="flex space-x-3">
              <button 
                on:click={handleRetry}
                class="flex-1 bg-gradient-to-r from-red-500 to-pink-600 text-white py-3 px-4 rounded-lg font-medium hover:from-red-600 hover:to-pink-700 transition-all transform hover:scale-105 shadow-lg"
              >
                Try Again
              </button>
              <button 
                on:click={handleClose}
                class="px-4 py-3 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
              >
                Cancel
              </button>
            </div>
          </div>
        {:else}
          <div class="text-center py-8">
            <div class="animate-spin text-4xl mb-4">🔄</div>
            <p class="text-gray-600 dark:text-gray-300">
              {currentState.description}
            </p>
          </div>
        {/if}
      </div>

      <!-- Release Notes Modal -->
      {#if showReleaseNotes}
        <ReleaseNotes 
          {updateInfo}
          on:close={() => showReleaseNotes = false}
        />
      {/if}
    </div>
  </div>
{/if}

<style>
  button {
    transition: all 0.2s ease;
  }
  
  button:active {
    transform: scale(0.98);
  }
</style>
