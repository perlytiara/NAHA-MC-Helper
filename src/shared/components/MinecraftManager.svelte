<script>
  import { onMount, createEventDispatcher } from 'svelte';
  import { currentPage } from '../stores/appStore';

  const dispatch = createEventDispatcher();

  // State management
  let instances = [];
  let isScanning = false;
  let isInstalling = false;
  let isUpdating = false;
  let selectedModpack = 'neoforge'; // 'neoforge' or 'fabric'
  let scanError = null;
  let operationStatus = null;

  // Available modpacks
  const modpackTypes = [
    { value: 'neoforge', label: 'NeoForge', description: 'Modern Forge alternative', icon: '⚡' },
    { value: 'fabric', label: 'Fabric', description: 'Lightweight mod loader', icon: '🧵' }
  ];

  onMount(() => {
    scanInstances();
  });

  // Scan for existing instances
  async function scanInstances() {
    try {
      isScanning = true;
      scanError = null;
      
      console.log('🔍 Scanning for Minecraft instances...');
      
      // Call the minecraft-updater scan function
      if (window.nahaAPI?.minecraftUpdater?.scanInstances) {
        const result = await window.nahaAPI.minecraftUpdater.scanInstances();
        instances = result.instances || [];
        console.log('✅ Found instances:', instances);
      } else {
        // Fallback: simulate scanning
        console.log('⚠️ minecraft-updater not available, using mock data');
        instances = [
          {
            name: 'NAHA-Neoforge-1.21.1-0.0.18',
            launcher_type: 'AstralRinth',
            minecraft_version: '1.21.1',
            mod_loader: 'neoforge',
            instance_path: 'C:\\Users\\user\\AppData\\Roaming\\AstralRinthApp\\profiles\\NAHA-Neoforge-1.21.1-0.0.18',
            mod_count: 53
          }
        ];
      }
    } catch (error) {
      console.error('❌ Failed to scan instances:', error);
      scanError = error.message;
      instances = [];
    } finally {
      isScanning = false;
    }
  }

  // Install new modpack
  async function handleInstall() {
    try {
      isInstalling = true;
      operationStatus = { type: 'info', message: '🚀 Installing new modpack...' };
      
      console.log(`📦 Installing ${selectedModpack} modpack...`);
      
      // Call the minecraft-installer
      if (window.nahaAPI?.minecraftInstaller?.installModpack) {
        const result = await window.nahaAPI.minecraftInstaller.installModpack(selectedModpack);
        
        if (result.success) {
          operationStatus = { type: 'success', message: '✅ Installation completed successfully!' };
          // Refresh instances list
          setTimeout(() => scanInstances(), 2000);
        } else {
          operationStatus = { type: 'error', message: `❌ Installation failed: ${result.error}` };
        }
      } else {
        // Fallback: simulate installation
        console.log('⚠️ minecraft-installer not available, simulating...');
        setTimeout(() => {
          operationStatus = { type: 'success', message: '✅ Installation simulated successfully!' };
          isInstalling = false;
        }, 3000);
      }
    } catch (error) {
      console.error('❌ Installation failed:', error);
      operationStatus = { type: 'error', message: `❌ Installation failed: ${error.message}` };
    } finally {
      isInstalling = false;
    }
  }

  // Update existing instance
  async function handleUpdate(instance) {
    try {
      isUpdating = true;
      operationStatus = { type: 'info', message: `🔄 Updating ${instance.name}...` };
      
      console.log(`🔄 Updating instance: ${instance.name}`);
      
      // Call the minecraft-updater
      if (window.nahaAPI?.minecraftUpdater?.updateInstance) {
        const result = await window.nahaAPI.minecraftUpdater.updateInstance(
          instance.instance_path,
          instance.mod_loader
        );
        
        if (result.success) {
          const updatedCount = result.updated_mods?.length || 0;
          const newCount = result.new_mods?.length || 0;
          operationStatus = { 
            type: 'success', 
            message: `✅ Updated ${instance.name}! ${updatedCount} mods updated, ${newCount} new mods added.` 
          };
          // Refresh instances list
          setTimeout(() => scanInstances(), 2000);
        } else {
          operationStatus = { type: 'error', message: `❌ Update failed: ${result.error}` };
        }
      } else {
        // Fallback: simulate update
        console.log('⚠️ minecraft-updater not available, simulating...');
        setTimeout(() => {
          operationStatus = { type: 'success', message: `✅ Updated ${instance.name} successfully!` };
          isUpdating = false;
        }, 3000);
      }
    } catch (error) {
      console.error('❌ Update failed:', error);
      operationStatus = { type: 'error', message: `❌ Update failed: ${error.message}` };
    } finally {
      isUpdating = false;
    }
  }

  // Skip setup
  function handleSkip() {
    console.log('⏭️ Skipping setup');
    dispatch('skip');
    currentPage.set('homepage');
  }

  // Get instance display name
  function getInstanceDisplayName(instance) {
    return instance.name.replace('NAHA-', '').replace(/-/g, ' ');
  }

  // Get modpack icon
  function getModpackIcon(modLoader) {
    return modLoader === 'neoforge' ? '⚡' : '🧵';
  }
