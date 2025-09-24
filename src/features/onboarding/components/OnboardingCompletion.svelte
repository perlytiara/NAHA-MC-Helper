<script>
  import { createEventDispatcher } from 'svelte';
  import { onMount } from 'svelte';

  const dispatch = createEventDispatcher();

  export let selectedLauncher = null;
  export let selectedServer = null;
  export let hasMinecraftInstalled = false;
  export let minecraftPath = '';
  export let customPath = '';

  let launcherDisplayName = '';
  let serverDisplayName = '';

  // Map launcher IDs to pretty display names
  const launcherNames = {
    'prism': 'Prism Launcher',
    'astralrinth': 'AstralRinth',
    'modrinth': 'ModRinth',
    'xmcl': 'XMCL',
    'cracked': 'Prism Launcher',
    'prismlauncher': 'Prism Launcher',
    'astralrinthapp': 'AstralRinth',
    'other': 'Other'
  };

  // Map server IDs to pretty display names
  const serverNames = {
    'fabric': 'Fabric',
    'neoforge-create': 'NeoForge',
    'better-than-wolves': 'Better Than Wolves',
    'forge': 'Forge'
  };

  $: launcherDisplayName = (() => {
    if (!selectedLauncher) return 'Unknown Launcher';
    
    // Handle both string ID and object formats
    if (typeof selectedLauncher === 'string') {
      return launcherNames[selectedLauncher] || selectedLauncher;
    } else {
      return launcherNames[selectedLauncher?.id] || selectedLauncher?.name || 'Unknown Launcher';
    }
  })();
  
  $: serverDisplayName = serverNames[selectedServer] || selectedServer || 'Unknown Modpack';

  function handleFinish() {
    dispatch('finish');
  }

  function handleBack() {
    dispatch('back');
  }
</script>

