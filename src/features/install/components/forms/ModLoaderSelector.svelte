<script>
    import { createEventDispatcher } from 'svelte';
    export let loader = 'fabric';
    const dispatch = createEventDispatcher();

    function handleLoaderChange(e) {
        const newLoader = e.target.value;
        dispatch('loaderChange', { loader: newLoader });
    }
</script>

<div class="mod-loader-selector">
    <h3 class="selector-title">Choose Mod Loader</h3>
    <div class="loader-options">
        <label class="loader-option" class:selected={loader === 'fabric'}>
            <input type="radio" name="loader" value="fabric" checked={loader === 'fabric'} on:change={handleLoaderChange} />
            <div class="option-content">
                <div class="option-icon">📜</div>
                <div class="option-info">
                    <div class="option-title">Fabric</div>
                    <div class="option-description">Lightweight, fast mod loader</div>
                </div>
            </div>
        </label>
        
        <label class="loader-option" class:selected={loader === 'neoforge'}>
            <input type="radio" name="loader" value="neoforge" checked={loader === 'neoforge'} on:change={handleLoaderChange} />
            <div class="option-content">
                <div class="option-icon">🦊</div>
                <div class="option-info">
                    <div class="option-title">NeoForge</div>
                    <div class="option-description">Powerful modding framework</div>
                </div>
            </div>
        </label>
        
        <div class="loader-option disabled">
            <div class="option-content">
                <div class="option-icon">🐺</div>
                <div class="option-info">
                    <div class="option-title">BTW</div>
                    <div class="option-description">Better Than Wolves (coming soon)</div>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    .mod-loader-selector {
        width: 100%;
    }
    
    .selector-title {
        font-size: 1.125rem;
        font-weight: 600;
        color: var(--text, #374151);
        margin: 0 0 1rem 0;
        text-align: center;
    }
    
    .loader-options {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
        gap: 0.75rem;
    }
    
    .loader-option {
        position: relative;
        cursor: pointer;
        border: 2px solid rgba(203, 213, 225, 0.5);
        border-radius: 12px;
        padding: 1rem;
        background: linear-gradient(145deg, 
            rgba(255, 255, 255, 0.8) 0%, 
            rgba(255, 255, 255, 0.4) 100%
        );
        backdrop-filter: blur(10px);
        transition: all 0.3s ease;
        overflow: hidden;
    }
    
    .loader-option:not(.disabled):hover {
        border-color: rgba(139, 92, 246, 0.4);
        transform: translateY(-2px);
        box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
    }
    
    .loader-option.selected {
        border-color: rgba(139, 92, 246, 0.6);
        background: linear-gradient(145deg, 
            rgba(139, 92, 246, 0.1) 0%, 
            rgba(196, 181, 253, 0.05) 100%
        );
        box-shadow: 0 8px 24px rgba(139, 92, 246, 0.15);
    }
    
    .loader-option.selected::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 3px;
        background: linear-gradient(90deg, #8b5cf6, #7c3aed);
        border-radius: 12px 12px 0 0;
    }
    
    .loader-option.disabled {
        opacity: 0.6;
        cursor: not-allowed;
        background: linear-gradient(145deg, 
            rgba(156, 163, 175, 0.3) 0%, 
            rgba(209, 213, 219, 0.2) 100%
        );
    }
    
    .loader-option input[type="radio"] {
        position: absolute;
        opacity: 0;
        width: 0;
        height: 0;
    }
    
    .option-content {
        display: flex;
        align-items: center;
        gap: 1rem;
        position: relative;
        z-index: 2;
    }
    
    .option-icon {
        font-size: 2rem;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
        transition: transform 0.3s ease;
    }
    
    .loader-option:not(.disabled):hover .option-icon {
        transform: scale(1.1);
    }
    
    .option-info {
        flex: 1;
    }
    
    .option-title {
        font-size: 1rem;
        font-weight: 600;
        color: var(--text, #1f2937);
        margin-bottom: 0.25rem;
        transition: color 0.3s ease;
    }
    
    .loader-option.selected .option-title {
        color: #7c3aed;
    }
    
    .loader-option.disabled .option-title {
        color: #9ca3af;
    }
    
    .option-description {
        font-size: 0.875rem;
        color: var(--text-secondary, #6b7280);
        line-height: 1.4;
        transition: color 0.3s ease;
    }
    
    .loader-option.selected .option-description {
        color: #6d28d9;
    }
    
    .loader-option.disabled .option-description {
        color: #9ca3af;
    }
    
    /* Dark theme */
    @media (prefers-color-scheme: dark) {
        .selector-title {
            color: var(--text, #f9fafb);
        }
        
        .loader-option {
            background: linear-gradient(145deg, 
                rgba(31, 41, 55, 0.8) 0%, 
                rgba(17, 24, 39, 0.4) 100%
            );
            border-color: rgba(75, 85, 99, 0.5);
        }
        
        .loader-option:not(.disabled):hover {
            border-color: rgba(139, 92, 246, 0.6);
        }
        
        .loader-option.selected {
            background: linear-gradient(145deg, 
                rgba(139, 92, 246, 0.15) 0%, 
                rgba(196, 181, 253, 0.08) 100%
            );
        }
        
        .loader-option.disabled {
            background: linear-gradient(145deg, 
                rgba(55, 65, 81, 0.6) 0%, 
                rgba(75, 85, 99, 0.4) 100%
            );
        }
        
        .option-title {
            color: var(--text, #f9fafb);
        }
        
        .loader-option.selected .option-title {
            color: #c4b5fd;
        }
        
        .option-description {
            color: var(--text-secondary, #9ca3af);
        }
        
        .loader-option.selected .option-description {
            color: #ddd6fe;
        }
    }
    
    /* Responsive design */
    @media (max-width: 640px) {
        .loader-options {
            grid-template-columns: 1fr;
        }
        
        .loader-option {
            padding: 0.75rem;
        }
        
        .option-content {
            gap: 0.75rem;
        }
        
        .option-icon {
            font-size: 1.75rem;
        }
        
        .option-title {
            font-size: 0.9rem;
        }
        
        .option-description {
            font-size: 0.8rem;
        }
    }
</style>
