<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { t } from '../../stores/i18nStore';
  // Import from global types
  type UpdateInfo = any;
  type DownloadProgress = any;
  type UpdateError = any;

  const dispatch = createEventDispatcher();

  export let show = false;
  export let updateInfo: UpdateInfo | null = null;
  export let downloadProgress: DownloadProgress | null = null;
  export let isChecking = false;
  export let error: UpdateError | null = null;
  export let currentVersion = '1.0.1';
  
  function handleDownload() {
    dispatch('download');
  }

  function handleInstall() {
    dispatch('install');
  }

  function handleClose() {
    dispatch('close');
  }
</script>

{#if show}
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <div class="modal-overlay" on:click={handleClose} on:keydown={(e) => e.key === 'Escape' && handleClose()} role="dialog" aria-modal="true" tabindex="-1">
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="modal-content" on:click|stopPropagation on:keydown|stopPropagation role="document">
      <button class="close-button" on:click={handleClose}>✕</button>
      
      <div class="modal-header">
        <div class="update-icon">
          {#if isChecking}
            <span class="spinner">🔄</span>
          {:else if error}
            <span class="error-icon">⚠️</span>
          {:else if updateInfo}
            <span class="available-icon">📥</span>
          {:else}
            <span class="check-icon">✅</span>
          {/if}
        </div>
        <h2 class="modal-title">
          {#if isChecking}
            {$t('updateModal.checking')}
          {:else if error}
            {$t('updateModal.updateError')}
          {:else if updateInfo}
            {$t('updateModal.updateAvailable')}
          {:else}
            {$t('updateModal.noUpdates')}
          {/if}
        </h2>
      </div>

      <div class="modal-body">
        {#if isChecking}
          <p class="checking-text">{$t('updateModal.lookingForLatest')}</p>
        {:else if error}
          <div class="error-message">
            <p>{$t('updateModal.failedToCheck')}</p>
            <p class="error-details">{error.message}</p>
          </div>
        {:else if updateInfo}
          <div class="update-info">
            <div class="version-comparison">
              <div class="version-box current">
                <span class="version-label">Current</span>
                <span class="version-number">v{currentVersion}</span>
              </div>
              <span class="arrow">→</span>
              <div class="version-box new">
                <span class="version-label">New</span>
                <span class="version-number">v{updateInfo.version}</span>
              </div>
            </div>
            
            {#if downloadProgress}
              <div class="download-progress">
                <div class="progress-header">
                  <span>{$t('updateModal.downloadingUpdate')}</span>
                  <span class="progress-percent">{downloadProgress.percent?.toFixed(0)}%</span>
                </div>
                <div class="progress-bar">
                  <div class="progress-fill" style="width: {downloadProgress.percent}%"></div>
                </div>
                <div class="progress-stats">
                  <span class="transferred">{(downloadProgress.transferred / 1024 / 1024).toFixed(1)} MB</span>
                  <span class="total">of {(downloadProgress.total / 1024 / 1024).toFixed(1)} MB</span>
                </div>
              </div>
            {:else}
              <div class="release-notes">
                <h3>{$t('updateModal.whatsNew')}</h3>
                <div class="release-notes-content">
                  {#each (updateInfo.releaseNotes || 'Bug fixes and improvements').split('\n').filter(line => line.trim()) as line}
                    <p class="release-note-item">{line}</p>
                  {/each}
                </div>
              </div>
            {/if}
          </div>
        {:else}
          <p class="up-to-date-text">{$t('updateModal.latestVersion')} (v{currentVersion})</p>
        {/if}
      </div>

      <div class="modal-footer">
        {#if updateInfo?.readyToRestart}
          <button class="install-button" on:click={handleInstall}>
            🔄 Restart Now
          </button>
          <button class="close-button-footer" on:click={handleClose}>
            Later
          </button>
        {:else if updateInfo && !downloadProgress}
          <button class="download-button" on:click={handleDownload}>
            📥 {$t('updateModal.downloadUpdate')}
          </button>
        {:else if !isChecking && !downloadProgress}
          <button class="close-button-footer" on:click={handleClose}>
            {$t('updateModal.close')}
          </button>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    animation: fadeIn 0.2s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .modal-content {
    background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
    border: 2px solid rgba(255, 255, 255, 0.1);
    border-radius: 16px;
    padding: 2rem;
    max-width: 500px;
    width: 90%;
    max-height: 80vh;
    overflow-y: auto;
    position: relative;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
    animation: slideIn 0.3s ease;
  }

  @keyframes slideIn {
    from {
      transform: translateY(-20px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }

  .close-button {
    position: absolute;
    top: 1rem;
    right: 1rem;
    background: rgba(255, 255, 255, 0.1);
    border: none;
    color: white;
    width: 32px;
    height: 32px;
    border-radius: 50%;
    cursor: pointer;
    font-size: 1.2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s ease;
  }

  .close-button:hover {
    background: rgba(255, 255, 255, 0.2);
    transform: rotate(90deg);
  }

  .modal-header {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .update-icon {
    font-size: 3rem;
  }

  .spinner {
    display: inline-block;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .modal-title {
    font-size: 1.5rem;
    font-weight: 700;
    color: white;
    margin: 0;
    text-align: center;
  }

  .modal-body {
    margin-bottom: 1.5rem;
    color: rgba(255, 255, 255, 0.9);
  }

  .checking-text,
  .up-to-date-text {
    text-align: center;
    font-size: 1rem;
    color: rgba(255, 255, 255, 0.8);
  }

  .error-message {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: 8px;
    padding: 1rem;
  }

  .error-details {
    color: #ef4444;
    font-size: 0.9rem;
    margin-top: 0.5rem;
  }

  .update-info {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .version-comparison {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1rem;
  }

  .version-box {
    background: rgba(255, 255, 255, 0.08);
    border: 2px solid rgba(255, 255, 255, 0.15);
    border-radius: 12px;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    min-width: 120px;
  }

  .version-box.new {
    border-color: #10b981;
    background: rgba(16, 185, 129, 0.1);
  }

  .version-label {
    font-size: 0.75rem;
    color: rgba(255, 255, 255, 0.6);
    text-transform: uppercase;
    font-weight: 600;
  }

  .version-number {
    font-size: 1.25rem;
    font-weight: 700;
    color: white;
  }

  .arrow {
    font-size: 1.5rem;
    color: #10b981;
  }

  .download-progress {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .progress-header {
    display: flex;
    justify-content: space-between;
    font-size: 0.9rem;
    font-weight: 600;
  }

  .progress-percent {
    color: #10b981;
  }

  .progress-bar {
    width: 100%;
    height: 8px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #10b981, #059669);
    transition: width 0.3s ease;
  }

  .progress-stats {
    display: flex;
    justify-content: space-between;
    font-size: 0.8rem;
    color: rgba(255, 255, 255, 0.6);
  }

  .release-notes h3 {
    font-size: 1rem;
    font-weight: 600;
    margin: 0 0 0.75rem 0;
    color: #10b981;
  }

  .release-notes-content {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .release-note-item {
    font-size: 0.9rem;
    line-height: 1.5;
    color: rgba(255, 255, 255, 0.9);
    margin: 0;
    padding-left: 0.25rem;
  }

  .modal-footer {
    display: flex;
    justify-content: center;
    gap: 1rem;
  }

  .download-button,
  .install-button,
  .close-button-footer {
    border: none;
    border-radius: 8px;
    padding: 0.75rem 1.5rem;
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .download-button,
  .install-button {
    background: linear-gradient(135deg, #10b981, #059669);
    color: white;
    box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
  }

  .download-button:hover,
  .install-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(16, 185, 129, 0.4);
  }

  .close-button-footer {
    background: rgba(255, 255, 255, 0.1);
    color: white;
    border: 2px solid rgba(255, 255, 255, 0.2);
  }

  .close-button-footer:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  @media (max-width: 640px) {
    .modal-content {
      padding: 1.5rem;
      max-width: 95%;
    }

    .modal-title {
      font-size: 1.25rem;
    }

    .version-comparison {
      flex-direction: column;
      gap: 0.5rem;
    }

    .arrow {
      transform: rotate(90deg);
    }
  }
</style>

