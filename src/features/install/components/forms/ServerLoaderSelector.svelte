<!-- src/features/install/components/forms/ServerLoaderSelector.svelte -->
<script>
    import { createEventDispatcher } from 'svelte';

    export let loader = 'fabric';
    const dispatch = createEventDispatcher();

    function handleChange(e) {
        const newLoader = e.target.value;
        dispatch('loaderChange', { loader: newLoader });
    }
</script>

<div class="server-loader-selector" title="Choose your server loader (🦊=NeoForge, 📜=Fabric)">
    <label class="badge" for="loader-fabric">Server Loader</label>
    <div class="loader-options">
        <label class="loader-option" for="loader-fabric">
            <input id="loader-fabric" type="radio" name="loader" value="fabric" bind:group={loader} on:change={handleChange} />
            <span class="loader-icon">📜</span>
            <span class="loader-name">Fabric</span>
        </label>
        <label class="loader-option" for="loader-neo">
            <input id="loader-neo" type="radio" name="loader" value="neoforge" bind:group={loader} on:change={handleChange} />
            <span class="loader-icon">🦊</span>
            <span class="loader-name">NeoForge</span>
        </label>
    </div>
</div>

<style>
    .server-loader-selector {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        width: 100%;
    }
    
    .badge {
        font-size: 0.875rem;
        font-weight: 600;
        color: var(--text, #374151);
        background: rgba(59, 130, 246, 0.1);
        padding: 0.375rem 0.75rem;
        border-radius: 6px;
        border: 1px solid rgba(59, 130, 246, 0.2);
        display: inline-flex;
        align-items: center;
        justify-content: center;
        max-width: fit-content;
        margin: 0 auto;
    }
    
    .loader-options {
        display: flex;
        gap: 1rem;
        justify-content: center;
        flex-wrap: wrap;
    }
    
    .loader-option {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        cursor: pointer;
        padding: 0.5rem 1rem;
        border: 2px solid rgba(203, 213, 225, 0.5);
        border-radius: 8px;
        background: linear-gradient(145deg, 
            rgba(255, 255, 255, 0.8) 0%, 
            rgba(255, 255, 255, 0.4) 100%
        );
        backdrop-filter: blur(10px);
        transition: all 0.3s ease;
        min-width: 120px;
        justify-content: center;
    }
    
    .loader-option:hover {
        border-color: rgba(59, 130, 246, 0.4);
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    }
    
    .loader-option:has(input:checked) {
        border-color: rgba(59, 130, 246, 0.6);
        background: linear-gradient(145deg, 
            rgba(59, 130, 246, 0.1) 0%, 
            rgba(147, 197, 253, 0.05) 100%
        );
        box-shadow: 0 4px 12px rgba(59, 130, 246, 0.15);
    }
    
    .loader-option input[type="radio"] {
        position: absolute;
        opacity: 0;
        width: 0;
        height: 0;
    }
    
    .loader-icon {
        font-size: 1.25rem;
        filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.1));
    }
    
    .loader-name {
        font-size: 0.9rem;
        font-weight: 500;
        color: var(--text, #374151);
    }
    
    .loader-option:has(input:checked) .loader-name {
        color: #1e40af;
        font-weight: 600;
    }
    
    /* Dark theme */
    @media (prefers-color-scheme: dark) {
        .badge {
            color: var(--text, #f9fafb);
            background: rgba(59, 130, 246, 0.15);
            border-color: rgba(59, 130, 246, 0.3);
        }
        
        .loader-option {
            background: linear-gradient(145deg, 
                rgba(31, 41, 55, 0.8) 0%, 
                rgba(17, 24, 39, 0.4) 100%
            );
            border-color: rgba(75, 85, 99, 0.5);
        }
        
        .loader-option:hover {
            border-color: rgba(59, 130, 246, 0.6);
        }
        
        .loader-option:has(input:checked) {
            background: linear-gradient(145deg, 
                rgba(59, 130, 246, 0.15) 0%, 
                rgba(147, 197, 253, 0.08) 100%
            );
        }
        
        .loader-name {
            color: var(--text, #f9fafb);
        }
        
        .loader-option:has(input:checked) .loader-name {
            color: #93c5fd;
        }
    }
    
    /* Responsive design */
    @media (max-width: 640px) {
        .loader-options {
            flex-direction: column;
            align-items: center;
        }
        
        .loader-option {
            min-width: 140px;
        }
    }
</style>
