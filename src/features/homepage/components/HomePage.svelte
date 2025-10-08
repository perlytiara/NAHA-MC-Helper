<script>
import { onMount } from 'svelte';
import { currentPage, debug, onboardingCompleted } from '../../../shared/stores/appStore';
import { resetOnboarding } from '../../../shared/utils/onboardingUtils';
import NavigationBar from '../../../shared/components/ui/navigation/NavigationBar.svelte';

// Update check state
let isCheckingUpdate = false;
let updateStatus = null;
let appVersion = '1.0.1'; // Default fallback

function startSetupFlow() {
  resetOnboarding();
  onboardingCompleted.set(false);
  currentPage.set('onboarding');
}

// Update check function
async function handleCheckForUpdates() {
  try {
    console.log('🎯 HomePage: handleCheckForUpdates button clicked!');
    isCheckingUpdate = true;
    updateStatus = null;
    
    console.log('HomePage: Checking for updates...');
    console.log('HomePage: window.manualUpdateCheck available:', typeof window.manualUpdateCheck);
    console.log('HomePage: window object available:', !!window);
    
    // Call the global update check function
    if (window.manualUpdateCheck) {
      await window.manualUpdateCheck();
      
      // Show success message
      updateStatus = {
        type: 'success',
        message: '✅ Update check completed!'
      };
    } else {
      updateStatus = {
        type: 'error',
        message: '❌ Update check not available'
      };
    }
  } catch (error) {
    console.error('HomePage: Update check failed:', error);
    updateStatus = {
      type: 'error',
      message: `❌ Update check failed: ${error.message}`
    };
  } finally {
    isCheckingUpdate = false;
    
    // Clear status after 5 seconds
    setTimeout(() => {
      updateStatus = null;
    }, 5000);
  }
}

// Debug logging
onMount(async () => {
  console.log('HomePage component mounted');
  console.log('Debug mode:', $debug);
  console.log('Current page:', $currentPage);
  
  // Get app version from Electron API
  if (window.nahaAPI?.getAppVersion) {
    try {
      appVersion = await window.nahaAPI.getAppVersion();
      console.log('App version:', appVersion);
    } catch (error) {
      console.error('Failed to get app version:', error);
    }
  }
});

// Key features and benefits
const features = [
  {
    icon: '🎮',
    title: 'Multiple Launchers',
    description: 'PrismLauncher, AstralRinth, XMCL, and ModRinth support'
  },
  {
    icon: '📦',
    title: 'Popular Modpacks',
    description: 'Fabric, NeoForge, and Better Than Wolves ready to install'
  },
  {
    icon: '⚡',
    title: 'Quick Setup',
    description: 'Automated installation and configuration in minutes'
  },
  {
    icon: '🛠️',
    title: 'Custom Paths',
    description: 'Flexible Minecraft installation directory selection'
  }
];
</script>

