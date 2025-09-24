<script>
    export let title = "Minecraft NAHA Launcher Installer";
    export let subtitle = "Transform your Minecraft experience with our powerful launcher";
    export let currentStep = 1;
    export let totalSteps = 3;
</script>

<div class="install-header">
    <!-- Animated background -->
    <div class="header-bg">
        <div class="bg-gradient"></div>
        <div class="floating-particles">
            {#each Array(6) as _, i}
                <div class="particle particle-{i + 1}"></div>
            {/each}
        </div>
    </div>
    
    <!-- Content -->
    <div class="header-content">
        <div class="title-section">
            <h1 class="main-title">{title}</h1>
            <p class="subtitle">{subtitle}</p>
        </div>
        
        <!-- Progress indicator -->
        <div class="progress-section">
            <div class="step-indicator">
                {#each Array(totalSteps) as _, i}
                    <div class="step" class:active={i + 1 <= currentStep} class:current={i + 1 === currentStep}>
                        <div class="step-number">{i + 1}</div>
                        <div class="step-line" class:completed={i + 1 < currentStep}></div>
                    </div>
                {/each}
            </div>
            <div class="step-labels">
                <span class="step-label">Choose Mode</span>
                <span class="step-label">Select Asset</span>
                <span class="step-label">Install</span>
            </div>
        </div>
    </div>
</div>

<style>
    .install-header {
        position: relative;
        overflow: hidden;
        border-radius: 24px;
        margin-bottom: 1rem;
        min-height: 260px;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    
    .header-bg {
        position: absolute;
        inset: 0;
        z-index: 0;
    }
    
    .bg-gradient {
        position: absolute;
        inset: 0;
        background: linear-gradient(135deg, 
            rgba(59, 130, 246, 0.9) 0%, 
            rgba(139, 92, 246, 0.8) 25%,
            rgba(236, 72, 153, 0.7) 50%,
            rgba(251, 146, 60, 0.6) 75%,
            rgba(34, 197, 94, 0.5) 100%
        );
        animation: gradientShift 8s ease-in-out infinite;
    }
    
    @keyframes gradientShift {
        0%, 100% { 
            background: linear-gradient(135deg, 
                rgba(59, 130, 246, 0.9) 0%, 
                rgba(139, 92, 246, 0.8) 25%,
                rgba(236, 72, 153, 0.7) 50%,
                rgba(251, 146, 60, 0.6) 75%,
                rgba(34, 197, 94, 0.5) 100%
            );
        }
        50% { 
            background: linear-gradient(135deg, 
                rgba(34, 197, 94, 0.5) 0%,
                rgba(59, 130, 246, 0.9) 25%, 
                rgba(139, 92, 246, 0.8) 50%,
                rgba(236, 72, 153, 0.7) 75%,
                rgba(251, 146, 60, 0.6) 100%
            );
        }
    }
    
    .floating-particles {
        position: absolute;
        inset: 0;
        pointer-events: none;
    }
    
    .particle {
        position: absolute;
        width: 4px;
        height: 4px;
        background: rgba(255, 255, 255, 0.6);
        border-radius: 50%;
        animation: float 6s ease-in-out infinite;
    }
    
    .particle-1 { top: 20%; left: 10%; animation-delay: 0s; }
    .particle-2 { top: 80%; left: 20%; animation-delay: 1s; }
    .particle-3 { top: 30%; left: 80%; animation-delay: 2s; }
    .particle-4 { top: 70%; left: 70%; animation-delay: 3s; }
    .particle-5 { top: 50%; left: 30%; animation-delay: 4s; }
    .particle-6 { top: 10%; left: 60%; animation-delay: 5s; }
    
    @keyframes float {
        0%, 100% { transform: translateY(0px) scale(1); opacity: 0.6; }
        50% { transform: translateY(-20px) scale(1.2); opacity: 1; }
    }
    
    .header-content {
        position: relative;
        z-index: 1;
        text-align: center;
        color: white;
        padding: 3rem 2rem;
        width: 100%;
    }
    
    .title-section {
        margin-bottom: 2rem;
    }
    
    .main-title {
        font-size: 2.5rem;
        font-weight: 800;
        margin-bottom: 0.5rem;
        text-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
        background: linear-gradient(45deg, #ffffff, #f0f9ff);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
        animation: titleGlow 3s ease-in-out infinite;
    }
    
    @keyframes titleGlow {
        0%, 100% { text-shadow: 0 4px 12px rgba(0, 0, 0, 0.3); }
        50% { text-shadow: 0 4px 20px rgba(255, 255, 255, 0.4); }
    }
    
    .subtitle {
        font-size: 1.1rem;
        opacity: 0.9;
        font-weight: 400;
        text-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
    }
    
    .progress-section {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
    }
    
    .step-indicator {
        display: flex;
        align-items: center;
        gap: 2rem;
    }
    
    .step {
        display: flex;
        align-items: center;
        position: relative;
    }
    
    .step-number {
        width: 40px;
        height: 40px;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-weight: 600;
        font-size: 0.9rem;
        background: rgba(255, 255, 255, 0.2);
        border: 2px solid rgba(255, 255, 255, 0.3);
        transition: all 0.3s ease;
        position: relative;
        z-index: 2;
    }
    
    .step.active .step-number {
        background: rgba(255, 255, 255, 0.9);
        color: #3b82f6;
        border-color: rgba(255, 255, 255, 0.9);
        box-shadow: 0 4px 12px rgba(255, 255, 255, 0.3);
    }
    
    .step.current .step-number {
        animation: pulse 2s infinite;
    }
    
    @keyframes pulse {
        0%, 100% { transform: scale(1); }
        50% { transform: scale(1.1); }
    }
    
    .step-line {
        width: 60px;
        height: 2px;
        background: rgba(255, 255, 255, 0.3);
        margin-left: 1rem;
        transition: all 0.3s ease;
    }
    
    .step-line.completed {
        background: rgba(255, 255, 255, 0.8);
        box-shadow: 0 0 8px rgba(255, 255, 255, 0.4);
    }
    
    .step:last-child .step-line {
        display: none;
    }
    
    .step-labels {
        display: flex;
        gap: 4rem;
        margin-top: 0.5rem;
    }
    
    .step-label {
        font-size: 0.875rem;
        opacity: 0.8;
        font-weight: 500;
        text-align: center;
        min-width: 80px;
    }
    
    /* Responsive design */
    @media (max-width: 768px) {
        .install-header {
            min-height: 220px;
        }
        
        .header-content {
            padding: 2.5rem 1.5rem;
        }
        
        .main-title {
            font-size: 2rem;
        }
        
        .subtitle {
            font-size: 1rem;
        }
        
        .step-indicator {
            gap: 1rem;
        }
        
        .step-line {
            width: 40px;
        }
        
        .step-labels {
            gap: 2rem;
        }
        
        .step-label {
            font-size: 0.75rem;
            min-width: 60px;
        }
    }
    
    @media (max-width: 640px) {
        .install-header {
            min-height: 200px;
        }
        
        .header-content {
            padding: 2rem 1rem;
        }
    }
</style>
