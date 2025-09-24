<script>
    export let title = "";
    export let subtitle = "";
    export let icon = "⚡";
    export let variant = "default"; // default, primary, success, warning, error
    export let glowing = false;
    export let className = "";
</script>

<div class="install-card {variant} {className}" class:glowing>
    <div class="card-header">
        <div class="icon-container">
            <span class="icon">{icon}</span>
        </div>
        <div class="header-text">
            <h3 class="card-title">{title}</h3>
            {#if subtitle}
                <p class="card-subtitle">{subtitle}</p>
            {/if}
        </div>
    </div>
    
    <div class="card-content">
        <slot />
    </div>
    
    <!-- Decorative elements -->
    <div class="card-decoration">
        <div class="decoration-line"></div>
        <div class="decoration-dots">
            <span class="dot"></span>
            <span class="dot"></span>
            <span class="dot"></span>
        </div>
    </div>
</div>

<style>
    .install-card {
        width: 100%;
        background: linear-gradient(145deg, 
            rgba(255, 255, 255, 0.9) 0%, 
            rgba(255, 255, 255, 0.7) 100%
        );
        border: 1px solid rgba(255, 255, 255, 0.2);
        border-radius: 20px;
        padding: 1.5rem;
        position: relative;
        overflow: hidden;
        backdrop-filter: blur(20px);
        box-sizing: border-box;
        box-shadow: 
            0 8px 32px rgba(0, 0, 0, 0.1),
            inset 0 1px 0 rgba(255, 255, 255, 0.2);
        transition: all 0.3s ease;
    }
    
    .install-card:hover {
        transform: translateY(-2px);
        box-shadow: 
            0 12px 40px rgba(0, 0, 0, 0.15),
            inset 0 1px 0 rgba(255, 255, 255, 0.3);
    }
    
    .install-card.glowing {
        animation: cardGlow 3s ease-in-out infinite;
    }
    
    @keyframes cardGlow {
        0%, 100% { 
            box-shadow: 
                0 8px 32px rgba(0, 0, 0, 0.1),
                inset 0 1px 0 rgba(255, 255, 255, 0.2);
        }
        50% { 
            box-shadow: 
                0 8px 32px rgba(59, 130, 246, 0.2),
                inset 0 1px 0 rgba(255, 255, 255, 0.3),
                0 0 20px rgba(59, 130, 246, 0.1);
        }
    }
    
    /* Variant styles */
    .install-card.primary {
        background: linear-gradient(145deg, 
            rgba(59, 130, 246, 0.1) 0%, 
            rgba(147, 197, 253, 0.05) 100%
        );
        border-color: rgba(59, 130, 246, 0.2);
    }
    
    .install-card.success {
        background: linear-gradient(145deg, 
            rgba(34, 197, 94, 0.1) 0%, 
            rgba(134, 239, 172, 0.05) 100%
        );
        border-color: rgba(34, 197, 94, 0.2);
    }
    
    .install-card.warning {
        background: linear-gradient(145deg, 
            rgba(251, 146, 60, 0.1) 0%, 
            rgba(253, 186, 116, 0.05) 100%
        );
        border-color: rgba(251, 146, 60, 0.2);
    }
    
    .install-card.error {
        background: linear-gradient(145deg, 
            rgba(239, 68, 68, 0.1) 0%, 
            rgba(252, 165, 165, 0.05) 100%
        );
        border-color: rgba(239, 68, 68, 0.2);
    }
    
    .card-header {
        display: flex;
        align-items: center;
        gap: 1rem;
        margin-bottom: 1rem;
    }
    
    .icon-container {
        width: 48px;
        height: 48px;
        border-radius: 12px;
        background: linear-gradient(135deg, 
            rgba(59, 130, 246, 0.8) 0%, 
            rgba(139, 92, 246, 0.6) 100%
        );
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
        transition: all 0.3s ease;
    }
    
    .install-card:hover .icon-container {
        transform: scale(1.05);
        box-shadow: 0 6px 16px rgba(59, 130, 246, 0.4);
    }
    
    .icon {
        font-size: 1.5rem;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
    }
    
    .header-text {
        flex: 1;
    }
    
    .card-title {
        font-size: 1.25rem;
        font-weight: 700;
        color: var(--text, #1f2937);
        margin: 0 0 0.25rem 0;
        background: linear-gradient(135deg, #1f2937, #4b5563);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
    }
    
    .card-subtitle {
        font-size: 0.875rem;
        color: var(--text-secondary, #6b7280);
        margin: 0;
        opacity: 0.8;
    }
    
    .card-content {
        position: relative;
        z-index: 2;
        width: 100%;
        max-width: 100%;
        box-sizing: border-box;
        overflow: hidden;
    }
    
    .card-decoration {
        position: absolute;
        top: 1rem;
        right: 1rem;
        display: flex;
        flex-direction: column;
        align-items: flex-end;
        gap: 0.5rem;
        opacity: 0.3;
        z-index: 1;
    }
    
    .decoration-line {
        width: 40px;
        height: 2px;
        background: linear-gradient(90deg, transparent, rgba(59, 130, 246, 0.5));
        border-radius: 1px;
    }
    
    .decoration-dots {
        display: flex;
        gap: 0.25rem;
    }
    
    .dot {
        width: 4px;
        height: 4px;
        border-radius: 50%;
        background: rgba(59, 130, 246, 0.4);
        animation: dotPulse 2s ease-in-out infinite;
    }
    
    .dot:nth-child(2) { animation-delay: 0.3s; }
    .dot:nth-child(3) { animation-delay: 0.6s; }
    
    @keyframes dotPulse {
        0%, 100% { opacity: 0.4; transform: scale(1); }
        50% { opacity: 1; transform: scale(1.2); }
    }
    
    /* Dark theme */
    @media (prefers-color-scheme: dark) {
        .install-card {
            background: linear-gradient(145deg, 
                rgba(31, 41, 55, 0.9) 0%, 
                rgba(17, 24, 39, 0.7) 100%
            );
            border-color: rgba(75, 85, 99, 0.3);
        }
        
        .card-title {
            color: var(--text, #f9fafb);
            background: linear-gradient(135deg, #f9fafb, #d1d5db);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }
        
        .card-subtitle {
            color: var(--text-secondary, #9ca3af);
        }
    }
    
    /* Responsive design */
    @media (max-width: 640px) {
        .install-card {
            padding: 1rem;
        }
        
        .icon-container {
            width: 40px;
            height: 40px;
        }
        
        .icon {
            font-size: 1.25rem;
        }
        
        .card-title {
            font-size: 1.125rem;
        }
    }
</style>
