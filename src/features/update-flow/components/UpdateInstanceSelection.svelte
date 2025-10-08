<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { t } from '../../../shared/stores/i18nStore';
  import type { MinecraftInstance } from '../types';
  
  export let launcher: string | null;
  export let modpackType: string | null;
  
  const dispatch = createEventDispatcher();
  
  let instances: MinecraftInstance[] = [];
  let isLoading = true;
  let error: string | null = null;

  onMount(async () => {
    await loadInstances();
  });

  async function loadInstances() {
    isLoading = true;
    error = null;
    
    try {
      if (window.nahaAPI?.minecraftUpdater?.scanInstances) {
        const result = await window.nahaAPI.minecraftUpdater.scanInstances('json', launcher);
        
        if (result.success && result.instances) {
          // Filter by modpack type
          instances = result.instances.filter((instance: MinecraftInstance) => 
            instance.mod_loader.toLowerCase().includes(modpackType?.toLowerCase() || '')
          );
        }
      }
    } catch (e: any) {
      console.error('Failed to load instances:', e);
      error = e.message;
    } finally {
      isLoading = false;
    }
  }

  function selectInstance(instance: MinecraftInstance) {
    dispatch('instanceSelected', { instance });
  }

  function goBack() {
    dispatch('back');
  }
</script>