<div class="completion-content">
  <!-- Header -->
  <div class="header-section">
    <div class="logo">
      <div class="logo-icon">⚡</div>
      <span class="logo-text">NAHA</span>
    </div>
  </div>
  
  <!-- Main content -->
  <div class="main-content">
    <div class="completion-header">
      <div class="success-icon">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
          <polyline points="22,4 12,14.01 9,11.01"></polyline>
        </svg>
      </div>
      <h1 class="completion-title">Installation Complete!</h1>
      <p class="completion-subtitle">
        Your Minecraft setup has been successfully configured and is ready to use.
      </p>
    </div>

    <!-- Installation Summary -->
    <div class="summary-section">
      <h2 class="section-title">Installation Summary</h2>
      <div class="summary-grid">
        <div class="summary-card">
          <div class="summary-icon">🎮</div>
          <div class="summary-content">
            <h3 class="summary-label">Modpack</h3>
            <p class="summary-value">{serverDisplayName}</p>
          </div>
        </div>
        <div class="summary-card">
          <div class="summary-icon">🚀</div>
          <div class="summary-content">
            <h3 class="summary-label">Launcher</h3>
            <p class="summary-value">{launcherDisplayName}</p>
          </div>
        </div>
        {#if hasMinecraftInstalled}
          <div class="summary-card">
            <div class="summary-icon">📂</div>
            <div class="summary-content">
              <h3 class="summary-label">Minecraft Path</h3>
              <p class="summary-value path-value">{minecraftPath}</p>
            </div>
          </div>
        {/if}
        {#if customPath}
          <div class="summary-card">
            <div class="summary-icon">🔧</div>
            <div class="summary-content">
              <h3 class="summary-label">Custom Path</h3>
              <p class="summary-value path-value">{customPath}</p>
            </div>
          </div>
        {/if}
      </div>
    </div>

    <!-- Next Steps -->
    <div class="steps-section">
      <h2 class="section-title">Next Steps</h2>
      <div class="steps-list">
        <div class="step-item">
          <div class="step-number">1</div>
          <div class="step-content">
            <h3 class="step-title">Launch Your Launcher</h3>
            <p class="step-description">
              Open <strong>{launcherDisplayName}</strong> from your desktop or start menu
            </p>
          </div>
        </div>
        <div class="step-item">
          <div class="step-number">2</div>
          <div class="step-content">
            <h3 class="step-title">Sign In</h3>
            <p class="step-description">
              Sign in with your Minecraft account (if required by your launcher)
            </p>
          </div>
        </div>
        <div class="step-item">
          <div class="step-number">3</div>
          <div class="step-content">
            <h3 class="step-title">Create Instance</h3>
            <p class="step-description">
              Create a new instance or import the <strong>{serverDisplayName}</strong> modpack
            </p>
          </div>
        </div>
        <div class="step-item">
          <div class="step-number">4</div>
          <div class="step-content">
            <h3 class="step-title">Start Playing</h3>
            <p class="step-description">
              Launch your instance and start playing with your favorite mods!
            </p>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Navigation -->
    <div class="navigation-section">
      <button class="back-button" on:click={handleBack}>
        ← Back
      </button>
      <button class="finish-button" on:click={handleFinish}>
        🎉 Finish
      </button>
    </div>
  </div>
</div>

<style>
  .completion-content {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    width: 100%;
    max-width: 100%;
    margin: 0 auto;
    height: auto;
    overflow: visible;
    padding: 0;
    box-sizing: border-box;
    align-items: center;
    justify-content: center;
  }
  
  .header-section {
    display: flex;
    justify-content: center;
  }
  
  .logo {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    padding: 0.3rem 0.6rem;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    backdrop-filter: blur(10px);
    margin-bottom: 15px;
  }
  
  .logo-icon {
    width: 16px;
    height: 16px;
    background: linear-gradient(135deg, #22c55e, #16a34a);
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 8px;
    color: white;
    box-shadow: 0 2px 8px rgba(34, 197, 94, 0.3);
  }
  
  .logo-text {
    font-size: 1.1rem;
    font-weight: 800;
    color: white;
    letter-spacing: -0.5px;
  }
  
  .main-content {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    align-items: center;
    flex: 1;
    overflow: visible;
    width: 100%;
    max-width: 100%;
    justify-content: flex-start;
    padding: 0 0.5rem;
  }
  
  .completion-header {
    text-align: center;
    margin-bottom: 0.5rem;
  }
  
  .success-icon {
    color: #22c55e;
    margin-bottom: 0.5rem;
    display: flex;
    justify-content: center;
  }
  
  .completion-title {
    font-size: 1.25rem;
    font-weight: 700;
    color: white;
    margin: 0 0 0.25rem 0;
    line-height: 1.2;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  }
  
  .completion-subtitle {
    font-size: 0.8rem;
    color: rgba(255, 255, 255, 0.7);
    line-height: 1.3;
    margin: 0;
    max-width: 400px;
  }
  
  .summary-section {
    width: 100%;
    margin-bottom: 0.5rem;
  }
  
  .section-title {
    font-size: 1rem;
    font-weight: 600;
    color: white;
    margin: 0 0 0.5rem 0;
    text-align: center;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
  }
  
  .summary-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 0.5rem;
    width: 100%;
  }
  
  .summary-card {
    background: linear-gradient(145deg, 
      rgba(255, 255, 255, 0.15) 0%, 
      rgba(255, 255, 255, 0.08) 100%
    );
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 6px;
    padding: 0.5rem;
    backdrop-filter: blur(10px);
    display: flex;
    align-items: center;
    gap: 0.4rem;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
  }
  
  .summary-icon {
    font-size: 1rem;
    flex-shrink: 0;
  }
  
  .summary-content {
    flex: 1;
    min-width: 0;
  }
  
  .summary-label {
    font-size: 0.65rem;
    font-weight: 500;
    color: rgba(255, 255, 255, 0.6);
    margin: 0 0 0.1rem 0;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }
  
  .summary-value {
    font-size: 0.75rem;
    font-weight: 600;
    color: white;
    margin: 0;
    line-height: 1.2;
    word-break: break-word;
  }
  
  .path-value {
    font-size: 0.7rem;
    color: rgba(255, 255, 255, 0.8);
    word-break: break-all;
  }
  
  .steps-section {
    width: 100%;
    margin-bottom: 0.5rem;
  }
  
  .steps-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .step-item {
    background: linear-gradient(145deg, 
      rgba(255, 255, 255, 0.1) 0%, 
      rgba(255, 255, 255, 0.05) 100%
    );
    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 6px;
    padding: 0.5rem;
    backdrop-filter: blur(10px);
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  }
  
  .step-number {
    background: linear-gradient(135deg, #3b82f6, #2563eb);
    color: white;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.7rem;
    font-weight: 600;
    flex-shrink: 0;
    box-shadow: 0 2px 4px rgba(59, 130, 246, 0.3);
  }
  
  .step-content {
    flex: 1;
    min-width: 0;
  }
  
  .step-title {
    font-size: 0.75rem;
    font-weight: 600;
    color: white;
    margin: 0 0 0.15rem 0;
    line-height: 1.2;
  }
  
  .step-description {
    font-size: 0.7rem;
    color: rgba(255, 255, 255, 0.7);
    margin: 0;
    line-height: 1.3;
  }
  
  .navigation-section {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    margin-top: auto;
    flex-shrink: 0;
    padding: 0.5rem 0;
    width: 100%;
  }
  
  .back-button {
    background: transparent;
    color: rgba(255, 255, 255, 0.6);
    border: 2px solid rgba(255, 255, 255, 0.2);
    padding: 0.5rem 0.75rem;
    border-radius: 6px;
    font-size: 0.8rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
    min-width: 80px;
  }
  
  .back-button:hover {
    background: rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.8);
    border-color: rgba(255, 255, 255, 0.3);
  }
  
  .finish-button {
    background: linear-gradient(135deg, #22c55e, #16a34a);
    color: white;
    border: none;
    padding: 0.5rem 0.75rem;
    border-radius: 6px;
    font-size: 0.8rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
    box-shadow: 0 4px 16px rgba(34, 197, 94, 0.3);
    min-width: 100px;
  }
  
  .finish-button:hover {
    background: linear-gradient(135deg, #16a34a, #15803d);
    transform: translateY(-1px);
    box-shadow: 0 6px 20px rgba(34, 197, 94, 0.4);
  }
  
  @media (max-width: 768px) {
    .summary-grid {
      grid-template-columns: 1fr;
    }
    
    .navigation-section {
      flex-direction: column;
      width: 100%;
    }
    
    .back-button,
    .finish-button {
      width: 100%;
      justify-content: center;
    }
  }
</style>

