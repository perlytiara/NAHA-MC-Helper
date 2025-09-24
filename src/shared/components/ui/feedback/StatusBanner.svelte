<!-- src/shared/components/ui/feedback/StatusBanner.svelte -->
<script>
    export let show = false;
    export let type = 'info'; // info, success, warning, error, loading
    export let message = '';
    export let title = '';
    export let dismissible = false;
    export let autoHide = false;
    export let autoHideDelay = 5000;
    
    import { onMount, createEventDispatcher } from 'svelte';
    
    const dispatch = createEventDispatcher();
    
    let timeoutId = null;
    
    const typeConfig = {
        info: {
            icon: 'ℹ️',
            bgColor: 'bg-blue-50',
            borderColor: 'border-blue-200',
            textColor: 'text-blue-800',
            iconBg: 'bg-blue-100'
        },
        success: {
            icon: '✅',
            bgColor: 'bg-green-50',
            borderColor: 'border-green-200',
            textColor: 'text-green-800',
            iconBg: 'bg-green-100'
        },
        warning: {
            icon: '⚠️',
            bgColor: 'bg-yellow-50',
            borderColor: 'border-yellow-200',
            textColor: 'text-yellow-800',
            iconBg: 'bg-yellow-100'
        },
        error: {
            icon: '❌',
            bgColor: 'bg-red-50',
            borderColor: 'border-red-200',
            textColor: 'text-red-800',
            iconBg: 'bg-red-100'
        },
        loading: {
            icon: '⏳',
            bgColor: 'bg-gray-50',
            borderColor: 'border-gray-200',
            textColor: 'text-gray-800',
            iconBg: 'bg-gray-100'
        }
    };
    
    $: config = typeConfig[type] || typeConfig.info;
    
    function dismiss() {
        show = false;
        dispatch('dismiss');
    }
    
    onMount(() => {
        if (autoHide && show) {
            timeoutId = setTimeout(() => {
                dismiss();
            }, autoHideDelay);
        }
        
        return () => {
            if (timeoutId) {
                clearTimeout(timeoutId);
            }
        };
    });
    
    $: if (show && autoHide) {
        if (timeoutId) clearTimeout(timeoutId);
        timeoutId = setTimeout(() => {
            dismiss();
        }, autoHideDelay);
    }
</script>

