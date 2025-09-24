<!-- src/features/onboarding/components/OnboardingWelcome.svelte -->
<script>
    import { createEventDispatcher } from 'svelte';
    
    const dispatch = createEventDispatcher();
    
    function handleGetStarted() {
        dispatch('next');
    }
    
    function handleSkip() {
        dispatch('skip');
    }
</script>

<div class="welcome-content">
    <!-- Header -->
    <div class="header-section">
        <div class="logo">
            <div class="logo-icon">⚡</div>
            <span class="logo-text">NAHA</span>
        </div>
    </div>
    
    <!-- Main content -->
    <div class="main-content">
        <h1 class="welcome-title">Welcome to NAHA Minecraft Helper</h1>
        
        <p class="welcome-description">
            NAHA Minecraft Helper simplifies the installation process and server setup for Minecraft, making it easier than ever to jump into the game. Let's get started!
        </p>
        
        <div class="action-buttons">
            <button class="get-started-button" on:click={handleGetStarted}>
                Get Started
            </button>
            
            <button class="skip-button" on:click={handleSkip}>
                Skip Setup
            </button>
        </div>
    </div>
    
    <!-- Footer -->
    <div class="footer-section">
        <div class="footer-tip">
            <span class="tip-icon">💡</span>
            <span class="tip-text">This setup will help you configure your Minecraft experience</span>
        </div>
    </div>
</div>

<style>
    .welcome-content {
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
        gap: 1.5rem;
        align-items: center;
    }
    
    .welcome-title {
        font-size: 2.5rem;
        font-weight: 700;
        color: white;
        margin: 0;
        line-height: 1.2;
        text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    }
    
    .welcome-description {
        font-size: 1.125rem;
        color: rgba(255, 255, 255, 0.8);
        line-height: 1.6;
        margin: 0;
        max-width: 450px;
    }
    
    .action-buttons {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        width: 100%;
        max-width: 300px;
    }
    
    .get-started-button {
        background: linear-gradient(135deg, #22c55e, #16a34a);
        color: white;
        border: none;
        padding: 1rem 2rem;
        border-radius: 12px;
        font-size: 1.125rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.3s ease;
        box-shadow: 0 4px 16px rgba(34, 197, 94, 0.3);
        text-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
    }
    
    .get-started-button:hover {
        background: linear-gradient(135deg, #16a34a, #15803d);
        transform: translateY(-2px);
        box-shadow: 0 8px 24px rgba(34, 197, 94, 0.4);
    }
    
    .get-started-button:active {
        transform: translateY(0);
    }
    
    .skip-button {
        background: transparent;
        color: rgba(255, 255, 255, 0.6);
        border: 1px solid rgba(255, 255, 255, 0.2);
        padding: 0.75rem 1.5rem;
        border-radius: 8px;
        font-size: 1rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.3s ease;
    }
    
    .skip-button:hover {
        background: rgba(255, 255, 255, 0.1);
        color: rgba(255, 255, 255, 0.8);
        border-color: rgba(255, 255, 255, 0.3);
    }
    
    .footer-section {
        display: flex;
        justify-content: center;
        margin-top: 1rem;
    }
    
    .footer-tip {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        color: rgba(255, 255, 255, 0.6);
        font-size: 0.875rem;
        font-weight: 500;
        padding: 0.5rem 1rem;
        background: rgba(255, 255, 255, 0.05);
        border-radius: 20px;
        backdrop-filter: blur(10px);
    }
    
    .tip-icon {
        font-size: 1rem;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
    }
    
    /* Responsive design */
    @media (max-width: 768px) {
        .welcome-content {
            gap: 1.5rem;
            max-width: 400px;
        }
        
        .welcome-title {
            font-size: 2rem;
        }
        
        .welcome-description {
            font-size: 1rem;
        }
        
        .action-buttons {
            max-width: 250px;
        }
        
        .get-started-button {
            padding: 0.875rem 1.5rem;
            font-size: 1rem;
        }
    }
    
    @media (max-width: 640px) {
        .welcome-content {
            gap: 1.25rem;
            max-width: 350px;
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
        
        .welcome-title {
            font-size: 1.75rem;
        }
        
        .welcome-description {
            font-size: 0.9rem;
        }
        
        .action-buttons {
            max-width: 200px;
        }
        
        .get-started-button {
            padding: 0.75rem 1.25rem;
            font-size: 0.9rem;
        }
        
        .skip-button {
            padding: 0.625rem 1.25rem;
            font-size: 0.875rem;
        }
    }
</style>
