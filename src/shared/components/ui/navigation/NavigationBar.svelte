<script>
import { onMount } from 'svelte';
import { currentPage, debug, notification } from '../../../stores/appStore';

let brandButton;

onMount(async () => {
  // Initialize debug state from main process
  if (window.prism?.getDebugMode) {
    try {
      const debugState = await window.prism.getDebugMode();
      debug.set(debugState);
    } catch (error) {
      console.warn('Failed to get initial debug state:', error);
    }
  }
  
  // Listen for debug changes from main process (menu)
  if (window.prism?.onDebugChanged) {
    const removeDebugListener = window.prism.onDebugChanged((event, debugEnabled) => {
      debug.set(debugEnabled);
    });
    
    // Cleanup function
    return removeDebugListener;
  }
  
  
  // Listen for reset data confirmation from main process
  
  if (window.prism?.onResetDataConfirmed) {
    const removeResetListener = window.prism.onResetDataConfirmed(async () => {
      try {
        // Show loading notification
        notification.set({ type: 'info', message: 'Resetting application data...' });
        
        // Call the reset function
        const result = await window.prism.resetApplicationData();
        
        // Check if result exists and has the expected structure
        if (result && typeof result === 'object' && result.success) {
          // Reset local stores to default state
          debug.set(false);
          currentPage.set('homepage');
          
          // Reset onboarding data immediately
          import('../../../utils/onboardingUtils').then(({ resetOnboarding }) => {
            resetOnboarding();
            
            // Clear all localStorage immediately
            try {
              localStorage.clear();
            } catch (error) {
              console.error('Failed to clear localStorage:', error);
            }
            
            // Show success notification
            notification.set({ 
              type: 'success', 
              message: 'Application data reset successfully. Refreshing to show onboarding...' 
            });
            
            // Refresh the page after a short delay to show onboarding
            setTimeout(() => {
              window.location.reload();
            }, 2000);
            
          }).catch(error => {
            console.error('Failed to reset onboarding data:', error);
            notification.set({
              type: 'error',
              message: 'Failed to reset onboarding data'
            });
          });
        } else {
          // Show error notification - handle both missing result and failed result
          const errorMessage = result?.message || 'Reset operation failed with unknown error';
          notification.set({ 
            type: 'error', 
            message: errorMessage
          });
          
          // Auto-clear notification after 5 seconds
          setTimeout(() => {
            notification.set(null);
          }, 5000);
        }
      } catch (error) {
        console.error('Reset data error:', error);
        notification.set({ 
          type: 'error', 
          message: `Failed to reset data: ${error.message}` 
        });
        
        // Auto-clear notification after 5 seconds
        setTimeout(() => {
          notification.set(null);
        }, 5000);
      }
    });
    
    // Cleanup function
    return removeResetListener;
  }
  
});

// Manual reset function for debugging and menu reset
async function manualReset() {
  try {
    // Show loading notification
    notification.set({ type: 'info', message: 'Resetting application data...' });
    
    // Call the reset function
    const result = await window.prism.resetApplicationData();
    
    // Check if result exists and has the expected structure
    if (result && typeof result === 'object' && result.success) {
      // Reset local stores to default state
      debug.set(false);
      currentPage.set('homepage');
      
      // Reset onboarding data immediately
      import('../../../utils/onboardingUtils').then(({ resetOnboarding }) => {
        resetOnboarding();
        
        // Clear all localStorage immediately
        try {
          localStorage.clear();
        } catch (error) {
          console.error('Failed to clear localStorage:', error);
        }
        
        // Show success notification
        notification.set({ 
          type: 'success', 
          message: 'Application data reset successfully. Refreshing to show onboarding...' 
        });
        
        // Refresh the page after a short delay to show onboarding
        setTimeout(() => {
          window.location.reload();
        }, 2000);
        
      }).catch(error => {
        console.error('Failed to reset onboarding data:', error);
        notification.set({
          type: 'error',
          message: 'Failed to reset onboarding data'
        });
      });
    } else {
      // Show error notification - handle both missing result and failed result
      const errorMessage = result?.message || 'Reset operation failed with unknown error';
      notification.set({ 
        type: 'error', 
        message: errorMessage
      });
      
      // Auto-clear notification after 5 seconds
      setTimeout(() => {
        notification.set(null);
      }, 5000);
    }
  } catch (error) {
    console.error('Reset error:', error);
    notification.set({ 
      type: 'error', 
      message: `Failed to reset data: ${error.message}` 
    });
    
    // Auto-clear notification after 5 seconds
    setTimeout(() => {
      notification.set(null);
    }, 5000);
  }
}

