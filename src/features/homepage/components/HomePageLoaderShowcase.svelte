<!-- src/features/homepage/components/HomePageLoaderShowcase.svelte -->
<script>
    import { createEventDispatcher } from 'svelte';
    import fabricImage from '../../../assets/images/mc-servers/fabric-art-welcome.jpg';
    import btwImage from '../../../assets/images/mc-servers/btw-art-welcome.jpg';
    import neoforgeImage from '../../../assets/images/mc-servers/neoforge-art-welcome.jpg';
    
    const dispatch = createEventDispatcher();
    
    export let fabricData = { name: '', mc: '', mods: null };
    export let neoforgeData = { name: '', mc: '', mods: null };
    
    const loaders = [
        {
            id: 'fabric',
            title: 'Fabric Client',
            subtitle: 'Lightweight & Fast',
            description: 'The most popular modding platform with extensive mod support and excellent performance.',
            image: fabricImage,
            buttonText: 'Open Fabric Updater',
            gradient: 'from-blue-500 to-indigo-600',
            accent: '#3b82f6',
            features: ['Fast Loading', 'Extensive Mods', 'Active Community'],
            data: fabricData
        },
        {
            id: 'neoforge',
            title: 'NeoForge Client',
            subtitle: 'Powerful & Feature-Rich',
            description: 'Advanced modding framework with powerful APIs and comprehensive mod compatibility.',
            image: neoforgeImage,
            buttonText: 'Open NeoForge Updater',
            gradient: 'from-orange-500 to-red-600',
            accent: '#f97316',
            features: ['Advanced APIs', 'Rich Features', 'Stable Platform'],
            data: neoforgeData
        },
        {
            id: 'btw',
            title: 'Better Than Wolves',
            subtitle: 'Classic & Hardcore',
            description: 'The legendary total conversion mod that transforms Minecraft into a more challenging experience.',
            image: btwImage,
            buttonText: 'View Applications',
            gradient: 'from-amber-500 to-orange-600',
            accent: '#f59e0b',
            features: ['Total Conversion', 'Hardcore Mode', 'Unique Experience'],
            disabled: true,
            data: null
        }
    ];
    
    function handleLoaderActivate(loaderId) {
        if (loaderId !== 'btw') {
            dispatch('loaderSelect', { loader: loaderId });
        }
    }
</script>