<div class="homepage-container">
  <!-- Background decoration -->
  <div class="background-decoration">
    <div class="decoration-grid"></div>
    <div class="decoration-orbs">
      <div class="orb orb-1"></div>
      <div class="orb orb-2"></div>
      <div class="orb orb-3"></div>
    </div>
  </div>
  
  <!-- Main content -->
  <div class="content-wrapper">
    <NavigationBar debug={$debug} />
    
    <!-- Debug info -->
    {#if $debug}
      <div class="debug-info">
        <p>Debug: HomePage loaded successfully</p>
        <p>Current page: {$currentPage}</p>
        <p>Onboarding completed: {$onboardingCompleted}</p>
      </div>
    {/if}
    
    <!-- Hero section -->
    <div class="hero-section">
        <div class="hero-content">
            <img src="/naha-mc-helper-logo.jpg" alt="NAHA MC Helper Logo" class="hero-logo" />
            <h1 class="hero-title">Minecraft Helper</h1>
            <p class="hero-subtitle">Your complete toolkit for managing Minecraft modpacks and launchers</p>
        </div>
    </div>
    
    <!-- Features grid -->
    <div class="features-section">
      <div class="features-grid">
        {#each features as feature}
          <div class="feature-card">
            <div class="feature-icon">{feature.icon}</div>
            <h3 class="feature-title">{feature.title}</h3>
            <p class="feature-description">{feature.description}</p>
        </div>
        {/each}
      </div>
    </div>
    
        <!-- Call to action -->
        <div class="cta-section">
          <div class="cta-buttons">
            <button class="cta-button side-button" on:click={() => currentPage.set('minecraft-manager')}>
              <span class="cta-icon">🎮</span>
              <span class="cta-text">Install Modpack</span>
            </button>
            <button class="cta-button main-button" on:click={startSetupFlow}>
              <span class="cta-icon">🚀</span>
              <span class="cta-text">Take me through the setup</span>
            </button>
            <button class="cta-button side-button" on:click={() => currentPage.set('update-instance')}>
              <span class="cta-icon">🔄</span>
              <span class="cta-text">Update Instance</span>
            </button>
          </div>
        </div>
    
    <!-- Update Status Section -->
    <div class="update-section">
      <div class="update-card">
        <div class="update-header">
          <div class="update-icon">🔄</div>
          <div class="update-info">
            <h3 class="update-title">Stay Updated</h3>
            <p class="update-subtitle">Current version: v{appVersion}</p>
          </div>
        </div>
        <div class="update-actions">
          <button 
            class="update-button" 
            on:click={handleCheckForUpdates}
            disabled={isCheckingUpdate}
          >
            <span class="update-button-icon">📡</span>
            {#if isCheckingUpdate}
              Checking...
            {:else}
              Check for Updates
            {/if}
          </button>
        </div>
        {#if updateStatus}
          <div class="update-status {updateStatus.type}">
            {updateStatus.message}
          </div>
        {/if}
      </div>
    </div>
    
    <!-- Version info -->
    <div class="version-info">
      <p>NAHA MC Helper v{appVersion} - Always improving</p>
    </div>
    
    <!-- Hidden onboarding reset (for debugging/testing) -->
    {#if $debug}
      <div class="debug-section">
        <button class="debug-button" on:click={startSetupFlow}>
          🔄 Reset Onboarding
        </button>
      </div>
    {/if}
  </div>
</div>

<style>
    .homepage-container {
        height: 100vh;
        position: relative;
        background: linear-gradient(135deg, 
            #0f172a 0%, 
            #1e293b 25%, 
            #334155 50%, 
            #475569 100%
        );
        overflow: hidden;
        color: white;
        display: flex;
        flex-direction: column;
    }
    
    .background-decoration {
        position: fixed;
        inset: 0;
        pointer-events: none;
        z-index: 0;
    }
    
    .decoration-grid {
        position: absolute;
        inset: 0;
        background-image: 
            linear-gradient(rgba(139, 92, 246, 0.02) 1px, transparent 1px),
            linear-gradient(90deg, rgba(139, 92, 246, 0.02) 1px, transparent 1px);
        background-size: 60px 60px;
        animation: gridDrift 25s linear infinite;
    }
    
    @keyframes gridDrift {
        0% { transform: translate(0, 0); }
        100% { transform: translate(60px, 60px); }
    }
    
    .decoration-orbs {
        position: absolute;
        inset: 0;
        overflow: hidden;
    }
    
    .orb {
        position: absolute;
        border-radius: 50%;
        background: radial-gradient(circle, 
            rgba(139, 92, 246, 0.08) 0%, 
            rgba(59, 130, 246, 0.04) 50%, 
            transparent 100%
        );
        animation: orbDance 12s ease-in-out infinite;
    }
    
    .orb-1 {
        width: 300px;
        height: 300px;
        top: -150px;
        left: -150px;
        animation-delay: 0s;
    }
    
    .orb-2 {
        width: 250px;
        height: 250px;
        top: 30%;
        right: -125px;
        animation-delay: 4s;
    }
    
    .orb-3 {
        width: 200px;
        height: 200px;
        bottom: -100px;
        left: 50%;
        animation-delay: 8s;
    }
    
    @keyframes orbDance {
        0%, 100% { 
            transform: translate(0, 0) scale(1) rotate(0deg); 
            opacity: 0.4; 
        }
        33% { 
            transform: translate(20px, -20px) scale(1.05) rotate(120deg); 
            opacity: 0.6; 
        }
        66% { 
            transform: translate(-15px, 15px) scale(0.95) rotate(240deg); 
            opacity: 0.3; 
        }
    }
    
    .content-wrapper {
        position: relative;
        z-index: 1;
        max-width: 900px;
        margin: 0 auto;
        padding: 0.25rem 0.5rem;
        height: 100vh;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        gap: 0.75rem;
    }
    
    /* Debug info */
    .debug-info {
        background: rgba(255, 0, 0, 0.1);
        padding: 0.5rem;
        border-radius: 8px;
        font-size: 0.8rem;
        margin-bottom: 0.5rem;
    }
    
    /* Hero Section */
    .hero-section {
        text-align: center;
        margin: 0;
        display: flex;
        justify-content: center;
        align-items: center;
    }
    
    .hero-content {
        background: rgba(255, 255, 255, 0.08);
        border-radius: 14px;
        padding: 0.75rem 1.25rem;
        border: 2px solid rgba(255, 255, 255, 0.15);
        box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
        backdrop-filter: blur(10px);
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }
    
    .hero-logo {
        width: 64px;
        height: 64px;
        border-radius: 10px;
        margin-bottom: 0.5rem;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
        transition: transform 0.3s ease, box-shadow 0.3s ease;
    }
    
    .hero-logo:hover {
        transform: scale(1.05);
        box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3);
    }
    
    .hero-title {
        font-size: 1.35rem;
        font-weight: 800;
        margin: 0 0 0.2rem 0;
        background: linear-gradient(45deg, #a78bfa, #06b6d4);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
    }
    
    .hero-subtitle {
        font-size: 0.75rem;
        color: rgba(255, 255, 255, 0.8);
        margin: 0;
        line-height: 1.3;
    }
    
    /* Features Section */
    .features-section {
        display: flex;
        align-items: center;
        justify-content: center;
        margin: 0;
        width: 100%;
    }
    
    .features-grid {
        display: grid;
        grid-template-columns: repeat(4, 1fr);
        gap: 0.5rem;
        width: 100%;
        max-width: 750px;
    }
    
    .feature-card {
        background: rgba(255, 255, 255, 0.08);
        border: 2px solid rgba(255, 255, 255, 0.15);
        border-radius: 10px;
        padding: 0.6rem;
        text-align: center;
        backdrop-filter: blur(10px);
        transition: all 0.3s ease;
    }
    
    .feature-card:hover {
        transform: translateY(-2px);
        border-color: rgba(255, 255, 255, 0.25);
        box-shadow: 0 8px 16px rgba(0, 0, 0, 0.15);
    }
    
    .feature-icon {
        font-size: 1.25rem;
        margin-bottom: 0.4rem;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.3));
    }
    
    .feature-title {
        font-size: 0.9rem;
        font-weight: 700;
        margin: 0 0 0.2rem 0;
        color: white;
    }
    
    .feature-description {
        font-size: 0.7rem;
        color: rgba(255, 255, 255, 0.7);
        margin: 0;
        line-height: 1.2;
    }
    
    /* Call to Action */
    .cta-section {
        text-align: center;
        margin: 0;
        display: flex;
        justify-content: center;
        align-items: center;
        width: 100%;
    }

    .cta-buttons {
        display: flex;
        gap: 0.75rem;
        justify-content: center;
        align-items: center;
        flex-wrap: nowrap;
        width: 100%;
        max-width: 750px;
    }
    
    .cta-button {
        border: none;
        border-radius: 12px;
        padding: 0.75rem 1rem;
        font-size: 0.85rem;
        font-weight: 700;
        cursor: pointer;
        transition: all 0.3s ease;
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: 0.4rem;
        backdrop-filter: blur(10px);
        flex: 1;
    }

    .cta-button.main-button {
        background: linear-gradient(135deg, #8b5cf6, #a855f7);
        color: white;
        box-shadow: 0 6px 16px rgba(139, 92, 246, 0.4);
        flex: 1.2;
        font-size: 0.9rem;
        padding: 0.85rem 1.25rem;
    }

    .cta-button.side-button {
        background: linear-gradient(135deg, #3b82f6, #2563eb);
        color: white;
        box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
        flex: 0.9;
    }
    
    .cta-button.main-button:hover {
        transform: translateY(-3px);
        box-shadow: 0 10px 28px rgba(139, 92, 246, 0.5);
        background: linear-gradient(135deg, #7c3aed, #9333ea);
    }

    .cta-button.side-button:hover {
        transform: translateY(-2px);
        box-shadow: 0 6px 18px rgba(59, 130, 246, 0.4);
        background: linear-gradient(135deg, #2563eb, #1d4ed8);
    }
    
    .cta-button:active {
        transform: translateY(0);
    }
    
    .cta-icon {
        font-size: 1.2rem;
    }
    
    .cta-text {
        font-size: 1rem;
    }
    
    /* Update Section */
    .update-section {
        text-align: center;
        margin: 0;
        width: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
    }
    
    .update-card {
        background: rgba(255, 255, 255, 0.08);
        border: 2px solid rgba(255, 255, 255, 0.15);
        border-radius: 12px;
        padding: 0.75rem;
        backdrop-filter: blur(10px);
        transition: all 0.3s ease;
    }
    
    .update-card:hover {
        border-color: rgba(255, 255, 255, 0.25);
        box-shadow: 0 8px 16px rgba(0, 0, 0, 0.15);
    }
    
    .update-header {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        margin-bottom: 0.5rem;
    }
    
    .update-icon {
        font-size: 1.5rem;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.3));
    }
    
    .update-info {
        text-align: left;
    }
    
    .update-title {
        font-size: 1rem;
        font-weight: 700;
        margin: 0 0 0.25rem 0;
        color: white;
    }
    
    .update-subtitle {
        font-size: 0.8rem;
        color: rgba(255, 255, 255, 0.7);
        margin: 0;
    }
    
    .update-actions {
        margin-bottom: 0.5rem;
    }
    
    .update-button {
        background: linear-gradient(135deg, #10b981, #059669);
        color: white;
        border: none;
        border-radius: 12px;
        padding: 0.75rem 1.5rem;
        font-size: 0.9rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.3s ease;
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
        backdrop-filter: blur(10px);
    }
    
    .update-button:hover:not(:disabled) {
        transform: translateY(-2px);
        box-shadow: 0 6px 16px rgba(16, 185, 129, 0.4);
        background: linear-gradient(135deg, #059669, #047857);
    }
    
    .update-button:disabled {
        opacity: 0.7;
        cursor: not-allowed;
        transform: none;
    }
    
    .update-button-icon {
        font-size: 1rem;
    }
    
    .update-status {
        font-size: 0.8rem;
        padding: 0.5rem;
        border-radius: 8px;
        margin-top: 0.5rem;
        font-weight: 600;
    }
    
    .update-status.success {
        background: rgba(16, 185, 129, 0.2);
        color: #10b981;
        border: 1px solid rgba(16, 185, 129, 0.3);
    }
    
    .update-status.error {
        background: rgba(239, 68, 68, 0.2);
        color: #ef4444;
        border: 1px solid rgba(239, 68, 68, 0.3);
    }

    /* Version info */
    .version-info {
        text-align: center;
        margin-top: 0.5rem;
    }
    
    .version-info p {
        font-size: 0.7rem;
        color: rgba(255, 255, 255, 0.5);
        margin: 0;
    }
    
    /* Debug Section */
    .debug-section {
        text-align: center;
        margin-top: 0.5rem;
    }
    
    .debug-button {
        background: rgba(251, 191, 36, 0.2);
        color: #fbbf24;
        border: 1px solid rgba(251, 191, 36, 0.3);
        border-radius: 12px;
        padding: 0.5rem 1rem;
        font-size: 0.8rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.3s ease;
    }
    
    .debug-button:hover {
        background: rgba(251, 191, 36, 0.3);
        border-color: rgba(251, 191, 36, 0.5);
        transform: translateY(-1px);
    }
    
    /* Responsive design */
    @media (max-width: 900px) {
        .features-grid {
            grid-template-columns: repeat(2, 1fr);
        }
        
        .cta-buttons {
            flex-wrap: wrap;
            max-width: 600px;
        }
        
        .cta-button.main-button {
            flex: 1 0 100%;
            order: -1;
        }
        
        .cta-button.side-button {
            flex: 1;
        }
    }
    
    @media (max-width: 640px) {
        .content-wrapper {
            padding: 0.25rem;
            gap: 0.5rem;
        }
        
        .hero-content {
            padding: 0.6rem 1rem;
        }
        
        .hero-logo {
            width: 56px;
            height: 56px;
            margin-bottom: 0.4rem;
        }
        
        .hero-title {
            font-size: 1.2rem;
        }
        
        .hero-subtitle {
            font-size: 0.7rem;
        }
        
        .features-grid {
            grid-template-columns: 1fr 1fr;
            gap: 0.4rem;
        }
        
        .feature-card {
            padding: 0.5rem;
        }
        
        .feature-icon {
            font-size: 1.1rem;
            margin-bottom: 0.3rem;
        }
        
        .feature-title {
            font-size: 0.8rem;
        }
        
        .feature-description {
            font-size: 0.65rem;
        }
        
        .cta-button {
            padding: 0.65rem 0.9rem;
            font-size: 0.8rem;
        }
        
        .cta-button.main-button {
            font-size: 0.85rem;
            padding: 0.75rem 1rem;
        }
    }
</style>