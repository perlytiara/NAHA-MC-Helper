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
  export let currentVersion = '1.0.3';
  export let isInstalling = false;
  export let showRestartPrompt = false;
  
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
          {:else if showRestartPrompt}
            <span class="check-icon">✨</span>
          {:else if downloadProgress || isInstalling}
            <span class="available-icon">📥</span>
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
          {:else if showRestartPrompt}
            Ready to Restart
          {:else if isInstalling}
            Installing Update
          {:else if downloadProgress}
            Downloading Update
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
        {:else if showRestartPrompt}
          <!-- Restart Prompt -->
          <div class="restart-prompt">
            <svg class="cute-cat" viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
              <!-- Cat body (more round and cute) -->
              <ellipse cx="100" cy="130" rx="50" ry="45" fill="#10b981" class="cat-body"/>
              
              <!-- Cat head (bigger and rounder) -->
              <circle cx="100" cy="80" r="40" fill="#10b981" class="cat-head"/>
              
              <!-- Cat ears (rounded) -->
              <ellipse cx="75" cy="50" rx="18" ry="25" fill="#10b981" class="ear-left"/>
              <ellipse cx="125" cy="50" rx="18" ry="25" fill="#10b981" class="ear-right"/>
              
              <!-- Inner ears (pink/light green) -->
              <ellipse cx="75" cy="55" rx="10" ry="15" fill="#6ee7b7" class="inner-ear"/>
              <ellipse cx="125" cy="55" rx="10" ry="15" fill="#6ee7b7" class="inner-ear"/>
              
              <!-- Cat tail (curly and cute) -->
              <path d="M 145 135 Q 170 130 175 110 Q 178 90 172 75" stroke="#10b981" stroke-width="14" fill="none" stroke-linecap="round" class="tail"/>
              
              <!-- Cat eyes (bigger and cuter) -->
              <ellipse cx="85" cy="75" rx="10" ry="14" fill="white"/>
              <ellipse cx="115" cy="75" rx="10" ry="14" fill="white"/>
              
              <!-- Cat pupils (gentle swing) -->
              <circle cx="85" cy="77" r="6" fill="#064e3b">
                <animate attributeName="cx" values="83;87;83" dur="2.5s" repeatCount="indefinite"/>
              </circle>
              <circle cx="115" cy="77" r="6" fill="#064e3b">
                <animate attributeName="cx" values="113;117;113" dur="2.5s" repeatCount="indefinite"/>
              </circle>
              
              <!-- Cute highlights in eyes -->
              <circle cx="87" cy="74" r="3" fill="white" opacity="0.8"/>
              <circle cx="117" cy="74" r="3" fill="white" opacity="0.8"/>
              
              <!-- Cat nose (cute triangle) -->
              <path d="M 100 88 L 96 94 L 104 94 Z" fill="#059669"/>
              
              <!-- Cat mouth (cute smile) -->
              <path d="M 100 94 Q 92 100 85 98 M 100 94 Q 108 100 115 98" stroke="#059669" stroke-width="2.5" fill="none" stroke-linecap="round" class="mouth"/>
              
              <!-- Blush marks -->
              <ellipse cx="65" cy="85" rx="8" ry="5" fill="#f472b6" opacity="0.3"/>
              <ellipse cx="135" cy="85" rx="8" ry="5" fill="#f472b6" opacity="0.3"/>
              
              <!-- Cat whiskers (delicate) -->
              <line x1="55" y1="80" x2="35" y2="78" stroke="#047857" stroke-width="1.5" opacity="0.7" class="whisker"/>
              <line x1="55" y1="86" x2="35" y2="88" stroke="#047857" stroke-width="1.5" opacity="0.7" class="whisker"/>
              <line x1="145" y1="80" x2="165" y2="78" stroke="#047857" stroke-width="1.5" opacity="0.7" class="whisker"/>
              <line x1="145" y1="86" x2="165" y2="88" stroke="#047857" stroke-width="1.5" opacity="0.7" class="whisker"/>
            </svg>
            
            <p class="restart-message">Update installed! Ready to restart</p>
          </div>
        {:else if downloadProgress || isInstalling}
          <!-- Download/Install with Cat Animation -->
          <div class="update-animation">
            <svg class="cute-cat" viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
              <!-- Cat body (more round and cute) -->
              <ellipse cx="100" cy="130" rx="50" ry="45" fill="#10b981" class="cat-body"/>
              
              <!-- Cat head (bigger and rounder) -->
              <circle cx="100" cy="80" r="40" fill="#10b981" class="cat-head"/>
              
              <!-- Cat ears (rounded) -->
              <ellipse cx="75" cy="50" rx="18" ry="25" fill="#10b981" class="ear-left"/>
              <ellipse cx="125" cy="50" rx="18" ry="25" fill="#10b981" class="ear-right"/>
              
              <!-- Inner ears (pink/light green) -->
              <ellipse cx="75" cy="55" rx="10" ry="15" fill="#6ee7b7" class="inner-ear"/>
              <ellipse cx="125" cy="55" rx="10" ry="15" fill="#6ee7b7" class="inner-ear"/>
              
              <!-- Cat tail (curly and cute) -->
              <path d="M 145 135 Q 170 130 175 110 Q 178 90 172 75" stroke="#10b981" stroke-width="14" fill="none" stroke-linecap="round" class="tail"/>
              
              <!-- Cat eyes (bigger and cuter) -->
              <ellipse cx="85" cy="75" rx="10" ry="14" fill="white"/>
              <ellipse cx="115" cy="75" rx="10" ry="14" fill="white"/>
              
              <!-- Cat pupils (gentle swing) -->
              <circle cx="85" cy="77" r="6" fill="#064e3b">
                <animate attributeName="cx" values="83;87;83" dur="2.5s" repeatCount="indefinite"/>
              </circle>
              <circle cx="115" cy="77" r="6" fill="#064e3b">
                <animate attributeName="cx" values="113;117;113" dur="2.5s" repeatCount="indefinite"/>
              </circle>
              
              <!-- Cute highlights in eyes -->
              <circle cx="87" cy="74" r="3" fill="white" opacity="0.8"/>
              <circle cx="117" cy="74" r="3" fill="white" opacity="0.8"/>
              
              <!-- Cat nose (cute triangle) -->
              <path d="M 100 88 L 96 94 L 104 94 Z" fill="#059669"/>
              
              <!-- Cat mouth (cute smile) -->
              <path d="M 100 94 Q 92 100 85 98 M 100 94 Q 108 100 115 98" stroke="#059669" stroke-width="2.5" fill="none" stroke-linecap="round" class="mouth"/>
              
              <!-- Blush marks -->
              <ellipse cx="65" cy="85" rx="8" ry="5" fill="#f472b6" opacity="0.3"/>
              <ellipse cx="135" cy="85" rx="8" ry="5" fill="#f472b6" opacity="0.3"/>
              
              <!-- Cat whiskers (delicate) -->
              <line x1="55" y1="80" x2="35" y2="78" stroke="#047857" stroke-width="1.5" opacity="0.7" class="whisker"/>
              <line x1="55" y1="86" x2="35" y2="88" stroke="#047857" stroke-width="1.5" opacity="0.7" class="whisker"/>
              <line x1="145" y1="80" x2="165" y2="78" stroke="#047857" stroke-width="1.5" opacity="0.7" class="whisker"/>
              <line x1="145" y1="86" x2="165" y2="88" stroke="#047857" stroke-width="1.5" opacity="0.7" class="whisker"/>
            </svg>
            
            {#if downloadProgress}
              <div class="progress-display">
                <span class="progress-percent">{downloadProgress.percent?.toFixed(0) || 0}%</span>
                <div class="progress-bar">
                  <div class="progress-fill" style="width: {downloadProgress.percent || 0}%"></div>
                </div>
              </div>
            {:else}
              <p class="status-text">Installing...</p>
            {/if}
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
            
            <div class="release-notes">
              <h3>{$t('updateModal.whatsNew')}</h3>
              <div class="release-notes-content">
                {#each (updateInfo.releaseNotes || 'Bug fixes and improvements').split('\n').filter((line: string) => line.trim()) as line}
                  <p class="release-note-item">{line}</p>
                {/each}
              </div>
            </div>
          </div>
        {:else}
          <p class="up-to-date-text">{$t('updateModal.latestVersion')} (v{currentVersion})</p>
        {/if}
      </div>

      <div class="modal-footer">
        {#if showRestartPrompt}
          <button class="install-button" on:click={handleInstall}>
            🔄 Restart Now
          </button>
          <button class="close-button-footer" on:click={handleClose}>
            Later
          </button>
        {:else if downloadProgress || isInstalling}
          <!-- No buttons during download/install -->
        {:else if updateInfo}
          <button class="download-button" on:click={handleDownload}>
            📥 {$t('updateModal.downloadUpdate')}
          </button>
        {:else if !isChecking}
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
    padding: 1.5rem;
    max-width: 420px;
    width: 90%;
    max-height: fit-content;
    overflow: hidden;
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
    gap: 0.75rem;
    margin-bottom: 1rem;
  }

  .update-icon {
    font-size: 2.5rem;
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
    margin-bottom: 1rem;
    color: rgba(255, 255, 255, 0.9);
    min-height: fit-content;
    max-height: fit-content;
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

  /* Update Animation Styles */
  .update-animation,
  .restart-prompt {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 0;
  }

  .cute-cat {
    width: 120px;
    height: 120px;
    margin: 0 auto;
    animation: gentle-bounce 2.5s ease-in-out infinite;
  }

  @keyframes gentle-bounce {
    0%, 100% { transform: translateY(0px); }
    50% { transform: translateY(-8px); }
  }

  .cute-cat .cat-body {
    animation: body-wiggle 3s ease-in-out infinite;
    transform-origin: center;
  }

  @keyframes body-wiggle {
    0%, 100% { transform: scaleX(1); }
    50% { transform: scaleX(1.05); }
  }

  .cute-cat .cat-head {
    animation: head-tilt 3s ease-in-out infinite;
    transform-origin: 100px 80px;
  }

  @keyframes head-tilt {
    0%, 100% { transform: rotate(0deg); }
    25% { transform: rotate(-3deg); }
    75% { transform: rotate(3deg); }
  }

  .cute-cat .ear-left,
  .cute-cat .ear-right {
    animation: ear-wiggle 2s ease-in-out infinite;
    transform-origin: center;
  }

  .cute-cat .ear-right {
    animation-delay: 0.3s;
  }

  @keyframes ear-wiggle {
    0%, 100% { transform: rotate(0deg) scaleY(1); }
    50% { transform: rotate(5deg) scaleY(1.1); }
  }

  .cute-cat .tail {
    animation: tail-sway 2.5s ease-in-out infinite;
    transform-origin: 145px 135px;
  }

  @keyframes tail-sway {
    0%, 100% { transform: rotate(0deg); }
    50% { transform: rotate(15deg); }
  }

  .cute-cat .mouth {
    animation: smile-blink 4s ease-in-out infinite;
  }

  @keyframes smile-blink {
    0%, 90%, 100% { opacity: 1; }
    95% { opacity: 0.5; }
  }

  .cute-cat .whisker {
    animation: whisker-twitch 3.5s ease-in-out infinite;
  }

  @keyframes whisker-twitch {
    0%, 100% { opacity: 0.7; }
    50% { opacity: 1; }
  }

  .restart-message,
  .status-text {
    font-size: 0.9rem;
    color: rgba(255, 255, 255, 0.9);
    text-align: center;
    margin: 0;
    font-weight: 500;
  }

  .progress-display {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
  }

  .progress-percent {
    font-size: 1.5rem;
    font-weight: 700;
    color: #10b981;
    font-family: 'Segoe UI', system-ui, sans-serif;
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
    border-radius: 4px;
    transition: width 0.4s ease-out;
  }

  @media (max-width: 640px) {
    .modal-content {
      padding: 1.25rem;
      max-width: 95%;
    }

    .modal-title {
      font-size: 1.15rem;
    }

    .version-comparison {
      flex-direction: column;
      gap: 0.5rem;
    }

    .arrow {
      transform: rotate(90deg);
    }

    .cute-cat {
      width: 100px;
      height: 100px;
    }

    .progress-percent {
      font-size: 1.25rem;
    }
  }
</style>