</script>

<div class="minecraft-manager">
  <div class="manager-header">
    <h2 class="manager-title">🎮 Minecraft Modpack Manager</h2>
    <p class="manager-subtitle">Install, update, or manage your Minecraft modpacks</p>
  </div>

  <!-- Scan Status -->
  <div class="scan-section">
    <div class="scan-header">
      <button class="scan-button" on:click={scanInstances} disabled={isScanning}>
        {#if isScanning}
          <span class="loading loading-spinner loading-sm"></span>
          Scanning...
        {:else}
          🔍 Scan Instances
        {/if}
      </button>
      {#if scanError}
        <div class="error-message">❌ {scanError}</div>
      {/if}
    </div>
  </div>

  <!-- Existing Instances -->
  {#if instances.length > 0}
    <div class="instances-section">
      <h3 class="section-title">📂 Existing Instances</h3>
      <div class="instances-grid">
        {#each instances as instance}
          <div class="instance-card">
            <div class="instance-header">
              <div class="instance-icon">{getModpackIcon(instance.mod_loader)}</div>
              <div class="instance-info">
                <h4 class="instance-name">{getInstanceDisplayName(instance)}</h4>
                <p class="instance-details">
                  {instance.minecraft_version} | {instance.mod_loader} | {instance.mod_count} mods
                </p>
                <p class="instance-launcher">via {instance.launcher_type}</p>
              </div>
            </div>
            <div class="instance-actions">
              <button 
                class="update-btn" 
                on:click={() => handleUpdate(instance)}
                disabled={isUpdating}
              >
                {#if isUpdating}
                  <span class="loading loading-spinner loading-xs"></span>
                {:else}
                  🔄
                {/if}
                Update
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Install New Modpack -->
  <div class="install-section">
    <h3 class="section-title">📦 Install New Modpack</h3>
    
    <!-- Modpack Type Selection -->
    <div class="modpack-selection">
      <p class="selection-label">Choose modpack type:</p>
      <div class="modpack-options">
        {#each modpackTypes as modpack}
          <label class="modpack-option">
            <input 
              type="radio" 
              bind:group={selectedModpack} 
              value={modpack.value}
            />
            <div class="option-card {selectedModpack === modpack.value ? 'selected' : ''}">
              <div class="option-icon">{modpack.icon}</div>
              <div class="option-info">
                <h4>{modpack.label}</h4>
                <p>{modpack.description}</p>
              </div>
            </div>
          </label>
        {/each}
      </div>
    </div>

    <!-- Action Buttons -->
    <div class="action-buttons">
      <button 
        class="install-btn primary" 
        on:click={handleInstall}
        disabled={isInstalling || isUpdating}
      >
        {#if isInstalling}
          <span class="loading loading-spinner loading-sm"></span>
          Installing...
        {:else}
          🚀 Install {selectedModpack === 'neoforge' ? 'NeoForge' : 'Fabric'}
        {/if}
      </button>
      
      <button 
        class="skip-btn secondary" 
        on:click={handleSkip}
        disabled={isInstalling || isUpdating}
      >
        ⏭️ Skip Setup
      </button>
    </div>
  </div>

  <!-- Operation Status -->
  {#if operationStatus}
    <div class="status-message {operationStatus.type}">
      {operationStatus.message}
    </div>
  {/if}
</div>

<style>
  .minecraft-manager {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
    background: var(--bg-primary);
    border-radius: 12px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }

  .manager-header {
    text-align: center;
    margin-bottom: 2rem;
  }

  .manager-title {
    font-size: 2rem;
    font-weight: 700;
    color: var(--text-primary);
    margin-bottom: 0.5rem;
  }

  .manager-subtitle {
    color: var(--text-secondary);
    font-size: 1.1rem;
  }

  .scan-section {
    margin-bottom: 2rem;
  }

  .scan-button {
    background: linear-gradient(135deg, #3b82f6, #8b5cf6);
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .scan-button:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
  }

  .scan-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .error-message {
    margin-top: 1rem;
    padding: 1rem;
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    border-radius: 8px;
    color: #ef4444;
  }

  .instances-section {
    margin-bottom: 2rem;
  }

  .section-title {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 1rem;
  }

  .instances-grid {
    display: grid;
    gap: 1rem;
  }

  .instance-card {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1.5rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    transition: all 0.2s ease;
  }

  .instance-card:hover {
    border-color: #3b82f6;
    box-shadow: 0 2px 8px rgba(59, 130, 246, 0.1);
  }

  .instance-header {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .instance-icon {
    font-size: 2rem;
  }

  .instance-name {
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 0.25rem;
  }

  .instance-details {
    color: var(--text-secondary);
    font-size: 0.9rem;
    margin-bottom: 0.25rem;
  }

  .instance-launcher {
    color: var(--text-tertiary);
    font-size: 0.8rem;
  }

  .update-btn {
    background: #10b981;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .update-btn:hover:not(:disabled) {
    background: #059669;
    transform: translateY(-1px);
  }

  .update-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .install-section {
    margin-bottom: 2rem;
  }

  .selection-label {
    color: var(--text-secondary);
    margin-bottom: 1rem;
    font-weight: 500;
  }

  .modpack-options {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    margin-bottom: 2rem;
  }

  .modpack-option {
    cursor: pointer;
  }

  .modpack-option input[type="radio"] {
    display: none;
  }

  .option-card {
    background: var(--bg-secondary);
    border: 2px solid var(--border-color);
    border-radius: 8px;
    padding: 1.5rem;
    text-align: center;
    transition: all 0.2s ease;
  }

  .option-card:hover {
    border-color: #3b82f6;
  }

  .option-card.selected {
    border-color: #3b82f6;
    background: rgba(59, 130, 246, 0.05);
  }

  .option-icon {
    font-size: 2.5rem;
    margin-bottom: 1rem;
  }

  .option-info h4 {
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 0.5rem;
  }

  .option-info p {
    color: var(--text-secondary);
    font-size: 0.9rem;
  }

  .action-buttons {
    display: flex;
    gap: 1rem;
    justify-content: center;
    flex-wrap: wrap;
  }

  .install-btn, .skip-btn {
    padding: 1rem 2rem;
    border-radius: 8px;
    font-weight: 600;
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.2s ease;
    border: none;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .install-btn.primary {
    background: linear-gradient(135deg, #10b981, #059669);
    color: white;
  }

  .install-btn.primary:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
  }

  .skip-btn.secondary {
    background: var(--bg-secondary);
    color: var(--text-secondary);
    border: 1px solid var(--border-color);
  }

  .skip-btn.secondary:hover:not(:disabled) {
    background: var(--bg-tertiary);
    transform: translateY(-1px);
  }

  .install-btn:disabled, .skip-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none !important;
  }

  .status-message {
    margin-top: 1rem;
    padding: 1rem;
    border-radius: 8px;
    font-weight: 500;
  }

  .status-message.success {
    background: rgba(16, 185, 129, 0.1);
    border: 1px solid rgba(16, 185, 129, 0.3);
    color: #10b981;
  }

  .status-message.error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #ef4444;
  }

  .status-message.info {
    background: rgba(59, 130, 246, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.3);
    color: #3b82f6;
  }

  /* CSS Variables for theming */
  :global {
    :root {
      --bg-primary: #ffffff;
      --bg-secondary: #f8fafc;
      --bg-tertiary: #f1f5f9;
      --text-primary: #1e293b;
      --text-secondary: #64748b;
      --text-tertiary: #94a3b8;
      --border-color: #e2e8f0;
    }

    [data-theme="dark"] {
      --bg-primary: #0f172a;
      --bg-secondary: #1e293b;
      --bg-tertiary: #334155;
      --text-primary: #f8fafc;
      --text-secondary: #cbd5e1;
      --text-tertiary: #94a3b8;
      --border-color: #334155;
    }
  }
</style>
