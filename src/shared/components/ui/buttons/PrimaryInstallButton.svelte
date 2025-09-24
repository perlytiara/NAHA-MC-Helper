<script>
    import { createEventDispatcher } from 'svelte';
    export let disabled = false;
    export let text = 'Start Installation';
    export let loadingText = 'Installing...';
    export let variant = 'primary'; // primary, secondary, success, warning, error
    export let size = 'medium'; // small, medium, large
    
    const dispatch = createEventDispatcher();

    function handleClick() {
        if (!disabled) dispatch('click');
    }
    
    $: buttonText = disabled ? loadingText : text;
</script>

<button 
    class="primary-install-button {variant} {size}" 
    {disabled} 
    on:click={handleClick}
    aria-label={buttonText}
>
    <div class="button-content">
        {#if disabled}
            <div class="loading-spinner"></div>
        {/if}
        <span class="button-text">{buttonText}</span>
    </div>
</button>

<style>
    .primary-install-button {
        position: relative;
        width: 100%;
        border: none;
        border-radius: 12px;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.3s ease;
        overflow: hidden;
        backdrop-filter: blur(10px);
        box-sizing: border-box;
        
        /* Default primary style */
        background: linear-gradient(135deg, #3b82f6, #8b5cf6);
        color: white;
        box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
    }
    
    .primary-install-button:hover:not(:disabled) {
        transform: translateY(-2px);
        box-shadow: 0 8px 24px rgba(59, 130, 246, 0.4);
    }
    
    .primary-install-button:active:not(:disabled) {
        transform: translateY(0);
    }
    
    .primary-install-button:disabled {
        cursor: not-allowed;
        opacity: 0.7;
        transform: none;
    }
    
    /* Size variants */
    .primary-install-button.small {
        padding: 0.5rem 1rem;
        font-size: 0.875rem;
    }
    
    .primary-install-button.medium {
        padding: 0.75rem 1.5rem;
        font-size: 1rem;
    }
    
    .primary-install-button.large {
        padding: 1rem 2rem;
        font-size: 1.125rem;
    }
    
    /* Color variants */
    .primary-install-button.secondary {
        background: linear-gradient(135deg, #6b7280, #9ca3af);
        box-shadow: 0 4px 12px rgba(107, 114, 128, 0.3);
    }
    
    .primary-install-button.secondary:hover:not(:disabled) {
        box-shadow: 0 8px 24px rgba(107, 114, 128, 0.4);
    }
    
    .primary-install-button.success {
        background: linear-gradient(135deg, #10b981, #34d399);
        box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
    }
    
    .primary-install-button.success:hover:not(:disabled) {
        box-shadow: 0 8px 24px rgba(16, 185, 129, 0.4);
    }
    
    .primary-install-button.warning {
        background: linear-gradient(135deg, #f59e0b, #fbbf24);
        box-shadow: 0 4px 12px rgba(245, 158, 11, 0.3);
    }
    
    .primary-install-button.warning:hover:not(:disabled) {
        box-shadow: 0 8px 24px rgba(245, 158, 11, 0.4);
    }
    
    .primary-install-button.error {
        background: linear-gradient(135deg, #dc2626, #ef4444);
        box-shadow: 0 4px 12px rgba(220, 38, 38, 0.3);
    }
    
    .primary-install-button.error:hover:not(:disabled) {
        box-shadow: 0 8px 24px rgba(220, 38, 38, 0.4);
    }
    
    .button-content {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
    }
    
    .button-text {
        font-weight: inherit;
        text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
    }
    
    .loading-spinner {
        width: 1rem;
        height: 1rem;
        border: 2px solid rgba(255, 255, 255, 0.3);
        border-top: 2px solid white;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }
    
    @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }
    
    /* Dark theme */
    @media (prefers-color-scheme: dark) {
        .primary-install-button {
            box-shadow: 0 4px 12px rgba(59, 130, 246, 0.4);
        }
        
        .primary-install-button:hover:not(:disabled) {
            box-shadow: 0 8px 24px rgba(59, 130, 246, 0.5);
        }
    }
    
    /* Responsive design */
    @media (max-width: 640px) {
        .primary-install-button.large {
            padding: 0.875rem 1.5rem;
            font-size: 1rem;
        }
        
        .primary-install-button.medium {
            padding: 0.625rem 1.25rem;
            font-size: 0.9rem;
        }
    }
</style>
