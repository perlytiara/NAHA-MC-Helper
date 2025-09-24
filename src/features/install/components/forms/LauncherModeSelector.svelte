<script>
    import { createEventDispatcher } from 'svelte';
    export let mode = 'cracked';
    const dispatch = createEventDispatcher();

    function handleModeChange(e) {
        const newMode = e.target.value;
        dispatch('modeChange', { mode: newMode });
    }
</script>

<div class="launcher-mode-selector">
    <h3 class="selector-title">Choose Your Launcher</h3>
    <p class="selector-subtitle">Select the launcher that best fits your needs</p>
    <div class="mode-options">
        <label class="mode-option" class:selected={mode === 'cracked'}>
            <input type="radio" name="mode" value="cracked" checked={mode === 'cracked'} on:change={handleModeChange} />
            <div class="option-content">
                <div class="option-header">
                    <div class="option-icon">🎮</div>
                    <div class="recommended-badge">Popular</div>
                </div>
                <div class="option-info">
                    <div class="option-title">Prism Launcher</div>
                    <div class="option-description">A user-friendly launcher with mod support</div>
                    <div class="option-features">
                        <span class="feature-tag">Free</span>
                        <span class="feature-tag">Mod Support</span>
                        <span class="feature-tag">Multi-Platform</span>
                    </div>
                </div>
            </div>
            <div class="option-background cracked-bg"></div>
        </label>
        
        <label class="mode-option" class:selected={mode === 'official'}>
            <input type="radio" name="mode" value="official" checked={mode === 'official'} on:change={handleModeChange} />
            <div class="option-content">
                <div class="option-header">
                    <div class="option-icon">⚡</div>
                    <div class="recommended-badge">Official</div>
                </div>
                <div class="option-info">
                    <div class="option-title">XMCL</div>
                    <div class="option-description">A lightweight and fast launcher</div>
                    <div class="option-features">
                        <span class="feature-tag">Fast</span>
                        <span class="feature-tag">Lightweight</span>
                        <span class="feature-tag">Official</span>
                    </div>
                </div>
            </div>
            <div class="option-background official-bg"></div>
        </label>
        
        <label class="mode-option" class:selected={mode === 'astralrinth'}>
            <input type="radio" name="mode" value="astralrinth" checked={mode === 'astralrinth'} on:change={handleModeChange} />
            <div class="option-content">
                <div class="option-header">
                    <div class="option-icon">🌟</div>
                    <div class="recommended-badge">Advanced</div>
                </div>
                <div class="option-info">
                    <div class="option-title">AstralRinth</div>
                    <div class="option-description">A launcher with advanced features</div>
                    <div class="option-features">
                        <span class="feature-tag">Advanced</span>
                        <span class="feature-tag">Ely.by</span>
                        <span class="feature-tag">2FA</span>
                    </div>
                </div>
            </div>
            <div class="option-background astralrinth-bg"></div>
        </label>
    </div>
</div>

