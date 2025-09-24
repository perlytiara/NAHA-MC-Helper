<!-- src/features/onboarding/components/OnboardingServerSelection.svelte -->
<script>
    import { createEventDispatcher } from 'svelte';
    
    const dispatch = createEventDispatcher();
    
    // Props
    export let hasMinecraftInstalled = false;
    export let selectedServer = null;
    
    const servers = [
        {
            id: 'fabric',
            name: 'Fabric',
            description: 'Modern mod loader with excellent performance and compatibility.',
            icon: '🧵',
            gradient: 'from-purple-500 to-pink-600',
            accent: '#8b5cf6',
            badge: 'Light',
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
    
    function handleServerSelect(serverId) {
        selectedServer = serverId;
        dispatch('server-selected', { server: serverId });
    }
    
    function handleBack() {
        dispatch('back');
    }
    
    function handleNext() {
        if (selectedServer) {
            dispatch('next', { selectedServer });
        }
    }
</script>

<div class="server-selection-content">
    <!-- Header -->
    <div class="header-section">
        <div class="logo">
            <div class="logo-icon">⚡</div>
            <span class="logo-text">NAHA</span>
        </div>
    </div>
    
    <!-- Main content -->
    <div class="main-content">
        <h1 class="server-title">
            {#if hasMinecraftInstalled}
                Setup Your Modpack
            {:else}
                Choose Your Server
            {/if}
        </h1>
        
        <p class="server-subtitle">
            {#if hasMinecraftInstalled}
                Select which NAHA modpack you'd like to install and configure.
            {:else}
                Select which NAHA server you'd like to join and play on.
            {/if}
        </p>
        
        <div class="server-options">
            <!-- Top row: Fabric, NeoForge, and Forge -->
            <div class="server-row top-row">
                {#each servers.filter(s => s.id !== 'better-than-wolves') as server}
                    <button 
                        type="button"
                        class="server-option" 
                        class:selected={selectedServer === server.id}
                        class:disabled={server.disabled}
                        on:click={() => !server.disabled && handleServerSelect(server.id)}
                        on:keydown={(e) => !server.disabled && e.key === 'Enter' && handleServerSelect(server.id)}
                        aria-checked={selectedServer === server.id}
                        aria-disabled={server.disabled}
                        role="radio"
                        disabled={server.disabled}
                    >
                        <input 
                            type="radio" 
                            name="server" 
                            value={server.id} 
                            checked={selectedServer === server.id} 
                            on:change={() => handleServerSelect(server.id)}
                            style="display: none;"
                        />
                        <div class="option-content">
                            <div class="option-header">
                                <div class="option-title-icon">
                                    <div class="option-icon">{server.icon}</div>
                                    <h3 class="option-title">{server.name}</h3>
                                </div>
                                <div class="option-badge" class:green={server.badgeColor === 'green'} class:blue={server.badgeColor === 'blue'} class:orange={server.badgeColor === 'orange'} class:purple={server.badgeColor === 'purple'} class:gray={server.badgeColor === 'gray'}>
                                    {server.badge}
                                </div>
                            </div>
                            <div class="option-info">
                                <p class="option-description">{server.description}</p>
                                <div class="option-features">
                                    {#each server.features as feature}
                                        <span class="feature-tag">{feature}</span>
                                    {/each}
                                </div>
                            </div>
                        </div>
                        <div class="option-background {server.gradient}"></div>
                    </button>
                {/each}
            </div>
            
            <!-- Bottom row: Better Than Wolves (bigger) -->
            <div class="server-row bottom-row">
                {#each servers.filter(s => s.id === 'better-than-wolves') as server}
                    <button 
                        type="button"
                        class="server-option server-option-large" 
                        class:selected={selectedServer === server.id}
                        class:disabled={server.disabled}
                        on:click={() => !server.disabled && handleServerSelect(server.id)}
                        on:keydown={(e) => !server.disabled && e.key === 'Enter' && handleServerSelect(server.id)}
                        aria-checked={selectedServer === server.id}
                        aria-disabled={server.disabled}
                        role="radio"
                        disabled={server.disabled}
                    >
                        <input 
                            type="radio" 
                            name="server" 
                            value={server.id} 
                            checked={selectedServer === server.id} 
                            on:change={() => handleServerSelect(server.id)}
                            style="display: none;"
                        />
                        <div class="option-content">
                            <div class="option-header">
                                <div class="option-title-icon">
                                    <div class="option-icon">{server.icon}</div>
                                    <h3 class="option-title">{server.name}</h3>
                                </div>
                                <div class="option-badge" class:green={server.badgeColor === 'green'} class:blue={server.badgeColor === 'blue'} class:orange={server.badgeColor === 'orange'} class:purple={server.badgeColor === 'purple'} class:gray={server.badgeColor === 'gray'}>
                                    {server.badge}
                                </div>
                            </div>
                            <div class="option-info">
                                <p class="option-description">{server.description}</p>
                                <div class="option-features">
                                    {#each server.features as feature}
                                        <span class="feature-tag">{feature}</span>
                                    {/each}
                                </div>
                            </div>
                        </div>
                        <div class="option-background {server.gradient}"></div>
                    </button>
                {/each}
            </div>
        </div>
        
        <!-- Navigation -->
        <div class="navigation-section">
            <button class="back-button" on:click={handleBack}>
                ← Back
            </button>
            <button 
                class="next-button" 
                class:disabled={!selectedServer}
                disabled={!selectedServer}
                on:click={handleNext}
            >
                Next
            </button>
        </div>
    </div>
</div>

<style>
    .server-selection-content {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
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
        border: 1px solid rgba(255, 255, 255, 0.2);
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
        justify-content: center;
    }
    
    .server-title {
        font-size: 1.1rem;
        font-weight: 700;
        color: white;
        margin: 0;
        line-height: 1.2;
        text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
        text-align: center;
    }
    
    .server-subtitle {
        font-size: 0.75rem;
        color: rgba(255, 255, 255, 0.7);
        line-height: 1.4;
        margin: 0;
        text-align: center;
        max-width: 400px;
    }
    
    .server-options {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        width: 100%;
        max-width: none;
        flex: 0 0 auto;
        overflow-x: visible;
        overflow-y: visible;
        justify-content: center;
        align-items: center;
        margin-bottom: 0.5rem;
        padding: 0.5rem 0;
        min-width: fit-content;
    }
    
    .server-row {
        display: flex;
        flex-direction: row;
        gap: 1.2rem;
        justify-content: center;
        align-items: center;
        width: 100%;
    }
    
    .server-option {
        position: relative;
        cursor: pointer;
        border: 2px solid rgba(255, 255, 255, 0.3);
        border-radius: 12px;
        padding: 1rem;
        background: linear-gradient(145deg, 
            rgba(255, 255, 255, 0.15) 0%, 
            rgba(255, 255, 255, 0.08) 100%
        );
        backdrop-filter: blur(15px);
        transition: all 0.3s ease;
        overflow: hidden;
        height: 160px;
        width: 280px;
        flex: 0 0 280px;
        display: flex;
        flex-direction: column;
        box-shadow: 0 6px 24px rgba(0, 0, 0, 0.3);
    }
    
    .server-option-large {
        height: 160px;
        width: 400px;
        flex: 0 0 400px;
    }
    
    .server-option:hover {
        background: rgba(255, 255, 255, 0.15);
        border-color: rgba(255, 255, 255, 0.3);
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    }
    
    .server-option.selected {
        border-color: var(--accent-color);
        background: rgba(255, 255, 255, 0.2);
        box-shadow: 0 6px 20px rgba(0, 0, 0, 0.2);
        transform: translateY(-1px);
    }
    
    .server-option.disabled {
        opacity: 0.5;
        cursor: not-allowed;
        background: linear-gradient(145deg, 
            rgba(156, 163, 175, 0.1) 0%, 
            rgba(107, 114, 128, 0.05) 100%
        );
        border-color: rgba(156, 163, 175, 0.3);
    }
    
    .server-option.disabled:hover {
        transform: none;
        box-shadow: none;
        background: linear-gradient(145deg, 
            rgba(156, 163, 175, 0.1) 0%, 
            rgba(107, 114, 128, 0.05) 100%
        );
        border-color: rgba(156, 163, 175, 0.3);
    }
    
    .option-background {
        position: absolute;
        inset: 0;
        opacity: 0.1;
        transition: opacity 0.3s ease;
    }
    
    .server-option.selected .option-background {
        opacity: 0.2;
    }
    
    .from-purple-500.to-pink-600 {
        background: linear-gradient(135deg, #8b5cf6, #db2777);
    }
    
    .from-blue-500.to-cyan-600 {
        background: linear-gradient(135deg, #3b82f6, #06b6d4);
    }
    
    .from-orange-500.to-red-600 {
        background: linear-gradient(135deg, #f97316, #dc2626);
    }
    
    .from-red-500.to-orange-600 {
        background: linear-gradient(135deg, #ef4444, #ea580c);
    }
    
    .option-content {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        position: relative;
        z-index: 2;
        height: 100%;
        justify-content: space-between;
    }
    
    .option-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 0.3rem;
    }
    
    .option-icon {
        font-size: 1.5rem;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
        transition: transform 0.3s ease;
        margin-right: 0.5rem;
    }
    
    .option-title-icon {
        display: flex;
        align-items: center;
        flex: 1;
    }
    
    .server-option:hover .option-icon {
        transform: scale(1.1);
    }
    
    .option-badge {
        padding: 0.25rem 0.6rem;
        border-radius: 8px;
        font-size: 0.65rem;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
        white-space: nowrap;
    }
    
    .option-badge.green {
        background: linear-gradient(135deg, #22c55e, #16a34a);
        color: white;
    }
    
    .option-badge.blue {
        background: linear-gradient(135deg, #3b82f6, #2563eb);
        color: white;
    }
    
    .option-badge.orange {
        background: linear-gradient(135deg, #f97316, #ea580c);
        color: white;
    }
    
    .option-badge.purple {
        background: linear-gradient(135deg, #8b5cf6, #7c3aed);
        color: white;
    }
    
    .option-badge.gray {
        background: linear-gradient(135deg, #6b7280, #4b5563);
        color: white;
    }
    
    .option-info {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: 0.2rem;
        justify-content: space-between;
    }
    
    .option-title {
        font-size: 1rem;
        font-weight: 700;
        color: white;
        margin: 0;
        text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
        transition: color 0.3s ease;
        line-height: 1.2;
        white-space: nowrap;
    }
    
    .server-option.selected .option-title {
        color: #60a5fa;
    }
    
    .option-description {
        font-size: 0.8rem;
        color: rgba(255, 255, 255, 0.7);
        line-height: 1.3;
        transition: color 0.3s ease;
        margin: 0;
        text-align: center;
    }
    
    .server-option.selected .option-description {
        color: rgba(255, 255, 255, 0.9);
    }
    
    .option-features {
        display: flex;
        flex-wrap: nowrap;
        gap: 0.1rem;
        margin-top: 0.3rem;
        justify-content: center;
        overflow-x: auto;
    }
    
    .feature-tag {
        background: rgba(255, 255, 255, 0.1);
        color: rgba(255, 255, 255, 0.8);
        padding: 0.1rem 0.25rem;
        border-radius: 4px;
        font-size: 0.6rem;
        font-weight: 500;
        border: 1px solid rgba(255, 255, 255, 0.2);
        white-space: nowrap;
        flex-shrink: 0;
    }
    
    .server-option.selected .feature-tag {
        background: rgba(255, 255, 255, 0.15);
        color: rgba(255, 255, 255, 0.9);
        border-color: rgba(255, 255, 255, 0.3);
    }
    
    .navigation-section {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 2rem;
        margin-top: 0.5rem;
        flex-shrink: 0;
        padding: 0.5rem 0;
    }
    
    .back-button {
        background: transparent;
        color: rgba(255, 255, 255, 0.6);
        border: 2px solid rgba(255, 255, 255, 0.2);
        padding: 0.6rem 1rem;
        border-radius: 8px;
        font-size: 0.85rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.3s ease;
        min-width: 100px;
    }
    
    .back-button:hover {
        background: rgba(255, 255, 255, 0.1);
        color: rgba(255, 255, 255, 0.8);
        border-color: rgba(255, 255, 255, 0.3);
    }
    
    .next-button {
        background: linear-gradient(135deg, #22c55e, #16a34a);
        color: white;
        border: none;
        padding: 0.6rem 1rem;
        border-radius: 8px;
        font-size: 0.85rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.3s ease;
        box-shadow: 0 4px 16px rgba(34, 197, 94, 0.3);
        min-width: 100px;
    }
    
    .next-button:hover:not(.disabled) {
        background: linear-gradient(135deg, #16a34a, #15803d);
        transform: translateY(-1px);
        box-shadow: 0 6px 20px rgba(34, 197, 94, 0.4);
    }
    
    .next-button.disabled {
        background: rgba(255, 255, 255, 0.2);
        color: rgba(255, 255, 255, 0.4);
        cursor: not-allowed;
        box-shadow: none;
    }
</style>
