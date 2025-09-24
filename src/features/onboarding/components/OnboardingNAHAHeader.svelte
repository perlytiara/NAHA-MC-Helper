<!-- src/features/onboarding/components/OnboardingNAHAHeader.svelte -->
<script>
    // Simple header component that can be reused across different pages
</script>

<div class="header-section">
    <div class="logo">
        <div class="logo-icon">⚡</div>
        <span class="logo-text">NAHA</span>
    </div>
</div>

<style>
    .header-section {
        display: flex;
        justify-content: center;
    }
    
    .logo {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.4rem 0.8rem;
        background: rgba(255, 255, 255, 0.1);
        border-radius: 10px;
        backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 255, 255, 0.2);
    }
    
    .logo-icon {
        width: 20px;
        height: 20px;
        background: linear-gradient(135deg, #22c55e, #16a34a);
        border-radius: 5px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 10px;
        color: white;
        box-shadow: 0 2px 8px rgba(34, 197, 94, 0.3);
    }
    
    .logo-text {
        font-size: 1.3rem;
        font-weight: 800;
        color: white;
        letter-spacing: -0.5px;
    }
</style>
