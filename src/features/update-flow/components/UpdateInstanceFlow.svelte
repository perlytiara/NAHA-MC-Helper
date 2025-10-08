<script>
  import { onMount, createEventDispatcher } from 'svelte';
  import { currentPage } from '../../../shared/stores/appStore';

  const dispatch = createEventDispatcher();

  // State management
  let currentStep = 1;
  let totalSteps = 4;
  let selectedLauncher = null;
  let selectedModpackType = null;
  let selectedInstance = null;
  let availableLaunchers = [];
  let availableInstances = [];
  let isLoading = false;
  let error = null;
  let updateProgress = '';
  let progressLog = [];

  // Step titles
  const stepTitles = [
    'Select Launcher',
    'Choose Modpack Type', 
    'Select Instance',
    'Update Progress'
  ];

  onMount(async () => {
    await loadAvailableLaunchers();
    
    // Listen for progress events from the updater
    if (window.nahaAPI?.ipcRenderer) {
      window.nahaAPI.ipcRenderer.on('minecraft-updater:progress', (event, message) => {
        console.log('Progress update:', message);
        updateProgress = message;
        progressLog = [...progressLog, message];
      });
    }
  });

  async function loadAvailableLaunchers() {
    isLoading = true;
    error = null;
    try {
      // Scan all instances to get available launchers
      if (window.nahaAPI?.minecraftUpdater?.scanInstances) {
        const result = await window.nahaAPI.minecraftUpdater.scanInstances('json');
        if (result.success && result.instances) {
          // Extract unique launchers from instances
          const launcherSet = new Set();
          result.instances.forEach(instance => {
            launcherSet.add(instance.launcher_type);
          });
          availableLaunchers = Array.from(launcherSet).map(launcher => ({
            name: launcher,
            icon: getLauncherIcon(launcher),
            count: result.instances.filter(i => i.launcher_type === launcher).length
          }));
        }
      }
    } catch (e) {
      console.error('Failed to load launchers:', e);
      error = e.message;
    } finally {
      isLoading = false;
    }
  }

  async function loadInstancesForLauncher(launcher) {
    isLoading = true;
    error = null;
    try {
      if (window.nahaAPI?.minecraftUpdater?.scanInstances) {
        const result = await window.nahaAPI.minecraftUpdater.scanInstances('json', launcher);
        if (result.success && result.instances) {
          // Store all instances for now, we'll filter by modpack type when selecting
          availableInstances = result.instances;
        }
      }
    } catch (e) {
      console.error('Failed to load instances:', e);
      error = e.message;
    } finally {
      isLoading = false;
    }
  }

  function getLauncherIcon(launcherName) {
    switch (launcherName.toLowerCase()) {
      case 'astralrinth':
        return '🌟';
      case 'modrinthapp':
        return '📱';
      case 'xmcl':
        return '🎯';
      case 'prismlauncher':
        return '🔷';
      case 'official minecraft':
      case 'official':
        return '🎮';
      default:
        return '📦';
    }
  }

  function selectLauncher(launcher) {
    selectedLauncher = launcher;
    loadInstancesForLauncher(launcher.name);
    nextStep();
  }

  function selectModpackType(modpackType) {
    selectedModpackType = modpackType;
    nextStep();
  }

  async function selectInstance(instance) {
    selectedInstance = instance;
    nextStep();
    // Automatically start the update
    await startUpdate();
  }

  function nextStep() {
    if (currentStep < totalSteps) {
      currentStep++;
    }
  }

  function prevStep() {
    if (currentStep > 1) {
      currentStep--;
    }
  }

  async function startUpdate() {
    isLoading = true;
    error = null;
    updateProgress = 'Starting update...';
    progressLog = ['🔄 Initializing update process...'];
    
    try {
      if (window.nahaAPI?.minecraftUpdater?.updateInstance) {
        const result = await window.nahaAPI.minecraftUpdater.updateInstance(
          selectedInstance.instance_path,
          selectedModpackType,
          null
        );
        
        if (result.success) {
          updateProgress = `✅ Update completed!`;
          progressLog = [...progressLog, `✅ Update completed successfully!`];
          if (result.updated_mods && result.updated_mods.length > 0) {
            progressLog = [...progressLog, `📦 Updated ${result.updated_mods.length} mods`];
          }
          if (result.new_mods && result.new_mods.length > 0) {
            progressLog = [...progressLog, `➕ Added ${result.new_mods.length} new mods`];
          }
          if (result.preserved_mods && result.preserved_mods.length > 0) {
            progressLog = [...progressLog, `💾 Preserved ${result.preserved_mods.length} user mods`];
          }
          setTimeout(() => {
            dispatch('complete');
          }, 3000);
        } else {
          throw new Error(result.error || 'Update failed');
        }
      } else {
        throw new Error('Update functionality not available');
      }
    } catch (e) {
      console.error('Update failed:', e);
      error = e.message;
      updateProgress = `❌ Update failed: ${e.message}`;
      progressLog = [...progressLog, `❌ Error: ${e.message}`];
    } finally {
      isLoading = false;
    }
  }

  function goBack() {
    dispatch('back');
  }
