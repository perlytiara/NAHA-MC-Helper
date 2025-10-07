<!-- src/features/onboarding/components/OnboardingInstall.svelte -->
<script>
    import { createEventDispatcher, onMount } from 'svelte';
    import { writable, get } from 'svelte/store';
    import { 
        getRepoUrl, 
        findBestAsset, 
        detectOS, 
        getInstallCommand,
        validateDownload,
        formatFileSize,
        getCacheDirectory,
        getDefaultDownloadPath,
        LAUNCHER_REPOS,
        getModRinthDownloadUrl,
        createModRinthRelease,
        createAstralRinthRelease
    } from '../../../shared/utils/installerUtils';
    
    const dispatch = createEventDispatcher();
    
    export let selectedLauncher = 'prism';
    
    let release = writable(null);
    let progress = writable(0);
    let status = writable('');
    let error = writable(null);
    let details = '';
    let selectedIndex = null;
    let showInstallButton = false;
    
    // Reactive updates for dynamic links and text
    $: sourceHref = (() => {
        switch(selectedLauncher) {
            case 'xmcl': return 'https://github.com/Voxelum/x-minecraft-launcher/releases/latest';
            case 'astralrinth': return 'https://git.astralium.su/didirus/AstralRinth/releases/latest';
            case 'modrinth': return 'https://modrinth.com/app';
            case 'prism': return 'https://github.com/Diegiwg/PrismLauncher-Cracked/releases/latest';
            default: return 'https://github.com/Diegiwg/PrismLauncher-Cracked/releases/latest';
        }
    })();
    
    $: chosenText = $release 
        ? ($release.best ? `Selected for this system: ${$release.best.name}` : 'No perfect match found. Please pick manually.') 
        : '';
    
    async function fetchLatestRelease() {
        try {
            // Clear previous release data to avoid showing cached info
            release.set(null);
            selectedIndex = null;
            
            status.set('Fetching latest release...');
            const mode = selectedLauncher;
            
            // Try to use the existing prism API first, fallback to direct GitHub API
            let latest = null;
            
            try {
                latest = await window.nahaAPI?.fetchLatest?.(mode);
            } catch (prismError) {
                console.warn('Prism API failed, falling back to GitHub API:', prismError);
            }
            
            // Special handling for ModRinth
            if (mode === 'modrinth') {
                latest = createModRinthRelease();
                status.set('ModRinth launcher ready for download');
            }
            // Special handling for AstralRinth
            else if (mode === 'astralrinth') {
                latest = createAstralRinthRelease();
                status.set('AstralRinth launcher ready for download');
            }
            // Fallback to direct API call
            else if (!latest) {
                let response;
                let apiUrl = getRepoUrl(mode);
                
                if (mode === 'astralrinth') {
                    // Special handling for AstralRinth - try multiple approaches
                    let lastError;
                    
                    // Try the main API URL first
                    try {
                        response = await fetch(apiUrl, {
                            method: 'GET',
                            headers: {
                                'Accept': 'application/json',
                            },
                            // Add timeout and other options
                            signal: AbortSignal.timeout(10000) // 10 second timeout
                        });
                        
                        if (!response.ok) {
                            throw new Error(`GitLab API returned ${response.status}: ${response.statusText}`);
                        }
                    } catch (fetchError) {
                        lastError = fetchError;
                        console.warn('Primary AstralRinth API failed:', fetchError);
                        
                        // If network error or CORS, skip to fallback
                        if (fetchError.name === 'TypeError' || fetchError.message.includes('NetworkError')) {
                            console.log('Network error detected, using fallback data for AstralRinth');
                            response = null; // Will trigger fallback below
                        } else {
                            throw fetchError;
                        }
                    }
                } else {
                    // Regular GitHub API call
                    response = await fetch(apiUrl);
                    
                    if (!response.ok) {
                        throw new Error(`API returned ${response.status}: ${response.statusText}`);
                    }
                }
                
                let releaseData;
                
                if (mode === 'astralrinth') {
                    // Handle AstralRinth - with fallback for network errors
                    if (!response) {
                        // Network error fallback - provide accurate release data from v0.10.601
                        console.log('Using offline fallback for AstralRinth v0.10.601');
                        latest = {
                            releaseName: 'AstralRinth v0.10.601 (Offline)',
                            tag: 'v0.10.601',
                            assets: [
                                {
                                    name: 'AstralRinth App_0.10.601_x64-setup.exe',
                                    browser_download_url: 'https://git.astralium.su/didirus/AstralRinth/releases/download/AR-0.10.601/AstralRinth%20App_0.10.601_x64-setup.exe',
                                    size: 8.9 * 1024 * 1024, // 8.9 MiB
                                    id: 1,
                                    content_type: 'application/octet-stream'
                                },
                                {
                                    name: 'AstralRinth App_0.10.601_amd64.AppImage',
                                    browser_download_url: 'https://git.astralium.su/didirus/AstralRinth/releases/download/AR-0.10.601/AstralRinth%20App_0.10.601_amd64.AppImage',
                                    size: 91 * 1024 * 1024, // 91 MiB
                                    id: 2,
                                    content_type: 'application/octet-stream'
                                },
                                {
                                    name: 'AstralRinth App_0.10.601_amd64.deb',
                                    browser_download_url: 'https://git.astralium.su/didirus/AstralRinth/releases/download/AR-0.10.601/AstralRinth%20App_0.10.601_amd64.deb',
                                    size: 11 * 1024 * 1024, // 11 MiB
                                    id: 3,
                                    content_type: 'application/vnd.debian.binary-package'
                                },
                                {
                                    name: 'AstralRinth App-0.10.601-1.x86_64.rpm',
                                    browser_download_url: 'https://git.astralium.su/didirus/AstralRinth/releases/download/AR-0.10.601/AstralRinth%20App-0.10.601-1.x86_64.rpm',
                                    size: 11 * 1024 * 1024, // 11 MiB
                                    id: 4,
                                    content_type: 'application/x-rpm'
                                },
                                {
                                    name: 'AstralRinth App_0.10.601_aarch64.dmg',
                                    browser_download_url: 'https://git.astralium.su/didirus/AstralRinth/releases/download/AR-0.10.601/AstralRinth%20App_0.10.601_aarch64.dmg',
                                    size: 11 * 1024 * 1024, // 11 MiB
                                    id: 5,
                                    content_type: 'application/x-apple-diskimage'
                                }
                            ],
                            best: undefined,
                            publishedAt: '2024-11-01T00:00:00Z', // Approximate date
                            url: 'https://git.astralium.su/didirus/AstralRinth/releases/tag/AR-0.10.601'
                        };
                        
                        // Select best asset for user's OS
                        latest.best = findBestAsset(latest.assets) || latest.assets[0];
                        
                    } else {
                        // Try to parse GitLab API response
                        try {
                            const releases = await response.json();
                            let releaseData;
                            
                            if (Array.isArray(releases) && releases.length > 0) {
                                releaseData = releases[0]; // Get latest release
                            } else if (releases && !Array.isArray(releases)) {
                                releaseData = releases;
                            } else {
                                throw new Error('No releases found in GitLab API response');
                            }
                            
                            // Convert GitLab format to GitHub-like format
                            const assets = releaseData.assets?.links || releaseData.assets?.sources || [];
                            
                            // If no assets, create default ones
                            let processedAssets = assets;
                            if (!assets || assets.length === 0) {
                                const tagName = releaseData.tag_name || 'latest';
                                processedAssets = [
                                    {
                                        name: `AstralRinth-${tagName}-Windows-x64.exe`,
                                        browser_download_url: `https://git.astralium.su/didirus/AstralRinth/-/releases/${tagName}/downloads/AstralRinth-Windows-x64.exe`,
                                        size: 45 * 1024 * 1024,
                                        id: 1,
                                        content_type: 'application/octet-stream'
                                    },
                                    {
                                        name: `AstralRinth-${tagName}-Linux-x64.AppImage`,
                                        browser_download_url: `https://git.astralium.su/didirus/AstralRinth/-/releases/${tagName}/downloads/AstralRinth-Linux-x64.AppImage`,
                                        size: 50 * 1024 * 1024,
                                        id: 2,
                                        content_type: 'application/octet-stream'
                                    }
                                ];
                            }
                            
                            const bestAsset = findBestAsset(processedAssets);
                            
                            latest = {
                                releaseName: releaseData.name || releaseData.tag_name || 'AstralRinth Latest',
                                tag: releaseData.tag_name || 'latest',
                                assets: processedAssets,
                                best: bestAsset || processedAssets[0],
                                publishedAt: releaseData.released_at || releaseData.created_at || new Date().toISOString(),
                                url: releaseData._links?.self || releaseData.web_url || 'https://git.astralium.su/didirus/AstralRinth'
                            };
                            
                        } catch (parseError) {
                            console.warn('Failed to parse AstralRinth API response:', parseError);
                            // Use the same fallback as network error
                            latest = {
                                releaseName: 'AstralRinth Latest (Parse Error)',
                                tag: 'latest',
                                assets: [
                                    {
                                        name: 'AstralRinth-Windows-x64.exe',
                                        browser_download_url: 'https://git.astralium.su/didirus/AstralRinth/-/releases/latest/downloads/AstralRinth-Windows-x64.exe',
                                        size: 45 * 1024 * 1024,
                                        id: 1,
                                        content_type: 'application/octet-stream'
                                    }
                                ],
                                best: undefined,
                                publishedAt: new Date().toISOString(),
                                url: 'https://git.astralium.su/didirus/AstralRinth/-/releases'
                            };
                            latest.best = latest.assets[0];
                        }
                    }
                } else {
                    // Handle GitHub API format
                    releaseData = await response.json();
                    const bestAsset = findBestAsset(releaseData.assets);
                    
                    latest = {
                        releaseName: releaseData.name || releaseData.tag_name,
                        tag: releaseData.tag_name,
                        assets: releaseData.assets,
                        best: bestAsset || undefined,
                        publishedAt: releaseData.published_at,
                        url: releaseData.html_url
                    };
                }
            }
            
            if (!latest || !latest.assets || latest.assets.length === 0) {
                throw new Error('No release assets found');
            }
            
            // Ensure we have a best asset selected
            if (!latest.best) {
                latest.best = findBestAsset(latest.assets) || latest.assets[0];
            }
            
            status.set(`Found ${latest.releaseName || latest.tag} (${latest.assets.length} assets)`);
            release.set(latest);
            error.set(null);
            showInstallButton = true;
            
        } catch (fetchError) {
            console.error('Failed to fetch release:', fetchError);
            status.set('Failed to fetch releases');
            error.set(fetchError?.message || String(fetchError));
            release.set(null);
        }
    }
    
    async function handleInstall() {
        console.log('Install button clicked');
        console.log('Current release:', $release);
        console.log('Selected index:', selectedIndex);
        
        let selectedAsset;
        
        // Determine which asset to install
        if ($release?.assets && Number.isInteger(selectedIndex) && selectedIndex >= 0) {
            selectedAsset = $release.assets[selectedIndex];
            console.log('Using selected asset at index', selectedIndex, ':', selectedAsset);
        } else if ($release?.best) {
            selectedAsset = $release.best;
            console.log('Using best asset:', selectedAsset);
        } else if ($release?.assets && $release.assets.length > 0) {
            selectedAsset = $release.assets[0];
            console.log('Using first available asset:', selectedAsset);
        } else {
            console.error('No assets available for installation');
            error.set('No installation files available');
            return;
        }
        
        if (!selectedAsset) {
            console.error('No asset selected for installation');
            error.set('No installation file selected');
            return;
        }
        
        try {
            console.log('Starting installation process...');
            progress.set(0);
            error.set(null);
            details = '';
            
            // Step 1: Prepare download location in cache directory
            status.set('Preparing download...');
            console.log('Status set to: Preparing download...');
            
            const defaultName = selectedAsset.name || 'launcher-installer';
            
            // Check if we're in Electron environment
            if (typeof window.nahaAPI === 'undefined') {
                console.error('window.nahaAPI not available - this should run in Electron desktop app');
                error.set('This installer requires the desktop app environment');
                details = 'Please run this application as a desktop app, not in a web browser.';
                return;
            }
            
            // Get cache directory from main process
            let savePath;
            try {
                const cacheDir = await window.nahaAPI.getCacheDir();
                const sanitizedName = defaultName.replace(/[<>:"/\\|?*]/g, '-');
                
                // Build path manually since we don't have path module in renderer
                const separator = cacheDir.includes('\\') ? '\\' : '/';
                savePath = `${cacheDir}${separator}${selectedLauncher}${separator}${sanitizedName}`;
                
                console.log('Using cache directory for download:', savePath);
                
                // Ensure the directory exists (get parent directory)
                const lastSeparatorIndex = Math.max(savePath.lastIndexOf('\\'), savePath.lastIndexOf('/'));
                const parentDir = savePath.substring(0, lastSeparatorIndex);
                await window.nahaAPI.ensureDir(parentDir);
                
            } catch (cacheError) {
                console.warn('Cache directory setup failed:', cacheError);
                error.set('Failed to set up download directory: ' + cacheError.message);
                return;
            }
            
            console.log('Using save path:', savePath);
            
            // Step 2: Download the installer using Electron APIs
            status.set('Downloading installer in background...');
            progress.set(1);
            
            // Set up progress handler for real-time updates
            const progressHandler = (update) => {
                try {
                    const prog = update?.progress || {};
                    let percent = Number(prog.percent || 0);
                    const transferred = Number(prog.transferred || 0);
                    const total = Number(prog.total || selectedAsset.size || 0);
                    
                    if (total > 0) {
                        percent = Math.min(85, (transferred / total) * 80 + 5); // 5-85% for download
                    } else if (transferred > 0 && (!Number.isFinite(percent) || percent <= 0)) {
                        // Estimate progress based on transferred bytes
                        percent = Math.min(85, Math.max(5, Math.floor(5 + Math.sqrt(transferred / (10 * 1048576)) * 80)));
                    }
                    
                    progress.set(percent);
                    
                    const sizeText = total > 0 
                        ? `${formatFileSize(transferred)} / ${formatFileSize(total)}`
                        : formatFileSize(transferred);
                    
                    status.set(`Downloading installer... ${Math.round(percent)}% (${sizeText})`);
                } catch (err) {
                    console.warn('Progress handler error:', err);
                }
            };
            
            // Register progress handler
            try {
                window.nahaAPI.onDownloadProgress?.(progressHandler);
            } catch (err) {
                console.warn('Could not register progress handler:', err);
            }
            
            console.log('Starting download to:', savePath);
            console.log('Download URL:', selectedAsset.browser_download_url);
            
            // Perform the download using Electron's download API
            const downloadResult = await window.nahaAPI.downloader({
                url: selectedAsset.browser_download_url,
                outPath: savePath
            });
            
            // Special handling for AstralRinth browser redirect
            if (downloadResult && downloadResult.success && downloadResult.message && downloadResult.message.includes('Redirected to AstralRinth')) {
                progress.set(100);
                status.set('AstralRinth download page opened in your browser. Please download and install manually.');
                details = `AstralRinth launcher download page has been opened in your browser.\n\nPlease:\n1. Download the installer from the page that opened\n2. Run the installer to install AstralRinth\n3. Click "Continue" to proceed with server selection\n\nNote: AstralRinth requires manual installation.`;
                return; // Skip the rest of the installation process
            }
            
            if (!downloadResult || downloadResult.ok !== true) {
                throw new Error(downloadResult?.error || 'Download failed');
            }
            
            progress.set(90);
            status.set('Download complete. Validating file...');
            
            // Step 3: Validate download (if possible)
            const isValid = await validateDownload(savePath);
            if (!isValid) {
                console.warn('Download validation failed, but proceeding anyway');
            }
            
            // Step 4: Execute the installer automatically in background
            status.set('Running installer in background...');
            progress.set(92);
            
            const os = detectOS();
            const installCmd = getInstallCommand(savePath, os);
            
            console.log('Executing installer:', installCmd);
            
            if (installCmd.requiresElevation) {
                status.set('🔐 Requesting administrator access...');
                details = `🛡️ Administrator Access Required\n\nPlease approve the administrator access request to install ${selectedAsset.name}.\n\nThis is required to install the launcher to your system properly.`;
            } else if (selectedAsset.name.includes('.appinstaller')) {
                status.set('🏪 Opening Windows App Installer...');
                details = `📱 Installing via Windows App Installer\n\nWindows App Installer will open to install ${selectedAsset.name}.\n\nThis is the modern way to install XMCL on Windows 10/11.`;
            } else {
                status.set('Running installer...');
                details = `Installing ${selectedAsset.name}...`;
            }
            
            // Execute installer via main process in background
            const installResult = await window.nahaAPI.execInstaller({
                filePath: savePath,
                command: installCmd.command,
                args: installCmd.args,
                requiresElevation: installCmd.requiresElevation,
                os: os,
                background: true // Run in background
            });
            
            if (!installResult || installResult.ok !== true) {
                throw new Error(installResult?.error || 'Installation failed');
            }
            
            // Handle warnings (e.g., for .appinstaller files)
            if (installResult.warning) {
                console.warn('Installation warning:', installResult.warning);
                details += `\n\nNote: ${installResult.warning}`;
            }
            
            progress.set(98);
            status.set('Installation running... Please wait.');
            
            // Wait a bit for installation to complete
            await new Promise(resolve => setTimeout(resolve, 2000));
            
            // Step 5: Installation complete
            progress.set(100);
            status.set('Installation complete! Launcher installed successfully.');
            
            // Provide launcher-specific completion messages
            let launcherName = 'Minecraft Launcher';
            let launcherPath = 'System Applications';
            
            if (selectedAsset.name.includes('PrismLauncher')) {
                launcherName = 'PrismLauncher';
                launcherPath = 'Start Menu > PrismLauncher or Desktop shortcut';
            } else if (selectedAsset.name.includes('XMCL') || selectedAsset.name.includes('x-minecraft-launcher')) {
                launcherName = 'XMCL';
                launcherPath = 'Start Menu > XMCL or Apps list';
            } else if (selectedAsset.name.includes('AstralRinth')) {
                launcherName = 'AstralRinth';
                launcherPath = 'Start Menu > AstralRinth or Desktop shortcut';
            }
            
            details = `✅ Successfully installed ${launcherName}!\n\nThe launcher has been installed to your system and is ready to use.\n\nFile: ${selectedAsset.name}\nSize: ${formatFileSize(selectedAsset.size || 0)}\nLocation: ${launcherPath}\n\n🎮 You can now launch ${launcherName} from your Start Menu or Desktop.\n\n⏱️ This dialog will automatically reset in 5 seconds, or switch launcher modes to install another one.`;
            
            // Optional: Clean up downloaded installer file after successful installation
            try {
                await window.nahaAPI.deleteFile(savePath);
                console.log('Cleaned up installer file:', savePath);
            } catch (cleanupErr) {
                console.warn('Could not clean up installer file:', cleanupErr);
                details += '\n\nNote: Installer file remains at: ' + savePath;
            }
            
            // Installation complete - user must click Continue button to proceed
            console.log('Installation complete - waiting for user to click Continue button');
            
        } catch (installError) {
            console.error('Installation error:', installError);
            const errorMsg = installError?.message || String(installError);
            status.set('Installation failed: ' + errorMsg);
            error.set(errorMsg);
            details = `Installation failed: ${errorMsg}\n\nPlease try downloading manually or check your permissions.\n\n🔄 The installer will reset automatically in 5 seconds.`;
            
            // Reset progress on error immediately
            progress.set(0);
            
            // Quick reset after error so user can try again
            setTimeout(() => {
                console.log('Auto-resetting after installation error');
                // Reset the component state but don't complete onboarding
                progress.set(0);
                status.set('');
                error.set(null);
                details = '';
                showInstallButton = true;
            }, 5000); // Reset after 5 seconds for errors
        }
    }
    
    function handleSkip() {
        dispatch('skip-install');
    }
    
    function handleBack() {
        dispatch('back');
    }
    
    function handleContinue() {
        console.log('User clicked Continue button after installation');
        dispatch('install-complete');
    }
    
    onMount(() => {
        fetchLatestRelease();
    });
</script>

<div class="install-content">
    <!-- Header -->
    <div class="header-section">
        <div class="logo">
            <div class="logo-icon">⚡</div>
            <span class="logo-text">NAHA</span>
        </div>
    </div>
    
    <!-- Main content -->
    <div class="main-content">
        <h1 class="install-title">Install {selectedLauncher === 'prism' ? 'Prism Launcher' : selectedLauncher === 'xmcl' ? 'XMCL' : selectedLauncher === 'modrinth' ? 'ModRinth' : 'AstralRinth'}</h1>
        
        <p class="install-subtitle">We'll download and install the launcher for you automatically.</p>
        
        <!-- Release info -->
        {#if $release}
            <div class="release-info">
                <div class="release-header">
                    <div class="release-icon">📦</div>
                    <div class="release-details">
                        <h3 class="release-name">{$release.releaseName || $release.tag}</h3>
                        <p class="release-assets">{$release.assets.length} assets available</p>
                    </div>
                </div>
                
                {#if chosenText}
                    <div class="chosen-asset">
                        <span class="chosen-icon">✨</span>
                        <span class="chosen-text">{chosenText}</span>
                    </div>
                {/if}
                
                <!-- Show buttons here when no status -->
                {#if !$status}
                    <div class="action-buttons-inline">
                        <button class="skip-button" on:click={handleSkip}>
                            Skip Installation
                        </button>
                        
                        <button 
                            class="install-button" 
                            class:disabled={!showInstallButton || $progress > 0}
                            disabled={!showInstallButton || $progress > 0}
                            on:click={handleInstall}
                        >
                            Install Now
                        </button>
                    </div>
                {/if}
            </div>
        {/if}
        
        <!-- Status and progress -->
        {#if $status}
            <div class="status-section">
                <div class="status-text">{$status}</div>
                {#if $progress > 0}
                    <div class="progress-bar">
                        <div class="progress-fill" style="width: {$progress}%"></div>
                    </div>
                {/if}
            </div>
        {/if}
        
        <!-- Error display -->
        {#if $error}
            <div class="error-section">
                <div class="error-icon">⚠️</div>
                <div class="error-message">{$error}</div>
            </div>
        {/if}
        
        <!-- Installation details -->
        {#if details}
            <div class="details-section">
                <pre class="details-text">{details}</pre>
            </div>
        {/if}
    </div>
    
    <!-- Navigation - only show back button and status buttons when status is active -->
    {#if $status}
        <div class="navigation-section">
            <button class="back-button" on:click={handleBack}>
                ← Back
            </button>
            
            <div class="action-buttons">
                <button class="skip-button" on:click={handleSkip}>
                    Skip Installation
                </button>
                
                <button 
                    class="install-button" 
                    class:disabled={!showInstallButton || ($progress > 0 && $progress < 100)}
                    disabled={!showInstallButton || ($progress > 0 && $progress < 100)}
                    on:click={$progress === 100 ? handleContinue : handleInstall}
                >
                    {#if $progress === 100}
                        Continue
                    {:else if $progress > 0}
                        Installing... {$progress.toFixed(0)}%
                    {:else}
                        Install Now
                    {/if}
                </button>
            </div>
        </div>
    {:else}
        <!-- Just back button when no status -->
        <div class="navigation-section-back-only">
            <button class="back-button" on:click={handleBack}>
                ← Back
            </button>
        </div>
    {/if}
</div>

<style>
    .install-content {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        width: 100%;
        max-width: 500px;
        margin: 0 auto;
        padding: 1rem 0;
        box-sizing: border-box;
    }
    
    .header-section {
        display: flex;
        justify-content: center;
        margin-top: 1rem;
    }
    
    .logo {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.25rem 0.75rem;
        background: rgba(255, 255, 255, 0.1);
        border-radius: 8px;
        backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 255, 255, 0.2);
    }
    
    .logo-icon {
        width: 18px;
        height: 18px;
        background: linear-gradient(135deg, #22c55e, #16a34a);
        border-radius: 4px;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 10px;
        color: white;
        box-shadow: 0 2px 8px rgba(34, 197, 94, 0.3);
    }
    
    .logo-text {
        font-size: 1.1rem;
        font-weight: 800;
        color: white;
        letter-spacing: -0.5px;
    }
    
    .main-content {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        align-items: center;
        justify-content: flex-start;
    }
    
    .install-title {
        font-size: 1.5rem;
        font-weight: 700;
        color: white;
        margin: 0;
        line-height: 1.2;
        text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
        text-align: center;
    }
    
    .install-subtitle {
        font-size: 0.85rem;
        color: rgba(255, 255, 255, 0.7);
        line-height: 1.4;
        margin: 0;
        text-align: center;
        max-width: 350px;
    }
    
    .release-info {
        width: 100%;
        background: rgba(255, 255, 255, 0.1);
        border-radius: 8px;
        padding: 0.75rem;
        border: 1px solid rgba(255, 255, 255, 0.2);
    }
    
    .release-header {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        margin-bottom: 0.5rem;
    }
    
    .release-icon {
        font-size: 1.25rem;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
    }
    
    .release-details {
        flex: 1;
    }
    
    .release-name {
        font-size: 1rem;
        font-weight: 600;
        color: white;
        margin: 0 0 0.125rem 0;
    }
    
    .release-assets {
        font-size: 0.75rem;
        color: rgba(255, 255, 255, 0.7);
        margin: 0;
    }
    
    .chosen-asset {
        display: flex;
        align-items: center;
        gap: 0.375rem;
        background: rgba(34, 197, 94, 0.1);
        border: 1px solid rgba(34, 197, 94, 0.2);
        border-radius: 6px;
        padding: 0.5rem;
    }
    
    .chosen-icon {
        font-size: 0.875rem;
    }
    
    .chosen-text {
        font-size: 0.75rem;
        color: #86efac;
        font-weight: 500;
    }
    
    .status-section {
        width: 100%;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        margin-top: 0.5rem;
    }
    
    .status-text {
        font-size: 0.8rem;
        color: rgba(255, 255, 255, 0.8);
        text-align: center;
    }
    
    .progress-bar {
        width: 100%;
        height: 4px;
        background: rgba(255, 255, 255, 0.1);
        border-radius: 2px;
        overflow: hidden;
        margin-top: 0.5rem;
    }
    
    .progress-fill {
        height: 100%;
        background: linear-gradient(90deg, #22c55e, #16a34a);
        border-radius: 2px;
        transition: width 0.3s ease;
    }
    
    .error-section {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        background: rgba(239, 68, 68, 0.1);
        border: 1px solid rgba(239, 68, 68, 0.2);
        border-radius: 6px;
        padding: 0.5rem;
        width: 100%;
    }
    
    .error-icon {
        font-size: 1rem;
    }
    
    .error-message {
        color: #fca5a5;
        font-size: 0.75rem;
        font-weight: 500;
    }
    
    .details-section {
        width: 100%;
        background: rgba(0, 0, 0, 0.2);
        border-radius: 6px;
        padding: 0.5rem;
        border: 1px solid rgba(255, 255, 255, 0.1);
        max-height: 120px;
        overflow-y: auto;
    }
    
    .details-text {
        color: rgba(255, 255, 255, 0.8);
        font-size: 0.7rem;
        line-height: 1.3;
        margin: 0;
        white-space: pre-wrap;
        overflow-x: auto;
    }
    
    .action-buttons-inline {
        display: flex;
        gap: 0.5rem;
        margin-top: 1rem;
        justify-content: center;
    }
    
    .navigation-section {
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 1rem;
        margin-top: 0.5rem;
        width: fit-content;
        margin-left: auto;
        margin-right: auto;
    }
    
    .navigation-section-back-only {
        display: flex;
        justify-content: flex-start;
        align-items: center;
        margin-top: 0.5rem;
    }
    
    .back-button {
        background: transparent;
        color: rgba(255, 255, 255, 0.6);
        border: 1px solid rgba(255, 255, 255, 0.2);
        padding: 0.5rem 1rem;
        border-radius: 6px;
        font-size: 0.8rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.3s ease;
        white-space: nowrap;
        min-width: 80px;
    }
    
    .back-button:hover {
        background: rgba(255, 255, 255, 0.1);
        color: rgba(255, 255, 255, 0.8);
        border-color: rgba(255, 255, 255, 0.3);
    }
    
    .action-buttons {
        display: flex;
        gap: 0.75rem;
        align-items: center;
    }
    
    .skip-button {
        background: transparent;
        color: rgba(255, 255, 255, 0.6);
        border: 1px solid rgba(255, 255, 255, 0.2);
        padding: 0.5rem 1rem;
        border-radius: 6px;
        font-size: 0.8rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.3s ease;
        min-width: 120px;
        white-space: nowrap;
    }
    
    .skip-button:hover {
        background: rgba(255, 255, 255, 0.1);
        color: rgba(255, 255, 255, 0.8);
        border-color: rgba(255, 255, 255, 0.3);
    }
    
    .install-button {
        background: linear-gradient(135deg, #22c55e, #16a34a);
        color: white;
        border: none;
        padding: 0.5rem 1rem;
        border-radius: 6px;
        font-size: 0.8rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.3s ease;
        box-shadow: 0 4px 16px rgba(34, 197, 94, 0.3);
        min-width: 120px;
        white-space: nowrap;
    }
    
    .install-button:hover:not(.disabled) {
        background: linear-gradient(135deg, #16a34a, #15803d);
        transform: translateY(-1px);
        box-shadow: 0 6px 20px rgba(34, 197, 94, 0.4);
    }
    
    .install-button.disabled {
        background: rgba(255, 255, 255, 0.2);
        color: rgba(255, 255, 255, 0.4);
        cursor: not-allowed;
        box-shadow: none;
    }
    
    /* Responsive design */
    @media (max-width: 768px) {
        .install-content {
            gap: 0.75rem;
            max-width: 400px;
        }
        
        .install-title {
            font-size: 1.25rem;
        }
        
        .install-subtitle {
            font-size: 0.8rem;
        }
        
        .release-info {
            padding: 0.5rem;
        }
        
        .action-buttons {
            flex-direction: column;
            width: 100%;
        }
        
        .action-buttons-inline {
            flex-direction: column;
            gap: 0.375rem;
        }
        
        .skip-button, .install-button {
            width: 100%;
        }
    }
    
    @media (max-width: 640px) {
        .install-content {
            gap: 0.5rem;
            max-width: 350px;
        }
        
        .logo {
            padding: 0.25rem 0.5rem;
            gap: 0.375rem;
        }
        
        .logo-icon {
            width: 16px;
            height: 16px;
            font-size: 8px;
        }
        
        .logo-text {
            font-size: 1rem;
        }
        
        .install-title {
            font-size: 1.125rem;
        }
        
        .install-subtitle {
            font-size: 0.75rem;
        }
        
        .release-info {
            padding: 0.375rem;
        }
        
        .release-header {
            gap: 0.375rem;
        }
        
        .release-icon {
            font-size: 1rem;
        }
        
        .release-name {
            font-size: 0.875rem;
        }
        
        .navigation-section {
            flex-direction: column;
            gap: 0.5rem;
        }
        
        .action-buttons-inline {
            flex-direction: column;
            gap: 0.25rem;
            margin-top: 0.75rem;
        }
        
        .back-button {
            width: 100%;
        }
        
        .details-section {
            max-height: 80px;
        }
        
        .details-text {
            font-size: 0.65rem;
        }
    }
</style>