<div class="loader-showcase-section">
    <div class="section-header">
        <h2 class="section-title">Mod Loaders</h2>
        <p class="section-subtitle">Choose your preferred modding platform and get started instantly</p>
    </div>
    
    <div class="loader-grid">
        {#each loaders as loader, index}
            <div 
                class="loader-card" 
                class:disabled={loader.disabled}
                style="--animation-delay: {index * 0.2}s; --accent-color: {loader.accent}"
                on:click={() => handleLoaderActivate(loader.id)}
                role="button"
                tabindex={loader.disabled ? -1 : 0}
                on:keydown={(e) => e.key === 'Enter' && handleLoaderActivate(loader.id)}
            >
                <!-- Background image -->
                <div class="card-background">
                    <!-- Use placeholder background instead of image for now -->
                    <div class="background-placeholder {loader.gradient}"></div>
                    <div class="image-overlay"></div>
                    <div class="gradient-overlay {loader.gradient}"></div>
                </div>
                
                <!-- Card content -->
                <div class="card-content">
                    <div class="card-header">
                        <div class="title-section">
                            <h3 class="loader-title">{loader.title}</h3>
                            <p class="loader-subtitle">{loader.subtitle}</p>
                        </div>
                        
                        {#if loader.disabled}
                            <div class="status-badge disabled">Coming Soon</div>
                        {:else}
                            <div class="status-badge active">Available</div>
                        {/if}
                    </div>
                    
                    <p class="loader-description">{loader.description}</p>
                    
                    <!-- Features list -->
                    <div class="features-list">
                        {#each loader.features as feature}
                            <div class="feature-tag">
                                <span class="feature-icon">✨</span>
                                <span class="feature-text">{feature}</span>
                            </div>
                        {/each}
                    </div>
                    
                    <!-- Mod info slot -->
                    {#if loader.data && loader.data.mods}
                        <div class="mod-info-section">
                            <div class="mod-info">
                                <div class="mod-stat">
                                    <span class="stat-label">Version:</span>
                                    <span class="stat-value">{loader.data.mc}</span>
                                </div>
                                <div class="mod-stat">
                                    <span class="stat-label">Mods:</span>
                                    <span class="stat-value">{loader.data.mods}</span>
                                </div>
                            </div>
                        </div>
                    {/if}
                    
                    <!-- Action button -->
                    <div class="card-footer">
                        <button 
                            class="loader-button"
                            class:disabled={loader.disabled}
                            disabled={loader.disabled}
                            on:click|stopPropagation={() => handleLoaderActivate(loader.id)}
                        >
                            <span class="button-icon">
                                {#if loader.disabled}
                                    🔒
                                {:else if loader.id === 'fabric'}
                                    📜
                                {:else if loader.id === 'neoforge'}
                                    🔥
                                {:else}
                                    🐺
                                {/if}
                            </span>
                            <span class="button-text">{loader.buttonText}</span>
                            {#if !loader.disabled}
                                <span class="button-arrow">→</span>
                            {/if}
                        </button>
                    </div>
                </div>
                
                <!-- Hover effects -->
                <div class="card-shine"></div>
                <div class="card-glow"></div>
            </div>
        {/each}
    </div>
</div>

<style>
    .loader-showcase-section {
        margin-bottom: 1.5rem;
        animation: sectionSlideIn 0.8s ease-out 0.3s both;
    }
    
    @keyframes sectionSlideIn {
        from { opacity: 0; transform: translateX(-30px); }
        to { opacity: 1; transform: translateX(0); }
    }
    
    .section-header {
        text-align: center;
        margin-bottom: 1.25rem;
    }
    
    .section-title {
        font-size: 1.8rem;
        font-weight: 800;
        margin: 0 0 0.25rem 0;
        background: linear-gradient(135deg, #f97316, #dc2626);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
    }
    
    .section-subtitle {
        font-size: 1.125rem;
        color: var(--text-secondary, #6b7280);
        margin: 0;
        opacity: 0.9;
    }
    
    .loader-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
        gap: 1rem;
        align-items: stretch;
    }
    
    .loader-card {
        position: relative;
        background: rgba(255, 255, 255, 0.9);
        border: 1px solid rgba(0, 0, 0, 0.1);
        border-radius: 16px;
        overflow: hidden;
        cursor: pointer;
        transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
        backdrop-filter: blur(10px);
        height: 320px;
        max-height: 320px;
        opacity: 0;
        transform: translateY(50px) rotateX(10deg);
        animation: cardReveal 0.8s ease-out forwards;
        animation-delay: var(--animation-delay);
    }
    
    @keyframes cardReveal {
        to {
            opacity: 1;
            transform: translateY(0) rotateX(0deg);
        }
    }
    
    .loader-card:hover:not(.disabled) {
        transform: translateY(-12px) scale(1.02);
        box-shadow: 0 25px 50px rgba(0, 0, 0, 0.15);
        border-color: var(--accent-color);
    }
    
    .loader-card:active:not(.disabled) {
        transform: translateY(-8px) scale(1.01);
    }
    
    .loader-card.disabled {
        opacity: 0.7;
        cursor: not-allowed;
    }
    
    .card-background {
        position: absolute;
        inset: 0;
        z-index: 1;
    }
    
    .background-placeholder {
        width: 100%;
        height: 100%;
        transition: transform 0.4s ease;
    }
    
    .loader-card:hover:not(.disabled) .background-placeholder {
        transform: scale(1.05);
    }
    
    .image-overlay {
        position: absolute;
        inset: 0;
        background: linear-gradient(
            135deg,
            rgba(0, 0, 0, 0.4) 0%,
            rgba(0, 0, 0, 0.2) 50%,
            rgba(0, 0, 0, 0.6) 100%
        );
        transition: opacity 0.3s ease;
    }
    
    .loader-card:hover:not(.disabled) .image-overlay {
        opacity: 0.8;
    }
    
    .gradient-overlay {
        position: absolute;
        inset: 0;
        opacity: 0.3;
        transition: opacity 0.3s ease;
    }
    
    .loader-card:hover:not(.disabled) .gradient-overlay {
        opacity: 0.5;
    }
    
    .from-blue-500.to-indigo-600 {
        background: linear-gradient(135deg, #3b82f6, #4f46e5);
    }
    
    .from-orange-500.to-red-600 {
        background: linear-gradient(135deg, #f97316, #dc2626);
    }
    
    .from-amber-500.to-orange-600 {
        background: linear-gradient(135deg, #f59e0b, #f97316);
    }
    
    .card-content {
        position: relative;
        z-index: 2;
        padding: 1rem;
        height: 100%;
        display: flex;
        flex-direction: column;
        color: white;
        overflow: hidden;
        box-sizing: border-box;
    }
    
    .card-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        margin-bottom: 0.5rem;
        flex-shrink: 0;
    }
    
    .title-section {
        flex: 1;
    }
    
    .loader-title {
        font-size: 1.5rem;
        font-weight: 800;
        margin: 0 0 0.25rem 0;
        text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    }
    
    .loader-subtitle {
        font-size: 0.9rem;
        margin: 0;
        opacity: 0.9;
        font-weight: 500;
        text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
    }
    
    .status-badge {
        padding: 0.375rem 0.75rem;
        border-radius: 50px;
        font-size: 0.75rem;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.05em;
    }
    
    .status-badge.active {
        background: rgba(34, 197, 94, 0.2);
        color: #86efac;
        border: 1px solid rgba(34, 197, 94, 0.3);
    }
    
    .status-badge.disabled {
        background: rgba(156, 163, 175, 0.2);
        color: #d1d5db;
        border: 1px solid rgba(156, 163, 175, 0.3);
    }
    
    .loader-description {
        font-size: 0.85rem;
        line-height: 1.3;
        margin: 0 0 0.75rem 0;
        opacity: 0.95;
        text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
        flex-shrink: 0;
    }
    
    .features-list {
        display: flex;
        flex-wrap: wrap;
        gap: 0.375rem;
        margin-bottom: 0.75rem;
        flex-shrink: 0;
    }
    
    .feature-tag {
        display: flex;
        align-items: center;
        gap: 0.375rem;
        background: rgba(255, 255, 255, 0.15);
        backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 255, 255, 0.2);
        border-radius: 50px;
        padding: 0.375rem 0.75rem;
        font-size: 0.8rem;
        font-weight: 500;
    }
    
    .feature-icon {
        font-size: 0.875rem;
    }
    
    .mod-info-section {
        background: rgba(0, 0, 0, 0.2);
        border-radius: 8px;
        padding: 0.5rem;
        margin-bottom: 0.75rem;
        backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 255, 255, 0.1);
        flex-shrink: 0;
        max-height: 60px;
        overflow: hidden;
    }
    
    .mod-info {
        display: flex;
        gap: 1rem;
        flex-wrap: wrap;
    }
    
    .mod-stat {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }
    
    .stat-label {
        font-size: 0.75rem;
        opacity: 0.8;
        text-transform: uppercase;
        letter-spacing: 0.05em;
    }
    
    .stat-value {
        font-size: 0.875rem;
        font-weight: 600;
    }
    
    .card-footer {
        margin-top: auto;
    }
    
    .loader-button {
        width: 100%;
        background: rgba(255, 255, 255, 0.15);
        backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 255, 255, 0.2);
        border-radius: 16px;
        padding: 1rem 1.5rem;
        color: white;
        font-weight: 600;
        font-size: 1rem;
        cursor: pointer;
        transition: all 0.3s ease;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.75rem;
    }
    
    .loader-button:hover:not(.disabled) {
        background: rgba(255, 255, 255, 0.25);
        border-color: rgba(255, 255, 255, 0.4);
        transform: translateY(-2px);
    }
    
    .loader-button:active:not(.disabled) {
        transform: translateY(0);
    }
    
    .loader-button.disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }
    
    .button-icon {
        font-size: 1.25rem;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.2));
    }
    
    .button-arrow {
        font-size: 1.125rem;
        transition: transform 0.3s ease;
    }
    
    .loader-button:hover:not(.disabled) .button-arrow {
        transform: translateX(4px);
    }
    
    .card-shine {
        position: absolute;
        top: 0;
        left: -100%;
        width: 100%;
        height: 100%;
        background: linear-gradient(
            90deg,
            transparent 0%,
            rgba(255, 255, 255, 0.1) 50%,
            transparent 100%
        );
        z-index: 3;
        transition: left 0.6s ease;
    }
    
    .loader-card:hover:not(.disabled) .card-shine {
        left: 100%;
    }
    
    .card-glow {
        position: absolute;
        inset: -2px;
        background: var(--accent-color);
        border-radius: 26px;
        opacity: 0;
        z-index: 0;
        transition: opacity 0.3s ease;
        filter: blur(20px);
    }
    
    .loader-card:hover:not(.disabled) .card-glow {
        opacity: 0.4;
    }
    
    /* Dark theme */
    @media (prefers-color-scheme: dark) {
        .loader-card {
            background: rgba(15, 23, 42, 0.9);
            border-color: rgba(255, 255, 255, 0.1);
        }
        
        .section-subtitle {
            color: var(--text-secondary, #9ca3af);
        }
    }
    
    /* Responsive design */
    @media (max-width: 768px) {
        .loader-grid {
            grid-template-columns: 1fr;
            gap: 1.5rem;
        }
        
        .section-title {
            font-size: 2rem;
        }
        
        .section-subtitle {
            font-size: 1rem;
        }
        
        .loader-card {
            height: 400px;
            max-height: 400px;
        }
        
        .card-content {
            padding: 1.25rem;
        }
    }
    
    @media (max-width: 640px) {
        .loader-grid {
            gap: 1rem;
        }
        
        .section-title {
            font-size: 1.75rem;
        }
        
        .loader-card {
            height: 360px;
            max-height: 360px;
            border-radius: 16px;
        }
        
        .card-content {
            padding: 1rem;
        }
        
        .card-header {
            flex-direction: column;
            gap: 0.75rem;
            align-items: flex-start;
        }
        
        .status-badge {
            align-self: flex-end;
        }
        
        .loader-title {
            font-size: 1.25rem;
        }
        
        .features-list {
            gap: 0.375rem;
        }
        
        .feature-tag {
            font-size: 0.75rem;
            padding: 0.25rem 0.5rem;
        }
    }
</style>