<div class="instance-selection">
  <h2 class="title">{$t('updateFlow.instanceSelection.title')}</h2>
  <p class="subtitle">{$t('updateFlow.instanceSelection.description')}</p>

  {#if isLoading}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>{$t('common.loading')}</p>
    </div>
  {:else if error}
    <div class="error-state">
      <span class="error-icon">⚠️</span>
      <p>{$t('updateFlow.errors.updateFailed')}: {error}</p>
      <button class="retry-button" on:click={loadInstances}>
        🔄 {$t('common.retry')}
      </button>
    </div>
  {:else if instances.length === 0}
    <div class="empty-state">
      <span class="empty-icon">📭</span>
      <p>{$t('updateFlow.errors.noInstances')}</p>
      <button class="back-button" on:click={goBack}>
        ← {$t('common.back')}
      </button>
    </div>
    {:else}
    <div class="instance-list-wrapper" class:no-scroll={instances.length <= 3}>
      <div class="fade-overlay fade-top"></div>
      <div class="instance-list-scroll">
        <div class="instance-list">
          {#each instances as instance}
            <button class="instance-card" on:click={() => selectInstance(instance)}>
              <div class="instance-header">
                <h3 class="instance-name">{instance.name}</h3>
                <span class="mod-count-badge">{instance.mod_count} mods</span>
              </div>
              <div class="instance-details">
                <div class="detail-item">
                  <span class="detail-label">{$t('updateFlow.instanceSelection.version')}</span>
                  <span class="detail-value">{instance.minecraft_version}</span>
                </div>
                <div class="detail-item">
                  <span class="detail-label">{$t('updateFlow.instanceSelection.loader')}</span>
                  <span class="detail-value">{instance.mod_loader}</span>
                </div>
              </div>
              <div class="instance-path">{instance.instance_path}</div>
              <div class="update-indicator">
                <span>{$t('updateFlow.instanceSelection.update')} →</span>
              </div>
            </button>
          {/each}
        </div>
      </div>
      <div class="fade-overlay fade-bottom"></div>
    </div>

    <div class="actions">
      <button class="back-button" on:click={goBack}>
        ← Back
      </button>
    </div>
  {/if}
</div>

<style>
  .instance-selection {
    display: flex;
    flex-direction: column;
    gap: 0.875rem;
    height: 100%;
    justify-content: flex-start;
    padding: 1.5rem 2rem 1.5rem 2rem;
  }

  .title {
    font-size: 1.5rem;
    font-weight: 800;
    text-align: center;
    background: linear-gradient(45deg, #10b981, #06b6d4);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin: 0;
  }

  .subtitle {
    font-size: 0.9rem;
    color: rgba(255, 255, 255, 0.7);
    text-align: center;
    margin: 0;
  }

  .loading-state,
  .error-state,
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    padding: 2rem;
    text-align: center;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 4px solid rgba(255, 255, 255, 0.1);
    border-top: 4px solid #10b981;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .error-icon,
  .empty-icon {
    font-size: 3rem;
  }

  .instance-list-wrapper {
    position: relative;
    flex: 1;
    margin: 0;
  }
  
  .instance-list-wrapper.no-scroll .fade-overlay {
    display: none;
  }

  .instance-list-scroll {
    max-height: 100%;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 0 0.5rem 0 0.5rem;
  }
  
  .instance-list-wrapper.no-scroll .instance-list-scroll {
    padding: 0;
    overflow-y: hidden;
  }

  .instance-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin: 0;
    padding: 0;
  }
  
  /* Fade overlays */
  .fade-overlay {
    position: absolute;
    left: 0;
    right: 0;
    height: 40px;
    pointer-events: none;
    z-index: 10;
    opacity: 0;
    transition: opacity 0.3s ease;
  }
  
  .fade-top {
    top: 0;
    background: linear-gradient(to bottom, rgba(0, 0, 0, 0.3), transparent);
  }
  
  .fade-bottom {
    bottom: 0;
    background: linear-gradient(to top, rgba(0, 0, 0, 0.3), transparent);
  }
  
  .instance-list-wrapper:not(.no-scroll) .fade-top,
  .instance-list-wrapper:not(.no-scroll) .fade-bottom {
    opacity: 1;
  }
  
  /* Custom scrollbar */
  .instance-list-scroll::-webkit-scrollbar {
    width: 4px;
  }

  .instance-list-scroll::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 2px;
  }

  .instance-list-scroll::-webkit-scrollbar-thumb {
    background: rgba(16, 185, 129, 0.4);
    border-radius: 2px;
  }

  .instance-list-scroll::-webkit-scrollbar-thumb:hover {
    background: rgba(16, 185, 129, 0.6);
  }

  .instance-card {
    background: rgba(255, 255, 255, 0.08);
    border: 2px solid rgba(255, 255, 255, 0.15);
    border-radius: 10px;
    padding: 0.75rem;
    cursor: pointer;
    transition: all 0.3s ease;
    text-align: left;
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    width: 100%;
    min-height: 110px;
  }

  .instance-card:hover {
    border-color: #10b981;
    box-shadow: 0 6px 20px rgba(16, 185, 129, 0.4);
    background: rgba(16, 185, 129, 0.15);
  }

  .instance-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
  }

  .instance-name {
    font-size: 1.1rem;
    font-weight: 700;
    color: white;
    margin: 0;
    flex: 1;
  }

  .mod-count-badge {
    background: rgba(16, 185, 129, 0.2);
    color: #10b981;
    padding: 0.25rem 0.75rem;
    border-radius: 12px;
    font-size: 0.75rem;
    font-weight: 600;
    border: 1px solid rgba(16, 185, 129, 0.3);
  }

  .instance-details {
    display: flex;
    gap: 1.5rem;
  }

  .detail-item {
    display: flex;
    gap: 0.5rem;
    font-size: 0.85rem;
  }

  .detail-label {
    color: rgba(255, 255, 255, 0.5);
    font-weight: 600;
  }

  .detail-value {
    color: rgba(255, 255, 255, 0.9);
  }

  .instance-path {
    font-size: 0.7rem;
    color: rgba(255, 255, 255, 0.4);
    font-family: monospace;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .update-indicator {
    position: absolute;
    right: 1rem;
    top: 50%;
    transform: translateY(-50%);
    color: #10b981;
    font-weight: 600;
    font-size: 0.9rem;
    opacity: 0;
    transition: all 0.3s ease;
  }

  .instance-card:hover .update-indicator {
    opacity: 1;
    right: 0.75rem;
  }

  .actions {
    display: flex;
    justify-content: center;
    margin-top: auto;
    padding-top: 0.5rem;
  }

  .back-button,
  .retry-button {
    background: rgba(255, 255, 255, 0.1);
    border: 2px solid rgba(255, 255, 255, 0.2);
    border-radius: 8px;
    padding: 0.75rem 1.5rem;
    font-size: 0.9rem;
    font-weight: 600;
    color: white;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .back-button:hover,
  .retry-button:hover {
    background: rgba(255, 255, 255, 0.15);
    border-color: rgba(255, 255, 255, 0.3);
    transform: translateY(-2px);
  }

  @media (max-width: 640px) {
    .title {
      font-size: 1.25rem;
    }

    .instance-selection {
      padding: 1rem 1.5rem;
    }

    .instance-card {
      padding: 0.75rem;
    }

    .instance-name {
      font-size: 1rem;
    }

    .instance-details {
      flex-direction: column;
      gap: 0.5rem;
    }
  }
</style>

