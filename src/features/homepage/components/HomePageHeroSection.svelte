<!-- src/features/homepage/components/HomePageHeroSection.svelte -->
<script>
    import { onMount } from 'svelte';
    
    let mounted = false;
    
    onMount(() => {
        mounted = true;
    });
</script>

<div class="hero-section" class:mounted>
    <!-- Main hero content -->
    <div class="hero-content">
        <div class="hero-badge">
            <span class="badge-icon">🚀</span>
            <span class="badge-text">Advanced Minecraft Helper</span>
        </div>
        
        <h1 class="hero-title">
            <span class="title-primary">NAHA</span>
            <span class="title-secondary">Minecraft Helper</span>
        </h1>
        
        <p class="hero-description">
            Transform your Minecraft journey with powerful launcher management and mod updates.
        </p>
        
        <div class="hero-stats">
            <div class="stat-item">
                <span class="stat-number">2</span>
                <span class="stat-label">Launchers</span>
            </div>
            <div class="stat-divider"></div>
            <div class="stat-item">
                <span class="stat-number">2</span>
                <span class="stat-label">Loaders</span>
            </div>
            <div class="stat-divider"></div>
            <div class="stat-item">
                <span class="stat-number">∞</span>
                <span class="stat-label">Tools</span>
            </div>
        </div>
    </div>
</div>

<style>
    .hero-section {
        position: relative;
        min-height: 240px;
        display: flex;
        align-items: center;
        justify-content: center;
        margin-bottom: 1.5rem;
        overflow: hidden;
        border-radius: 16px;
        background: linear-gradient(135deg, 
            rgba(139, 92, 246, 0.08) 0%, 
            rgba(59, 130, 246, 0.06) 100%
        );
        border: 1px solid rgba(139, 92, 246, 0.15);
        opacity: 0;
        transform: translateY(10px);
        transition: all 0.4s ease-out;
        flex-shrink: 0;
        box-sizing: border-box;
    }
    
    .hero-section.mounted {
        opacity: 1;
        transform: translateY(0);
    }
    
    
    .hero-content {
        position: relative;
        z-index: 2;
        text-align: center;
        max-width: 100%;
        padding: 2.5rem 1rem;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.75rem;
        box-sizing: border-box;
    }
    
    .hero-badge {
        display: inline-flex;
        align-items: center;
        gap: 0.375rem;
        background: rgba(139, 92, 246, 0.1);
        border: 1px solid rgba(139, 92, 246, 0.15);
        border-radius: 20px;
        padding: 0.375rem 0.75rem;
        font-size: 0.8rem;
        font-weight: 500;
        color: #8b5cf6;
    }
    
    .badge-icon {
        font-size: 0.875rem;
    }
    
    .hero-title {
        font-size: clamp(1.4rem, 4vw, 2rem);
        font-weight: 700;
        line-height: 1.1;
        margin: 0;
        text-align: center;
        display: flex;
        flex-direction: row;
        gap: 0.75rem;
        align-items: center;
        justify-content: center;
        flex-wrap: wrap;
    }
    
    .title-primary {
        background: linear-gradient(135deg, #8b5cf6, #a78bfa);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
    }
    
    .title-secondary {
        background: linear-gradient(135deg, #3b82f6, #60a5fa);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
        font-size: 0.85em;
        opacity: 0.9;
    }
    
    .hero-description {
        font-size: 0.95rem;
        line-height: 1.4;
        color: var(--text-secondary, #6b7280);
        margin: 0;
        max-width: 500px;
        opacity: 0.9;
        font-weight: 400;
        text-align: center;
    }
    
    .hero-stats {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 1.25rem;
        margin: 0;
    }
    
    .stat-item {
        text-align: center;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.25rem;
    }
    
    .stat-number {
        font-size: 1.4rem;
        font-weight: 600;
        background: linear-gradient(135deg, #8b5cf6, #3b82f6);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
        line-height: 1;
    }
    
    .stat-label {
        font-size: 0.75rem;
        color: var(--text-secondary, #6b7280);
        font-weight: 500;
        opacity: 0.8;
    }
    
    .stat-divider {
        width: 1px;
        height: 28px;
        background: linear-gradient(to bottom, 
            transparent 0%, 
            rgba(139, 92, 246, 0.3) 50%, 
            transparent 100%
        );
    }
    
    /* Dark theme */
    @media (prefers-color-scheme: dark) {
        .hero-section {
            background: linear-gradient(135deg, 
                rgba(139, 92, 246, 0.12) 0%, 
                rgba(59, 130, 246, 0.1) 50%,
                rgba(16, 185, 129, 0.08) 100%
            );
            border-color: rgba(139, 92, 246, 0.25);
        }
        
        .hero-description {
            color: var(--text-secondary, #9ca3af);
        }
        
        .stat-label {
            color: var(--text-secondary, #9ca3af);
        }
        
        .hero-badge {
            background: rgba(139, 92, 246, 0.15);
            border-color: rgba(139, 92, 246, 0.25);
        }
    }
    
    /* Responsive design */
    @media (max-width: 768px) {
        .hero-section {
            min-height: 200px;
        }
        
        .hero-content {
            padding: 2rem 0.75rem;
            gap: 0.5rem;
        }
        
        .hero-badge {
            font-size: 0.7rem;
            padding: 0.25rem 0.5rem;
        }
        
        .hero-title {
            font-size: clamp(1.1rem, 3.5vw, 1.5rem);
        }
        
        .hero-description {
            font-size: 0.8rem;
        }
        
        .hero-stats {
            gap: 1rem;
        }
        
        .stat-number {
            font-size: 1.1rem;
        }
        
        .stat-label {
            font-size: 0.65rem;
        }
        
        .stat-divider {
            height: 22px;
        }
    }
    
    @media (max-width: 640px) {
        .hero-section {
            min-height: 180px;
            border-radius: 12px;
        }
        
        .hero-content {
            padding: 1.75rem 0.625rem;
            gap: 0.4rem;
        }
        
        .hero-badge {
            font-size: 0.65rem;
            padding: 0.2rem 0.4rem;
        }
        
        .badge-icon {
            font-size: 0.7rem;
        }
        
        .hero-title {
            font-size: clamp(1rem, 3vw, 1.3rem);
            gap: 0.5rem;
        }
        
        .hero-description {
            font-size: 0.75rem;
            max-width: 350px;
        }
        
        .hero-stats {
            gap: 0.8rem;
        }
        
        .stat-number {
            font-size: 1rem;
        }
        
        .stat-label {
            font-size: 0.6rem;
        }
        
        .stat-divider {
            height: 20px;
        }
    }
</style>
