<!-- src/features/onboarding/components/OnboardingNavigationButtons.svelte -->
<script>
    import { createEventDispatcher } from 'svelte';
    
    const dispatch = createEventDispatcher();
    
    // Props
    export let showBack = true;
    export let showNext = false;
    export let showCreate = false;
    export let showSkip = false;
    export let nextDisabled = false;
    export let createDisabled = false;
    export let skipDisabled = false;
    export let isCreating = false;
    export let nextText = 'Next';
    export let createText = '🚀 Install Modpack';
    export let skipText = 'Skip Installation';
    export let layout = 'default'; // 'default' or 'instance-creation'
    
    function handleBack() {
        dispatch('back');
    }
    
    function handleNext() {
        dispatch('next');
    }
    
    function handleCreate() {
        dispatch('create');
    }
    
    function handleSkip() {
        dispatch('skip');
    }
</script>

<div class="navigation-section" class:instance-creation-mode={layout === 'instance-creation'}>
    {#if showBack}
        <button class="back-button" on:click={handleBack} disabled={isCreating}>
            ← Back
        </button>
    {/if}
    
    {#if showNext}
        <button 
            class="next-button" 
            class:disabled={nextDisabled}
            disabled={nextDisabled || isCreating}
            on:click={handleNext}
        >
            {nextText}
        </button>
    {/if}
    
    {#if showCreate || showSkip}
        <div class="create-button-container">
            {#if showCreate}
                <button 
                    class="create-button" 
                    disabled={createDisabled || isCreating}
                    on:click={handleCreate}
                >
                    {#if isCreating}
                        <div class="button-spinner"></div>
                        Installing...
                    {:else}
                        {createText}
                    {/if}
                </button>
            {/if}
            
            {#if showSkip}
                <button 
                    class="skip-button" 
                    disabled={skipDisabled || isCreating}
                    on:click={handleSkip}
                >
                    {skipText}
                </button>
            {/if}
        </div>
    {/if}
</div>

<style>
    .navigation-section {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 1rem;
        margin-top: 1rem;
        flex-shrink: 0;
        padding: 1rem 0;
    }
    
    .navigation-section.instance-creation-mode {
        justify-content: space-between;
        align-items: center;
        position: relative;
        margin-top: 1rem;
        padding: 1rem 0;
    }
    
    .navigation-section.instance-creation-mode .back-button {
        position: static;
        transform: none;
    }
    
    .navigation-section.instance-creation-mode .create-button-container {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 0.75rem;
        width: auto;
        max-width: none;
    }
    
    .back-button {
        background: transparent;
        color: rgba(255, 255, 255, 0.6);
        border: 1px solid rgba(255, 255, 255, 0.2);
        padding: 0.4rem 0.6rem;
        border-radius: 6px;
        font-size: 0.75rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.3s ease;
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
        padding: 0.4rem 0.8rem;
        border-radius: 6px;
        font-size: 0.75rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.3s ease;
        box-shadow: 0 4px 16px rgba(34, 197, 94, 0.3);
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
    
    .create-button-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        width: 100%;
        margin: 0 auto;
        padding: 0.5rem 0;
    }
    
    .create-button {
        background: linear-gradient(135deg, #22c55e, #16a34a);
        color: white;
        border: none;
        padding: 0.6rem 1.5rem;
        border-radius: 8px;
        font-size: 0.85rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.3s ease;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        box-shadow: 0 4px 16px rgba(34, 197, 94, 0.4);
        min-width: 180px;
        width: auto;
        max-width: none;
        min-height: 44px;
    }
    
    .create-button:hover:not(:disabled) {
        background: linear-gradient(135deg, #16a34a, #15803d);
        transform: translateY(-2px);
        box-shadow: 0 8px 25px rgba(34, 197, 94, 0.5);
    }
    
    .create-button:disabled {
        opacity: 0.7;
        cursor: not-allowed;
        transform: none;
        box-shadow: 0 4px 12px rgba(34, 197, 94, 0.2);
    }
    
    .skip-button {
        background: rgba(255, 255, 255, 0.05);
        color: rgba(255, 255, 255, 0.7);
        border: 1px solid rgba(255, 255, 255, 0.15);
        padding: 0.6rem 1.25rem;
        border-radius: 8px;
        font-size: 0.8rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.3s ease;
        min-width: 140px;
        width: auto;
        max-width: none;
        min-height: 44px;
    }
    
    .skip-button:hover:not(:disabled) {
        background: rgba(255, 255, 255, 0.1);
        color: rgba(255, 255, 255, 0.9);
        border-color: rgba(255, 255, 255, 0.3);
        transform: translateY(-1px);
    }
    
    .skip-button:disabled {
        opacity: 0.7;
        cursor: not-allowed;
    }
    
    .button-spinner {
        width: 16px;
        height: 16px;
        border: 2px solid rgba(255, 255, 255, 0.3);
        border-top: 2px solid white;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }
    
    @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }
</style>
