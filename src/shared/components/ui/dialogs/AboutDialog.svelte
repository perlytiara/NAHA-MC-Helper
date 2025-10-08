<script>
  import { onMount, createEventDispatcher } from 'svelte';
  import { t } from '../../../stores/i18nStore';
  
  export let show = false;
  
  const dispatch = createEventDispatcher();
  
  let appInfo = null;
  let loading = true;
  
  onMount(async () => {
    if (window.nahaAPI?.getAppInfo) {
      try {
        appInfo = await window.nahaAPI.getAppInfo();
      } catch (error) {
        console.error('Failed to get app info:', error);
        appInfo = {
          name: 'NAHA MC Helper',
          version: '1.0.0',
          description: 'Minecraft modpack helper application',
          author: 'Unknown'
        };
      }
    }
    loading = false;
  });
  
  function close() {
    show = false;
    dispatch('close');
  }
  
  function handleKeydown(event) {
    if (event.key === 'Escape') {
      close();
    }
  }
  
  function formatAppName(name) {
    return name.replace(/-/g, ' ').replace(/\b\w/g, l => l.toUpperCase());
  }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if show}
  <div class="modal-backdrop" on:click={close} role="button" tabindex="-1" on:keydown={(e) => e.key === 'Enter' && close()}>
    <div class="about-dialog" on:click|stopPropagation role="dialog" tabindex="-1" on:keydown={(e) => e.key === 'Enter' && e.stopPropagation()}>
      <div class="dialog-header">
        <div class="app-icon">⚡</div>
        <button class="close-button" on:click={close} aria-label="Close dialog">
          <span class="close-icon">×</span>
        </button>
      </div>
      
      {#if loading}
        <div class="loading-content">
          <div class="loading-spinner"></div>
          <p>{$t('common.loading')}</p>
        </div>
      {:else if appInfo}
        <div class="dialog-content">
          <h1 class="app-title">{formatAppName(appInfo.name)}</h1>
          <p class="app-version">{$t('dialogs.about.version')} {appInfo.version}</p>
          
          {#if appInfo.description}
            <p class="app-description">{appInfo.description}</p>
          {/if}
          
          {#if appInfo.author && appInfo.author !== ''}
            <p class="app-author">by {appInfo.author}</p>
          {/if}
          
          <div class="system-info">
            <h3>System Information</h3>
            <div class="info-grid">
              <div class="info-item">
                <span class="info-label">Platform:</span>
                <span class="info-value">{appInfo.platform} ({appInfo.arch})</span>
              </div>
              {#if appInfo.electron}
                <div class="info-item">
                  <span class="info-label">Electron:</span>
                  <span class="info-value">{appInfo.electron}</span>
                </div>
              {/if}
              {#if appInfo.chrome}
                <div class="info-item">
                  <span class="info-label">Chromium:</span>
                  <span class="info-value">{appInfo.chrome}</span>
                </div>
              {/if}
              {#if appInfo.node}
                <div class="info-item">
                  <span class="info-label">Node.js:</span>
                  <span class="info-value">{appInfo.node}</span>
                </div>
              {/if}
            </div>
          </div>
        </div>
      {:else}
        <div class="error-content">
          <p>Unable to load application information.</p>
        </div>
      {/if}
      
      <div class="dialog-footer">
        <button class="ok-button" on:click={close}>OK</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 10000;
    animation: fadeIn 0.2s ease-out;
  }
  
  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
  
  .about-dialog {
    background: rgba(30, 30, 30, 0.95);
    backdrop-filter: blur(20px);
    border: 1px solid rgba(255,255,255,0.2);
    border-radius: 16px;
    box-shadow: 0 8px 32px rgba(0,0,0,0.5);
    width: 90%;
    max-width: 500px;
    max-height: 80vh;
    overflow: hidden;
    animation: slideIn 0.3s ease-out;
    position: relative;
  }
  
  @keyframes slideIn {
    from {
      opacity: 0;
      transform: scale(0.9) translateY(-20px);
    }
    to {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
  }
  
  .dialog-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1.5rem 2rem 1rem;
    border-bottom: 1px solid rgba(255,255,255,0.1);
  }
  
  .app-icon {
    width: 48px;
    height: 48px;
    background: linear-gradient(135deg, #3b82f6, #8b5cf6);
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 24px;
    color: #ffffff;
    box-shadow: 0 4px 16px rgba(59, 130, 246, 0.3);
  }
  
  .close-button {
    background: none;
    border: none;
    cursor: pointer;
    padding: 8px;
    border-radius: 8px;
    transition: all 0.2s ease;
    color: #e5e7eb;
  }
  
  .close-button:hover {
    background: rgba(255,255,255,0.1);
    color: #ffffff;
  }
  
  .close-icon {
    font-size: 24px;
    font-weight: bold;
    line-height: 1;
    color: inherit;
  }
  
  .dialog-content {
    padding: 1.5rem 2rem;
    text-align: center;
  }
  
  .loading-content {
    padding: 2rem;
    text-align: center;
    color: #e5e7eb;
  }
  
  .loading-spinner {
    width: 32px;
    height: 32px;
    border: 3px solid rgba(59, 130, 246, 0.2);
    border-top: 3px solid #3b82f6;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin: 0 auto 1rem;
  }
  
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  
  .error-content {
    padding: 2rem;
    text-align: center;
    color: #e5e7eb;
  }
  
  .app-title {
    font-size: 2rem;
    font-weight: 700;
    margin: 0 0 0.5rem;
    background: linear-gradient(135deg, #3b82f6, #8b5cf6);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    color: #3b82f6; /* Fallback for browsers that don't support gradient text */
  }
  
  .app-version {
    font-size: 1.125rem;
    color: #e5e7eb;
    opacity: 0.9;
    margin: 0 0 1rem;
    font-weight: 500;
  }
  
  .app-description {
    font-size: 1rem;
    color: #d1d5db;
    opacity: 0.9;
    margin: 0 0 0.5rem;
    line-height: 1.5;
  }
  
  .app-author {
    font-size: 0.875rem;
    color: #9ca3af;
    opacity: 0.9;
    margin: 0 0 1.5rem;
    font-style: italic;
  }
  
  .system-info {
    margin-top: 1.5rem;
    text-align: left;
  }
  
  .system-info h3 {
    font-size: 1rem;
    font-weight: 600;
    color: #f3f4f6;
    margin: 0 0 1rem;
    text-align: center;
  }
  
  .info-grid {
    display: grid;
    gap: 0.75rem;
  }
  
  .info-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 0;
    border-bottom: 1px solid rgba(255,255,255,0.1);
  }
  
  .info-item:last-child {
    border-bottom: none;
  }
  
  .info-label {
    font-weight: 500;
    color: #e5e7eb;
    opacity: 0.9;
  }
  
  .info-value {
    font-family: monospace;
    font-size: 0.875rem;
    color: #f3f4f6;
    opacity: 0.9;
    background: rgba(255,255,255,0.1);
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
  }
  
  .dialog-footer {
    padding: 1rem 2rem 1.5rem;
    border-top: 1px solid rgba(255,255,255,0.1);
    text-align: center;
  }
  
  .ok-button {
    background: linear-gradient(135deg, #3b82f6, #8b5cf6);
    color: white;
    border: none;
    padding: 0.75rem 2rem;
    border-radius: 8px;
    cursor: pointer;
    font-weight: 500;
    font-size: 1rem;
    transition: all 0.2s ease;
    min-width: 100px;
  }
  
  .ok-button:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
  }
  
  
  /* Responsive design */
  @media (max-width: 640px) {
    .about-dialog {
      width: 95%;
      margin: 1rem;
    }
    
    .dialog-header {
      padding: 1rem 1.5rem 0.75rem;
    }
    
    .dialog-content {
      padding: 1rem 1.5rem;
    }
    
    .dialog-footer {
      padding: 0.75rem 1.5rem 1rem;
    }
    
    .app-title {
      font-size: 1.5rem;
    }
    
    .app-version {
      font-size: 1rem;
    }
  }
</style>
