<!-- src/features/onboarding/components/OnboardingCreateInstance.svelte -->
<script>
    import { createEventDispatcher, onMount, onDestroy } from 'svelte';
    import { installerStore } from '../../../shared/stores/installerStore.js';
    import { debug } from '../../../shared/stores/appStore.js';
    import OnboardingServerSelection from './OnboardingServerSelection.svelte';
    
    const dispatch = createEventDispatcher();
    const { currentMode, status, error } = installerStore;
    
    // Props
    export let hasMinecraftInstalled = false;
    export let selectedLauncherFromFlow = null;
    export let selectedServerFromFlow = null;
    export let isInstanceCreationMode = false;
    
    // State
    let selectedServer = null;
    let selectedLauncher = null;
    let isDetecting = false;
    let detectedLaunchers = [];
    let customMinecraftPath = '';
    let debugLog = [];
    let isParsing = false;
    let minecraftInstallPath = '';
    let isMinecraftPathValid = false;
    let minecraftPathError = '';
    let showMinecraftPathBrowser = false;
    
    // Installer integration
    let installerAvailable = false;
    let installerVersion = null;
    
    // Reactive statements to update selections when props change
    $: if (selectedLauncherFromFlow) {
        // Setting selectedLauncher from flow
        
        // Handle both string and object launcher formats
        let launcherId = selectedLauncherFromFlow;
        if (typeof selectedLauncherFromFlow === 'object') {
            launcherId = selectedLauncherFromFlow.id || selectedLauncherFromFlow.type;
        }
        
        // If we have detected launchers, find the matching one
        if (detectedLaunchers.length > 0) {
            const matchingLauncher = detectedLaunchers.find(launcher => 
                launcher.id === launcherId || 
                launcher.type === launcherId
            );
            
            if (matchingLauncher) {
                // Found matching launcher
                selectedLauncher = matchingLauncher;
            } else {
                // No matching launcher found, creating launcher object
                selectedLauncher = {
                    name: launcherId === 'xmcl' ? 'XMCL' : 
                          launcherId === 'prism' ? 'Prism Launcher' : 
                          launcherId === 'astralrinth' ? 'AstralRinth' : 
                          launcherId === 'modrinth' ? 'ModRinth' : 
                          launcherId === 'xmcl' ? 'Minecraft Launcher' : 
                          launcherId === 'cracked' ? 'Prism Launcher' : 
                          launcherId === 'other' ? 'Other' : launcherId,
                    id: launcherId,
                    type: launcherId,
                    path: '',
                    description: 'Launcher selected from previous step'
                };
            }
        } else {
            // If no detected launchers yet, create a basic launcher object
            selectedLauncher = {
                name: launcherId === 'xmcl' ? 'XMCL' : 
                      launcherId === 'prism' ? 'Prism Launcher' : 
                      launcherId === 'astralrinth' ? 'AstralRinth' : 
                      launcherId === 'modrinth' ? 'ModRinth' : 
                      launcherId === 'official' ? 'Minecraft Launcher' : 
                      launcherId === 'cracked' ? 'Prism Launcher' : 
                      launcherId === 'other' ? 'Other' : launcherId,
                id: launcherId,
                type: launcherId,
                path: '',
                description: 'Launcher selected from previous step'
            };
        }
    }
    
    $: if (selectedServerFromFlow) {
        // Setting selectedServer from flow
        selectedServer = selectedServerFromFlow;
    }
    
    // Reactive statement to update launcher when detected launchers change
    $: if (detectedLaunchers.length > 0 && selectedLauncherFromFlow && !selectedLauncher) {
        // Updating launcher after detection with flow
        
        let launcherId = selectedLauncherFromFlow;
        if (typeof selectedLauncherFromFlow === 'object') {
            launcherId = selectedLauncherFromFlow.id || selectedLauncherFromFlow.type;
        }
        
        const matchingLauncher = detectedLaunchers.find(launcher => 
            launcher.id === launcherId || 
            launcher.type === launcherId
        );
        
        if (matchingLauncher) {
            // Found matching launcher after detection
            selectedLauncher = matchingLauncher;
        }
    }
    
    const servers = [
        {
            id: 'fabric',
            name: 'Fabric',
            description: 'Modern mod loader with excellent performance and compatibility.',
            icon: '🧵',
            gradient: 'from-purple-500 to-pink-600',
            accent: '#8b5cf6',
            badge: 'Popular',
            badgeColor: 'purple',
            features: ['Modern', 'Fast', 'Compatible']
        },
        {
            id: 'neoforge-create',
            name: 'NeoForge',
            description: 'Advanced modpack with creative building and automation.',
            icon: '⚙️',
            gradient: 'from-blue-500 to-cyan-600',
            accent: '#06b6d4',
            badge: 'Create',
            badgeColor: 'blue',
            features: ['Automation', 'Building', 'Tech']
        },
        {
            id: 'better-than-wolves',
            name: 'Better Than Wolves',
            description: 'Hardcore survival experience with unique mechanics.',
            icon: '🐺',
            gradient: 'from-orange-500 to-red-600',
            accent: '#f97316',
            badge: 'Unavailable',
            badgeColor: 'gray',
            features: ['Survival', 'Challenging', 'Unique'],
            disabled: true
        },
        {
            id: 'forge',
            name: 'Forge',
            description: 'Classic mod loader with extensive mod support.',
            icon: '🔨',
            gradient: 'from-red-500 to-orange-600',
            accent: '#ef4444',
            badge: 'Unavailable',
            badgeColor: 'gray',
            features: ['Classic', 'Extensive', 'Stable'],
            disabled: true
        }
    ];
    
    onMount(async () => {
        await detectInstalledLaunchers();
        await checkInstallerAvailability();
        
        
        // Auto-select the launcher from the previous step
        if (selectedLauncherFromFlow && detectedLaunchers.length > 0) {
            // Auto-selecting launcher from flow
            
            // Handle both string and object launcher formats
            let launcherId = selectedLauncherFromFlow;
            if (typeof selectedLauncherFromFlow === 'object') {
                launcherId = selectedLauncherFromFlow.id || selectedLauncherFromFlow.type;
            }
            
            const matchingLauncher = detectedLaunchers.find(launcher => 
                launcher.id === launcherId || 
                launcher.type === launcherId
            );
            
            if (matchingLauncher) {
                // Found matching launcher
                selectedLauncher = matchingLauncher;
            } else {
                // No matching launcher found, creating launcher object
                // Create a proper launcher object for the flow launcher
                selectedLauncher = {
                    name: launcherId === 'xmcl' ? 'XMCL' : 
                          launcherId === 'prism' ? 'Prism Launcher' : 
                          launcherId === 'astralrinth' ? 'AstralRinth' : 
                          launcherId === 'modrinth' ? 'ModRinth' : 
                          launcherId === 'xmcl' ? 'Minecraft Launcher' : 
                          launcherId === 'cracked' ? 'Prism Launcher' : 
                          'Unknown Launcher',
                    id: launcherId,
                    type: launcherId,
                    path: 'Path not available'
                };
            }
        }
        
        // Auto-select the server from the previous step ONLY if in instance creation mode
        if (selectedServerFromFlow && isInstanceCreationMode) {
            // Auto-selecting server from flow (only in instance creation mode)
            console.log('🔧 [CREATE_INSTANCE] Auto-selecting server from flow:', selectedServerFromFlow, 'isInstanceCreationMode:', isInstanceCreationMode);
            
            // Handle both string and object server formats
            let serverId = selectedServerFromFlow;
            if (typeof selectedServerFromFlow === 'object') {
                serverId = selectedServerFromFlow.id || selectedServerFromFlow.type;
            }
            
            selectedServer = serverId;
            console.log('🔧 [CREATE_INSTANCE] Selected server ID:', selectedServer);
        } else if (selectedServerFromFlow && !isInstanceCreationMode) {
            console.log('🔧 [CREATE_INSTANCE] Server selection mode - NOT auto-selecting server from flow:', selectedServerFromFlow);
        }
    });
    
    
    async function detectInstalledLaunchers() {
        try {
            isDetecting = true;
            status.set('Detecting installed launchers...');
            
            if (typeof window.prism !== 'undefined') {
                const result = await window.prism.detectLaunchers();
                console.log('Launcher detection result:', result);
                
                if (result.success) {
                    detectedLaunchers = result.launchers;
                    console.log('🎮 Detected launchers:', detectedLaunchers);
                    
                    // Add "Other" option for custom Minecraft installations
                    detectedLaunchers.push({
                        name: 'Other',
                        id: 'other',
                        type: 'other',
                        path: '',
                        description: 'Custom Minecraft installation or other launcher'
                    });
                    
                    // Only auto-select first launcher if no launcher was selected from flow
                    if (detectedLaunchers.length > 0 && !selectedLauncherFromFlow) {
                        selectedLauncher = detectedLaunchers[0];
                        console.log('Auto-selected first launcher:', selectedLauncher);
                    }
                    status.set('');
                } else {
                    console.error('Launcher detection failed:', result.error);
                    error.set('Failed to detect launchers: ' + result.error);
                }
            } else {
                console.error('Window.prism API not available');
                error.set('Window.prism API not available');
            }
        } catch (err) {
            console.error('Error detecting launchers:', err);
            error.set('Error detecting launchers: ' + err.message);
        } finally {
            isDetecting = false;
        }
    }
    
    async function checkInstallerAvailability() {
        try {
            if (typeof window.prism !== 'undefined' && window.prism.installer) {
                installerAvailable = await window.prism.installer.isAvailable();
                if (installerAvailable) {
                    const versionInfo = await window.prism.installer.getVersion();
                    installerVersion = versionInfo;
                    addDebugLog(`✅ Minecraft Installer available: ${versionInfo.version || 'Unknown version'}`, 'success');
            } else {
                    addDebugLog('❌ Minecraft Installer not available', 'error');
                }
            }
        } catch (error) {
            console.error('Error checking installer availability:', error);
            addDebugLog(`❌ Error checking installer: ${error.message}`, 'error');
        }
    }
    
    function addDebugLog(message, type = 'info') {
        const timestamp = new Date().toLocaleTimeString();
        debugLog = [...debugLog, { message, type, timestamp }];
        
        // Keep only last 50 log entries to prevent memory issues
        if (debugLog.length > 50) {
            debugLog = debugLog.slice(-50);
        }
    }
    
    
    function startInstallation() {
        if (!selectedLauncher || !selectedServer) {
            error.set('Please select a launcher and modpack');
            return;
        }
        
        // For users with Minecraft installed, validate the path (only if they're using "Other" launcher)
        if (hasMinecraftInstalled && selectedLauncher.id === 'other' && minecraftInstallPath && !isMinecraftPathValid) {
            error.set('Please select a valid Minecraft installation directory');
            return;
        }
        
        // For "Other" launcher, require custom path
        if (selectedLauncher.id === 'other' && (!customMinecraftPath || !customMinecraftPath.trim())) {
            error.set('Please select a Minecraft installation directory');
            return;
        }
        
        // Navigate to installation progress page
        dispatch('start-installation', {
            selectedLauncher,
            selectedServer,
            targetLauncher: selectedLauncher.id === 'other' ? 'other' : selectedLauncher.id,
            minecraftPath: hasMinecraftInstalled ? minecraftInstallPath : null,
            customPath: selectedLauncher.id === 'other' ? customMinecraftPath : null
        });
    }
    
    // Event handlers
    function handleServerSelected(event) {
        selectedServer = event.detail.server;
        error.set(null); // Clear any errors when server is selected
        dispatch('server-selected', event.detail);
    }
    
    function handleNext(event) {
        // Let the parent flow handle step progression
        error.set(null); // Clear any errors when navigating forward
        dispatch('next', event.detail);
    }
    
    function handleBack() {
        // Always go back to the previous onboarding step
        error.set(null); // Clear any errors when navigating back
        dispatch('back');
    }
    
    function handleCreateInstance() {
        startInstallation();
    }
    
    function handleSkipInstallation() {
        dispatch('continue');
    }

    // Minecraft path functions
    async function browseMinecraftPath() {
        try {
            const result = await window.prism?.showOpenDialog({
                properties: ['openDirectory'],
                title: 'Select Minecraft Installation Folder',
                defaultPath: getDefaultMinecraftPath()
            });
            
            if (result && !result.canceled && result.filePaths.length > 0) {
                const selectedPath = result.filePaths[0];
                
                if (selectedLauncher?.id === 'other') {
                    customMinecraftPath = selectedPath;
                    currentPathInput = selectedPath;
                } else {
                    minecraftInstallPath = selectedPath;
                    currentPathInput = selectedPath;
                    await validateMinecraftPath(minecraftInstallPath);
                }
            }
        } catch (error) {
            console.error('Error browsing for Minecraft path:', error);
        }
    }

    function getDefaultMinecraftPath() {
        const os = navigator.platform.toLowerCase();
        if (os.includes('win')) {
            // Use a common Windows path instead of process.env
            return 'C:\\Users\\user\\AppData\\Roaming\\.minecraft';
        } else if (os.includes('mac')) {
            return '~/Library/Application Support/minecraft';
        } else {
            return '~/.minecraft';
        }
    }

    async function validateMinecraftPath(path) {
        if (!path) {
            isMinecraftPathValid = false;
            minecraftPathError = '';
            return;
        }

        // Only validate if user is using "Other" launcher
        if (selectedLauncher?.id !== 'other') {
            isMinecraftPathValid = true; // Assume valid for non-"Other" launchers
            minecraftPathError = '';
            return;
        }

        try {
            // Check if the folder exists and contains Minecraft files
            const isValid = await window.prism?.validateMinecraftInstallation?.(path);
            
            if (isValid) {
                isMinecraftPathValid = true;
                minecraftPathError = '';
            } else {
                isMinecraftPathValid = false;
                minecraftPathError = 'Selected folder does not appear to be a valid Minecraft installation';
            }
        } catch (error) {
            console.error('Error validating Minecraft path:', error);
            isMinecraftPathValid = false;
            minecraftPathError = 'Failed to validate Minecraft installation';
        }
    }

    function setDefaultMinecraftPath() {
        minecraftInstallPath = getDefaultMinecraftPath();
        currentPathInput = minecraftInstallPath; // Direct update
        validateMinecraftPath(minecraftInstallPath);
    }

    // Direct state variable for the current path input
    let currentPathInput = '';
    
    // Update currentPathInput when relevant variables change
    $: if (selectedLauncher?.id === 'other') {
        currentPathInput = customMinecraftPath;
    } else {
        currentPathInput = minecraftInstallPath;
    }

    // Handle path input changes
    function handlePathInput(event) {
        const value = event.target.value;
        if (selectedLauncher?.id === 'other') {
            customMinecraftPath = value;
        } else {
            minecraftInstallPath = value;
            validateMinecraftPath(minecraftInstallPath);
        }
    }

    // Initialize default Minecraft path when component mounts and user has Minecraft
    onMount(async () => {
        // Only set default path for "Other" launcher users
        if (hasMinecraftInstalled && !minecraftInstallPath && selectedLauncher?.id === 'other') {
            setDefaultMinecraftPath();
        }
        
        // Initialize currentPathInput
        if (selectedLauncher?.id === 'other') {
            currentPathInput = customMinecraftPath;
        } else {
            currentPathInput = minecraftInstallPath;
        }
    });

    // Clear error when component is destroyed/unmounted
    onDestroy(() => {
        error.set(null);
    });
