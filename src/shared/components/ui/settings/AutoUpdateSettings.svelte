<script>
  import { onMount } from 'svelte';
  import { autoUpdateSettings, settingsActions } from '../../../stores/updateStore.js';

  let isLoaded = false;

  onMount(() => {
    // Load settings from localStorage if available
    loadSettings();
  });

  function loadSettings() {
    try {
      const savedSettings = localStorage.getItem('autoUpdateSettings');
      if (savedSettings) {
        const settings = JSON.parse(savedSettings);
        settingsActions.updateSettings(settings);
      }
      isLoaded = true;
    } catch (error) {
      console.error('Failed to load auto-update settings:', error);
      isLoaded = true;
    }
  }

  function saveSettings() {
    try {
      localStorage.setItem('autoUpdateSettings', JSON.stringify($autoUpdateSettings));
    } catch (error) {
      console.error('Failed to save auto-update settings:', error);
    }
  }

  function handleSettingChange(setting, value) {
    settingsActions.updateSettings({ [setting]: value });
    saveSettings();
  }

  function handleIntervalChange(event) {
    const hours = parseInt(event.target.value);
    settingsActions.setCheckInterval(hours);
    saveSettings();
  }
</script>

<div class="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700 p-6">
  <div class="flex items-center space-x-3 mb-6">
    <div class="text-2xl">🔄</div>
    <div>
      <h3 class="text-lg font-semibold text-gray-900 dark:text-gray-100">
        Auto-Update Settings
      </h3>
      <p class="text-sm text-gray-500 dark:text-gray-400">
        Configure automatic update checking and installation
      </p>
    </div>
  </div>

  {#if isLoaded}
    <div class="space-y-6">
      <!-- Enable Auto-Update -->
      <div class="flex items-center justify-between">
        <div class="flex-1">
          <h4 class="font-medium text-gray-900 dark:text-gray-100">
            Enable Auto-Update
          </h4>
          <p class="text-sm text-gray-500 dark:text-gray-400">
            Automatically check for and download updates
          </p>
        </div>
        <label class="relative inline-flex items-center cursor-pointer">
          <input 
            type="checkbox" 
            class="sr-only peer"
            checked={$autoUpdateSettings.enabled}
            on:change={(e) => handleSettingChange('enabled', e.target.checked)}
          />
          <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
        </label>
      </div>

      {#if $autoUpdateSettings.enabled}
        <!-- Check on Startup -->
        <div class="flex items-center justify-between">
          <div class="flex-1">
            <h4 class="font-medium text-gray-900 dark:text-gray-100">
              Check on Startup
            </h4>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              Check for updates when the application starts
            </p>
          </div>
          <label class="relative inline-flex items-center cursor-pointer">
            <input 
              type="checkbox" 
              class="sr-only peer"
              checked={$autoUpdateSettings.checkOnStartup}
              on:change={(e) => handleSettingChange('checkOnStartup', e.target.checked)}
            />
            <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
          </label>
        </div>

        <!-- Check Interval -->
        <div class="space-y-3">
          <div>
            <h4 class="font-medium text-gray-900 dark:text-gray-100">
              Check Interval
            </h4>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              How often to check for updates automatically
            </p>
          </div>
          <div class="flex items-center space-x-4">
            <select 
              class="bg-gray-50 dark:bg-gray-700 border border-gray-300 dark:border-gray-600 text-gray-900 dark:text-gray-100 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block p-2.5"
              value={$autoUpdateSettings.checkInterval}
              on:change={handleIntervalChange}
            >
              <option value="1">Every hour</option>
              <option value="3">Every 3 hours</option>
              <option value="6">Every 6 hours</option>
              <option value="12">Every 12 hours</option>
              <option value="24">Daily</option>
            </select>
            <span class="text-sm text-gray-500 dark:text-gray-400">
              Currently: {$autoUpdateSettings.checkInterval}h
            </span>
          </div>
        </div>

        <!-- Auto Download -->
        <div class="flex items-center justify-between">
          <div class="flex-1">
            <h4 class="font-medium text-gray-900 dark:text-gray-100">
              Auto Download
            </h4>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              Automatically download updates when available
            </p>
          </div>
          <label class="relative inline-flex items-center cursor-pointer">
            <input 
              type="checkbox" 
              class="sr-only peer"
              checked={$autoUpdateSettings.autoDownload}
              on:change={(e) => handleSettingChange('autoDownload', e.target.checked)}
            />
            <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
          </label>
        </div>

        <!-- Auto Install -->
        <div class="flex items-center justify-between">
          <div class="flex-1">
            <h4 class="font-medium text-gray-900 dark:text-gray-100">
              Auto Install
            </h4>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              Automatically install updates when downloaded
            </p>
          </div>
          <label class="relative inline-flex items-center cursor-pointer">
            <input 
              type="checkbox" 
              class="sr-only peer"
              checked={$autoUpdateSettings.autoInstall}
              on:change={(e) => handleSettingChange('autoInstall', e.target.checked)}
            />
            <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
          </label>
        </div>

        <!-- Warning for auto-install -->
        {#if $autoUpdateSettings.autoInstall}
          <div class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-4">
            <div class="flex items-center space-x-2">
              <span class="text-yellow-600 dark:text-yellow-400">⚠️</span>
              <div>
                <h5 class="font-medium text-yellow-800 dark:text-yellow-200">
                  Auto-Install Warning
                </h5>
                <p class="text-sm text-yellow-700 dark:text-yellow-300">
                  Enabling auto-install will restart the application automatically when updates are downloaded. Make sure to save your work before updates are installed.
                </p>
              </div>
            </div>
          </div>
        {/if}
      {/if}

      <!-- Manual Check Button -->
      <div class="pt-4 border-t border-gray-200 dark:border-gray-700">
        <button 
          class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-lg font-medium transition-colors"
          on:click={() => {
            // Trigger manual update check
            if (window.manualUpdateCheck) {
              window.manualUpdateCheck();
            } else {
              console.error('Manual update check function not available');
            }
          }}
        >
          Check for Updates Now
        </button>
        <p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
          Manually check for available updates
        </p>
      </div>
    </div>
  {:else}
    <div class="flex items-center justify-center py-8">
      <div class="animate-spin text-blue-500">🔄</div>
      <span class="ml-2 text-gray-500 dark:text-gray-400">Loading settings...</span>
    </div>
  {/if}
</div>

<style>
  /* Custom toggle switch styles */
  input[type="checkbox"]:checked + div {
    background-color: rgb(37 99 235);
  }
  
  input[type="checkbox"]:checked + div:after {
    transform: translateX(1.25rem);
  }
</style>
