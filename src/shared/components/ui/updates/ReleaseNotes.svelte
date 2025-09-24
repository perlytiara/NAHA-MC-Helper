<script>
  import { fade, fly } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';

  export let updateInfo = null;
  export let onClose = () => {};

  function handleClose() {
    onClose();
  }

  function handleExternalLink() {
    // Open release notes in browser
    if (updateInfo?.version) {
      window.open(`https://github.com/perlytiara/NAHA-MC-Helper/releases/tag/${updateInfo.version}`, '_blank');
    }
  }
</script>

<div 
  class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
  transition:fade={{ duration: 300 }}
  on:click={handleClose}
  role="button"
  tabindex="0"
>
  <div 
    class="bg-white dark:bg-gray-800 rounded-2xl shadow-2xl max-w-2xl w-full mx-4 max-h-[80vh] overflow-hidden"
    transition:fly={{ y: 50, duration: 400, easing: quintOut }}
    on:click|stopPropagation
  >
    <!-- Header -->
    <div class="bg-gradient-to-r from-blue-500 to-purple-600 p-6 text-white">
      <div class="flex items-center justify-between">
        <div>
          <h2 class="text-2xl font-bold">Release Notes</h2>
          <p class="text-blue-100">Version {updateInfo?.version}</p>
        </div>
        <button 
          on:click={handleClose}
          class="text-white/80 hover:text-white transition-colors"
        >
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
          </svg>
        </button>
      </div>
    </div>

    <!-- Content -->
    <div class="p-6 overflow-y-auto max-h-[60vh]">
      {#if updateInfo?.releaseNotes}
        <div class="prose prose-gray dark:prose-invert max-w-none">
          <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-4 mb-6">
            <div class="flex items-center space-x-2 mb-3">
              <span class="text-blue-600 dark:text-blue-400">📅</span>
              <span class="font-medium text-gray-700 dark:text-gray-300">Release Date</span>
            </div>
            <p class="text-gray-600 dark:text-gray-300">
              {new Date(updateInfo.releaseDate).toLocaleDateString('en-US', {
                weekday: 'long',
                year: 'numeric',
                month: 'long',
                day: 'numeric'
              })}
            </p>
          </div>

          <div class="space-y-4">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 flex items-center space-x-2">
              <span>✨</span>
              <span>What's New</span>
            </h3>
            
            <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-4">
              <div class="text-gray-700 dark:text-gray-300 whitespace-pre-wrap">
                {updateInfo.releaseNotes}
              </div>
            </div>
          </div>

          <!-- Features highlight -->
          <div class="mt-6 grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="bg-gradient-to-br from-green-50 to-emerald-50 dark:from-green-900/20 dark:to-emerald-900/20 rounded-lg p-4">
              <div class="flex items-center space-x-2 mb-2">
                <span class="text-green-600 dark:text-green-400">🚀</span>
                <span class="font-medium text-green-800 dark:text-green-200">Performance</span>
              </div>
              <p class="text-green-700 dark:text-green-300 text-sm">
                Improved app startup time and memory usage
              </p>
            </div>

            <div class="bg-gradient-to-br from-blue-50 to-indigo-50 dark:from-blue-900/20 dark:to-indigo-900/20 rounded-lg p-4">
              <div class="flex items-center space-x-2 mb-2">
                <span class="text-blue-600 dark:text-blue-400">🎨</span>
                <span class="font-medium text-blue-800 dark:text-blue-200">UI/UX</span>
              </div>
              <p class="text-blue-700 dark:text-blue-300 text-sm">
                Enhanced user interface with new animations
              </p>
            </div>

            <div class="bg-gradient-to-br from-purple-50 to-pink-50 dark:from-purple-900/20 dark:to-pink-900/20 rounded-lg p-4">
              <div class="flex items-center space-x-2 mb-2">
                <span class="text-purple-600 dark:text-purple-400">🔧</span>
                <span class="font-medium text-purple-800 dark:text-purple-200">Bug Fixes</span>
              </div>
              <p class="text-purple-700 dark:text-purple-300 text-sm">
                Resolved various stability issues
              </p>
            </div>

            <div class="bg-gradient-to-br from-orange-50 to-red-50 dark:from-orange-900/20 dark:to-red-900/20 rounded-lg p-4">
              <div class="flex items-center space-x-2 mb-2">
                <span class="text-orange-600 dark:text-orange-400">🔒</span>
                <span class="font-medium text-orange-800 dark:text-orange-200">Security</span>
              </div>
              <p class="text-orange-700 dark:text-orange-300 text-sm">
                Enhanced security and privacy features
              </p>
            </div>
          </div>
        </div>
      {:else}
        <div class="text-center py-8">
          <div class="text-4xl mb-4">📝</div>
          <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100 mb-2">
            Release Notes
          </h3>
          <p class="text-gray-600 dark:text-gray-300 mb-4">
            No release notes available for this version.
          </p>
          <button 
            on:click={handleExternalLink}
            class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-lg transition-colors"
          >
            View on GitHub
          </button>
        </div>
      {/if}
    </div>

    <!-- Footer -->
    <div class="bg-gray-50 dark:bg-gray-700/50 px-6 py-4 border-t border-gray-200 dark:border-gray-700">
      <div class="flex items-center justify-between">
        <div class="text-sm text-gray-500 dark:text-gray-400">
          Want to see more details?
        </div>
        <button 
          on:click={handleExternalLink}
          class="text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 font-medium text-sm transition-colors"
        >
          View Full Release Notes →
        </button>
      </div>
    </div>
  </div>
</div>

<style>
  .prose {
    line-height: 1.7;
  }
  
  .prose h1, .prose h2, .prose h3, .prose h4 {
    margin-top: 1.5em;
    margin-bottom: 0.5em;
  }
  
  .prose ul, .prose ol {
    margin: 1em 0;
    padding-left: 1.5em;
  }
  
  .prose li {
    margin: 0.25em 0;
  }
  
  .prose code {
    background-color: rgb(243 244 246);
    padding: 0.125em 0.25em;
    border-radius: 0.25rem;
    font-size: 0.875em;
  }
  
  .dark .prose code {
    background-color: rgb(55 65 81);
  }
</style>
