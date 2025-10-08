<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  
  const dispatch = createEventDispatcher();
  
  interface Launcher {
    name: string;
    displayName: string;
    icon: string;
    instanceCount: number;
  }

  let launchers: Launcher[] = [];
  let isLoading = true;
  let error: string | null = null;

  onMount(async () => {
    await loadLaunchers();
  });

  async function loadLaunchers() {
    isLoading = true;
    error = null;
    
    try {
      if (window.nahaAPI?.minecraftUpdater?.scanInstances) {
        const result = await window.nahaAPI.minecraftUpdater.scanInstances('json', null);
        
        if (result.success && result.instances) {
          // Group instances by launcher
          const launcherMap = new Map<string, number>();
          result.instances.forEach((instance: any) => {
            const count = launcherMap.get(instance.launcher_type) || 0;
            launcherMap.set(instance.launcher_type, count + 1);
          });

          // Convert to launcher objects
          launchers = Array.from(launcherMap.entries()).map(([name, count]) => ({
            name,
            displayName: name,
            icon: getLauncherIcon(name),
            instanceCount: count
          }));
        }
      }
    } catch (e: any) {
      console.error('Failed to load launchers:', e);
      error = e.message;
    } finally {
      isLoading = false;
    }
  }

  function getLauncherIcon(launcherName: string): string {
    switch (launcherName.toLowerCase()) {
      case 'astralrinth':
        return '🌟';
      case 'modrinthapp':
        return '📱';
      case 'xmcl':
        return '🎯';
      case 'prismlauncher':
        return '🔷';
      case 'official':
      case 'official minecraft':
        return '🎮';
      default:
        return '📦';
    }
  }

  function selectLauncher(launcher: Launcher) {
    dispatch('launcherSelected', { launcher: launcher.name });
  }

  function goBack() {
    dispatch('back');
  }
</script>

<div class="launcher-selection">
  <h2 class="title">Select Your Minecraft Launcher</h2>
  <p class="subtitle">Choose which launcher contains the instances you want to update</p>

  {#if isLoading}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>Scanning for launchers...</p>
    </div>
  {:else if error}
    <div class="error-state">
      <span class="error-icon">⚠️</span>
      <p>Failed to scan launchers: {error}</p>
      <button class="retry-button" on:click={loadLaunchers}>
        🔄 Retry
      </button>
    </div>
  {:else if launchers.length === 0}
    <div class="empty-state">
      <span class="empty-icon">📭</span>
      <p>No Minecraft launchers found with instances</p>
      <button class="back-button" on:click={goBack}>
        ← Go Back
      </button>
    </div>
  {:else}
    <div class="launcher-grid">
      {#each launchers as launcher}
        <button class="launcher-card" on:click={() => selectLauncher(launcher)}>
          <div class="launcher-icon">{launcher.icon}</div>
          <h3 class="launcher-name">{launcher.displayName}</h3>
          <p class="instance-count">{launcher.instanceCount} {launcher.instanceCount === 1 ? 'instance' : 'instances'}</p>
        </button>
      {/each}
    </div>

    <div class="actions">
      <button class="back-button" on:click={goBack}>
        ← Back
      </button>
    </div>
  {/if}
</div>

<style>
  .launcher-selection {
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

  .launcher-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.75rem;
    margin: 0;
  }

  .launcher-card {
    background: rgba(255, 255, 255, 0.08);
    border: 2px solid rgba(255, 255, 255, 0.15);
    border-radius: 10px;
    padding: 0.875rem;
    cursor: pointer;
    transition: all 0.3s ease;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.4rem;
    text-align: center;
    min-height: 120px;
  }

  .launcher-card:hover {
    transform: translateY(-4px);
    border-color: #10b981;
    box-shadow: 0 8px 24px rgba(16, 185, 129, 0.3);
    background: rgba(16, 185, 129, 0.1);
  }

  .launcher-icon {
    font-size: 2.5rem;
    filter: drop-shadow(0 2px 8px rgba(0, 0, 0, 0.3));
  }

  .launcher-name {
    font-size: 1rem;
    font-weight: 700;
    color: white;
    margin: 0;
  }

  .instance-count {
    font-size: 0.8rem;
    color: rgba(255, 255, 255, 0.6);
    margin: 0;
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

    .launcher-grid {
      grid-template-columns: repeat(2, 1fr) !important;
      gap: 0.75rem;
    }

    .launcher-card {
      padding: 1rem;
      min-height: 140px;
    }

    .launcher-icon {
      font-size: 2rem;
    }
  }
</style>

