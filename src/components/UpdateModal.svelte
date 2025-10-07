<script>
  import { createEventDispatcher } from 'svelte';
  
  export let isVisible = false;
  export let updateInfo = null;
  export let downloadProgress = null;
  export let isDownloading = false;
  export let isChecking = false;
  export let error = null;
  export let currentVersion = '1.2.2';
  
  const dispatch = createEventDispatcher();
  
  function handleDownload() {
    console.log('🔥 UpdateModal: handleDownload called!');
    console.log('🔥 UpdateModal: Current state - updateInfo:', !!updateInfo, 'isDownloading:', isDownloading, 'isChecking:', isChecking);
    dispatch('download');
  }
  
  function handleInstall() {
    dispatch('install');
  }
  
  function handleLater() {
    dispatch('later');
  }
  
  function handleViewReleaseNotes() {
    dispatch('viewReleaseNotes');
  }
  
  function handleClose() {
    dispatch('close');
  }
  
  function formatBytes(bytes) {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }
  
  function formatSpeed(bytesPerSecond) {
    return formatBytes(bytesPerSecond) + '/s';
  }
</script>

{#if isVisible}
  <div class="modal modal-open">
    <div class="modal-box max-w-2xl">
      <div class="flex justify-between items-center mb-4">
        <h3 class="text-2xl font-bold text-primary">
          {#if isChecking}
            <span class="loading loading-spinner loading-md mr-2"></span>
            Checking for Updates...
          {:else if updateInfo}
            <span class="text-success">🎉 Update Available!</span>
          {:else if downloadProgress}
            <span class="loading loading-spinner loading-md mr-2"></span>
            Downloading Update...
          {:else if error}
            <span class="text-error">⚠️ Update Error</span>
          {:else}
            <span class="text-info">ℹ️ No Updates Available</span>
          {/if}
        </h3>
        <button class="btn btn-sm btn-circle btn-ghost" on:click={handleClose}>
          ✕
        </button>
      </div>
      
      <div class="space-y-4">
        {#if updateInfo}
          <!-- Update Available -->
          <div class="bg-success/10 border border-success/20 rounded-lg p-4">
            <div class="flex items-center gap-3 mb-3">
              <div class="badge badge-success badge-lg">NEW</div>
              <span class="text-lg font-semibold">Version {updateInfo.version}</span>
            </div>
            <p class="text-base-content/70 mb-2">
              You're currently running version <strong>{currentVersion}</strong>
            </p>
            <!-- Development mode notice -->
            <div class="alert alert-info text-sm mb-3">
              <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-4 w-4" fill="none" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
              </svg>
              <span>In development mode, the download will open GitHub releases page for manual installation.</span>
            </div>
            {#if updateInfo.releaseNotes}
              <div class="collapse collapse-arrow bg-base-100">
                <input type="checkbox" />
                <div class="collapse-title text-sm font-medium">
                  View Release Notes
                </div>
                <div class="collapse-content">
                  <div class="text-sm whitespace-pre-wrap">{updateInfo.releaseNotes}</div>
                </div>
              </div>
            {/if}
          </div>
          
          <div class="flex gap-3 justify-end">
            <button class="btn btn-outline" on:click={handleLater}>
              Later
            </button>
            <button class="btn btn-primary" on:click={handleDownload}>
              {#if isDownloading}
                <span class="loading loading-spinner loading-sm mr-2"></span>
                Downloading...
              {:else}
                📥 Download Update
              {/if}
            </button>
          </div>
          
        {:else if downloadProgress}
          <!-- Download Progress -->
          <div class="space-y-3">
            <div class="flex justify-between items-center">
              <span class="text-sm font-medium">Downloading {updateInfo?.version || 'update'}...</span>
              <span class="text-sm text-base-content/70">
                {downloadProgress.percent}%
              </span>
            </div>
            
            <progress 
              class="progress progress-primary w-full" 
              value={downloadProgress.percent} 
              max="100"
            ></progress>
            
            <div class="flex justify-between text-xs text-base-content/60">
              <span>{formatBytes(downloadProgress.transferred)} / {formatBytes(downloadProgress.total)}</span>
              <span>{formatSpeed(downloadProgress.bytesPerSecond)}</span>
            </div>
          </div>
          
        {:else if error}
          <!-- Error State -->
          <div class="alert alert-error">
            <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <div>
              <h3 class="font-bold">Update Check Failed</h3>
              <div class="text-xs">{error.message || 'An unknown error occurred'}</div>
            </div>
          </div>
          
          <div class="flex justify-end">
            <button class="btn btn-outline" on:click={handleClose}>
              Close
            </button>
          </div>
          
        {:else if isChecking}
          <!-- Checking State -->
          <div class="flex flex-col items-center py-8">
            <span class="loading loading-spinner loading-lg mb-4"></span>
            <p class="text-base-content/70">Checking for updates...</p>
          </div>
          
        {:else}
          <!-- No Updates -->
          <div class="text-center py-8">
            <div class="text-6xl mb-4">✅</div>
            <h4 class="text-xl font-semibold mb-2">You're up to date!</h4>
            <p class="text-base-content/70 mb-4">
              NAHA MC Helper {currentVersion} is the latest version.
            </p>
            <button class="btn btn-primary" on:click={handleClose}>
              Continue
            </button>
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}
