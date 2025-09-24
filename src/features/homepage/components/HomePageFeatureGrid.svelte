<!-- src/features/homepage/components/HomePageFeatureGrid.svelte -->
<script>
    import { createEventDispatcher } from 'svelte';
    import { debug } from '../../../shared/stores/appStore';
    
    const dispatch = createEventDispatcher();
    
    // Enhanced feature data with more details
    const features = [
        {
            id: 'install',
            title: 'Install Launchers',
            description: 'Install PrismLauncher (Cracked) or XMCL (Official) for your OS.',
            icon: '🎮',
            gradient: 'from-purple-500 to-indigo-600',
            category: 'Essential',
            stats: { time: '2 min', difficulty: 'Easy' }
        },
        {
            id: 'update',
            title: 'Update Mods',
            description: 'Update or switch mod loaders (Fabric / NeoForge) for an instance.',
            icon: '🔄',
            gradient: 'from-blue-500 to-cyan-600',
            category: 'Essential',
            stats: { time: '5 min', difficulty: 'Easy' }
        },
        {
            id: 'packer',
            title: 'Packer',
            description: 'Create a bundle pack for modpacks (developer tool).',
            icon: '📦',
            gradient: 'from-green-500 to-teal-600',
            category: 'Developer',
            debugOnly: true,
            stats: { time: '10 min', difficulty: 'Advanced' }
        },
        {
            id: 'mrpack-editor',
            title: 'MRPack Editor',
            description: 'Edit .mrpack metadata (developer tool).',
            icon: '✏️',
            gradient: 'from-orange-500 to-red-600',
            category: 'Developer',
            debugOnly: true,
            stats: { time: '3 min', difficulty: 'Medium' }
        },
        {
            id: 'presets',
            title: 'Presets Creator',
            description: 'Create compact mod selection presets (developer tool).',
            icon: '⚙️',
            gradient: 'from-pink-500 to-rose-600',
            category: 'Developer',
            debugOnly: true,
            stats: { time: '15 min', difficulty: 'Advanced' }
        }
    ];
    
    function handleFeatureClick(featureId) {
        dispatch('featureSelect', { feature: featureId });
    }
    
    // Filter features based on debug mode
    $: visibleFeatures = features.filter(feature => !feature.debugOnly || $debug);
</script>