{#if show}
    <div 
        class="status-banner animate-in"
        class:bg-blue-50={config.bgColor === 'bg-blue-50'}
        class:border-blue-200={config.borderColor === 'border-blue-200'}
        class:bg-green-50={config.bgColor === 'bg-green-50'}
        class:border-green-200={config.borderColor === 'border-green-200'}
        class:bg-yellow-50={config.bgColor === 'bg-yellow-50'}
        class:border-yellow-200={config.borderColor === 'border-yellow-200'}
        class:bg-red-50={config.bgColor === 'bg-red-50'}
        class:border-red-200={config.borderColor === 'border-red-200'}
        class:bg-gray-50={config.bgColor === 'bg-gray-50'}
        class:border-gray-200={config.borderColor === 'border-gray-200'}
        role="alert"
        aria-live="polite"
    >
        <div class="banner-content">
            <div class="banner-icon"
                 class:bg-blue-100={config.iconBg === 'bg-blue-100'}
                 class:bg-green-100={config.iconBg === 'bg-green-100'}
                 class:bg-yellow-100={config.iconBg === 'bg-yellow-100'}
                 class:bg-red-100={config.iconBg === 'bg-red-100'}
                 class:bg-gray-100={config.iconBg === 'bg-gray-100'}>
                {#if type === 'loading'}
                    <div class="loading-spinner"></div>
                {:else}
                    <span class="icon-emoji">{config.icon}</span>
                {/if}
            </div>
            
            <div class="banner-text">
                {#if title}
                    <h4 class="banner-title"
                        class:text-blue-800={config.textColor === 'text-blue-800'}
                        class:text-green-800={config.textColor === 'text-green-800'}
                        class:text-yellow-800={config.textColor === 'text-yellow-800'}
                        class:text-red-800={config.textColor === 'text-red-800'}
                        class:text-gray-800={config.textColor === 'text-gray-800'}>{title}</h4>
                {/if}
                {#if message}
                    <p class="banner-message"
                       class:text-blue-800={config.textColor === 'text-blue-800'}
                       class:text-green-800={config.textColor === 'text-green-800'}
                       class:text-yellow-800={config.textColor === 'text-yellow-800'}
                       class:text-red-800={config.textColor === 'text-red-800'}
                       class:text-gray-800={config.textColor === 'text-gray-800'}>{message}</p>
                {/if}
            </div>
            
            {#if dismissible}
                <button
                    type="button"
                    class="dismiss-button"
                    on:click={dismiss}
                    aria-label="Dismiss notification"
                >
                    <span class="dismiss-icon">×</span>
                </button>
            {/if}
        </div>
    </div>
{/if}

<style>
    .status-banner {
        border: 1px solid;
        border-radius: 12px;
        padding: 1.5rem 1rem;
        margin-bottom: 1rem;
        position: relative;
        overflow: hidden;
        backdrop-filter: blur(10px);
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        display: flex;
        align-items: center;
        min-height: 4rem;
    }
    
    .status-banner.animate-in {
        animation: slideIn 0.3s ease-out;
    }
    
    @keyframes slideIn {
        from {
            opacity: 0;
            transform: translateY(-10px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }
    
    .banner-content {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        width: 100%;
    }
    
    .banner-icon {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 2rem;
        height: 2rem;
        border-radius: 8px;
        flex-shrink: 0;
    }
    
    .icon-emoji {
        font-size: 1rem;
        filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.1));
    }
    
    .loading-spinner {
        width: 1rem;
        height: 1rem;
        border: 2px solid transparent;
        border-top: 2px solid currentColor;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }
    
    @keyframes spin {
        to { transform: rotate(360deg); }
    }
    
    .banner-text {
        flex: 1;
        min-width: 0;
    }
    
    .banner-title {
        font-size: 0.875rem;
        font-weight: 600;
        margin: 0 0 0.25rem 0;
        line-height: 1.2;
    }
    
    .banner-message {
        font-size: 0.875rem;
        margin: 0;
        line-height: 1.4;
        opacity: 0.9;
    }
    
    .dismiss-button {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 1.5rem;
        height: 1.5rem;
        background: none;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        transition: background-color 0.2s ease;
        flex-shrink: 0;
    }
    
    .dismiss-button:hover {
        background: rgba(0, 0, 0, 0.1);
    }
    
    .dismiss-icon {
        font-size: 1.25rem;
        font-weight: bold;
        line-height: 1;
    }
    
    /* Color variants */
    .bg-blue-50 { background-color: rgba(239, 246, 255, 0.8); }
    .border-blue-200 { border-color: rgba(191, 219, 254, 0.5); }
    .text-blue-800 { color: #1e40af; }
    .bg-blue-100 { background-color: rgba(219, 234, 254, 0.8); }
    
    .bg-green-50 { background-color: rgba(240, 253, 244, 0.8); }
    .border-green-200 { border-color: rgba(187, 247, 208, 0.5); }
    .text-green-800 { color: #166534; }
    .bg-green-100 { background-color: rgba(220, 252, 231, 0.8); }
    
    .bg-yellow-50 { background-color: rgba(254, 252, 232, 0.8); }
    .border-yellow-200 { border-color: rgba(254, 240, 138, 0.5); }
    .text-yellow-800 { color: #92400e; }
    .bg-yellow-100 { background-color: rgba(254, 249, 195, 0.8); }
    
    .bg-red-50 { background-color: rgba(254, 242, 242, 0.8); }
    .border-red-200 { border-color: rgba(252, 165, 165, 0.5); }
    .text-red-800 { color: #991b1b; }
    .bg-red-100 { background-color: rgba(254, 226, 226, 0.8); }
    
    .bg-gray-50 { background-color: rgba(249, 250, 251, 0.8); }
    .border-gray-200 { border-color: rgba(229, 231, 235, 0.5); }
    .text-gray-800 { color: #1f2937; }
    .bg-gray-100 { background-color: rgba(243, 244, 246, 0.8); }
    
    /* Responsive design */
    @media (max-width: 768px) {
        .status-banner {
            padding: 1.25rem 0.875rem;
            min-height: 3.5rem;
        }
    }
    
    @media (max-width: 640px) {
        .status-banner {
            padding: 1rem 0.75rem;
            min-height: 3rem;
        }
    }
    
    /* Dark theme */
    @media (prefers-color-scheme: dark) {
        .bg-blue-50 { background-color: rgba(30, 58, 138, 0.2); }
        .border-blue-200 { border-color: rgba(59, 130, 246, 0.3); }
        .text-blue-800 { color: #93c5fd; }
        .bg-blue-100 { background-color: rgba(59, 130, 246, 0.2); }
        
        .bg-green-50 { background-color: rgba(22, 101, 52, 0.2); }
        .border-green-200 { border-color: rgba(34, 197, 94, 0.3); }
        .text-green-800 { color: #86efac; }
        .bg-green-100 { background-color: rgba(34, 197, 94, 0.2); }
        
        .bg-yellow-50 { background-color: rgba(146, 64, 14, 0.2); }
        .border-yellow-200 { border-color: rgba(251, 191, 36, 0.3); }
        .text-yellow-800 { color: #fde047; }
        .bg-yellow-100 { background-color: rgba(251, 191, 36, 0.2); }
        
        .bg-red-50 { background-color: rgba(153, 27, 27, 0.2); }
        .border-red-200 { border-color: rgba(239, 68, 68, 0.3); }
        .text-red-800 { color: #fca5a5; }
        .bg-red-100 { background-color: rgba(239, 68, 68, 0.2); }
        
        .bg-gray-50 { background-color: rgba(31, 41, 55, 0.2); }
        .border-gray-200 { border-color: rgba(75, 85, 99, 0.3); }
        .text-gray-800 { color: #d1d5db; }
        .bg-gray-100 { background-color: rgba(75, 85, 99, 0.2); }
        
        .dismiss-button:hover {
            background: rgba(255, 255, 255, 0.1);
        }
    }
</style>
