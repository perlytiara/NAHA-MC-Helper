<!-- src/features/onboarding/components/OnboardingMinecraftSetup.svelte -->
<script>
    import { createEventDispatcher, onMount } from 'svelte';
    
    const dispatch = createEventDispatcher();
    
    export let selectedLauncher = 'cracked';
    
    let currentStep = 1;
    let totalSteps = 4;
    let selectedModLoader = null;
    let selectedServerType = null;
    let minecraftVersion = null;
    let showProgress = false;
    let progress = 0;
    let status = '';
    let error = null;
    
    // Step configurations
    const steps = [
        {
            id: 1,
            title: 'Choose Mod Loader',
            subtitle: 'Select the mod loader for your Minecraft installation',
            icon: '🔧'
        },
        {
            id: 2,
            title: 'Select Server Type',
            subtitle: 'Choose the type of server you want to connect to',
            icon: '🌐'
        },
        {
            id: 3,
            title: 'Minecraft Version',
            subtitle: 'Pick your preferred Minecraft version',
            icon: '📦'
        },
        {
            id: 4,
            title: 'Setup Complete',
            subtitle: 'Your Minecraft setup is ready to go!',
            icon: '✅'
        }
    ];
    
    const modLoaders = [
        {
            id: 'fabric',
            name: 'Fabric',
            description: 'Lightweight and fast mod loader',
            icon: '🧵',
            color: 'from-purple-500 to-pink-600',
            features: ['Lightweight', 'Fast', 'Modern']
        },
        {
            id: 'forge',
            name: 'Forge',
            description: 'Traditional and widely supported',
            icon: '⚒️',
            color: 'from-orange-500 to-red-600',
            features: ['Stable', 'Wide Support', 'Mature']
        },
        {
            id: 'neoforge',
            name: 'NeoForge',
            description: 'Next generation Forge fork',
            icon: '⚡',
            color: 'from-blue-500 to-indigo-600',
            features: ['Modern', 'Performance', 'Future-proof']
        }
    ];
    
    const serverTypes = [
        {
            id: 'singleplayer',
            name: 'Single Player',
            description: 'Play alone with mods',
            icon: '👤',
            color: 'from-green-500 to-emerald-600'
        },
        {
            id: 'multiplayer',
            name: 'Multiplayer',
            description: 'Join or host servers',
            icon: '👥',
            color: 'from-blue-500 to-cyan-600'
        },
        {
            id: 'both',
            name: 'Both',
            description: 'Single player and multiplayer',
            icon: '🎮',
            color: 'from-purple-500 to-violet-600'
        }
    ];
    
    const minecraftVersions = [
        { version: '1.21.1', label: '1.21.1 (Latest)', recommended: true },
        { version: '1.21', label: '1.21' },
        { version: '1.20.6', label: '1.20.6', recommended: true },
        { version: '1.20.4', label: '1.20.4' },
        { version: '1.20.1', label: '1.20.1' },
        { version: '1.19.4', label: '1.19.4' }
    ];
    
    function nextStep() {
        if (currentStep < totalSteps) {
            currentStep++;
            if (currentStep === 4) {
                handleSetupComplete();
            }
        }
    }
    
    function previousStep() {
        if (currentStep > 1) {
            currentStep--;
        }
    }
    
    function handleModLoaderSelect(loaderId) {
        selectedModLoader = loaderId;
        nextStep();
    }
    
    function handleServerTypeSelect(serverId) {
        selectedServerType = serverId;
        nextStep();
    }
    
    function handleVersionSelect(version) {
        minecraftVersion = version;
        nextStep();
    }
    
    function handleVersionSubmit() {
        nextStep();
    }
    
    async function handleSetupComplete() {
        showProgress = true;
        progress = 0;
        status = 'Setting up your Minecraft installation...';
        
        // Simulate setup process
        const steps = [
            'Configuring mod loader...',
            'Setting up server connection...',
            'Downloading Minecraft version...',
            'Creating installation directory...',
            'Finalizing setup...'
        ];
        
        for (let i = 0; i < steps.length; i++) {
            status = steps[i];
            progress = (i + 1) * 20;
            await new Promise(resolve => setTimeout(resolve, 800));
        }
        
        progress = 100;
        status = 'Setup complete!';
        
        setTimeout(() => {
            dispatch('setup-complete', {
                modLoader: selectedModLoader,
                serverType: selectedServerType,
                version: minecraftVersion,
                path: installationPath
            });
        }, 1500);
    }
    
    function handleBack() {
        dispatch('back');
    }
    
    function handleSkip() {
        dispatch('skip-setup');
    }
    
    onMount(() => {
        // Component initialization
    });