<div class="feature-grid-section">
    <div class="section-header">
        <h2 class="section-title">Powerful Tools</h2>
        <p class="section-subtitle">Everything you need to manage your Minecraft experience</p>
    </div>
    
    <div class="feature-grid">
        {#each visibleFeatures as feature, index}
            <div 
                class="feature-card"
                class:debug-only={feature.debugOnly}
                style="--animation-delay: {index * 0.1}s"
                on:click={() => handleFeatureClick(feature.id)}
                role="button"
                tabindex="0"
                on:keydown={(e) => e.key === 'Enter' && handleFeatureClick(feature.id)}
            >
                <!-- Card background decoration -->
                <div class="card-background">
                    <div class="gradient-overlay {feature.gradient}"></div>
                    <div class="pattern-overlay"></div>
                </div>
                
                <!-- Card content -->
                <div class="card-content">
                    <div class="card-header">
                        <div class="feature-icon">{feature.icon}</div>
                        <div class="category-badge">{feature.category}</div>
                    </div>
                    
                    <h3 class="feature-title">{feature.title}</h3>
                    <p class="feature-description">{feature.description}</p>
                    
                    <div class="feature-stats">
                        <div class="stat">
                            <span class="stat-icon">⏱️</span>
                            <span class="stat-text">{feature.stats.time}</span>
                        </div>
                        <div class="stat">
                            <span class="stat-icon">📊</span>
                            <span class="stat-text">{feature.stats.difficulty}</span>
                        </div>
                    </div>
                </div>
                
                <!-- Hover effects -->
                <div class="card-hover-effect"></div>
                <div class="card-glow"></div>
            </div>
        {/each}
    </div>
    
    {#if !$debug}
        <div class="debug-notice">
            <span class="notice-icon">🔧</span>
            <span class="notice-text">Enable debug mode to access developer tools</span>
        </div>
    {/if}
</div>

<style>
    .feature-grid-section {
        margin-bottom: 1.5rem;
        animation: sectionFadeIn 0.8s ease-out;
    }
    
    @keyframes sectionFadeIn {
        from { opacity: 0; transform: translateY(20px); }
        to { opacity: 1; transform: translateY(0); }
    }
    
    .section-header {
        text-align: center;
        margin-bottom: 1.25rem;
    }
    
    .section-title {
        font-size: 1.8rem;
        font-weight: 800;
        margin: 0 0 0.25rem 0;
        background: linear-gradient(135deg, #8b5cf6, #3b82f6);
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
    
    .feature-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
        gap: 1rem;
        margin-bottom: 1rem;
    }
    
    .feature-card {
        position: relative;
        background: rgba(255, 255, 255, 0.8);
        border: 1px solid rgba(139, 92, 246, 0.1);
        border-radius: 12px;
        padding: 1rem;
        cursor: pointer;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        backdrop-filter: blur(10px);
        overflow: hidden;
        opacity: 0;
        transform: translateY(30px);
        animation: cardSlideIn 0.6s ease-out forwards;
        animation-delay: var(--animation-delay);
    }
    
    @keyframes cardSlideIn {
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }
    
    .feature-card:hover {
        transform: translateY(-8px) scale(1.02);
        border-color: rgba(139, 92, 246, 0.3);
        box-shadow: 0 20px 40px rgba(139, 92, 246, 0.15);
    }
    
    .feature-card:active {
        transform: translateY(-4px) scale(1.01);
    }
    
    .feature-card.debug-only {
        border-color: rgba(251, 191, 36, 0.2);
    }
    
    .feature-card.debug-only:hover {
        border-color: rgba(251, 191, 36, 0.4);
        box-shadow: 0 20px 40px rgba(251, 191, 36, 0.15);
    }
    
    .card-background {
        position: absolute;
        inset: 0;
        pointer-events: none;
    }
    
    .gradient-overlay {
        position: absolute;
        inset: 0;
        opacity: 0.05;
        border-radius: 20px;
        transition: opacity 0.3s ease;
    }
    
    .feature-card:hover .gradient-overlay {
        opacity: 0.1;
    }
    
    .from-purple-500.to-indigo-600 {
        background: linear-gradient(135deg, #8b5cf6, #4f46e5);
    }
    
    .from-blue-500.to-cyan-600 {
        background: linear-gradient(135deg, #3b82f6, #0891b2);
    }
    
    .from-green-500.to-teal-600 {
        background: linear-gradient(135deg, #10b981, #0d9488);
    }
    
    .from-orange-500.to-red-600 {
        background: linear-gradient(135deg, #f97316, #dc2626);
    }
    
    .from-pink-500.to-rose-600 {
        background: linear-gradient(135deg, #ec4899, #e11d48);
    }
    
    .pattern-overlay {
        position: absolute;
        inset: 0;
        background-image: 
            radial-gradient(circle at 20% 20%, rgba(139, 92, 246, 0.1) 0%, transparent 50%),
            radial-gradient(circle at 80% 80%, rgba(59, 130, 246, 0.1) 0%, transparent 50%);
        border-radius: 20px;
        opacity: 0;
        transition: opacity 0.3s ease;
    }
    
    .feature-card:hover .pattern-overlay {
        opacity: 1;
    }
    
    .card-content {
        position: relative;
        z-index: 2;
    }
    
    .card-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin-bottom: 1rem;
    }
    
    .feature-icon {
        font-size: 2.5rem;
        filter: drop-shadow(0 4px 8px rgba(0, 0, 0, 0.1));
        animation: iconBob 3s ease-in-out infinite;
    }
    
    @keyframes iconBob {
        0%, 100% { transform: translateY(0); }
        50% { transform: translateY(-3px); }
    }
    
    .category-badge {
        background: rgba(139, 92, 246, 0.1);
        color: #8b5cf6;
        padding: 0.25rem 0.75rem;
        border-radius: 50px;
        font-size: 0.75rem;
        font-weight: 600;
        border: 1px solid rgba(139, 92, 246, 0.2);
    }
    
    .debug-only .category-badge {
        background: rgba(251, 191, 36, 0.1);
        color: #f59e0b;
        border-color: rgba(251, 191, 36, 0.2);
    }
    
    .feature-title {
        font-size: 1.25rem;
        font-weight: 700;
        margin: 0 0 0.75rem 0;
        color: var(--text, #374151);
    }
    
    .feature-description {
        font-size: 0.9rem;
        line-height: 1.5;
        color: var(--text-secondary, #6b7280);
        margin: 0 0 1.25rem 0;
    }
    
    .feature-stats {
        display: flex;
        gap: 1rem;
    }
    
    .stat {
        display: flex;
        align-items: center;
        gap: 0.375rem;
        font-size: 0.8rem;
        color: var(--text-secondary, #6b7280);
    }
    
    .stat-icon {
        font-size: 0.875rem;
    }
    
    .stat-text {
        font-weight: 500;
    }
    
    .card-hover-effect {
        position: absolute;
        inset: 0;
        background: linear-gradient(135deg, 
            rgba(255, 255, 255, 0.1) 0%, 
            transparent 50%
        );
        border-radius: 20px;
        opacity: 0;
        transition: opacity 0.3s ease;
        pointer-events: none;
    }
    
    .feature-card:hover .card-hover-effect {
        opacity: 1;
    }
    
    .card-glow {
        position: absolute;
        inset: -2px;
        background: linear-gradient(135deg, #8b5cf6, #3b82f6);
        border-radius: 22px;
        opacity: 0;
        z-index: -1;
        transition: opacity 0.3s ease;
        filter: blur(10px);
    }
    
    .feature-card:hover .card-glow {
        opacity: 0.3;
    }
    
    .debug-notice {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        background: rgba(251, 191, 36, 0.1);
        border: 1px solid rgba(251, 191, 36, 0.2);
        border-radius: 12px;
        padding: 1rem;
        font-size: 0.9rem;
        color: #f59e0b;
        font-weight: 500;
    }
    
    .notice-icon {
        font-size: 1.125rem;
    }
    
    /* Dark theme */
    @media (prefers-color-scheme: dark) {
        .feature-card {
            background: rgba(15, 23, 42, 0.8);
            border-color: rgba(139, 92, 246, 0.15);
        }
        
        .feature-card:hover {
            border-color: rgba(139, 92, 246, 0.4);
        }
        
        .feature-title {
            color: var(--text, #f9fafb);
        }
        
        .feature-description {
            color: var(--text-secondary, #9ca3af);
        }
        
        .stat {
            color: var(--text-secondary, #9ca3af);
        }
        
        .section-subtitle {
            color: var(--text-secondary, #9ca3af);
        }
    }
    
    /* Responsive design */
    @media (max-width: 768px) {
        .feature-grid {
            grid-template-columns: 1fr;
            gap: 1rem;
        }
        
        .section-title {
            font-size: 2rem;
        }
        
        .section-subtitle {
            font-size: 1rem;
        }
        
        .feature-card {
            padding: 1.25rem;
        }
        
        .feature-icon {
            font-size: 2rem;
        }
    }
    
    @media (max-width: 640px) {
        .section-header {
            margin-bottom: 2rem;
        }
        
        .section-title {
            font-size: 1.75rem;
        }
        
        .feature-card {
            padding: 1rem;
        }
        
        .card-header {
            flex-direction: column;
            align-items: flex-start;
            gap: 0.75rem;
        }
        
        .category-badge {
            align-self: flex-end;
        }
    }
</style>