</script>

<div class="update-instance-flow">
  <!-- Progress Bar -->
  <div class="progress-section">
    <div class="progress-bar">
      {#each Array(totalSteps) as _, i}
        <div class="progress-step" class:active={i + 1 <= currentStep} class:current={i + 1 === currentStep}>
          <div class="step-number">{i + 1}</div>
          <div class="step-label">{stepTitles[i]}</div>
        </div>
      {/each}
    </div>
    <div class="progress-text">Step {currentStep} of {totalSteps}: {stepTitles[currentStep - 1]}</div>
  </div>

  <!-- Step Content -->
  <div class="step-content">
    {#if currentStep === 1}
      <!-- Launcher Selection -->
      <div class="step-section">
        <h2>Select Your Minecraft Launcher</h2>
        <p>Choose which launcher you want to update instances from:</p>
        
        {#if isLoading}
          <div class="loading-state">
            <div class="loading-spinner"></div>
            <p>Scanning for launchers...</p>
          </div>
        {:else if error}
          <div class="error-state">
            <p>❌ {error}</p>
            <button class="btn btn-secondary" on:click={loadAvailableLaunchers}>Retry</button>
          </div>
        {:else if availableLaunchers.length === 0}
          <div class="empty-state">
            <p>No Minecraft launchers found with instances.</p>
            <p>Make sure you have Minecraft instances installed in one of the supported launchers.</p>
          </div>
        {:else}
          <div class="launcher-grid">
            {#each availableLaunchers as launcher}
              <button class="launcher-card" on:click={() => selectLauncher(launcher)}>
                <div class="launcher-icon">{launcher.icon}</div>
                <div class="launcher-name">{launcher.name}</div>
                <div class="launcher-count">{launcher.count} instance{launcher.count !== 1 ? 's' : ''}</div>
              </button>
            {/each}
          </div>
        {/if}
      </div>
    {:else if currentStep === 2}
      <!-- Modpack Type Selection -->
      <div class="step-section">
        <h2>Choose Modpack Type</h2>
        <p>Select the type of modpack you want to update:</p>
        
        <div class="modpack-grid">
          <button class="modpack-card" on:click={() => selectModpackType('neoforge')}>
            <div class="modpack-icon">🔧</div>
            <div class="modpack-name">NeoForge</div>
            <div class="modpack-desc">Modern Forge fork</div>
          </button>
          <button class="modpack-card" on:click={() => selectModpackType('fabric')}>
            <div class="modpack-icon">🧵</div>
            <div class="modpack-name">Fabric</div>
            <div class="modpack-desc">Lightweight mod loader</div>
          </button>
        </div>
      </div>
    {:else if currentStep === 3}
      <!-- Instance Selection -->
      <div class="step-section">
        <h2>Select Instance to Update</h2>
        <p>Choose which {selectedModpackType} instance you want to update:</p>
        
        {#if isLoading}
          <div class="loading-state">
            <div class="loading-spinner"></div>
            <p>Loading instances from {selectedLauncher?.name}...</p>
          </div>
        {:else if error}
          <div class="error-state">
            <p>❌ {error}</p>
            <button class="btn btn-secondary" on:click={() => loadInstancesForLauncher(selectedLauncher)}>Retry</button>
          </div>
        {:else if availableInstances.filter(i => i.mod_loader.toLowerCase().includes(selectedModpackType)).length === 0}
          <div class="empty-state">
            <p>No {selectedModpackType} instances found in {selectedLauncher?.name}.</p>
          </div>
        {:else}
          <div class="instance-list">
            {#each availableInstances.filter(i => i.mod_loader.toLowerCase().includes(selectedModpackType)) as instance}
              <button class="instance-card" on:click={() => selectInstance(instance)}>
                <div class="instance-info">
                  <div class="instance-name">{instance.name}</div>
                  <div class="instance-details">
                    <span class="minecraft-version">MC {instance.minecraft_version}</span>
                    <span class="mod-loader">{instance.mod_loader}</span>
                    <span class="mod-count">{instance.mod_count} mods</span>
                  </div>
                </div>
                <div class="instance-arrow">→</div>
              </button>
            {/each}
          </div>
        {/if}
      </div>
    {:else if currentStep === 4}
      <!-- Update Progress -->
      <div class="step-section">
        <h2>Updating Instance</h2>
        <p>Updating <strong>{selectedInstance?.name}</strong> with latest {selectedModpackType} mods...</p>
        
        {#if isLoading}
          <div class="update-progress">
            <div class="loading-spinner"></div>
            <div class="progress-log">
              {#each progressLog as logEntry}
                <div class="log-entry">{logEntry}</div>
              {/each}
            </div>
          </div>
        {:else}
          <div class="update-complete">
            <div class="progress-log completed">
              {#each progressLog as logEntry}
                <div class="log-entry">{logEntry}</div>
              {/each}
            </div>
            {#if error}
              <div class="error-message">{error}</div>
            {/if}
            <button class="btn btn-primary mt-4" on:click={goBack}>Back to Home</button>
          </div>
        {/if}
      </div>
    {/if}
  </div>

  <!-- Navigation -->
  <div class="step-navigation">
    {#if currentStep > 1 && currentStep < 4}
      <button class="btn btn-secondary" on:click={prevStep}>← Back</button>
    {/if}
    
    <button class="btn btn-ghost" on:click={goBack}>Cancel</button>
  </div>
</div>

<style>
  .update-instance-flow {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
  }

  .progress-section {
    margin-bottom: 3rem;
  }

  .progress-bar {
    display: flex;
    justify-content: space-between;
    margin-bottom: 1rem;
  }

  .progress-step {
    display: flex;
    flex-direction: column;
    align-items: center;
    flex: 1;
    position: relative;
  }

  .progress-step:not(:last-child)::after {
    content: '';
    position: absolute;
    top: 20px;
    left: 60px;
    right: -60px;
    height: 2px;
    background: #e5e7eb;
    z-index: -1;
  }

  .progress-step.active:not(:last-child)::after {
    background: #10b981;
  }

  .step-number {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: #e5e7eb;
    color: #6b7280;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    margin-bottom: 0.5rem;
    transition: all 0.3s ease;
  }

  .progress-step.active .step-number {
    background: #10b981;
    color: white;
  }

  .progress-step.current .step-number {
    background: #059669;
    transform: scale(1.1);
  }

  .step-label {
    font-size: 0.875rem;
    color: #6b7280;
    text-align: center;
  }

  .progress-step.active .step-label {
    color: #10b981;
    font-weight: 500;
  }

  .progress-text {
    text-align: center;
    font-weight: 500;
    color: #374151;
  }

  .step-content {
    min-height: 400px;
    display: flex;
    align-items: center;
  }

  .step-section {
    width: 100%;
    text-align: center;
  }

  .step-section h2 {
    font-size: 1.875rem;
    font-weight: 700;
    margin-bottom: 1rem;
    color: #111827;
  }

  .step-section p {
    font-size: 1.125rem;
    color: #6b7280;
    margin-bottom: 2rem;
  }

  .launcher-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    margin-top: 2rem;
  }

  .launcher-card {
    background: white;
    border: 2px solid #e5e7eb;
    border-radius: 12px;
    padding: 1.5rem;
    text-align: center;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .launcher-card:hover {
    border-color: #10b981;
    transform: translateY(-2px);
    box-shadow: 0 10px 25px rgba(16, 185, 129, 0.1);
  }

  .launcher-icon {
    font-size: 2.5rem;
    margin-bottom: 1rem;
  }

  .launcher-name {
    font-size: 1.125rem;
    font-weight: 600;
    margin-bottom: 0.5rem;
    color: #111827;
  }

  .launcher-count {
    font-size: 0.875rem;
    color: #6b7280;
  }

  .modpack-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
    margin-top: 2rem;
  }

  .modpack-card {
    background: white;
    border: 2px solid #e5e7eb;
    border-radius: 12px;
    padding: 2rem;
    text-align: center;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .modpack-card:hover {
    border-color: #3b82f6;
    transform: translateY(-2px);
    box-shadow: 0 10px 25px rgba(59, 130, 246, 0.1);
  }

  .modpack-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .modpack-name {
    font-size: 1.25rem;
    font-weight: 600;
    margin-bottom: 0.5rem;
    color: #111827;
  }

  .modpack-desc {
    font-size: 0.875rem;
    color: #6b7280;
  }

  .instance-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-top: 2rem;
    max-height: 400px;
    overflow-y: auto;
  }

  .instance-card {
    background: white;
    border: 2px solid #e5e7eb;
    border-radius: 8px;
    padding: 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .instance-card:hover {
    border-color: #10b981;
    transform: translateX(4px);
  }

  .instance-info {
    text-align: left;
  }

  .instance-name {
    font-size: 1.125rem;
    font-weight: 600;
    margin-bottom: 0.25rem;
    color: #111827;
  }

  .instance-details {
    display: flex;
    gap: 1rem;
    font-size: 0.875rem;
    color: #6b7280;
  }

  .instance-arrow {
    font-size: 1.5rem;
    color: #10b981;
  }

  .loading-state, .error-state, .empty-state {
    padding: 3rem;
  }

  .loading-spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #e5e7eb;
    border-top: 4px solid #10b981;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin: 0 auto 1rem;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .progress-log {
    max-width: 600px;
    margin: 2rem auto;
    background: #f9fafb;
    border: 1px solid #e5e7eb;
    border-radius: 8px;
    padding: 1rem;
    max-height: 300px;
    overflow-y: auto;
    text-align: left;
  }

  .progress-log.completed {
    background: #f0fdf4;
    border-color: #bbf7d0;
  }

  .log-entry {
    padding: 0.5rem;
    margin: 0.25rem 0;
    font-family: 'Courier New', monospace;
    font-size: 0.875rem;
    color: #374151;
    border-left: 3px solid transparent;
  }

  .log-entry:hover {
    background: rgba(16, 185, 129, 0.05);
    border-left-color: #10b981;
  }

  .error-message {
    background: #fef2f2;
    border: 1px solid #fecaca;
    color: #dc2626;
    padding: 1rem;
    border-radius: 8px;
    margin-top: 1rem;
  }

  .error-state {
    color: #dc2626;
  }

  .empty-state {
    color: #6b7280;
  }

  .update-progress, .update-complete {
    padding: 2rem;
    text-align: center;
  }

  .step-navigation {
    display: flex;
    justify-content: space-between;
    margin-top: 3rem;
    padding-top: 2rem;
    border-top: 1px solid #e5e7eb;
  }

  .btn {
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
    border: none;
  }

  .btn-primary {
    background: #10b981;
    color: white;
  }

  .btn-primary:hover {
    background: #059669;
  }

  .btn-secondary {
    background: #f3f4f6;
    color: #374151;
  }

  .btn-secondary:hover {
    background: #e5e7eb;
  }

  .btn-ghost {
    background: transparent;
    color: #6b7280;
  }

  .btn-ghost:hover {
    color: #374151;
  }
</style>