</script>

{#if !isInstanceCreationMode}
    <OnboardingServerSelection 
        {hasMinecraftInstalled}
        bind:selectedServer
        on:server-selected={handleServerSelected}
        on:next={handleNext}
        on:back={handleBack}
    />
{:else}
<div class="server-selection-content">
    <!-- Main content -->
    <div class="main-content">
        <!-- Instance Creation Mode -->
        <div class="instance-creation-container">
            <div class="instance-header">
                <h1 class="instance-title">Create Instance</h1>
                <p class="instance-subtitle">Ready to install your selected modpack and launcher configuration.</p>
            </div>
            
            <!-- User Choices Summary -->
            <div class="choices-summary">
                <div class="choice-card">
                    <div class="choice-header">
                        <span class="choice-icon">🎮</span>
                        <span class="choice-label">Modpack</span>
                                </div>
                    <div class="choice-content">
                        <h3 class="choice-title">{selectedServer ? (servers.find(s => s.id === selectedServer)?.name || selectedServer) : 'Unknown'}</h3>
                        <div class="choice-badges">
                            {#each (selectedServer ? servers.find(s => s.id === selectedServer)?.features || [] : []) as feature}
                                <span class="choice-badge">{feature}</span>
                                    {/each}
                                </div>
                            </div>
                </div>
                
                <div class="choice-card">
                    <div class="choice-header">
                        <span class="choice-icon">🚀</span>
                        <span class="choice-label">Launcher</span>
                            </div>
                    <div class="choice-content">
                        <h3 class="choice-title">{selectedLauncher?.name || 'No launcher selected'}</h3>
                        {#if selectedLauncher?.path}
                            <div class="choice-path">
                                <span class="path-icon">📂</span>
                                <span class="path-text">{selectedLauncher.path}</span>
                        </div>
                        {/if}
                            </div>
                        </div>
                    </div>
                    
            <!-- Minecraft Path Selection (for users with Minecraft installed) -->
            <!-- Path Selection (for users with Minecraft OR "Other" launcher) -->
            {#if selectedLauncher?.id === 'other'}
            <div class="minecraft-path-section">
                <div class="section-header">
                    <h3 class="section-title">
                        {#if selectedLauncher?.id === 'other'}
                            🔧 Minecraft Installation Path
                        {:else}
                            📁 Minecraft Installation Path
                        {/if}
                    </h3>
                    <p class="section-description">
                        {#if selectedLauncher?.id === 'other'}
                            Choose where to install Minecraft and the modloader for your custom launcher setup.
                        {:else}
                            Select where your Minecraft is installed to install the modloader.
                        {/if}
                    </p>
                    </div>
                    
                <div class="path-selection">
                    <div class="path-input-group">
                                <input 
                                    type="text" 
                            class="path-input" 
                            placeholder="Select Minecraft installation folder..."
                            value={currentPathInput}
                            readonly={selectedLauncher?.id !== 'other'}
                            on:input={handlePathInput}
                        />
                        <button class="browse-button" on:click={browseMinecraftPath}>
                            📂 Browse
                        </button>
                        {#if selectedLauncher?.id !== 'other'}
                            <button class="default-button" on:click={setDefaultMinecraftPath}>
                                🏠 Default
                            </button>
                        {/if}
                    </div>
                    
                    {#if selectedLauncher?.id === 'other'}
                        {#if customMinecraftPath && !customMinecraftPath.trim()}
                            <div class="path-error">
                                <span class="error-icon">⚠️</span>
                                <span class="error-text">Please enter a valid path</span>
                                </div>
                        {/if}
                    {:else}
                        {#if minecraftPathError}
                            <div class="path-error">
                                <span class="error-icon">⚠️</span>
                                <span class="error-text">{minecraftPathError}</span>
                            </div>
                        {/if}
                        
                        {#if isMinecraftPathValid}
                            <div class="path-success">
                                <span class="success-icon">✅</span>
                                <span class="success-text">Valid Minecraft installation detected</span>
                                    </div>
                        {/if}
                                {/if}
                            </div>
                                    </div>
            {/if}
                                
            <!-- Installer Status -->
            <div class="installer-status-card">
                {#if installerAvailable}
                    <div class="status-success">
                        <span class="status-icon">✅</span>
                        <div class="status-content">
                            <h4 class="status-title">Minecraft Installer Ready</h4>
                            <p class="status-description">The installer will automatically download and install your modpack.</p>
                                    </div>
                                    </div>
                {:else}
                    <div class="status-warning">
                        <span class="status-icon">⚠️</span>
                        <div class="status-content">
                            <h4 class="status-title">Legacy Installation</h4>
                            <p class="status-description">Using fallback installation method. Some features may be limited.</p>
                                        </div>
                                        </div>
                                    {/if}
                                </div>
                                
            <!-- Installation Progress -->
                                    </div>
        
        <!-- Action Buttons -->
        <div class="action-buttons-container">
            <button class="skip-button" on:click={handleSkipInstallation}>
                Skip Installation
            </button>
            
            <button 
                class="create-button" 
                disabled={!selectedLauncher || !selectedServer}
                on:click={handleCreateInstance}
            >
                🚀 Install Modpack
                                            </button>
                                        </div>
        
        <!-- Navigation -->
        <div class="navigation-section instance-creation-mode">
            <button class="back-button" on:click={handleBack}>
                ← Back
            </button>
                                                </div>
                                        </div>
                                    </div>
                                {/if}
    
    <!-- Status and Error Messages -->
    {#if $error}
        <div class="error-message">
            <div class="error-icon">⚠️</div>
            <div class="error-text">{$error}</div>
        </div>
    {:else if $status && !isParsing}
        <div class="status-message">
            <div class="status-icon">ℹ️</div>
            <div class="status-text">{$status}</div>
        </div>
    {/if}

<style>
    .server-selection-content {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        width: 100%;
        max-width: none;
        margin: 0 auto;
        height: auto;
        overflow: visible;
        padding: 0.5rem;
        box-sizing: border-box;
        justify-content: center;
        align-items: center;
    }
    
    
    .main-content {
        display: flex;
        flex-direction: column;
        gap: 0.05rem;
        align-items: center;
        flex: 1;
        overflow: visible;
        width: 100%;
        max-width: 650px;
    }
    
    /* Instance Creation Styles */
    .instance-creation-container {
        display: flex;
        flex-direction: column;
        gap: 0.2rem;
        height: 100%;
        max-width: 100%;
        overflow: hidden;
        flex: 1;
        min-height: 0;
    }
    
    .instance-header {
        text-align: center;
        margin-bottom: 0.2rem;
        flex-shrink: 0;
    }
    
    .instance-title {
        font-size: 1.2rem;
        font-weight: 800;
        color: white;
        margin: 0 0 0.1rem 0;
        background: linear-gradient(135deg, #3b82f6, #8b5cf6);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
        letter-spacing: -0.5px;
    }
    
    .instance-subtitle {
        color: rgba(255, 255, 255, 0.8);
        text-align: center;
        font-size: 0.75rem;
        margin: 0;
        margin-left: auto;
        margin-right: auto;
        max-width: 450px;
        line-height: 1.2;
    }
    
    /* Choice Cards Styles */
    .choices-summary {
        display: flex;
        flex-direction: column;
        gap: 0.15rem;
        margin-bottom: 0.2rem;
        padding: 0.4rem;
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid rgba(255, 255, 255, 0.08);
        border-radius: 10px;
        flex-shrink: 0;
        width: 100%;
    }
    
    .choice-card {
        background: rgba(255, 255, 255, 0.08);
        border: 1px solid rgba(255, 255, 255, 0.15);
        border-radius: 8px;
        padding: 0.25rem;
        backdrop-filter: blur(10px);
        transition: all 0.3s ease;
        flex-shrink: 0;
    }
    
    .choice-card:hover {
        background: rgba(255, 255, 255, 0.12);
        border-color: rgba(255, 255, 255, 0.25);
        transform: translateY(-2px);
    }
    
    .choice-header {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        margin-bottom: 0.4rem;
    }
    
    .choice-icon {
        font-size: 1.5rem;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
        margin-right: 0.25rem;
    }
    
    .choice-label {
        font-size: 0.8rem;
        color: rgba(255, 255, 255, 0.6);
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.5px;
    }
    
    .choice-content {
        display: flex;
        flex-direction: column;
        gap: 0.3rem;
    }
    
    .choice-title {
        font-size: 1rem;
        font-weight: 700;
        color: white;
        margin: 0 0 0 0.5rem;
        text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
    }
    
    
    .choice-badges {
        display: flex;
        flex-wrap: wrap;
        gap: 0.3rem;
        margin-top: 0.3rem;
    }
    
    .choice-badge {
        background: rgba(255, 255, 255, 0.1);
        color: rgba(255, 255, 255, 0.8);
        padding: 0.2rem 0.6rem;
        border-radius: 12px;
        font-size: 0.7rem;
        font-weight: 500;
        border: 1px solid rgba(255, 255, 255, 0.2);
    }
    
    .choice-path {
        display: flex;
        align-items: center;
        gap: 0.4rem;
        margin-top: 0.5rem;
        padding: 0.4rem 0.6rem;
        background: rgba(255, 255, 255, 0.05);
        border-radius: 6px;
        border: 1px solid rgba(255, 255, 255, 0.1);
    }
    
    .path-icon {
        font-size: 0.8rem;
        opacity: 0.8;
    }
    
    .path-text {
            font-size: 0.75rem;
        color: rgba(255, 255, 255, 0.7);
        font-family: monospace;
        word-break: break-all;
    }

    /* Minecraft Path Selection Styles */
    .minecraft-path-section {
        margin: 0.3rem 0;
        padding: 0.6rem;
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid rgba(255, 255, 255, 0.1);
        border-radius: 8px;
    }

    .section-header {
        margin-bottom: 0.4rem;
    }
    
    .section-title {
        font-size: 1rem;
        font-weight: 600;
        color: #ffffff;
        margin: 0 0 0.25rem 0;
    }
    
    .section-description {
        font-size: 0.8rem;
        color: rgba(255, 255, 255, 0.7);
        margin: 0;
        line-height: 1.4;
    }

    .path-selection {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .path-input-group {
        display: flex;
        gap: 0.5rem;
        align-items: center;
    }
    
    .path-input {
        flex: 1;
        padding: 0.6rem 0.8rem;
        background: rgba(255, 255, 255, 0.05);
        border: 1px solid rgba(255, 255, 255, 0.2);
        border-radius: 6px;
        color: #ffffff;
        font-size: 0.8rem;
        transition: all 0.3s ease;
    }
    
    .path-input:focus {
        outline: none;
        border-color: rgba(255, 255, 255, 0.4);
        background: rgba(255, 255, 255, 0.08);
    }

    .browse-button, .default-button {
        padding: 0.6rem 0.8rem;
        background: rgba(255, 255, 255, 0.1);
        border: 1px solid rgba(255, 255, 255, 0.2);
        border-radius: 6px;
        color: #ffffff;
        font-size: 0.8rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.3s ease;
        white-space: nowrap;
    }

    .browse-button:hover, .default-button:hover {
        background: rgba(255, 255, 255, 0.15);
        border-color: rgba(255, 255, 255, 0.3);
    }

    .path-error, .path-success {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem 0.75rem;
        border-radius: 6px;
        font-size: 0.8rem;
        font-weight: 500;
    }
    
    .path-error {
        background: rgba(239, 68, 68, 0.1);
        border: 1px solid rgba(239, 68, 68, 0.3);
        color: #fca5a5;
    }

    .path-success {
        background: rgba(34, 197, 94, 0.1);
        border: 1px solid rgba(34, 197, 94, 0.3);
        color: #86efac;
    }

    .error-icon, .success-icon {
        font-size: 0.9rem;
    }

    
    /* Installer Status Card */
    .installer-status-card {
        margin-bottom: 0.3rem;
        padding: 0.5rem;
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid rgba(255, 255, 255, 0.08);
        border-radius: 10px;
        flex-shrink: 0;
        width: 100%;
    }
    
    .status-success, .status-warning {
        display: flex;
        align-items: center;
        gap: 0.4rem;
        padding: 0.4rem;
        border-radius: 8px;
        backdrop-filter: blur(10px);
    }
    
    .status-success {
        background: rgba(34, 197, 94, 0.1);
        border: 1px solid rgba(34, 197, 94, 0.2);
    }
    
    .status-warning {
        background: rgba(245, 158, 11, 0.1);
        border: 1px solid rgba(245, 158, 11, 0.2);
    }
    
    .status-icon {
        font-size: 1.5rem;
        flex-shrink: 0;
    }
    
    .status-content {
        flex: 1;
    }
    
    .status-title {
        font-size: 0.9rem;
        font-weight: 600;
        color: white;
        margin: 0 0 0.25rem 0;
    }
    
    .status-description {
        font-size: 0.8rem;
        color: rgba(255, 255, 255, 0.7);
        margin: 0;
        line-height: 1.4;
    }
    
    
    
    
    
    
    
    /* Action Buttons */
    .action-buttons-container {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 1rem;
        margin-top: 0.5rem;
        margin-bottom: 0.25rem;
        flex-shrink: 0;
        width: 100%;
    }
    
    /* Navigation */
    .navigation-section {
        display: flex;
        justify-content: flex-start;
        align-items: center;
        gap: 1rem;
        margin-top: 0.25rem;
        flex-shrink: 0;
        padding: 0.5rem 0;
        width: 100%;
    }
    
    .navigation-section.instance-creation-mode {
        display: flex;
        justify-content: flex-start;
        align-items: center;
        position: relative;
        margin-top: 0.25rem;
        padding: 0.5rem 0;
        width: 100%;
    }
    
    .navigation-section.instance-creation-mode .back-button {
        position: static;
        transform: none;
    }
    
    .back-button {
        background: transparent;
        color: rgba(255, 255, 255, 0.6);
        border: 1px solid rgba(255, 255, 255, 0.2);
        padding: 0.5rem 0.75rem;
        border-radius: 6px;
        font-size: 0.8rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.3s ease;
        height: 36px;
        width: 80px;
        flex-shrink: 0;
    }
    
    .back-button:hover {
        background: rgba(255, 255, 255, 0.1);
        color: rgba(255, 255, 255, 0.8);
        border-color: rgba(255, 255, 255, 0.3);
    }
    
    
    .create-button {
        background: linear-gradient(135deg, #22c55e, #16a34a);
        color: white;
        border: none;
        padding: 0.5rem 1rem;
        border-radius: 6px;
        font-size: 0.8rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.3s ease;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.4rem;
        box-shadow: 0 4px 16px rgba(34, 197, 94, 0.4);
        min-width: 120px;
        width: auto;
        max-width: none;
        height: 36px;
    }
    
    .create-button:hover:not(:disabled) {
        background: linear-gradient(135deg, #16a34a, #15803d);
        transform: translateY(-2px);
        box-shadow: 0 8px 25px rgba(34, 197, 94, 0.5);
    }
    
    .create-button:disabled {
        opacity: 0.7;
        cursor: not-allowed;
        transform: none;
        box-shadow: 0 4px 12px rgba(34, 197, 94, 0.2);
    }
    
    .skip-button {
        background: rgba(255, 255, 255, 0.05);
        color: rgba(255, 255, 255, 0.7);
        border: 1px solid rgba(255, 255, 255, 0.15);
        padding: 0.5rem 0.75rem;
        border-radius: 6px;
        font-size: 0.8rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.3s ease;
        min-width: 100px;
        width: auto;
        max-width: none;
        height: 36px;
    }
    
    .skip-button:hover:not(:disabled) {
        background: rgba(255, 255, 255, 0.1);
        color: rgba(255, 255, 255, 0.9);
        border-color: rgba(255, 255, 255, 0.3);
        transform: translateY(-1px);
    }
    
    .skip-button:disabled {
        opacity: 0.7;
        cursor: not-allowed;
    }
    
    
    @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }
    
    /* Status and error messages */
    .status-message, .error-message {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.6rem;
        border-radius: 8px;
        margin-bottom: 0.3rem;
    }
    
    .status-message {
        background: rgba(59, 130, 246, 0.1);
        border: 1px solid rgba(59, 130, 246, 0.2);
    }
    
    .error-message {
        background: rgba(239, 68, 68, 0.1);
        border: 1px solid rgba(239, 68, 68, 0.2);
    }
    
    .status-icon, .error-icon {
        font-size: 1rem;
        flex-shrink: 0;
    }
    
    .status-text, .error-text {
        color: rgba(255, 255, 255, 0.9);
        font-size: 0.8rem;
        line-height: 1.3;
    }
</style>