<style>
    .launcher-mode-selector {
        width: 100%;
    }
    
    .selector-title {
        font-size: 1.5rem;
        font-weight: 700;
        color: var(--text, #374151);
        margin: 0 0 0.5rem 0;
        text-align: center;
        background: linear-gradient(135deg, #3b82f6, #8b5cf6);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
    }
    
    .selector-subtitle {
        font-size: 1rem;
        color: var(--text-secondary, #6b7280);
        margin: 0 0 1.5rem 0;
        text-align: center;
        opacity: 0.9;
    }
    
    .mode-options {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
        gap: 1rem;
    }
    
    .mode-option {
        position: relative;
        cursor: pointer;
        border: 2px solid rgba(203, 213, 225, 0.5);
        border-radius: 16px;
        padding: 1.5rem;
        background: linear-gradient(145deg, 
            rgba(255, 255, 255, 0.9) 0%, 
            rgba(255, 255, 255, 0.6) 100%
        );
        backdrop-filter: blur(10px);
        transition: all 0.3s ease;
        overflow: hidden;
        min-height: 160px;
    }
    
    .mode-option:hover {
        border-color: rgba(59, 130, 246, 0.4);
        transform: translateY(-2px);
        box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
    }
    
    .mode-option.selected {
        border-color: rgba(59, 130, 246, 0.6);
        background: linear-gradient(145deg, 
            rgba(59, 130, 246, 0.1) 0%, 
            rgba(147, 197, 253, 0.05) 100%
        );
        box-shadow: 0 8px 24px rgba(59, 130, 246, 0.15);
    }
    
    .mode-option.selected::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 3px;
        background: linear-gradient(90deg, #3b82f6, #8b5cf6);
        border-radius: 12px 12px 0 0;
    }
    
    .mode-option input[type="radio"] {
        position: absolute;
        opacity: 0;
        width: 0;
        height: 0;
    }
    
    .option-content {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        position: relative;
        z-index: 2;
        height: 100%;
    }
    
    .option-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
    }
    
    .option-icon {
        font-size: 2.5rem;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
        transition: transform 0.3s ease;
    }
    
    .mode-option:hover .option-icon {
        transform: scale(1.1);
    }
    
    .recommended-badge {
        background: linear-gradient(135deg, #22c55e, #16a34a);
        color: white;
        padding: 0.25rem 0.75rem;
        border-radius: 20px;
        font-size: 0.75rem;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        box-shadow: 0 2px 8px rgba(34, 197, 94, 0.3);
    }
    
    .option-info {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }
    
    .option-title {
        font-size: 1.25rem;
        font-weight: 700;
        color: var(--text, #1f2937);
        margin: 0;
        transition: color 0.3s ease;
    }
    
    .mode-option.selected .option-title {
        color: #1e40af;
    }
    
    .option-description {
        font-size: 0.9rem;
        color: var(--text-secondary, #6b7280);
        line-height: 1.4;
        transition: color 0.3s ease;
        margin: 0;
    }
    
    .mode-option.selected .option-description {
        color: #3730a3;
    }
    
    .option-features {
        display: flex;
        flex-wrap: wrap;
        gap: 0.375rem;
        margin-top: 0.5rem;
    }
    
    .feature-tag {
        background: rgba(59, 130, 246, 0.1);
        color: #3b82f6;
        padding: 0.25rem 0.5rem;
        border-radius: 12px;
        font-size: 0.75rem;
        font-weight: 500;
        border: 1px solid rgba(59, 130, 246, 0.2);
    }
    
    .mode-option.selected .feature-tag {
        background: rgba(59, 130, 246, 0.15);
        color: #1e40af;
        border-color: rgba(59, 130, 246, 0.3);
    }
    
    .option-background {
        position: absolute;
        inset: 0;
        opacity: 0.05;
        transition: opacity 0.3s ease;
        border-radius: 16px;
    }
    
    .mode-option:hover .option-background {
        opacity: 0.1;
    }
    
    .mode-option.selected .option-background {
        opacity: 0.15;
    }
    
    .cracked-bg {
        background: linear-gradient(135deg, #3b82f6, #4f46e5);
    }
    
    .official-bg {
        background: linear-gradient(135deg, #8b5cf6, #db2777);
    }
    
    .astralrinth-bg {
        background: linear-gradient(135deg, #f97316, #dc2626);
    }
    
    /* Dark theme */
    @media (prefers-color-scheme: dark) {
        .selector-title {
            color: var(--text, #f9fafb);
        }
        
        .selector-subtitle {
            color: var(--text-secondary, #9ca3af);
        }
        
        .mode-option {
            background: linear-gradient(145deg, 
                rgba(31, 41, 55, 0.9) 0%, 
                rgba(17, 24, 39, 0.6) 100%
            );
            border-color: rgba(75, 85, 99, 0.5);
        }
        
        .mode-option:hover {
            border-color: rgba(59, 130, 246, 0.6);
        }
        
        .mode-option.selected {
            background: linear-gradient(145deg, 
                rgba(59, 130, 246, 0.15) 0%, 
                rgba(147, 197, 253, 0.08) 100%
            );
        }
        
        .option-title {
            color: var(--text, #f9fafb);
        }
        
        .mode-option.selected .option-title {
            color: #93c5fd;
        }
        
        .option-description {
            color: var(--text-secondary, #9ca3af);
        }
        
        .mode-option.selected .option-description {
            color: #bfdbfe;
        }
        
        .feature-tag {
            background: rgba(59, 130, 246, 0.15);
            color: #93c5fd;
            border-color: rgba(59, 130, 246, 0.25);
        }
        
        .mode-option.selected .feature-tag {
            background: rgba(59, 130, 246, 0.2);
            color: #bfdbfe;
            border-color: rgba(59, 130, 246, 0.35);
        }
    }
    
    /* Responsive design */
    @media (max-width: 768px) {
        .mode-options {
            grid-template-columns: 1fr;
            gap: 1.25rem;
        }
        
        .selector-title {
            font-size: 1.375rem;
        }
        
        .selector-subtitle {
            font-size: 0.9rem;
        }
    }
    
    @media (max-width: 640px) {
        .mode-option {
            padding: 1.25rem;
            min-height: 140px;
        }
        
        .option-content {
            gap: 0.875rem;
        }
        
        .option-header {
            flex-direction: column;
            gap: 0.5rem;
            align-items: flex-start;
        }
        
        .option-icon {
            font-size: 2rem;
        }
        
        .recommended-badge {
            font-size: 0.7rem;
            padding: 0.2rem 0.6rem;
            align-self: flex-end;
        }
        
        .option-title {
            font-size: 1.125rem;
        }
        
        .option-description {
            font-size: 0.85rem;
        }
        
        .option-features {
            gap: 0.25rem;
        }
        
        .feature-tag {
            font-size: 0.7rem;
            padding: 0.2rem 0.4rem;
        }
    }
</style>