</script>

<div class="minecraft-setup-content">
    <!-- Header -->
    <div class="header-section">
        <div class="logo">
            <div class="logo-icon">⚡</div>
            <span class="logo-text">NAHA</span>
        </div>
    </div>
    
    <!-- Step indicator -->
    <div class="step-indicator">
        <div class="step-title">{steps[currentStep - 1].title}</div>
        <div class="step-subtitle">{steps[currentStep - 1].subtitle}</div>
    </div>
    
    <!-- Main content -->
    <div class="main-content">
        {#if !showProgress}
            <!-- Step 1: Mod Loader Selection -->
            {#if currentStep === 1}
                <div class="step-content">
                    <div class="step-header">
                        <div class="step-icon">{steps[0].icon}</div>
                    </div>
                    
                    <div class="options-grid">
                        {#each modLoaders as loader}
                            <button 
                                class="option-card {loader.color}" 
                                on:click={() => handleModLoaderSelect(loader.id)}
                            >
                                <div class="option-icon">{loader.icon}</div>
                                <div class="option-content">
                                    <h3 class="option-title">{loader.name}</h3>
                                    <p class="option-description">{loader.description}</p>
                                    <div class="option-features">
                                        {#each loader.features as feature}
                                            <span class="feature-tag">{feature}</span>
                                        {/each}
                                    </div>
                                </div>
                            </button>
                        {/each}
                    </div>
                </div>
            
            <!-- Step 2: Server Type Selection -->
            {:else if currentStep === 2}
                <div class="step-content">
                    <div class="step-header">
                        <div class="step-icon">{steps[1].icon}</div>
                    </div>
                    
                    <div class="options-row">
                        {#each serverTypes as server}
                            <button 
                                class="option-card {server.color}" 
                                on:click={() => handleServerTypeSelect(server.id)}
                            >
                                <div class="option-icon">{server.icon}</div>
                                <div class="option-content">
                                    <h3 class="option-title">{server.name}</h3>
                                    <p class="option-description">{server.description}</p>
                                </div>
                            </button>
                        {/each}
                    </div>
                </div>
            
            <!-- Step 3: Minecraft Version -->
            {:else if currentStep === 3}
                <div class="step-content">
                    <div class="step-header">
                        <div class="step-icon">{steps[2].icon}</div>
                    </div>
                    
                    <div class="version-list">
                        {#each minecraftVersions as version}
                            <button 
                                class="version-option" 
                                class:recommended={version.recommended}
                                on:click={() => handleVersionSelect(version.version)}
                            >
                                <div class="version-info">
                                    <span class="version-label">{version.label}</span>
                                    {#if version.recommended}
                                        <span class="recommended-badge">Recommended</span>
                                    {/if}
                                </div>
                                <div class="version-arrow">→</div>
                            </button>
                        {/each}
                    </div>
                </div>
            
            <!-- Step 4: Setup Complete -->
            {:else if currentStep === 4}
                <div class="step-content">
                    <div class="step-header">
                        <div class="step-icon">{steps[3].icon}</div>
                    </div>
                    
                    <div class="completion-summary">
                        <div class="completion-icon">🎉</div>
                        <h3 class="completion-title">Minecraft Setup Complete!</h3>
                        <p class="completion-description">
                            Your {modLoaders.find(l => l.id === selectedModLoader)?.name} installation is ready for Minecraft {minecraftVersion} using {selectedLauncher === 'cracked' ? 'Prism Launcher' : selectedLauncher === 'xmcl' ? 'XMCL' : 'AstralRinth'}.
                        </p>
                    </div>
                </div>
            {/if}
        {/if}
        
        <!-- Progress display -->
        {#if showProgress}
            <div class="progress-display">
                <div class="progress-status">{status}</div>
                <div class="progress-bar-large">
                    <div class="progress-fill-large" style="width: {progress}%"></div>
                </div>
                <div class="progress-percentage">{Math.round(progress)}%</div>
            </div>
        {/if}
    </div>
    
    <!-- Navigation -->
    <div class="navigation-section">
        <button class="back-button" on:click={currentStep === 1 ? handleBack : previousStep}>
            ← {currentStep === 1 ? 'Back to Launcher Selection' : 'Previous'}
        </button>
        
        <div class="navigation-actions">
            <button class="skip-button" on:click={handleSkip}>
                Skip Setup
            </button>
            
            {#if currentStep === 4 && !showProgress}
                <button class="complete-button" on:click={handleSetupComplete}>
                    Complete Setup
                </button>
            {/if}
        </div>
    </div>
</div>

<style>
    .minecraft-setup-content {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        width: 100%;
        max-width: 600px;
        margin: 0 auto;
        max-height: 100vh;
        overflow: hidden;
        padding: 1rem 0.5rem;
    }
    
    .header-section {
        display: flex;
        justify-content: center;
    }
    
    .logo {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 0.5rem 1rem;
        background: rgba(255, 255, 255, 0.1);
        border-radius: 12px;
        backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 255, 255, 0.2);
    }
    
    .logo-icon {
        width: 24px;
        height: 24px;
        background: linear-gradient(135deg, #22c55e, #16a34a);
        border-radius: 6px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 12px;
        color: white;
        box-shadow: 0 2px 8px rgba(34, 197, 94, 0.3);
    }
    
    .logo-text {
        font-size: 1.5rem;
        font-weight: 800;
        color: white;
        letter-spacing: -0.5px;
    }
    
    .step-indicator {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
        align-items: center;
        margin-top: 1rem;
        text-align: center;
    }
    
    .step-title {
        color: white;
        font-size: 1.125rem;
        font-weight: 600;
    }
    
    .step-subtitle {
        color: rgba(255, 255, 255, 0.7);
        font-size: 0.875rem;
    }
    
    .main-content {
        flex: 1;
        display: flex;
        flex-direction: column;
        justify-content: center;
        overflow: hidden;
    }
    
    .step-content {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        align-items: center;
        text-align: center;
    }
    
    .step-header {
        display: flex;
        justify-content: center;
        margin-bottom: 1rem;
    }
    
    .step-icon {
        font-size: 2rem;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
    }
    
    .options-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
        gap: 1rem;
        width: 100%;
        max-width: 500px;
    }
    
    .options-row {
        display: flex;
        gap: 1rem;
        width: 100%;
        max-width: 500px;
        justify-content: center;
    }
    
    .option-card {
        background: rgba(255, 255, 255, 0.1);
        border: 2px solid rgba(255, 255, 255, 0.2);
        border-radius: 12px;
        padding: 1rem;
        cursor: pointer;
        transition: all 0.3s ease;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.75rem;
        position: relative;
        overflow: hidden;
        min-height: 140px;
        backdrop-filter: blur(10px);
    }
    
    .option-card:hover {
        background: rgba(255, 255, 255, 0.15);
        border-color: rgba(255, 255, 255, 0.3);
        transform: translateY(-2px);
        box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
    }
    
    .option-card::before {
        content: '';
        position: absolute;
        inset: 0;
        background: linear-gradient(135deg, var(--gradient-start), var(--gradient-end));
        opacity: 0.1;
        transition: opacity 0.3s ease;
    }
    
    .option-card:hover::before {
        opacity: 0.2;
    }
    
    .from-purple-500.to-pink-600 {
        --gradient-start: #8b5cf6;
        --gradient-end: #db2777;
    }
    
    .from-orange-500.to-red-600 {
        --gradient-start: #f97316;
        --gradient-end: #dc2626;
    }
    
    .from-blue-500.to-indigo-600 {
        --gradient-start: #3b82f6;
        --gradient-end: #4f46e5;
    }
    
    .from-green-500.to-emerald-600 {
        --gradient-start: #22c55e;
        --gradient-end: #059669;
    }
    
    .from-blue-500.to-cyan-600 {
        --gradient-start: #3b82f6;
        --gradient-end: #0891b2;
    }
    
    .from-purple-500.to-violet-600 {
        --gradient-start: #8b5cf6;
        --gradient-end: #7c3aed;
    }
    
    .option-icon {
        font-size: 2rem;
        position: relative;
        z-index: 2;
    }
    
    .option-content {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        align-items: center;
        position: relative;
        z-index: 2;
    }
    
    .option-title {
        font-size: 1rem;
        font-weight: 600;
        color: white;
        margin: 0;
    }
    
    .option-description {
        font-size: 0.75rem;
        color: rgba(255, 255, 255, 0.8);
        margin: 0;
        text-align: center;
    }
    
    .option-features {
        display: flex;
        flex-wrap: wrap;
        gap: 0.25rem;
        justify-content: center;
    }
    
    .feature-tag {
        background: rgba(255, 255, 255, 0.1);
        color: rgba(255, 255, 255, 0.8);
        padding: 0.125rem 0.375rem;
        border-radius: 4px;
        font-size: 0.625rem;
        font-weight: 500;
        border: 1px solid rgba(255, 255, 255, 0.2);
    }
    
    .version-list {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        width: 100%;
        max-width: 400px;
    }
    
    .version-option {
        background: rgba(255, 255, 255, 0.1);
        border: 2px solid rgba(255, 255, 255, 0.2);
        border-radius: 8px;
        padding: 1rem;
        cursor: pointer;
        transition: all 0.3s ease;
        display: flex;
        justify-content: space-between;
        align-items: center;
        backdrop-filter: blur(10px);
    }
    
    .version-option:hover {
        background: rgba(255, 255, 255, 0.15);
        border-color: rgba(255, 255, 255, 0.3);
        transform: translateX(4px);
    }
    
    .version-option.recommended {
        border-color: rgba(34, 197, 94, 0.5);
        background: rgba(34, 197, 94, 0.1);
    }
    
    .version-info {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }
    
    .version-label {
        color: white;
        font-weight: 600;
        font-size: 0.875rem;
    }
    
    .recommended-badge {
        background: linear-gradient(135deg, #22c55e, #16a34a);
        color: white;
        padding: 0.125rem 0.375rem;
        border-radius: 4px;
        font-size: 0.625rem;
        font-weight: 600;
        align-self: flex-start;
    }
    
    .version-arrow {
        color: rgba(255, 255, 255, 0.6);
        font-size: 1rem;
        font-weight: bold;
    }
    
    .path-section {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        width: 100%;
        max-width: 500px;
    }
    
    .path-input-group {
        display: flex;
        gap: 0.5rem;
        align-items: center;
    }
    
    .path-label {
        color: white;
        font-weight: 600;
        font-size: 0.875rem;
        min-width: 140px;
        text-align: left;
    }
    
    .path-input {
        flex: 1;
        background: rgba(255, 255, 255, 0.1);
        border: 1px solid rgba(255, 255, 255, 0.2);
        border-radius: 6px;
        padding: 0.5rem;
        color: white;
        font-size: 0.875rem;
        backdrop-filter: blur(10px);
    }
    
    .path-input::placeholder {
        color: rgba(255, 255, 255, 0.5);
    }
    
    .browse-button {
        background: rgba(255, 255, 255, 0.1);
        color: white;
        border: 1px solid rgba(255, 255, 255, 0.2);
        border-radius: 6px;
        padding: 0.5rem 0.75rem;
        font-size: 0.875rem;
        cursor: pointer;
        transition: all 0.3s ease;
    }
    
    .browse-button:hover {
        background: rgba(255, 255, 255, 0.15);
        border-color: rgba(255, 255, 255, 0.3);
    }
    
    .path-help {
        color: rgba(255, 255, 255, 0.6);
        font-size: 0.75rem;
        margin: 0;
        text-align: left;
    }
    
    .summary-section {
        background: rgba(255, 255, 255, 0.05);
        border: 1px solid rgba(255, 255, 255, 0.1);
        border-radius: 8px;
        padding: 1rem;
        width: 100%;
        max-width: 500px;
    }
    
    .summary-title {
        color: white;
        font-size: 1rem;
        font-weight: 600;
        margin: 0 0 0.75rem 0;
    }
    
    .summary-items {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }
    
    .summary-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }
    
    .summary-label {
        color: rgba(255, 255, 255, 0.7);
        font-size: 0.875rem;
    }
    
    .summary-value {
        color: white;
        font-size: 0.875rem;
        font-weight: 600;
    }
    
    .continue-button {
        background: linear-gradient(135deg, #22c55e, #16a34a);
        color: white;
        border: none;
        border-radius: 8px;
        padding: 0.75rem 1.5rem;
        font-size: 0.875rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.3s ease;
        box-shadow: 0 4px 16px rgba(34, 197, 94, 0.3);
    }
    
    .continue-button:hover:not(:disabled) {
        background: linear-gradient(135deg, #16a34a, #15803d);
        transform: translateY(-1px);
        box-shadow: 0 6px 20px rgba(34, 197, 94, 0.4);
    }
    
    .continue-button:disabled {
        background: rgba(255, 255, 255, 0.2);
        color: rgba(255, 255, 255, 0.4);
        cursor: not-allowed;
        box-shadow: none;
    }
    
    .completion-summary {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        padding: 2rem;
        background: rgba(34, 197, 94, 0.1);
        border: 1px solid rgba(34, 197, 94, 0.2);
        border-radius: 12px;
        max-width: 500px;
    }
    
    .completion-icon {
        font-size: 3rem;
    }
    
    .completion-title {
        color: white;
        font-size: 1.5rem;
        font-weight: 700;
        margin: 0;
    }
    
    .completion-description {
        color: rgba(255, 255, 255, 0.8);
        font-size: 0.875rem;
        text-align: center;
        margin: 0;
    }
    
    .progress-display {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        align-items: center;
        padding: 2rem;
    }
    
    .progress-status {
        color: white;
        font-size: 1rem;
        font-weight: 600;
        text-align: center;
    }
    
    .progress-bar-large {
        width: 100%;
        max-width: 300px;
        height: 8px;
        background: rgba(255, 255, 255, 0.1);
        border-radius: 4px;
        overflow: hidden;
    }
    
    .progress-fill-large {
        height: 100%;
        background: linear-gradient(90deg, #22c55e, #16a34a);
        border-radius: 4px;
        transition: width 0.3s ease;
    }
    
    .progress-percentage {
        color: rgba(255, 255, 255, 0.8);
        font-size: 0.875rem;
        font-weight: 600;
    }
    
    .navigation-section {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 1rem;
        margin-top: 1rem;
        flex-shrink: 0;
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
    }
    
    .back-button:hover {
        background: rgba(255, 255, 255, 0.1);
        color: rgba(255, 255, 255, 0.8);
        border-color: rgba(255, 255, 255, 0.3);
    }
    
    .navigation-actions {
        display: flex;
        gap: 0.5rem;
    }
    
    .skip-button {
        background: transparent;
        color: rgba(255, 255, 255, 0.6);
        border: 1px solid rgba(255, 255, 255, 0.2);
        padding: 0.5rem 0.75rem;
        border-radius: 6px;
        font-size: 0.8rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.3s ease;
    }
    
    .skip-button:hover {
        background: rgba(255, 255, 255, 0.1);
        color: rgba(255, 255, 255, 0.8);
        border-color: rgba(255, 255, 255, 0.3);
    }
    
    .complete-button {
        background: linear-gradient(135deg, #22c55e, #16a34a);
        color: white;
        border: none;
        padding: 0.5rem 1rem;
        border-radius: 6px;
        font-size: 0.8rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.3s ease;
        box-shadow: 0 4px 16px rgba(34, 197, 94, 0.3);
    }
    
    .complete-button:hover {
        background: linear-gradient(135deg, #16a34a, #15803d);
        transform: translateY(-1px);
        box-shadow: 0 6px 20px rgba(34, 197, 94, 0.4);
    }
    
    /* Responsive design */
    @media (max-width: 768px) {
        .minecraft-setup-content {
            gap: 0.75rem;
            max-width: 500px;
            padding: 0.75rem 0.25rem;
        }
        
        .options-grid {
            grid-template-columns: 1fr;
            gap: 0.75rem;
        }
        
        .options-row {
            flex-direction: column;
            gap: 0.75rem;
        }
        
        .path-input-group {
            flex-direction: column;
            gap: 0.5rem;
            align-items: stretch;
        }
        
        .path-label {
            min-width: auto;
            text-align: center;
        }
        
        .navigation-section {
            flex-direction: column;
            gap: 0.75rem;
        }
        
        .navigation-actions {
            width: 100%;
            justify-content: center;
        }
    }
    
    @media (max-width: 640px) {
        .minecraft-setup-content {
            gap: 0.5rem;
            max-width: 400px;
            padding: 0.5rem 0.25rem;
        }
        
        .step-title-main {
            font-size: 1.25rem;
        }
        
        .step-subtitle {
            font-size: 0.8rem;
        }
        
        .option-card {
            padding: 0.75rem;
            min-height: 120px;
        }
        
        .option-icon {
            font-size: 1.5rem;
        }
        
        .option-title {
            font-size: 0.875rem;
        }
        
        .option-description {
            font-size: 0.7rem;
        }
    }
</style>
