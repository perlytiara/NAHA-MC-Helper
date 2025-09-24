<!-- src/features/install/components/MrpackDownloader.svelte -->
<script>
    import { createEventDispatcher } from 'svelte';
    import { installerStore } from '../../../shared/stores/installerStore.js';
    
    export let loader = 'fabric';
    export let status = '';
    
    const dispatch = createEventDispatcher();
    let isDownloading = false;
    let downloadProgress = 0;

    async function handleDownload() {
        if (isDownloading) return;
        
        isDownloading = true;
        downloadProgress = 0;
        dispatch('download', loader);
        
        // The actual download logic is handled by the parent component
        // This component just triggers the event and shows UI feedback
    }

    // Reactive variables for UI
    $: loaderName = loader === 'neoforge' ? 'NeoForge' : loader === 'fabric' ? 'Fabric' : 'Unknown';
    $: loaderIcon = loader === 'neoforge' ? '🦊' : loader === 'fabric' ? '📜' : '❓';
    $: buttonText = `Download Latest ${loaderName} Modpack`;
    $: isSuccess = status.includes('Saved to:');
    $: isError = status.includes('Failed') || status.includes('Error');
    $: isLoading = status.includes('Downloading') || status.includes('Querying');
    
    // Reset downloading state when status changes to completed or cancelled
    $: if (isSuccess || isError || status.includes('Save canceled.')) {
        isDownloading = false;
        downloadProgress = 0;
    }
</script>

