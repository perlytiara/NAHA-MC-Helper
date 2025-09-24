<script>
  export let progress = 0;
  export let transferred = 0;
  export let total = 0;
  export let bytesPerSecond = 0;

  $: progressPercentage = Math.min(100, Math.max(0, progress));
  $: downloadedMB = (transferred / (1024 * 1024)).toFixed(1);
  $: totalMB = (total / (1024 * 1024)).toFixed(1);
  $: speedMBps = (bytesPerSecond / (1024 * 1024)).toFixed(1);

  function formatTime(seconds) {
    if (seconds < 60) {
      return `${Math.round(seconds)}s`;
    } else if (seconds < 3600) {
      return `${Math.round(seconds / 60)}m ${Math.round(seconds % 60)}s`;
    } else {
      return `${Math.round(seconds / 3600)}h ${Math.round((seconds % 3600) / 60)}m`;
    }
  }

  $: remainingBytes = total - transferred;
  $: remainingTime = bytesPerSecond > 0 ? remainingBytes / bytesPerSecond : 0;
</script>

<div class="space-y-4">
  <!-- Progress Bar -->
  <div class="relative">
    <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-3 overflow-hidden">
      <div 
        class="h-full bg-gradient-to-r from-blue-500 to-purple-600 rounded-full transition-all duration-300 ease-out relative"
        style="width: {progressPercentage}%"
      >
        <!-- Animated shine effect -->
        <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent animate-pulse"></div>
      </div>
    </div>
    
    <!-- Progress percentage -->
    <div class="absolute top-0 right-0 -mt-6 text-sm font-medium text-gray-600 dark:text-gray-300">
      {progressPercentage}%
    </div>
  </div>

  <!-- Download Stats -->
  <div class="grid grid-cols-2 gap-4 text-sm">
    <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-3">
      <div class="flex items-center space-x-2 mb-1">
        <span class="text-blue-600 dark:text-blue-400">📦</span>
        <span class="font-medium text-gray-700 dark:text-gray-300">Downloaded</span>
      </div>
      <div class="text-lg font-mono text-gray-900 dark:text-gray-100">
        {downloadedMB} MB
      </div>
      <div class="text-xs text-gray-500 dark:text-gray-400">
        of {totalMB} MB
      </div>
    </div>

    <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-3">
      <div class="flex items-center space-x-2 mb-1">
        <span class="text-green-600 dark:text-green-400">⚡</span>
        <span class="font-medium text-gray-700 dark:text-gray-300">Speed</span>
      </div>
      <div class="text-lg font-mono text-gray-900 dark:text-gray-100">
        {speedMBps} MB/s
      </div>
      <div class="text-xs text-gray-500 dark:text-gray-400">
        {#if remainingTime > 0}
          {formatTime(remainingTime)} remaining
        {:else}
          Almost done!
        {/if}
      </div>
    </div>
  </div>

  <!-- Progress Animation -->
  <div class="flex justify-center">
    <div class="flex space-x-1">
      {#each Array(3) as _, i}
        <div 
          class="w-2 h-2 bg-blue-500 rounded-full animate-bounce"
          style="animation-delay: {i * 0.1}s"
        ></div>
      {/each}
    </div>
  </div>

  <!-- Status Message -->
  <div class="text-center">
    <p class="text-gray-600 dark:text-gray-300 text-sm">
      {#if progressPercentage < 10}
        Starting download...
      {:else if progressPercentage < 50}
        Downloading update files...
      {:else if progressPercentage < 90}
        Almost there...
      {:else if progressPercentage < 100}
        Finalizing download...
      {:else}
        Download complete!
      {/if}
    </p>
  </div>
</div>

<style>
  @keyframes bounce {
    0%, 20%, 53%, 80%, 100% {
      transform: translate3d(0,0,0);
    }
    40%, 43% {
      transform: translate3d(0, -8px, 0);
    }
    70% {
      transform: translate3d(0, -4px, 0);
    }
    90% {
      transform: translate3d(0, -2px, 0);
    }
  }

  .animate-bounce {
    animation: bounce 1.4s infinite;
  }

  .animate-pulse {
    animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }

  @keyframes pulse {
    0%, 100% {
      opacity: 1;
    }
    50% {
      opacity: .5;
    }
  }
</style>
