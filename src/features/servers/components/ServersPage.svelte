<script>
    import ServerBadge from '../../../shared/components/ui/display/ServerBadge.svelte';
    import NavigationBar from '../../../shared/components/ui/navigation/NavigationBar.svelte';
    import { currentPage, debug } from '../../../shared/stores/appStore';
    import fabricImage from '../../../assets/images/mc-servers/fabric-art-welcome.jpg';
    import btwImage from '../../../assets/images/mc-servers/btw-art-welcome.jpg';
    import neoforgeImage from '../../../assets/images/mc-servers/neoforge-art-welcome.jpg';

    /** Server data matching the image layout */
    let servers = [
        {
            id: 'fabric',
            name: 'Fabric',
            loader: 'fabric',
            mcVersion: '1.21.8',
            packVersion: '0.1.10',
            online: 5,
            max: 20,
            image: fabricImage,
            updateAvailable: true
        },
        {
            id: 'btw',
            name: 'BTW',
            loader: 'btw',
            mcVersion: '1.5.6',
            packVersion: '0.1.10',
            online: 5,
            max: 20,
            image: btwImage,
            updateAvailable: true
        },
        {
            id: 'neoforge',
            name: 'NeoForge',
            loader: 'neoforge',
            mcVersion: '1.21.1',
            packVersion: '0.0.11',
            online: 5,
            max: 20,
            image: neoforgeImage,
            updateAvailable: true
        }
    ];

    // Toggle state: true = APP mode (servers page), false = ADV mode (homepage)
    let isAppMode = true;

    function joinServer(id) {
        if (id === 'fabric' || id === 'neoforge') {
            currentPage.set('update');
        } else {
            alert('Applications for BTW will open soon.');
        }
    }

    function toggleMode() {
        isAppMode = !isAppMode;
        if (isAppMode) {
            // Stay on servers page (APP mode)
            currentPage.set('servers');
        } else {
            // Go to homepage (ADV mode - advanced GUI)
            currentPage.set('homepage');
        }
    }
</script>

