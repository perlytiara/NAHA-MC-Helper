<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import type { MinecraftInstance } from '../types';
  
  export let instance: MinecraftInstance | null;
  export let modpackType: string | null;
  
  const dispatch = createEventDispatcher();
  
  let progressLog: string[] = [];
  let isUpdating = true;
  let isComplete = false;
  let hasError = false;
  let errorMessage = '';

  onMount(async () => {
    await startUpdate();
  });

  async function startUpdate() {
    if (!instance) return;
    
    isUpdating = true;
    progressLog = [];
    
    try {
      addLog(`🚀 Starting update for ${instance.name}`);
      addLog(`📦 Modpack type: ${modpackType}`);
      addLog(`📍 Instance path: ${instance.instance_path}`);
      addLog('');
      
      if (window.nahaAPI?.minecraftUpdater?.updateInstance) {
        addLog('🔄 Connecting to update service...');
        
        // Listen for progress events
        if (window.nahaAPI?.ipcRenderer) {
          window.nahaAPI.ipcRenderer.on('minecraft-updater:progress', (_event: any, message: string) => {
            addLog(message);
          });
        }
        
        addLog('📥 Downloading latest modpack...');
        
        const result = await window.nahaAPI.minecraftUpdater.updateInstance(
          instance.instance_path,
          modpackType || 'neoforge',
          null
        );
        
        if (result.success) {
          addLog('');
          addLog('✅ Update completed successfully!');
          addLog(`✨ Updated ${result.updated_mods?.length || 0} mods`);
          addLog(`➕ Added ${result.new_mods?.length || 0} new mods`);
          addLog(`🔒 Preserved ${result.preserved_mods?.length || 0} custom mods`);
          isComplete = true;
          
          setTimeout(() => {
            dispatch('complete');
          }, 2000);
        } else {
          throw new Error(result.error || 'Update failed');
        }
      } else {
        throw new Error('Update service not available');
      }
    } catch (e: any) {
      console.error('Update failed:', e);
      hasError = true;
      errorMessage = e.message;
      addLog('');
      addLog(`❌ Update failed: ${e.message}`);
    } finally {
      isUpdating = false;
    }
  }

  function addLog(message: string) {
    progressLog = [...progressLog, message];
    // Auto-scroll to bottom
    setTimeout(() => {
      const logContainer = document.querySelector('.progress-log');
      if (logContainer) {
        logContainer.scrollTop = logContainer.scrollHeight;
      }
    }, 50);
  }
</script>

<div class="update-progress">
  <div class="progress-header">
    <h2 class="title">
      {#if isUpdating}
        <span class="spinner-icon">🔄</span> Updating Instance...
      {:else if isComplete}
        <span class="success-icon">✅</span> Update Complete!
      {:else if hasError}
        <span class="error-icon">❌</span> Update Failed
      {/if}
    </h2>
    <p class="subtitle">{instance?.name}</p>
  </div>

  <div class="progress-content">
    {#if isUpdating || isComplete || hasError}
      <div class="progress-log">
        {#each progressLog as log}
          <div class="log-line">{log}</div>
        {/each}
        {#if isUpdating}
          <div class="log-line loading-dots">
            <span>Processing</span>
            <span class="dots">
              <span class="dot">.</span>
              <span class="dot">.</span>
              <span class="dot">.</span>
            </span>
          </div>
        {/if}
      </div>
    {/if}

    {#if isComplete}
      <div class="completion-message">
        <div class="success-animation">
          <div class="checkmark">✓</div>
        </div>
        <p class="success-text">Your instance has been updated successfully!</p>
        <p class="redirect-text">Redirecting you back...</p>
      </div>
    {:else if hasError}
      <div class="error-message">
        <p class="error-text">{errorMessage}</p>
        <button class="retry-button" on:click={startUpdate}>
          🔄 Try Again
        </button>
      </div>
    {/if}
  </div>
</div>

<style>
  .update-progress {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .progress-header {
    text-align: center;
  }

  .title {
    font-size: 1.5rem;
    font-weight: 800;
    background: linear-gradient(45deg, #10b981, #06b6d4);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }

  .spinner-icon {
    animation: spin 1s linear infinite;
    display: inline-block;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .success-icon,
  .error-icon {
    font-size: 1.2em;
  }

  .subtitle {
    font-size: 0.9rem;
    color: rgba(255, 255, 255, 0.7);
    margin: 0.5rem 0 0;
  }

  .progress-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .progress-log {
    background: rgba(0, 0, 0, 0.3);
    border: 2px solid rgba(16, 185, 129, 0.3);
    border-radius: 10px;
    padding: 1rem;
    max-height: 300px;
    overflow-y: auto;
    font-family: 'Courier New', monospace;
    font-size: 0.85rem;
    line-height: 1.6;
  }

  .progress-log::-webkit-scrollbar {
    width: 6px;
  }

  .progress-log::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 3px;
  }

  .progress-log::-webkit-scrollbar-thumb {
    background: rgba(16, 185, 129, 0.3);
    border-radius: 3px;
  }

  .log-line {
    color: rgba(255, 255, 255, 0.9);
    margin-bottom: 0.25rem;
  }

  .log-line:empty {
    margin-bottom: 0.75rem;
  }

  .loading-dots {
    display: flex;
    gap: 0.25rem;
    color: #10b981;
  }

  .dots {
    display: inline-flex;
  }

  .dot {
    animation: blink 1.4s infinite;
  }

  .dot:nth-child(2) {
    animation-delay: 0.2s;
  }

  .dot:nth-child(3) {
    animation-delay: 0.4s;
  }

  @keyframes blink {
    0%, 20%, 100% { opacity: 0; }
    50% { opacity: 1; }
  }

  .completion-message {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    padding: 2rem;
  }

  .success-animation {
    position: relative;
  }

  .checkmark {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    background: linear-gradient(135deg, #10b981, #059669);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 3rem;
    color: white;
    animation: scaleIn 0.5s ease-out;
    box-shadow: 0 0 40px rgba(16, 185, 129, 0.5);
  }

  @keyframes scaleIn {
    0% {
      transform: scale(0);
      opacity: 0;
    }
    50% {
      transform: scale(1.2);
    }
    100% {
      transform: scale(1);
      opacity: 1;
    }
  }

  .success-text {
    font-size: 1.1rem;
    font-weight: 600;
    color: #10b981;
    margin: 0;
  }

  .redirect-text {
    font-size: 0.9rem;
    color: rgba(255, 255, 255, 0.6);
    margin: 0;
    animation: fadeInOut 2s infinite;
  }

  @keyframes fadeInOut {
    0%, 100% { opacity: 0.4; }
    50% { opacity: 1; }
  }

  .error-message {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    padding: 1.5rem;
  }

  .error-text {
    color: #ef4444;
    font-size: 0.95rem;
    text-align: center;
    margin: 0;
  }

  .retry-button {
    background: linear-gradient(135deg, #10b981, #059669);
    border: none;
    border-radius: 8px;
    padding: 0.75rem 1.5rem;
    font-size: 0.9rem;
    font-weight: 600;
    color: white;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .retry-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(16, 185, 129, 0.4);
  }

  @media (max-width: 640px) {
    .title {
      font-size: 1.25rem;
    }

    .progress-log {
      max-height: 250px;
      font-size: 0.75rem;
      padding: 0.75rem;
    }

    .checkmark {
      width: 60px;
      height: 60px;
      font-size: 2.5rem;
    }
  }
</style>

