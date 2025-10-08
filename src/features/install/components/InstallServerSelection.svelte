<!-- src/features/install/components/InstallServerSelection.svelte -->
<script>
    import { createEventDispatcher } from 'svelte';
    
    const dispatch = createEventDispatcher();
    
    function handleNeoforge() {
        dispatch('select', { serverType: 'neoforge' });
    }
    
    function handleFabric() {
        dispatch('select', { serverType: 'fabric' });
    }
</script>

<div class="selection-content">
    <!-- Header -->
    <div class="header-section">
        <div class="logo">
            <div class="logo-icon">⚡</div>
            <span class="logo-text">NAHA</span>
        </div>
    </div>
    
    <!-- Main content -->
    <div class="main-content">
        <h1 class="selection-title">Choose Your Modpack</h1>
        
        <p class="description-text">
            Select which modpack you'd like to install. Both options provide an amazing experience!
        </p>
        
        <div class="server-cards">
            <button class="server-card neoforge-card" on:click={handleNeoforge}>
                <div class="card-icon">🔥</div>
                <div class="card-title">NeoForge</div>
                <div class="card-description">
                    Performance-focused modpack with advanced features
                </div>
            </button>
            
            <button class="server-card fabric-card" on:click={handleFabric}>
                <div class="card-icon">🧵</div>
                <div class="card-title">Fabric</div>
                <div class="card-description">
                    Lightweight and modern modpack experience
                </div>
            </button>
        </div>
    </div>
</div>

<style>
    .selection-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 3rem;
        text-align: center;
        width: 100%;
        max-width: 700px;
        margin: 0 auto;
        padding: 2rem;
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
        width: 100%;
    }
    
    .selection-title {
        font-size: 2.5rem;
        font-weight: 700;
        color: white;
        margin: 0;
        line-height: 1.2;
        text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    }
    
    .description-text {
        font-size: 1.125rem;
        color: rgba(255, 255, 255, 0.8);
        line-height: 1.6;
        margin: 0;
        max-width: 500px;
    }
    
    .server-cards {
        display: flex;
        gap: 1.5rem;
        width: 100%;
        max-width: 600px;
    }
    
    .server-card {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        padding: 2rem 1.5rem;
        border: 2px solid rgba(255, 255, 255, 0.2);
        border-radius: 16px;
        background: rgba(255, 255, 255, 0.05);
        backdrop-filter: blur(10px);
        cursor: pointer;
        transition: all 0.3s ease;
    }
    
    .server-card:hover {
        transform: translateY(-4px);
        border-color: rgba(255, 255, 255, 0.4);
        background: rgba(255, 255, 255, 0.1);
    }
    
    .server-card:active {
        transform: translateY(-2px);
    }
    
    .neoforge-card:hover {
        border-color: #f97316;
        box-shadow: 0 8px 32px rgba(249, 115, 22, 0.3);
    }
    
    .fabric-card:hover {
        border-color: #8b5cf6;
        box-shadow: 0 8px 32px rgba(139, 92, 246, 0.3);
    }
    
    .card-icon {
        font-size: 3rem;
        margin-bottom: 0.5rem;
    }
    
    .card-title {
        font-size: 1.5rem;
        font-weight: 700;
        color: white;
        margin: 0;
    }
    
    .card-description {
        font-size: 0.9rem;
        color: rgba(255, 255, 255, 0.7);
        line-height: 1.5;
        margin: 0;
    }
    
    @media (max-width: 768px) {
        .server-cards {
            flex-direction: column;
            gap: 1rem;
        }
        
        .server-card {
            padding: 1.5rem 1rem;
        }
    }
    
    @media (max-width: 640px) {
        .selection-content {
            gap: 2rem;
            padding: 1.5rem;
        }
        
        .selection-title {
            font-size: 2rem;
        }
        
        .description-text {
            font-size: 1rem;
        }
        
        .card-icon {
            font-size: 2.5rem;
        }
        
        .card-title {
            font-size: 1.25rem;
        }
    }
</style>

