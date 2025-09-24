<script>
    export let release = null;
    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();

    let selectedIndex = 0;
    
    // Auto-select the best asset when release changes
    $: if (release?.best && release?.assets) {
        const bestIndex = release.assets.findIndex(a => a.id === release.best.id || a.name === release.best.name);
        if (bestIndex >= 0) {
            selectedIndex = bestIndex;
            // Dispatch the auto-selection
            dispatch('assetSelect', { index: bestIndex });
        }
    }

    function handleSelect(e) {
        dispatch('assetSelect', { index: Number(e.target.value) });
    }
</script>

<div class="asset-selector">
    <label for="asset" class="selector-label">Select Installation Asset</label>
    <div class="select-container">
        <select 
            id="asset" 
            class="asset-select" 
            on:change={handleSelect} 
            disabled={!release} 
            aria-label="Select release asset"
        >
            {#if release}
                {#each release.assets as asset, idx}
                    <option value={idx} selected={idx === selectedIndex}>
                        {asset.name} ({Math.round((asset.size || 0) / 1048576)} MB)
                    </option>
                {/each}
            {:else}
                <option>Loading assets...</option>
            {/if}
        </select>
        <div class="dropdown-icon" aria-hidden="true">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M6 8l4 4 4-4" />
            </svg>
        </div>
    </div>
</div>

<style>
    .asset-selector {
        width: 100%;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }
    
    .selector-label {
        font-size: 0.875rem;
        font-weight: 600;
        color: var(--text, #374151);
        margin-bottom: 0.25rem;
    }
    
    .select-container {
        position: relative;
        width: 100%;
        display: flex;
        align-items: center;
    }
    
    .asset-select {
        width: 100%;
        height: 3rem;
        padding: 0.75rem 3rem 0.75rem 0.75rem;
        border: 2px solid rgba(203, 213, 225, 0.5);
        border-radius: 8px;
        background: linear-gradient(145deg, 
            rgba(255, 255, 255, 0.9) 0%, 
            rgba(255, 255, 255, 0.6) 100%
        );
        backdrop-filter: blur(10px);
        color: var(--text, #374151);
        font-size: 0.9rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.3s ease;
        appearance: none;
        -webkit-appearance: none;
        -moz-appearance: none;
        line-height: 1.5;
    }
    
    .dropdown-icon {
        position: absolute;
        right: 0.75rem;
        top: 50%;
        transform: translateY(-50%);
        width: 1.5rem;
        height: 1.5rem;
        pointer-events: none;
        z-index: 10;
        display: flex;
        align-items: center;
        justify-content: center;
        color: #6b7280;
    }
    
    .dropdown-icon svg {
        width: 1.25rem;
        height: 1.25rem;
    }
    
    .asset-select:hover:not(:disabled) {
        border-color: rgba(59, 130, 246, 0.4);
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    }
    
    .asset-select:focus {
        outline: none;
        border-color: rgba(59, 130, 246, 0.6);
        box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
    }
    
    .asset-select:disabled {
        opacity: 0.6;
        cursor: not-allowed;
        background: linear-gradient(145deg, 
            rgba(156, 163, 175, 0.3) 0%, 
            rgba(209, 213, 219, 0.2) 100%
        );
    }
    
    .asset-select option {
        padding: 0.5rem;
        background: white;
        color: #374151;
    }
    
    /* Dark theme */
    @media (prefers-color-scheme: dark) {
        .selector-label {
            color: var(--text, #f9fafb);
        }
        
        .asset-select {
            background: linear-gradient(145deg, 
                rgba(31, 41, 55, 0.9) 0%, 
                rgba(17, 24, 39, 0.6) 100%
            );
            border-color: rgba(75, 85, 99, 0.5);
            color: var(--text, #f9fafb);
        }
        
        .dropdown-icon {
            color: #9ca3af;
        }
        
        .asset-select:hover:not(:disabled) {
            border-color: rgba(59, 130, 246, 0.6);
        }
        
        .asset-select:disabled {
            background: linear-gradient(145deg, 
                rgba(55, 65, 81, 0.6) 0%, 
                rgba(75, 85, 99, 0.4) 100%
            );
        }
        
        .asset-select option {
            background: #1f2937;
            color: #f9fafb;
        }
    }
    
    /* Responsive design */
    @media (max-width: 640px) {
        .asset-select {
            height: 2.75rem;
            padding: 0.625rem 2.75rem 0.625rem 0.625rem;
            font-size: 0.875rem;
        }
        
        .dropdown-icon {
            right: 0.625rem;
            width: 1.25rem;
            height: 1.25rem;
        }
        
        .dropdown-icon svg {
            width: 1rem;
            height: 1rem;
        }
    }
</style>