<div class="mrpack-downloader">
    <!-- Header section -->
    <div class="downloader-header">
        <div class="loader-info">
            <div class="loader-icon">{loaderIcon}</div>
            <div class="loader-details">
                <h4 class="loader-title">{loaderName} Modpack</h4>
                <p class="loader-description">Download the latest community modpack</p>
            </div>
        </div>
        
        <!-- Status indicator -->
        <div class="status-indicator" class:success={isSuccess} class:error={isError} class:loading={isLoading}>
            <div class="status-icon">
                {#if isSuccess}
                    ✅
                {:else if isError}
                    ❌
                {:else if isLoading}
                    <div class="spinner"></div>
                {:else}
                    📦
                {/if}
            </div>
        </div>
    </div>
    
    <!-- Download section -->
    <div class="download-section">
        <button 
            class="download-button" 
            class:downloading={isDownloading}
            class:success={isSuccess}
            class:error={isError}
            on:click={handleDownload}
            disabled={isDownloading}
        >
            <div class="button-content">
                <div class="button-icon">
                    {#if isDownloading}
                        <div class="download-spinner"></div>
                    {:else if isSuccess}
                        ✅
                    {:else if isError}
                        🔄
                    {:else}
                        ⬇️
                    {/if}
                </div>
                <div class="button-text">
                    <span class="button-title">
                        {#if isDownloading}
                            Downloading...
                        {:else if isSuccess}
                            Download Complete
                        {:else if isError}
                            Retry Download
                        {:else}
                            {buttonText}
                        {/if}
                    </span>
                    <span class="button-subtitle">
                        {#if isDownloading}
                            Please wait...
                        {:else if isSuccess}
                            Ready to use
                        {:else if isError}
                            Click to try again
                        {:else}
                            Get the latest {loaderName} mods
                        {/if}
                    </span>
                </div>
            </div>
            
            <!-- Progress bar for downloading state -->
            {#if isDownloading}
                <div class="download-progress">
                    <div class="progress-fill"></div>
                </div>
            {/if}
        </button>
        
        <!-- Status message -->
        {#if status}
            <div class="status-message" class:success={isSuccess} class:error={isError} class:loading={isLoading}>
                <div class="message-icon">
                    {#if isSuccess}
                        ℹ️
                    {:else if isError}
                        ⚠️
                    {:else}
                        💬
                    {/if}
                </div>
                <span class="message-text">{status}</span>
            </div>
        {/if}
    </div>
    
    <!-- Decorative elements -->
    <div class="decorative-elements">
        <div class="floating-dots">
            {#each Array(4) as _, i}
                <div class="dot dot-{i + 1}"></div>
            {/each}
        </div>
        <div class="wave-pattern"></div>
    </div>
</div>

<style>
    .mrpack-downloader {
        position: relative;
        width: 100%;
        max-width: 100%;
        box-sizing: border-box;
        background: linear-gradient(145deg, 
            rgba(139, 92, 246, 0.05) 0%, 
            rgba(196, 181, 253, 0.02) 100%
        );
        border: 1px solid rgba(139, 92, 246, 0.15);
        border-radius: 16px;
        padding: 1.5rem;
        overflow: hidden;
        backdrop-filter: blur(10px);
        box-shadow: 0 4px 16px rgba(139, 92, 246, 0.1);
        transition: all 0.3s ease;
    }
    
    .mrpack-downloader:hover {
        transform: translateY(-2px);
        box-shadow: 0 8px 24px rgba(139, 92, 246, 0.15);
        border-color: rgba(139, 92, 246, 0.25);
    }
    
    .downloader-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1.5rem;
        width: 100%;
        max-width: 100%;
        box-sizing: border-box;
    }
    
    .loader-info {
        display: flex;
        align-items: center;
        gap: 1rem;
        flex: 1;
        min-width: 0; /* Allow shrinking */
        max-width: calc(100% - 60px); /* Account for status indicator */
    }
    
    .loader-icon {
        font-size: 2.5rem;
        filter: drop-shadow(0 4px 8px rgba(0, 0, 0, 0.1));
        animation: iconFloat 3s ease-in-out infinite;
    }
    
    @keyframes iconFloat {
        0%, 100% { transform: translateY(0px); }
        50% { transform: translateY(-4px); }
    }
    
    .loader-details {
        flex: 1;
        min-width: 0; /* Allow shrinking */
        overflow: hidden;
    }
    
    .loader-title {
        font-size: 1.25rem;
        font-weight: 700;
        color: var(--text, #374151);
        margin: 0 0 0.25rem 0;
        background: linear-gradient(135deg, #7c3aed, #8b5cf6);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
    }
    
    .loader-description {
        font-size: 0.875rem;
        color: var(--text-secondary, #6b7280);
        margin: 0;
        opacity: 0.8;
    }
    
    .status-indicator {
        width: 48px;
        height: 48px;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        background: rgba(156, 163, 175, 0.1);
        border: 2px solid rgba(156, 163, 175, 0.2);
        transition: all 0.3s ease;
    }
    
    .status-indicator.success {
        background: rgba(34, 197, 94, 0.1);
        border-color: rgba(34, 197, 94, 0.3);
        animation: successPulse 2s ease-in-out infinite;
    }
    
    .status-indicator.error {
        background: rgba(239, 68, 68, 0.1);
        border-color: rgba(239, 68, 68, 0.3);
        animation: errorShake 0.5s ease-in-out;
    }
    
    .status-indicator.loading {
        background: rgba(139, 92, 246, 0.1);
        border-color: rgba(139, 92, 246, 0.3);
        animation: loadingPulse 1.5s ease-in-out infinite;
    }
    
    @keyframes successPulse {
        0%, 100% { transform: scale(1); }
        50% { transform: scale(1.05); }
    }
    
    @keyframes errorShake {
        0%, 100% { transform: translateX(0); }
        25% { transform: translateX(-2px); }
        75% { transform: translateX(2px); }
    }
    
    @keyframes loadingPulse {
        0%, 100% { transform: scale(1); opacity: 0.8; }
        50% { transform: scale(1.1); opacity: 1; }
    }
    
    .status-icon {
        font-size: 1.5rem;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
    }
    
    .spinner {
        width: 20px;
        height: 20px;
        border: 2px solid rgba(139, 92, 246, 0.3);
        border-top: 2px solid #8b5cf6;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }
    
    @keyframes spin {
        0% { transform: rotate(0deg); }
        100% { transform: rotate(360deg); }
    }
    
    .download-section {
        width: 100%;
        max-width: 100%;
        box-sizing: border-box;
    }
    
    .download-button {
        position: relative;
        width: 100%;
        max-width: 100%;
        box-sizing: border-box;
        background: linear-gradient(135deg, 
            rgba(139, 92, 246, 0.9) 0%, 
            rgba(124, 58, 237, 1) 100%
        );
        border: none;
        border-radius: 16px;
        padding: 1.5rem;
        cursor: pointer;
        transition: all 0.3s ease;
        overflow: hidden;
        box-shadow: 0 8px 24px rgba(139, 92, 246, 0.2);
    }
    
    .download-button:hover:not(:disabled) {
        transform: translateY(-3px);
        box-shadow: 0 12px 32px rgba(139, 92, 246, 0.3);
    }
    
    .download-button:active:not(:disabled) {
        transform: translateY(-1px);
    }
    
    .download-button:disabled {
        opacity: 0.8;
        cursor: not-allowed;
        transform: none;
    }
    
    .download-button.downloading {
        background: linear-gradient(135deg, 
            rgba(59, 130, 246, 0.9) 0%, 
            rgba(37, 99, 235, 1) 100%
        );
        animation: downloadingGlow 2s ease-in-out infinite;
    }
    
    .download-button.success {
        background: linear-gradient(135deg, 
            rgba(34, 197, 94, 0.9) 0%, 
            rgba(16, 185, 129, 1) 100%
        );
    }
    
    .download-button.error {
        background: linear-gradient(135deg, 
            rgba(239, 68, 68, 0.9) 0%, 
            rgba(220, 38, 38, 1) 100%
        );
    }
    
    @keyframes downloadingGlow {
        0%, 100% { box-shadow: 0 8px 24px rgba(59, 130, 246, 0.2); }
        50% { box-shadow: 0 8px 32px rgba(59, 130, 246, 0.4); }
    }
    
    .button-content {
        display: flex;
        align-items: center;
        gap: 1rem;
        color: white;
        position: relative;
        z-index: 2;
        width: 100%;
        max-width: 100%;
        box-sizing: border-box;
        overflow: hidden;
    }
    
    .button-icon {
        font-size: 2rem;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.2));
        display: flex;
        align-items: center;
        justify-content: center;
    }
    
    .download-spinner {
        width: 24px;
        height: 24px;
        border: 3px solid rgba(255, 255, 255, 0.3);
        border-top: 3px solid white;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }
    
    .button-text {
        flex: 1;
        text-align: left;
        min-width: 0; /* Allow shrinking */
        overflow: hidden;
    }
    
    .button-title {
        display: block;
        font-size: 1.125rem;
        font-weight: 700;
        margin-bottom: 0.25rem;
        text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }
    
    .button-subtitle {
        display: block;
        font-size: 0.875rem;
        opacity: 0.9;
        font-weight: 400;
    }
    
    .download-progress {
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        height: 4px;
        background: rgba(255, 255, 255, 0.2);
        overflow: hidden;
    }
    
    .progress-fill {
        height: 100%;
        background: linear-gradient(90deg, 
            rgba(255, 255, 255, 0.8) 0%, 
            rgba(255, 255, 255, 0.6) 100%
        );
        width: 100%;
        animation: progressSlide 2s ease-in-out infinite;
    }
    
    @keyframes progressSlide {
        0% { transform: translateX(-100%); }
        100% { transform: translateX(100%); }
    }
    
    .status-message {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        margin-top: 1rem;
        padding: 0.875rem 1rem;
        background: rgba(156, 163, 175, 0.05);
        border: 1px solid rgba(156, 163, 175, 0.1);
        border-radius: 12px;
        transition: all 0.3s ease;
    }
    
    .status-message.success {
        background: rgba(34, 197, 94, 0.05);
        border-color: rgba(34, 197, 94, 0.2);
        color: #166534;
    }
    
    .status-message.error {
        background: rgba(239, 68, 68, 0.05);
        border-color: rgba(239, 68, 68, 0.2);
        color: #dc2626;
    }
    
    .status-message.loading {
        background: rgba(139, 92, 246, 0.05);
        border-color: rgba(139, 92, 246, 0.2);
        color: #7c3aed;
    }
    
    .message-icon {
        font-size: 1.125rem;
        filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.1));
    }
    
    .message-text {
        flex: 1;
        font-weight: 500;
        font-size: 0.9rem;
        line-height: 1.4;
    }
    
    .decorative-elements {
        position: absolute;
        inset: 0;
        pointer-events: none;
        opacity: 0.3;
        overflow: hidden;
    }
    
    .floating-dots {
        position: absolute;
        inset: 0;
    }
    
    .dot {
        position: absolute;
        width: 3px;
        height: 3px;
        background: rgba(139, 92, 246, 0.4);
        border-radius: 50%;
        animation: dotFloat 4s ease-in-out infinite;
    }
    
    .dot-1 { top: 20%; left: 10%; animation-delay: 0s; }
    .dot-2 { top: 70%; left: 20%; animation-delay: 1s; }
    .dot-3 { top: 30%; left: 80%; animation-delay: 2s; }
    .dot-4 { top: 80%; left: 70%; animation-delay: 3s; }
    
    @keyframes dotFloat {
        0%, 100% { transform: translateY(0px) scale(1); opacity: 0.4; }
        50% { transform: translateY(-15px) scale(1.2); opacity: 0.8; }
    }
    
    .wave-pattern {
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        height: 30px;
        background: linear-gradient(90deg, 
            transparent 0%, 
            rgba(139, 92, 246, 0.1) 50%, 
            transparent 100%
        );
        animation: waveMove 3s ease-in-out infinite;
    }
    
    @keyframes waveMove {
        0%, 100% { transform: translateX(-10px); opacity: 0.3; }
        50% { transform: translateX(10px); opacity: 0.6; }
    }
    
    /* Dark theme */
    @media (prefers-color-scheme: dark) {
        .mrpack-downloader {
            background: linear-gradient(145deg, 
                rgba(139, 92, 246, 0.08) 0%, 
                rgba(196, 181, 253, 0.03) 100%
            );
            border-color: rgba(139, 92, 246, 0.2);
        }
        
        .loader-title {
            color: var(--text, #f9fafb);
            background: linear-gradient(135deg, #c4b5fd, #a78bfa);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }
        
        .loader-description {
            color: var(--text-secondary, #9ca3af);
        }
        
        .status-message.success {
            color: #86efac;
        }
        
        .status-message.error {
            color: #fca5a5;
        }
        
        .status-message.loading {
            color: #c4b5fd;
        }
        
        .message-text {
            color: inherit;
        }
    }
    
    /* Responsive design */
    @media (max-width: 768px) {
        .mrpack-downloader {
            padding: 1rem;
        }
        
        .downloader-header {
            flex-direction: column;
            gap: 1rem;
            text-align: center;
        }
        
        .loader-info {
            justify-content: center;
        }
        
        .download-button {
            padding: 1.25rem;
        }
        
        .button-content {
            flex-direction: column;
            text-align: center;
            gap: 0.75rem;
        }
        
        .button-text {
            text-align: center;
        }
    }
    
    @media (max-width: 640px) {
        .mrpack-downloader {
            padding: 0.875rem;
        }
        
        .loader-icon {
            font-size: 2rem;
        }
        
        .loader-title {
            font-size: 1.125rem;
        }
        
        .loader-description {
            font-size: 0.8rem;
        }
        
        .status-indicator {
            width: 40px;
            height: 40px;
        }
        
        .status-icon {
            font-size: 1.25rem;
        }
        
        .download-button {
            padding: 1rem;
        }
        
        .button-icon {
            font-size: 1.75rem;
        }
        
        .button-title {
            font-size: 1rem;
        }
        
        .button-subtitle {
            font-size: 0.8rem;
        }
        
        .status-message {
            padding: 0.75rem;
            margin-top: 0.75rem;
        }
        
        .message-text {
            font-size: 0.85rem;
        }
    }
</style>
