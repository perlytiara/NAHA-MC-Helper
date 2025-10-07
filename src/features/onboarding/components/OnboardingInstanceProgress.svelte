<!-- src/features/onboarding/components/OnboardingInstanceProgress.svelte -->
<script>
    import { createEventDispatcher, onMount, onDestroy } from 'svelte';
    import { installerStore } from '../../../shared/stores/installerStore.js';
    import { debug } from '../../../shared/stores/appStore.js';
    
    const dispatch = createEventDispatcher();
    const { currentMode, status, error } = installerStore;
    
    // Props
    export let selectedLauncher = null;
    export let selectedServer = null;
    export let targetLauncher = null;
    export let customPath = null;
    
    // State
    let isInstalling = false;
    let installationProgress = null;
    let installationLog = [];
    let currentInstallationStep = '';
    let installationPercentage = 0;
    let progressMessages = [];
    let currentProgressMessage = '';
    let downloadProgress = { current: 0, total: 0, file: '' };
    let installationComplete = false;
    let installationError = null;
    
    // Reactive statement to ensure button state updates
    $: buttonEnabled = installationComplete;
    
    
    // Set up installer progress listener via IPC
    let progressHandler;
    if (typeof window !== 'undefined' && window.nahaAPI && window.nahaAPI.ipcRenderer) {
        console.log('Setting up installer progress listener via IPC');
        progressHandler = (event, progressData) => {
            console.log('Progress received in UI:', progressData);
            handleInstallerProgress(progressData);
        };
        window.nahaAPI.ipcRenderer.on('installer-progress', progressHandler);
    } else {
        console.log('IPC renderer not available for progress listener');
    }
    
    // Cleanup event listener on component destroy
    onDestroy(() => {
        if (progressHandler && window.nahaAPI && window.nahaAPI.ipcRenderer) {
            window.nahaAPI.ipcRenderer.off('installer-progress', progressHandler);
        }
    });
    
    onMount(async () => {
        // Start installation immediately when component mounts
        await startInstallation();
    });
    
    async function startInstallation() {
        try {
            isInstalling = true;
            installationError = null;
            installationComplete = false;
            currentInstallationStep = 'Starting installation...';
            installationPercentage = 0;
            
            console.log('Starting installation with:', { selectedServer, targetLauncher });
            
            let result;
            
            // Use the appropriate installer command based on selected server
            switch (selectedServer) {
                case 'neoforge-create':
                    console.log('Calling downloadNeoForge with customPath:', customPath);
                    result = await window.nahaAPI.installer.downloadNeoForge(targetLauncher, true, null, customPath);
                    break;
                case 'fabric':
                    result = await window.nahaAPI.installer.downloadFabric(targetLauncher, true, null, customPath);
                    break;
                default:
                    throw new Error(`Unsupported server type: ${selectedServer}`);
            }
            
            if (result && result.success) {
                installationComplete = true;
                installationPercentage = 100;
                currentInstallationStep = 'Installation completed successfully!';
                currentProgressMessage = 'Your modpack has been installed and is ready to play!';
                
                // Auto-advance after 3 seconds
                setTimeout(() => {
                    dispatch('complete');
                }, 3000);
            } else {
                throw new Error(result?.stderr || result?.error || 'Installation failed');
            }
        } catch (err) {
            console.error('Installation error:', err);
            installationError = err.message || 'Installation failed';
            currentInstallationStep = 'Installation failed';
            currentProgressMessage = installationError;
        } finally {
            isInstalling = false;
        }
    }
    
    function handleInstallerProgress(progressData) {
        console.log('handleInstallerProgress called with:', progressData);
        const timestamp = new Date().toLocaleTimeString();
        const logEntry = {
            message: progressData.data,
            type: progressData.type === 'stderr' ? 'error' : 'info',
            timestamp
        };
        
        installationLog = [...installationLog, logEntry];
        
        // Keep only last 100 log entries
        if (installationLog.length > 100) {
            installationLog = installationLog.slice(-100);
        }
        
        // Parse progress information from the output
        let data = progressData.data;
        
        // Clean up ANSI escape sequences and formatting
        // eslint-disable-next-line no-control-regex
        data = data.replace(/\x1b\[[0-9;]*[mK]/g, ''); // Remove ANSI color codes
        // eslint-disable-next-line no-control-regex
        data = data.replace(/\x1b\[[0-9;]*[A-Za-z]/g, ''); // Remove other ANSI sequences
        data = data.replace(/^\s*[\d-]+\s+[\d:]+\s+Z\s+INFO\s+\w+::\w+\s*:\s*/, ''); // Remove timestamp and log level
        data = data.trim();
        
        const dataLower = data.toLowerCase();
        console.log('Parsing progress data:', data);
        
        // Update current step and progress based on output
        if (dataLower.includes('downloading neoforge modpack') && installationPercentage < 5) {
            currentInstallationStep = 'Downloading modpack...';
            currentProgressMessage = 'Fetching latest modpack from NAHA API';
            installationPercentage = Math.max(installationPercentage, 5);
        } else if (dataLower.includes('downloading 68 mod files') && installationPercentage < 10) {
            currentInstallationStep = 'Downloading mods...';
            currentProgressMessage = '';
            installationPercentage = Math.max(installationPercentage, 10);
        } else if (dataLower.includes('[') && dataLower.includes('/') && dataLower.includes(']')) {
            // Parse individual file download: "[1/68] Downloading: mods/..."
            const fileProgressMatch = data.match(/\[(\d+)\/(\d+)\]/);
            if (fileProgressMatch) {
                downloadProgress.current = parseInt(fileProgressMatch[1]);
                downloadProgress.total = parseInt(fileProgressMatch[2]);
                // Calculate progress between 10% and 85% for mod downloads
                const modProgress = Math.round((downloadProgress.current / downloadProgress.total) * 75) + 10;
                installationPercentage = Math.max(installationPercentage, modProgress);
                currentProgressMessage = `Mod ${downloadProgress.current}/${downloadProgress.total}`;
                console.log(`Progress updated: ${downloadProgress.current}/${downloadProgress.total} = ${installationPercentage}%`);
                
                // Extract filename and clean it up
                const fileMatch = data.match(/Downloading: (.+)$/);
                if (fileMatch) {
                    let filename = fileMatch[1];
                    // Clean up the filename for better display
                    filename = filename.replace(/^mods\//, ''); // Remove mods/ prefix
                    filename = filename.replace(/^resourcepacks\//, ''); // Remove resourcepacks/ prefix
                    filename = filename.replace(/^shaderpacks\//, ''); // Remove shaderpacks/ prefix
                    filename = filename.replace(/\.jar$/, ''); // Remove .jar extension
                    filename = filename.replace(/\.zip$/, ''); // Remove .zip extension
                    filename = filename.replace(/\.disabled$/, ' (disabled)'); // Handle disabled mods
                    downloadProgress.file = filename;
                }
            }
        } else if (dataLower.includes('✓ downloaded:')) {
            // File completed - update progress
            const fileMatch = data.match(/✓ Downloaded: (.+)$/);
            if (fileMatch) {
                let filename = fileMatch[1];
                // Clean up the filename for better display
                filename = filename.replace(/^mods\//, ''); // Remove mods/ prefix
                filename = filename.replace(/^resourcepacks\//, ''); // Remove resourcepacks/ prefix
                filename = filename.replace(/^shaderpacks\//, ''); // Remove shaderpacks/ prefix
                filename = filename.replace(/\.jar$/, ''); // Remove .jar extension
                filename = filename.replace(/\.zip$/, ''); // Remove .zip extension
                filename = filename.replace(/\.disabled$/, ' (disabled)'); // Handle disabled mods
                downloadProgress.file = filename;
            }
        } else if (dataLower.includes('mrpack installation completed') && installationPercentage < 90) {
            currentInstallationStep = 'Installing to launcher...';
            currentProgressMessage = 'Creating launcher instance';
            installationPercentage = Math.max(installationPercentage, 90);
        } else if (dataLower.includes('modpack installed successfully') && installationPercentage < 95) {
            currentInstallationStep = 'Finalizing installation...';
            currentProgressMessage = 'Setting up launcher configuration';
            installationPercentage = Math.max(installationPercentage, 95);
        } else if (dataLower.includes('instance created at') && installationPercentage < 98) {
            currentInstallationStep = 'Almost done...';
            currentProgressMessage = 'Configuring automodpack';
            installationPercentage = Math.max(installationPercentage, 98);
        } else if (dataLower.includes('neoforge modpack downloaded and installed successfully')) {
            currentInstallationStep = '';
            currentProgressMessage = '';
            installationPercentage = 100;
            installationComplete = true;
        }
        
        // Additional completion detection patterns
        if (dataLower.includes('installation completed') || 
            dataLower.includes('modpack installed successfully') ||
            dataLower.includes('instance created at') ||
            dataLower.includes('successfully injected profile')) {
            currentInstallationStep = '';
            currentProgressMessage = '';
            installationComplete = true;
            installationPercentage = 100;
        }
        
        // Update debug log if debug mode is enabled
        if ($debug) {
            // Clean up the original data for display
            let displayData = progressData.data;
            // eslint-disable-next-line no-control-regex
            displayData = displayData.replace(/\x1b\[[0-9;]*[mK]/g, ''); // Remove ANSI color codes
            // eslint-disable-next-line no-control-regex
            displayData = displayData.replace(/\x1b\[[0-9;]*[A-Za-z]/g, ''); // Remove other ANSI sequences
            displayData = displayData.replace(/^\s*[\d-]+\s+[\d:]+\s+Z\s+INFO\s+\w+::\w+\s*:\s*/, ''); // Remove timestamp and log level
            displayData = displayData.trim();
            
            addDebugLog(`[INSTALLER] ${displayData}`, progressData.type === 'stderr' ? 'error' : 'info');
        }
    }
    
    function addDebugLog(message, type = 'info') {
        const timestamp = new Date().toLocaleTimeString();
        const logEntry = { message, type, timestamp };
        progressMessages = [...progressMessages, logEntry];
        
        // Keep only last 50 log entries
        if (progressMessages.length > 50) {
            progressMessages = progressMessages.slice(-50);
        }
    }
    
    function handleBack() {
        dispatch('back');
    }
    
    function handleComplete() {
        dispatch('complete');
    }
</script>

<div class="installation-page">
    <div class="logo">
        <div class="logo-icon">⚡</div>
        <span class="logo-text">NAHA</span>
    </div>
    
    <div class="main-content">
        <div class="installation-header">
            <h1 class="installation-title">Installing Your Modpack</h1>
            <p class="installation-subtitle">
                Setting up {selectedServer === 'neoforge-create' ? 'NeoForge Create' : 'Fabric'} for {selectedLauncher?.name || 'your launcher'}
            </p>
        </div>
        
        <div class="progress-section">
            <div class="progress-bar-container">
                <div class="progress-bar">
                    <div class="progress-fill" style="width: {installationPercentage}%"></div>
                </div>
                <div class="progress-text">{Math.round(installationPercentage)}%</div>
            </div>
            
            {#if !installationComplete}
                <div class="progress-info">
                    <div class="current-step">{currentInstallationStep}</div>
                    <div class="progress-message">{currentProgressMessage}</div>
                    
                    {#if downloadProgress.file}
                        <div class="current-file">
                            <span class="file-name">{downloadProgress.file}</span>
                        </div>
                    {/if}
                </div>
            {/if}
        </div>
        
        {#if installationError}
            <div class="error-section">
                <div class="error-icon">❌</div>
                <div class="error-content">
                    <h3 class="error-title">Installation Failed</h3>
                    <p class="error-message">{installationError}</p>
                </div>
            </div>
        {/if}
        
        {#if installationComplete}
            <div class="success-section">
                <div class="success-icon">✅</div>
                <div class="success-content">
                    <h3 class="success-title">Installation Complete!</h3>
                    <p class="success-message">Your modpack is ready to play!</p>
                </div>
            </div>
        {/if}
        
        {#if $debug && progressMessages.length > 0}
            <div class="debug-section">
                <h4 class="debug-title">Debug Log</h4>
                <div class="debug-log">
                    {#each progressMessages as log}
                        <div class="debug-entry {log.type}">
                            <span class="log-timestamp">[{log.timestamp}]</span>
                            <span class="log-message">{log.message}</span>
                        </div>
                    {/each}
                </div>
            </div>
        {/if}
    </div>
    
    <div class="navigation-buttons">
        <button class="back-button" on:click={handleBack}>
            ← Back
        </button>
        
        <button 
            class="continue-button" 
            class:disabled={!buttonEnabled}
            on:click={handleComplete}
            disabled={!buttonEnabled}
        >
            Continue →
        </button>
    </div>
</div>

<style>
    .installation-page {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100vh;
        width: 100vw;
        padding: 3rem 2rem;
        color: white;
        font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
        box-sizing: border-box;
        overflow: hidden;
    }
    
    .logo {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        margin-bottom: 1rem;
        flex-shrink: 0;
    }
    
    .logo-icon {
        font-size: 1.5rem;
        color: #ffd93d;
        text-shadow: 0 0 10px rgba(255, 217, 61, 0.5);
    }
    
    .logo-text {
        font-size: 1.5rem;
        font-weight: 800;
        background: linear-gradient(135deg, #ffd93d, #a78bfa, #06b6d4);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
    }
    
    .main-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 500px;
        gap: 1.2rem;
        flex-shrink: 0;
    }
    
    .installation-header {
        text-align: center;
    }
    
    .installation-title {
        font-size: 1.25rem;
        font-weight: 800;
        margin: 0 0 0.15rem 0;
        background: linear-gradient(45deg, #a78bfa, #06b6d4);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
    }
    
    .installation-subtitle {
        font-size: 0.8rem;
        color: rgba(255, 255, 255, 0.8);
        margin: 0;
    }
    
    .progress-section {
        width: 100%;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        flex-shrink: 0;
    }
    
    .progress-bar-container {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }
    
    .progress-bar {
        width: 100%;
        height: 12px;
        background: rgba(255, 255, 255, 0.2);
        border-radius: 8px;
        overflow: hidden;
    }
    
    .progress-fill {
        height: 100%;
        background: linear-gradient(90deg, #a78bfa, #06b6d4);
        border-radius: 8px;
        transition: width 0.3s ease;
    }
    
    .progress-text {
        text-align: center;
        font-size: 1.2rem;
        font-weight: 700;
        color: #06b6d4;
    }
    
    .progress-info {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        text-align: center;
    }
    
    .current-step {
        font-size: 1.1rem;
        font-weight: 700;
        color: #a78bfa;
    }
    
    .progress-message {
        font-size: 0.85rem;
        color: rgba(255, 255, 255, 0.5);
        font-style: italic;
    }
    
    .current-file {
        display: flex;
        justify-content: center;
        margin-top: 0.5rem;
    }
    
    .file-name {
        font-size: 0.7rem;
        text-align: center;
        word-break: break-all;
        color: rgba(255, 255, 255, 0.4);
        font-style: italic;
    }
    
    .error-section, .success-section {
        display: flex;
        align-items: center;
        gap: 0.4rem;
        padding: 0.4rem;
        border-radius: 12px;
        width: 100%;
        flex-shrink: 0;
    }
    
    .error-section {
        background: rgba(255, 107, 107, 0.1);
        border: 1px solid rgba(255, 107, 107, 0.3);
    }
    
    .success-section {
        background: rgba(34, 197, 94, 0.1);
        border: 1px solid rgba(34, 197, 94, 0.3);
    }
    
    .error-icon, .success-icon {
        font-size: 1.25rem;
        flex-shrink: 0;
    }
    
    .error-content, .success-content {
        flex: 1;
    }
    
    .error-title, .success-title {
        margin: 0 0 0.1rem 0;
        font-size: 0.9rem;
        font-weight: 600;
    }
    
    .error-title {
        color: #f87171;
    }
    
    .success-title {
        color: #22c55e;
    }
    
    .error-message {
        margin: 0;
        font-size: 0.75rem;
        color: rgba(248, 113, 113, 0.9);
    }
    
    .success-message {
        margin: 0;
        font-size: 0.75rem;
        color: rgba(34, 197, 94, 0.9);
    }
    
    .debug-section {
        width: 100%;
        max-height: 120px;
        overflow-y: auto;
        flex-shrink: 0;
    }
    
    .debug-title {
        margin: 0 0 0.25rem 0;
        font-size: 0.8rem;
        color: rgba(255, 255, 255, 0.6);
    }
    
    .debug-log {
        background: rgba(0, 0, 0, 0.3);
        border-radius: 8px;
        padding: 0.25rem;
        font-family: 'Courier New', monospace;
        font-size: 0.6rem;
    }
    
    .debug-entry {
        margin-bottom: 0.15rem;
    }
    
    .debug-entry.error .log-message {
        color: #ff6b6b;
    }
    
    .debug-entry.success .log-message {
        color: #4ecdc4;
    }
    
    .log-timestamp {
        color: rgba(255, 255, 255, 0.5);
        margin-right: 0.5rem;
    }
    
    .navigation-buttons {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 1rem;
        margin-top: 1rem;
        width: 100%;
        flex-shrink: 0;
    }
    
    .back-button, .continue-button {
        padding: 0.75rem 1.5rem;
        border: none;
        border-radius: 12px;
        font-size: 0.9rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s ease;
        min-width: 120px;
    }
    
    .back-button {
        background: rgba(255, 255, 255, 0.1);
        color: white;
        border: 1px solid rgba(255, 255, 255, 0.3);
    }
    
    .back-button:hover {
        background: rgba(255, 255, 255, 0.2);
    }
    
    .continue-button {
        background: linear-gradient(45deg, #22c55e, #06b6d4);
        color: white;
        border: none;
    }
    
    .continue-button:hover {
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(34, 197, 94, 0.3);
    }
    
    .continue-button.disabled {
        background: rgba(34, 197, 94, 0.3);
        color: rgba(255, 255, 255, 0.6);
        cursor: not-allowed;
        border: 1px solid rgba(34, 197, 94, 0.3);
    }
    
    .continue-button.disabled:hover {
        transform: none;
        box-shadow: none;
        background: rgba(34, 197, 94, 0.3);
    }
    
</style>