// Make the function available globally for main process to call
window.manualReset = manualReset;

</script>

<nav class="navbar" aria-label="Primary navigation">
  <div class="nav-brand">
    <div class="brand-button" bind:this={brandButton} title="NAHA MC Helper">
      <div class="brand-icon">⚡</div>
      <span class="brand-text">NAHA</span>
    </div>
  </div>
  
  
  <div class="nav-links">
    <button type="button" class="nav-btn" class:active={$currentPage === 'homepage'} on:click={() => currentPage.set('homepage')}>
      <span class="nav-icon">🏠</span>
      <span class="nav-text">Home</span>
    </button>
    <button type="button" class="nav-btn unavailable" disabled title="Coming soon">
      <span class="nav-icon">⬇️</span>
      <span class="nav-text">Install</span>
    </button>
    <button type="button" class="nav-btn unavailable" disabled title="Coming soon">
      <span class="nav-icon">🔄</span>
      <span class="nav-text">Update</span>
    </button>
    <button type="button" class="nav-btn unavailable" disabled title="Coming soon">
      <span class="nav-icon">🌐</span>
      <span class="nav-text">Servers</span>
    </button>
    
    {#if $debug}
      <div class="nav-divider"></div>
      <button type="button" class="nav-btn debug" class:active={$currentPage === 'packer'} on:click={() => currentPage.set('packer')}>
        <span class="nav-icon">📦</span>
        <span class="nav-text">Packer</span>
      </button>
      <button type="button" class="nav-btn debug" class:active={$currentPage === 'mrpack-editor'} on:click={() => currentPage.set('mrpack-editor')}>
        <span class="nav-icon">✏️</span>
        <span class="nav-text">Editor</span>
      </button>
      <button type="button" class="nav-btn debug" class:active={$currentPage === 'presets'} on:click={() => currentPage.set('presets')}>
        <span class="nav-icon">⚙️</span>
        <span class="nav-text">Presets</span>
      </button>
    {/if}
  </div>
  
  <!-- Debug info display -->
  {#if $debug}
    <div class="debug-info">
      <div class="debug-info-content">
        Step:1 | Minecraft:null | Total:6 | Setup:N | Launcher:none | Server:none | Skip:N | SkipInst:N | Instances:0
      </div>
    </div>
  {/if}
</nav>

<style>
  .navbar {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 6px 12px;
    background: linear-gradient(135deg, rgba(255,255,255,0.1) 0%, rgba(255,255,255,0.05) 100%);
    backdrop-filter: blur(20px);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 12px;
    margin-bottom: 10px;
    box-shadow: 0 4px 16px rgba(0,0,0,0.1);
    position: relative;
    overflow: hidden;
  }
  
  .nav-brand {
    width: 100%;
    display: flex;
    justify-content: center;
    margin-bottom: 8px;
  }
  
  .nav-links {
    width: 100%;
    display: flex;
    justify-content: center;
    flex-wrap: wrap;
    gap: 4px;
  }
  
  .navbar::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 1px;
    background: linear-gradient(90deg, transparent, rgba(255,255,255,0.3), transparent);
  }
  
  .nav-brand {
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 800;
    font-size: 18px;
    color: var(--text);
    position: relative;
  }
  
  .brand-button {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px;
    border-radius: 8px;
  }
  
  .brand-icon {
    width: 24px;
    height: 24px;
    background: linear-gradient(135deg, #3b82f6, #8b5cf6);
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    box-shadow: 0 2px 8px rgba(59, 130, 246, 0.3);
  }
  
  .brand-text {
    background: linear-gradient(135deg, #3b82f6, #8b5cf6);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    letter-spacing: -0.5px;
  }
  
  .nav-links {
    display: flex;
    align-items: center;
    gap: 4px;
  }
  
  .nav-btn {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 6px 10px;
    background: none;
    border: none;
    border-radius: 8px;
    color: var(--text);
    font-weight: 500;
    font-size: 13px;
    cursor: pointer;
    transition: all 0.2s ease;
    position: relative;
    overflow: hidden;
  }
  
  .nav-btn::before {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(135deg, rgba(59, 130, 246, 0.1), rgba(139, 92, 246, 0.1));
    opacity: 0;
    transition: opacity 0.2s ease;
    border-radius: 12px;
  }
  
  .nav-btn:hover::before {
    opacity: 1;
  }
  
  .nav-btn:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0,0,0,0.15);
  }
  
  .nav-btn.active {
    background: linear-gradient(135deg, rgba(59, 130, 246, 0.2), rgba(139, 92, 246, 0.2));
    color: #3b82f6;
    font-weight: 600;
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.2);
  }
  
  .nav-btn.active::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 50%;
    transform: translateX(-50%);
    width: 20px;
    height: 2px;
    background: linear-gradient(90deg, #3b82f6, #8b5cf6);
    border-radius: 2px;
  }
  
  .nav-icon {
    font-size: 16px;
    filter: grayscale(0.3);
    transition: filter 0.2s ease;
  }
  
  .nav-btn:hover .nav-icon,
  .nav-btn.active .nav-icon {
    filter: grayscale(0);
  }
  
  .nav-text {
    font-size: 13px;
    letter-spacing: 0.2px;
  }
  
  .nav-divider {
    width: 1px;
    height: 24px;
    background: linear-gradient(to bottom, transparent, rgba(255,255,255,0.3), transparent);
    margin: 0 8px;
  }
  
  .nav-btn.debug {
    opacity: 0.8;
    font-size: 12px;
  }
  
  .nav-btn.debug .nav-text {
    font-size: 11px;
  }
  
  .nav-btn.debug .nav-icon {
    font-size: 14px;
  }
  
  .nav-btn.unavailable {
    opacity: 0.4;
    cursor: not-allowed;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
  }
  
  .nav-btn.unavailable:hover {
    transform: none;
    box-shadow: none;
  }
  
  .nav-btn.unavailable .nav-icon {
    filter: grayscale(0.8);
  }
  
  .nav-btn.unavailable .nav-text {
    color: rgba(255, 255, 255, 0.4);
  }
  
  /* Debug info section */
  .debug-info {
    width: 100%;
    margin-top: 8px;
    padding: 4px 8px;
    background: rgba(251, 191, 36, 0.1);
    border: 1px solid rgba(251, 191, 36, 0.2);
    border-radius: 8px;
    overflow: hidden;
  }
  
  .debug-info-content {
    font-size: 10px;
    color: #fbbf24;
    text-align: center;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 100%;
    font-family: 'Courier New', monospace;
  }
  
  
  /* Responsive design */
  @media (max-width: 768px) {
    .navbar {
      padding: 8px 12px;
      margin-bottom: 16px;
    }
    
    .nav-btn {
      padding: 8px 12px;
      gap: 4px;
    }
    
    .nav-text {
      display: none;
    }
    
    .nav-icon {
      font-size: 18px;
    }
    
    .brand-text {
      display: none;
    }
  }
  
  /* Dark theme adjustments */
  @media (prefers-color-scheme: dark) {
    .navbar {
      background: linear-gradient(135deg, rgba(255,255,255,0.08) 0%, rgba(255,255,255,0.03) 100%);
      border-color: rgba(255,255,255,0.08);
    }
  }
</style>
