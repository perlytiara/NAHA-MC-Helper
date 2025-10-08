<!-- src/features/onboarding/components/OnboardingMinecraftCheck.svelte -->
<script>
    import { createEventDispatcher } from 'svelte';
    import { hideUpdateButton } from '../../../shared/stores/appStore';
    import { t } from '../../../shared/stores/i18nStore';
    
    const dispatch = createEventDispatcher();
    
    function handleYes() {
        dispatch('minecraft-response', { hasMinecraft: true });
    }
    
    function handleNo() {
        dispatch('minecraft-response', { hasMinecraft: false });
    }
    
    function handleUpdate() {
        dispatch('minecraft-response', { hasMinecraft: true, wantsUpdate: true });
    }
    
    function handleBack() {
        dispatch('back');
    }
</script>

<div class="minecraft-check-content">
    <!-- Header -->
    <div class="header-section">
        <div class="logo">
            <div class="logo-icon">⚡</div>
            <span class="logo-text">NAHA</span>
        </div>
    </div>
    
    <!-- Main content -->
    <div class="main-content">
        <h1 class="question-title">{$t('onboarding.minecraftCheck.title')}</h1>
        
        <div class="response-buttons">
            <!-- Top row - two buttons -->
            <div class="button-row top-row">
                <button class="response-button yes-button" on:click={handleYes}>
                    <span class="button-icon">✅</span>
                    <span class="button-text">{$t('onboarding.minecraftCheck.yesHaveIt')}</span>
                </button>
                
                <button class="response-button no-button" on:click={handleNo}>
                    <span class="button-icon">📥</span>
                    <span class="button-text">{$t('onboarding.minecraftCheck.noInstallIt')}</span>
                </button>
            </div>
            
            <!-- Bottom row - centered update button -->
            {#if !$hideUpdateButton}
            <div class="button-row bottom-row">
                <button class="response-button update-button" on:click={handleUpdate}>
                    <span class="button-icon">🔄</span>
                    <span class="button-text">{$t('onboarding.minecraftCheck.updateInstances')}</span>
                </button>
            </div>
            {/if}
        </div>
        
        <p class="description-text">
            {$t('onboarding.minecraftCheck.description')}
        </p>
    </div>
    
    <!-- Navigation -->
    <div class="navigation-section">
        <button class="back-button" on:click={handleBack}>
            ← {$t('common.back')}
        </button>
    </div>
</div>

<style>
    .minecraft-check-content {
        display: flex;
        flex-direction: column;
        gap: 2rem;
        text-align: center;
        width: 100%;
        max-width: 500px;
        margin: 0 auto;
    }
    
    .header-section {
        display: flex;
        justify-content: center;
    }
    
    .logo {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 0.5rem 1rem;
        background: rgba(255, 255, 255, 0.1);
        border-radius: 12px;
        backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 255, 255, 0.2);
    }
    
    .logo-icon {
        width: 24px;
        height: 24px;
        background: linear-gradient(135deg, #22c55e, #16a34a);
        border-radius: 6px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 12px;
        color: white;
        box-shadow: 0 2px 8px rgba(34, 197, 94, 0.3);
    }
    
    .logo-text {
        font-size: 1.5rem;
        font-weight: 800;
        color: white;
        letter-spacing: -0.5px;
    }
    
    .main-content {
        display: flex;
        flex-direction: column;
        gap: 2rem;
        align-items: center;
    }
    
    .question-title {
        font-size: 2.25rem;
        font-weight: 700;
        color: white;
        margin: 0;
        line-height: 1.2;
        text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    }
    
    .response-buttons {
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        width: 100%;
        max-width: 550px;
    }
    
    .button-row {
        display: flex;
        gap: 1rem;
        width: 100%;
    }
    
    .top-row {
        justify-content: center;
    }
    
    .bottom-row {
        justify-content: center;
    }
    
    .response-button {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.75rem;
        border: none;
        padding: 1rem 1.5rem;
        border-radius: 12px;
        font-size: 1.125rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.3s ease;
        text-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
        flex: 1;
        min-width: 0;
    }
    
    .top-row .response-button {
        max-width: 250px;
    }
    
    .bottom-row .response-button {
        max-width: 300px;
    }
    
    .button-icon {
        font-size: 1.5rem;
        flex-shrink: 0;
    }
    
    .button-text {
        white-space: nowrap;
    }
    
    .yes-button {
        background: linear-gradient(135deg, #22c55e, #16a34a);
        color: white;
        box-shadow: 0 4px 16px rgba(34, 197, 94, 0.3);
    }
    
    .yes-button:hover {
        background: linear-gradient(135deg, #16a34a, #15803d);
        transform: translateY(-2px);
        box-shadow: 0 8px 24px rgba(34, 197, 94, 0.4);
    }
    
    .yes-button:active {
        transform: translateY(0);
    }
    
    .no-button {
        background: linear-gradient(135deg, #3b82f6, #2563eb);
        color: white;
        box-shadow: 0 4px 16px rgba(59, 130, 246, 0.3);
    }
    
    .no-button:hover {
        background: linear-gradient(135deg, #2563eb, #1d4ed8);
        transform: translateY(-2px);
        box-shadow: 0 8px 24px rgba(59, 130, 246, 0.4);
    }
    
    .no-button:active {
        transform: translateY(0);
    }
    
    .update-button {
        background: linear-gradient(135deg, #8b5cf6, #7c3aed);
        color: white;
        box-shadow: 0 4px 16px rgba(139, 92, 246, 0.3);
    }
    
    .update-button:hover {
        background: linear-gradient(135deg, #7c3aed, #6d28d9);
        transform: translateY(-2px);
        box-shadow: 0 8px 24px rgba(139, 92, 246, 0.4);
    }
    
    .update-button:active {
        transform: translateY(0);
    }
    
    .description-text {
        font-size: 1rem;
        color: rgba(255, 255, 255, 0.7);
        line-height: 1.5;
        margin: 0;
        max-width: 400px;
    }
    
    .navigation-section {
        display: flex;
        justify-content: flex-start;
    }
    
    .back-button {
        background: transparent;
        color: rgba(255, 255, 255, 0.6);
        border: 1px solid rgba(255, 255, 255, 0.2);
        padding: 0.75rem 1rem;
        border-radius: 8px;
        font-size: 0.9rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.3s ease;
    }
    
    .back-button:hover {
        background: rgba(255, 255, 255, 0.1);
        color: rgba(255, 255, 255, 0.8);
        border-color: rgba(255, 255, 255, 0.3);
    }
    
    /* Responsive design */
    @media (max-width: 768px) {
        .minecraft-check-content {
            gap: 1.5rem;
        }
        
        .question-title {
            font-size: 1.875rem;
        }
        
        .response-buttons {
            max-width: 100%;
        }
        
        .button-row {
            flex-direction: column;
            align-items: center;
        }
        
        .top-row .response-button,
        .bottom-row .response-button {
            max-width: 300px;
            width: 100%;
        }
        
        .response-button {
            padding: 0.875rem 1.25rem;
            font-size: 1rem;
        }
        
        .description-text {
            font-size: 0.9rem;
        }
    }
    
    @media (max-width: 640px) {
        .minecraft-check-content {
            gap: 1.25rem;
        }
        
        .logo {
            padding: 0.375rem 0.75rem;
            gap: 0.5rem;
        }
        
        .logo-icon {
            width: 20px;
            height: 20px;
            font-size: 10px;
        }
        
        .logo-text {
            font-size: 1.25rem;
        }
        
        .question-title {
            font-size: 1.5rem;
        }
        
        .top-row .response-button,
        .bottom-row .response-button {
            max-width: 250px;
        }
        
        .response-button {
            padding: 0.75rem 1rem;
            font-size: 0.9rem;
        }
        
        .button-text {
            font-size: 0.875rem;
        }
        
        .description-text {
            font-size: 0.85rem;
        }
        
        .back-button {
            padding: 0.625rem 0.875rem;
            font-size: 0.8rem;
        }
    }
</style>
