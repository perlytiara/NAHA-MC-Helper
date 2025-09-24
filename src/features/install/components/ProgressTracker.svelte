<script>
    export let progress = 0;
    export let status = '';
    export let showPercentage = true;
    export let variant = 'primary'; // primary, success, warning, error
    
    $: clampedProgress = Math.min(100, Math.max(0, progress));
    $: progressPercentage = Math.round(clampedProgress);
    
    // Determine current phase based on progress and status
    $: currentPhase = (() => {
        if (status.includes('Fetching') || status.includes('Preparing')) return 0;
        if (status.includes('Downloading')) return 1;
        if (status.includes('Validating') || status.includes('complete')) return 2;
        if (status.includes('Installing') || status.includes('Starting installation')) return 3;
        if (status.includes('Complete') || progress >= 100) return 4;
        return Math.floor(clampedProgress / 25);
    })();
    
    const phases = [
        { label: 'Prepare', icon: '⚙️' },
        { label: 'Download', icon: '📥' },
        { label: 'Validate', icon: '🔍' },
        { label: 'Install', icon: '🚀' },
        { label: 'Complete', icon: '✅' }
    ];
</script>

<div class="progress-tracker">
    {#if status}
        <div class="status-text" class:has-percentage={showPercentage && progress > 0}>
            <span class="status-message">{status}</span>
            {#if showPercentage && progress > 0}
                <span class="progress-percentage">{progressPercentage}%</span>
            {/if}
        </div>
    {/if}
    
    <div class="progress-container {variant}">
        <div class="progress-track">
            <div 
                class="progress-fill {variant}" 
                style="width: {clampedProgress}%"
                role="progressbar"
                aria-valuenow={clampedProgress}
                aria-valuemin="0"
                aria-valuemax="100"
                aria-label="Installation progress"
            >
                <div class="progress-shine"></div>
            </div>
        </div>
        
        <!-- Phase indicators -->
        <div class="phase-indicators">
            {#each phases as phase, i}
                <div class="phase-item" class:active={i <= currentPhase} class:current={i === currentPhase}>
                    <div class="phase-icon">{phase.icon}</div>
                    <span class="phase-label">{phase.label}</span>
                </div>
            {/each}
        </div>
    </div>
</div>

<style>
    .progress-tracker {
        width: 100%;
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }
    
    .status-text {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 1rem;
    }
    
    .status-text.has-percentage {
        flex-wrap: wrap;
    }
    
    .status-message {
        font-size: 0.9rem;
        font-weight: 500;
        color: var(--text, #374151);
        flex: 1;
        min-width: 0;
    }
    
    .progress-percentage {
        font-size: 0.875rem;
        font-weight: 600;
        color: var(--text-secondary, #6b7280);
        background: rgba(59, 130, 246, 0.1);
        padding: 0.25rem 0.5rem;
        border-radius: 6px;
        border: 1px solid rgba(59, 130, 246, 0.2);
        min-width: fit-content;
    }
    
    .progress-container {
        position: relative;
        width: 100%;
    }
    
    .progress-track {
        width: 100%;
        height: 8px;
        background: rgba(203, 213, 225, 0.3);
        border-radius: 8px;
        overflow: hidden;
        backdrop-filter: blur(10px);
        border: 1px solid rgba(203, 213, 225, 0.2);
        position: relative;
    }
    
    .progress-fill {
        height: 100%;
        border-radius: 8px;
        transition: width 0.6s cubic-bezier(0.4, 0, 0.2, 1);
        position: relative;
        overflow: hidden;
        
        /* Default primary style */
        background: linear-gradient(90deg, #3b82f6, #8b5cf6);
        box-shadow: 0 0 8px rgba(59, 130, 246, 0.3);
    }
    
    /* Variant styles */
    .progress-fill.success {
        background: linear-gradient(90deg, #10b981, #34d399);
        box-shadow: 0 0 8px rgba(16, 185, 129, 0.3);
    }
    
    .progress-fill.warning {
        background: linear-gradient(90deg, #f59e0b, #fbbf24);
        box-shadow: 0 0 8px rgba(245, 158, 11, 0.3);
    }
    
    .progress-fill.error {
        background: linear-gradient(90deg, #dc2626, #ef4444);
        box-shadow: 0 0 8px rgba(220, 38, 38, 0.3);
    }
    
    .progress-shine {
        position: absolute;
        top: 0;
        left: -100%;
        width: 100%;
        height: 100%;
        background: linear-gradient(
            90deg,
            transparent 0%,
            rgba(255, 255, 255, 0.3) 50%,
            transparent 100%
        );
        animation: shine 2s ease-in-out infinite;
    }
    
    @keyframes shine {
        0% { left: -100%; }
        100% { left: 100%; }
    }
    
    .phase-indicators {
        display: flex;
        justify-content: space-between;
        margin-top: 0.75rem;
        gap: 0.5rem;
    }
    
    .phase-item {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.25rem;
        flex: 1;
        transition: all 0.3s ease;
        opacity: 0.4;
    }
    
    .phase-item.active {
        opacity: 1;
    }
    
    .phase-item.current {
        opacity: 1;
        transform: scale(1.05);
    }
    
    .phase-icon {
        font-size: 1.25rem;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
        transition: all 0.3s ease;
    }
    
    .phase-item.current .phase-icon {
        animation: phaseIconPulse 1.5s ease-in-out infinite;
    }
    
    .phase-label {
        font-size: 0.75rem;
        font-weight: 500;
        color: var(--text-secondary, #6b7280);
        text-align: center;
        transition: all 0.3s ease;
    }
    
    .phase-item.active .phase-label {
        color: var(--text, #374151);
        font-weight: 600;
    }
    
    .phase-item.current .phase-label {
        color: #3b82f6;
        font-weight: 700;
    }
    
    @keyframes phaseIconPulse {
        0%, 100% { transform: scale(1); opacity: 1; }
        50% { transform: scale(1.15); opacity: 0.8; }
    }
    
    /* Container variant styles */
    .progress-container.success .progress-percentage {
        background: rgba(16, 185, 129, 0.1);
        border-color: rgba(16, 185, 129, 0.2);
    }
    
    .progress-container.success .phase-item.current .phase-label {
        color: #10b981;
    }
    
    .progress-container.warning .progress-percentage {
        background: rgba(245, 158, 11, 0.1);
        border-color: rgba(245, 158, 11, 0.2);
    }
    
    .progress-container.warning .phase-item.current .phase-label {
        color: #f59e0b;
    }
    
    .progress-container.error .progress-percentage {
        background: rgba(220, 38, 38, 0.1);
        border-color: rgba(220, 38, 38, 0.2);
    }
    
    .progress-container.error .phase-item.current .phase-label {
        color: #dc2626;
    }
    
    /* Dark theme */
    @media (prefers-color-scheme: dark) {
        .status-message {
            color: var(--text, #f9fafb);
        }
        
        .progress-percentage {
            color: var(--text-secondary, #9ca3af);
            background: rgba(59, 130, 246, 0.15);
            border-color: rgba(59, 130, 246, 0.3);
        }
        
        .progress-track {
            background: rgba(75, 85, 99, 0.4);
            border-color: rgba(75, 85, 99, 0.3);
        }
        
        .phase-label {
            color: var(--text-secondary, #9ca3af);
        }
        
        .phase-item.active .phase-label {
            color: var(--text, #f9fafb);
        }
        
        .progress-container.success .progress-percentage {
            background: rgba(16, 185, 129, 0.15);
            border-color: rgba(16, 185, 129, 0.3);
        }
        
        .progress-container.warning .progress-percentage {
            background: rgba(245, 158, 11, 0.15);
            border-color: rgba(245, 158, 11, 0.3);
        }
        
        .progress-container.error .progress-percentage {
            background: rgba(220, 38, 38, 0.15);
            border-color: rgba(220, 38, 38, 0.3);
        }
    }
    
    /* Responsive design */
    @media (max-width: 640px) {
        .status-text {
            flex-direction: column;
            align-items: flex-start;
            gap: 0.5rem;
        }
        
        .status-message {
            font-size: 0.85rem;
        }
        
        .progress-percentage {
            font-size: 0.8rem;
            align-self: flex-end;
        }
        
        .progress-track {
            height: 6px;
        }
        
        .phase-indicators {
            margin-top: 0.5rem;
            gap: 0.25rem;
        }
        
        .phase-icon {
            font-size: 1rem;
        }
        
        .phase-label {
            font-size: 0.7rem;
        }
    }
</style>