<div class="servers-container">
    <NavigationBar debug={$debug} />
    <div id="global-notice"></div>
    
    <!-- Three diagonal server panels -->
    <div class="diagonal-layout">
        {#each servers as server, index}
            <div class="server-panel panel-{index}" style="background-image: url('{server.image}')">
                <!-- Server Badge -->
                <div class="server-badge">
                    <ServerBadge loader={server.loader} />
                </div>
                
                <!-- Server Info -->
                <div class="server-info">
                    <div class="players-status">
                        <div class="status-indicator"></div>
                        <span>Players Online: {server.online}/{server.max}</span>
                    </div>
                    <div class="minecraft-version">Minecraft Version — {server.mcVersion}</div>
                    <div class="pack-version-row">
                        <span class="pack-version">Mod Pack Version | {server.packVersion}</span>
                        {#if server.updateAvailable}
                            <div class="update-badge">Update Available</div>
                        {/if}
                    </div>
                </div>
                
                <!-- Join button positioned in bottom right -->
                <button class="join-btn" on:click={() => joinServer(server.id)}>Join</button>
                
                <!-- Community section for middle panel -->
                {#if index === 1}
                    <div class="community-section">
                        <h2>Community & Resources</h2>
                        <button class="community-btn">View Tutorials</button>
                        <button class="community-btn discord">Discord</button>
                        <div class="mode-toggle">
                            <span class="mode-label" class:active={isAppMode}>APP</span>
                            <button 
                                type="button" 
                                class="toggle-switch" 
                                class:advanced={!isAppMode} 
                                on:click={toggleMode} 
                                aria-label="Toggle between APP and ADV mode"
                            ></button>
                            <span class="mode-label" class:active={!isAppMode}>ADV</span>
                        </div>
                    </div>
                {/if}
            </div>
        {/each}
    </div>
</div>

<style>
    .servers-container {
        width: 100vw;
        height: 100vh;
        overflow: hidden;
        position: relative;
    }
    
    .diagonal-layout {
        display: flex;
        width: 100%;
        height: 100%;
        position: relative;
    }
    
    .server-panel {
        flex: 1;
        position: relative;
        background-size: cover;
        background-position: center;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        padding: 20px;
        overflow: hidden;
    }
    
    /* Diagonal separators */
    .server-panel:not(:last-child)::after {
        content: '';
        position: absolute;
        top: 0;
        right: 0;
        width: 60px;
        height: 100%;
        background: linear-gradient(45deg, transparent 48%, rgba(0,0,0,0.8) 50%, transparent 52%);
        z-index: 2;
        transform: skewX(-15deg);
        transform-origin: top right;
    }
    
    /* Dark overlay for readability */
    .server-panel::before {
        content: '';
        position: absolute;
        inset: 0;
        background: linear-gradient(135deg, rgba(0,0,0,0.6) 0%, rgba(0,0,0,0.3) 100%);
        z-index: 1;
    }
    
    .server-badge {
        position: absolute;
        top: 20px;
        left: 20px;
        z-index: 3;
    }
    
    .server-info {
        position: absolute;
        bottom: 50px;
        left: 20px;
        z-index: 3;
        color: white;
        max-width: 60%;
    }
    
    .players-status {
        display: flex;
        align-items: center;
        gap: 8px;
        margin-bottom: 8px;
        font-size: 14px;
    }
    
    .status-indicator {
        width: 8px;
        height: 8px;
        border-radius: 50%;
        background: #10b981;
        box-shadow: 0 0 8px rgba(16, 185, 129, 0.6);
    }
    
    .minecraft-version {
        font-size: 16px;
        font-weight: 600;
        margin-bottom: 4px;
    }
    
    .pack-version-row {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 12px;
        width: 100%;
    }
    
    .pack-version {
        font-size: 14px;
        opacity: 0.9;
        flex-shrink: 0;
    }
    
    .update-badge {
        display: inline-block;
        background: #10b981;
        color: white;
        padding: 4px 12px;
        border-radius: 16px;
        font-size: 12px;
        font-weight: 600;
        flex-shrink: 0;
    }
    
    .join-btn {
        position: absolute;
        bottom: 50px;
        right: 20px;
        background: linear-gradient(135deg, #3b82f6, #1d4ed8);
        border: none;
        color: white;
        padding: 4px 16px;
        border-radius: 16px;
        font-weight: 600;
        cursor: pointer;
        font-size: 12px;
        transition: all 0.2s ease;
        z-index: 3;
        height: 24px;
        display: flex;
        align-items: center;
    }
    
    .join-btn:hover {
        transform: translateY(-2px);
        box-shadow: 0 8px 20px rgba(59, 130, 246, 0.4);
    }
    
    /* Community section styling */
    .community-section {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        text-align: center;
        z-index: 3;
        color: white;
    }
    
    .community-section h2 {
        font-size: 28px;
        font-weight: 700;
        margin-bottom: 20px;
        text-shadow: 0 2px 4px rgba(0,0,0,0.5);
    }
    
    .community-btn {
        display: block;
        background: rgba(255,255,255,0.1);
        border: 1px solid rgba(255,255,255,0.2);
        color: white;
        padding: 12px 24px;
        margin: 8px auto;
        border-radius: 8px;
        font-weight: 600;
        cursor: pointer;
        backdrop-filter: blur(10px);
        transition: all 0.2s ease;
        min-width: 150px;
    }
    
    .community-btn:hover {
        background: rgba(255,255,255,0.2);
        transform: translateY(-1px);
    }
    
    .community-btn.discord {
        background: #5865f2;
        border-color: #5865f2;
    }
    
    .mode-toggle {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 12px;
        margin-top: 20px;
    }
    
    .mode-label {
        font-weight: 700;
        font-size: 14px;
        opacity: 0.6;
        transition: opacity 0.2s ease;
    }
    
    .mode-label.active {
        opacity: 1;
        color: #10b981;
    }
    
    .toggle-switch {
        width: 40px;
        height: 20px;
        background: rgba(255,255,255,0.3);
        border: none;
        border-radius: 10px;
        position: relative;
        cursor: pointer;
        transition: all 0.2s ease;
    }
    
    .toggle-switch:hover {
        background: rgba(255,255,255,0.4);
    }
    
    .toggle-switch::after {
        content: '';
        position: absolute;
        width: 16px;
        height: 16px;
        background: white;
        border-radius: 50%;
        top: 2px;
        left: 2px;
        transition: transform 0.2s ease;
    }
    
    .toggle-switch.advanced::after {
        transform: translateX(20px);
    }
    
    /* Responsive adjustments */
    @media (max-width: 768px) {
        .diagonal-layout {
            flex-direction: column;
        }
        
        .server-panel:not(:last-child)::after {
            display: none;
        }
        
        .community-section h2 {
            font-size: 24px;
        }
    }
</style>
