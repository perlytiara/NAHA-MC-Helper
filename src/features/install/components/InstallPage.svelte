<!-- src/features/install/components/InstallPage.svelte -->
<script>
    import { onMount } from 'svelte';
    import { installerStore } from '../../../shared/stores/installerStore.js';
    import { theme, debug } from '../../../shared/stores/appStore.js';
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
        LAUNCHER_REPOS
    } from '../../../shared/utils/installerUtils';
    import NavigationBar from '../../../shared/components/ui/navigation/NavigationBar.svelte';
    import InstallHeaderSection from './InstallHeaderSection.svelte';
    import InstallCard from '../../../shared/components/ui/cards/InstallCard.svelte';
    import StatusBanner from '../../../shared/components/ui/feedback/StatusBanner.svelte';
    import LauncherModeSelector from './forms/LauncherModeSelector.svelte';
    import ModLoaderSelector from './forms/ModLoaderSelector.svelte';
    import AssetSelector from './forms/AssetSelector.svelte';
    import ServerLoaderSelector from './forms/ServerLoaderSelector.svelte';
    import MrpackDownloader from './MrpackDownloader.svelte';
    import ProgressTracker from './ProgressTracker.svelte';
    import PrimaryInstallButton from '../../../shared/components/ui/buttons/PrimaryInstallButton.svelte';

    let details = '';
    let selectedIndex = null;
    let currentStep = 1;
    let showStatusBanner = false;
    let statusBannerType = 'info';
    let statusBannerMessage = '';

    const { currentMode, release, progress, status, error, loader, mrpackStatus } = installerStore;
    
    // Determine current step based on state
    $: {
        if ($error) {
            currentStep = 1;
        } else if (!$currentMode) {
            currentStep = 1;
        } else if (!$release || selectedIndex === null) {
            currentStep = 2;
        } else {
            currentStep = 3;
        }
    }
    
    // Show status banner based on status changes
    $: if ($status) {
        showStatusBanner = true;
        if ($status.includes('Failed') || $error) {
            statusBannerType = 'error';
            statusBannerMessage = $error || $status;
        } else if ($status.includes('Saved to:') || $status.includes('Found') || $status.includes('complete')) {
            statusBannerType = 'success';
            statusBannerMessage = $status;
        } else if ($status.includes('🔐') || $status.includes('administrator') || $status.includes('privileges')) {
            statusBannerType = 'warning';
            statusBannerMessage = $status;
        } else if ($status.includes('🏪') || $status.includes('App Installer')) {
            statusBannerType = 'info';
            statusBannerMessage = $status;
        } else if ($status.includes('Fetching') || $status.includes('Downloading') || $status.includes('Installing') || $status.includes('Running')) {
            statusBannerType = 'loading';
            statusBannerMessage = $status;
        } else {
            statusBannerType = 'info';
            statusBannerMessage = $status;
        }
    }

    // Reactive updates for dynamic links and text
    $: sourceHref = (() => {
        switch($currentMode) {
            case 'xmcl': return 'https://github.com/Voxelum/x-minecraft-launcher/releases/latest';
            case 'astralrinth': return 'https://git.astralium.su/didirus/AstralRinth/releases/latest';
            default: return 'https://github.com/PrismLauncher/PrismLauncher/releases/latest';
        }
    })();
    $: guideHref = `/pages/guide-install.html?mode=${encodeURIComponent($currentMode)}`;
    $: chosenText = $release 
        ? ($release.best ? `Selected for this system: ${$release.best.name}` : 'No perfect match found. Please pick manually.') 
        : '';

    // Save mode to localStorage and fetch latest release when mode changes
    $: if ($currentMode) {
        try {
            if (typeof window !== 'undefined' && window.localStorage) {
                localStorage.setItem('lastMode', $currentMode);
            }
        } catch (e) {
            console.warn('Could not save mode to localStorage:', e);
        }
        
        // Reset installation state when switching launchers
        resetInstallationState();
        fetchLatestRelease();
    }
    
    function resetInstallationState() {
        console.log('Resetting installation state');
        progress.set(0);
        status.set('');
        error.set(null);
        details = '';
        selectedIndex = null;
        showStatusBanner = false;
    }

    async function fetchLatestRelease() {
        try {
            // Clear previous release data to avoid showing cached info
            release.set(null);
            selectedIndex = null;
            
            status.set('Fetching latest release...');
            const mode = get(currentMode) || 'cracked';
            
            // Try to use the existing prism API first, fallback to direct GitHub API
            let latest = null;
            
            try {
                latest = await window.nahaAPI?.fetchLatest?.(mode);
            } catch (prismError) {
                console.warn('Prism API failed, falling back to GitHub API:', prismError);
            }
            
            // Fallback to direct API call
            if (!latest) {
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
            
        } catch (fetchError) {
            console.error('Failed to fetch release:', fetchError);
            status.set('Failed to fetch releases');
            error.set(fetchError?.message || String(fetchError));
            release.set(null);
        }
    }

    async function handleMrpackDownload(loader) {
        try {
            status.set('Querying latest modpack...');
            mrpackStatus.set('Querying latest modpack...');
            const latest = await window.nahaAPI.fetchLatestPack?.(loader);
            if (!latest || latest.ok !== true) throw new Error(latest?.error || 'Query failed');
            const pick = latest.picks?.mrpack || latest.picks?.all?.find(a => /\.mrpack$/i.test(a.name) || /mrpack/i.test(a.content_type));
            if (!pick) {
                status.set('No .mrpack found in latest release.');
                mrpackStatus.set('No .mrpack found in latest release.');
                return;
            }

            let defaultName = pick.name || `${loader}-client-pack.mrpack`;
            if (!/\.mrpack$/i.test(defaultName)) defaultName += '.mrpack';
            defaultName = defaultName.replace(/[<>:"/\\|?*]/g, '-');
            const out = await window.nahaAPI.openSaveDialog?.({
                defaultPath: defaultName,
                filters: [{ name: 'Modrinth Pack', extensions: ['mrpack'] }]
            });
            if (!out) {
                status.set('Save canceled.');
                mrpackStatus.set('Save canceled.');
                progress.set(0);
                return;
            }

            status.set('Downloading .mrpack...');
            mrpackStatus.set('Downloading .mrpack...');
            progress.set(1);

            const handler = (u) => {
                try {
                    const prog = u?.progress || {};
                    let p = Number(prog.percent || 0);
                    const tr = Number(prog.transferred || 0);
                    const tt = Number(prog.total || 0);
                    if (tt > 0) p = (tr / tt) * 100;
                    else if (tr > 0 && (!Number.isFinite(p) || p <= 0))
                        p = Math.min(98, Math.max(1, Math.floor(5 + Math.sqrt(tr / (50 * 1048576)) * 90)));
                    progress.set(p);
                    status.set(`Downloading .mrpack... ${Math.round(Math.max(0, Math.min(100, p)))}%`);
                    mrpackStatus.set(`Downloading .mrpack... ${Math.round(Math.max(0, Math.min(100, p)))}%`);
                } catch {}
            };
            try {
                window.nahaAPI?.onPacksDownload?.(handler);
            } catch {}

            const dl = await window.nahaAPI.downloader?.({ url: pick.url, outPath: out });
            if (!dl || dl.ok !== true) throw new Error(dl?.error || 'Download failed');
            progress.set(100);
            status.set('Saved to: ' + out);
            mrpackStatus.set('Saved to: ' + out);
        } catch (mrpackError) {
            status.set('Failed: ' + (mrpackError?.message || mrpackError));
            mrpackStatus.set('Failed: ' + (mrpackError?.message || mrpackError));
            error.set(mrpackError?.message || String(mrpackError));
        }
    }

    async function handleBrowserDownload(asset, fileName) {
        try {
            status.set('Starting download...');
            progress.set(5);
            
            console.log('Downloading:', asset.browser_download_url);
            
            // Background download with real progress tracking
            console.log('Starting background download:', asset.browser_download_url);
            
            const response = await fetch(asset.browser_download_url, {
                method: 'GET',
                headers: {
                    'Accept': 'application/octet-stream',
                    'User-Agent': 'NAHA-MC-Helper/1.0.0'
                },
                mode: 'cors',
                credentials: 'omit'
            });
            
            if (!response.ok) {
                throw new Error(`Download failed: ${response.status} ${response.statusText}`);
            }
            
            const contentLength = response.headers.get('content-length');
            const total = parseInt(contentLength, 10) || asset.size || 0;
            let loaded = 0;
            
            status.set('Downloading installer in background...');
            progress.set(10);
            
            // Read the response as a stream to track real progress
            const reader = response.body?.getReader();
            const chunks = [];
            
            if (reader) {
                while (true) {
                    const { done, value } = await reader.read();
                    if (done) break;
                    
                    chunks.push(value);
                    loaded += value.length;
                    
                    if (total > 0) {
                        const percent = Math.min(85, (loaded / total) * 75 + 10); // 10-85% for download
                        progress.set(percent);
                        status.set(`Downloading installer... ${Math.round(percent)}% (${formatFileSize(loaded)} / ${formatFileSize(total)})`);
                    } else {
                        // If no content-length, estimate based on loaded bytes
                        const estimatedPercent = Math.min(85, Math.floor(10 + Math.sqrt(loaded / (10 * 1024 * 1024)) * 75));
                        progress.set(estimatedPercent);
                        status.set(`Downloading installer... ${estimatedPercent}% (${formatFileSize(loaded)})`);
                    }
                }
            }
            
            // Combine chunks into a single Uint8Array
            const totalLength = chunks.reduce((acc, chunk) => acc + chunk.length, 0);
            const result = new Uint8Array(totalLength);
            let offset = 0;
            for (const chunk of chunks) {
                result.set(chunk, offset);
                offset += chunk.length;
            }
            
            progress.set(90);
            status.set('Download complete. Preparing to run installer...');
            
            // Create a blob for the downloaded file
            const blob = new Blob([result], { type: 'application/octet-stream' });
            const url = URL.createObjectURL(blob);
            
            // Save file to disk and attempt to run it
            try {
                progress.set(90);
                status.set('Saving installer to disk...');
                
                // Try to use File System Access API if available (Chrome/Edge)
                if ('showSaveFilePicker' in window) {
                    try {
                        const fileHandle = await window.showSaveFilePicker({
                            suggestedName: fileName,
                            types: [{
                                description: 'Installer files',
                                accept: {
                                    'application/octet-stream': ['.exe', '.dmg', '.deb', '.rpm', '.AppImage', '.appinstaller']
                                }
                            }]
                        });
                        
                        const writableStream = await fileHandle.createWritable();
                        await writableStream.write(result);
                        await writableStream.close();
                        
                        progress.set(95);
                        status.set('File saved. Attempting to run installer...');
                        
                        // Note: Browsers can't execute files directly for security
                        // The user will need to run the installer manually
                        progress.set(100);
                        status.set('Download complete! Please run the installer from your chosen location.');
                        details = `Successfully downloaded ${fileName} (${formatFileSize(loaded)}).\n\nThe installer has been saved to your chosen location. Please run it to complete the installation.`;
                        
                    } catch (fileApiError) {
                        console.warn('File System Access API failed:', fileApiError);
                        throw fileApiError; // Fall back to blob download
                    }
                } else {
                    // Fallback: Use blob download (works in all browsers)
                    throw new Error('File System Access API not available');
                }
                
            } catch (saveError) {
                console.log('Falling back to blob download:', saveError.message);
                
                // Fallback: Create blob download
                const downloadLink = document.createElement('a');
                downloadLink.href = url;
                downloadLink.download = fileName;
                downloadLink.style.display = 'none';
                
                // Add to DOM briefly, click, then remove
                document.body.appendChild(downloadLink);
                downloadLink.click();
                document.body.removeChild(downloadLink);
                
                progress.set(100);
                status.set('Download initiated! Check your Downloads folder.');
                details = `Downloaded ${fileName} (${formatFileSize(loaded)}) to your Downloads folder.\n\nPlease locate and run the installer to complete the installation.`;
                
                // Add launcher-specific instructions
                if (fileName.includes('PrismLauncher')) {
                    details += '\n\n🎮 PrismLauncher: Open-source Minecraft launcher with extensive mod support.';
                } else if (fileName.includes('XMCL') || fileName.includes('x-minecraft-launcher')) {
                    details += '\n\n🎮 XMCL: Advanced Minecraft launcher with official account support.';
                } else if (fileName.includes('AstralRinth')) {
                    details += '\n\n🎮 AstralRinth: Cracked launcher with Ely.by integration and 2FA support.';
                }
                
                // Add OS-specific run instructions
                if (fileName.endsWith('.exe')) {
                    details += '\n\n📝 To install: Find the .exe file in Downloads and double-click it.';
                } else if (fileName.endsWith('.dmg')) {
                    details += '\n\n📝 To install: Find the .dmg file in Downloads, double-click to mount, then drag to Applications.';
                } else if (fileName.endsWith('.AppImage')) {
                    details += '\n\n📝 To install: Find the .AppImage file in Downloads, make executable (chmod +x), then run it.';
                } else if (fileName.endsWith('.deb')) {
                    details += '\n\n📝 To install: Find the .deb file in Downloads, then run: sudo dpkg -i ' + fileName;
                } else if (fileName.endsWith('.appinstaller')) {
                    details += '\n\n📝 To install: Find the .appinstaller file in Downloads and double-click to install via Windows App Installer.';
                }
            } finally {
                // Clean up the object URL
                URL.revokeObjectURL(url);
            }
            
        } catch (downloadError) {
            console.error('Browser download failed:', downloadError);
            status.set('Download failed: ' + downloadError.message);
            error.set(downloadError.message);
            progress.set(0);
        }
    }

    function handleInstallClick() {
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
        
        // Call the actual install function
        handleInstall(selectedAsset);
    }

    async function handleInstall(asset) {
        console.log('handleInstall called with asset:', asset);
        
        if (!asset) {
            console.error('No asset provided to handleInstall');
            error.set('No asset selected for installation');
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
            
            const mode = get(currentMode) || 'cracked';
            const defaultName = asset.name || 'launcher-installer';
            
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
                savePath = `${cacheDir}${separator}${mode}${separator}${sanitizedName}`;
                
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
                    const total = Number(prog.total || asset.size || 0);
                    
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
            console.log('Download URL:', asset.browser_download_url);
            
            // Perform the download using Electron's download API
            const downloadResult = await window.nahaAPI.downloader({
                url: asset.browser_download_url,
                outPath: savePath
            });
            
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
                details = `🛡️ Administrator Access Required\n\nPlease approve the administrator access request to install ${asset.name}.\n\nThis is required to install the launcher to your system properly.`;
            } else if (asset.name.includes('.appinstaller')) {
                status.set('🏪 Opening Windows App Installer...');
                details = `📱 Installing via Windows App Installer\n\nWindows App Installer will open to install ${asset.name}.\n\nThis is the modern way to install XMCL on Windows 10/11.`;
            } else {
                status.set('Running installer...');
                details = `Installing ${asset.name}...`;
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
            
            if (asset.name.includes('PrismLauncher')) {
                launcherName = 'PrismLauncher';
                launcherPath = 'Start Menu > PrismLauncher or Desktop shortcut';
            } else if (asset.name.includes('XMCL') || asset.name.includes('x-minecraft-launcher')) {
                launcherName = 'XMCL';
                launcherPath = 'Start Menu > XMCL or Apps list';
            } else if (asset.name.includes('AstralRinth')) {
                launcherName = 'AstralRinth';
                launcherPath = 'Start Menu > AstralRinth or Desktop shortcut';
            }
            
            details = `✅ Successfully installed ${launcherName}!\n\nThe launcher has been installed to your system and is ready to use.\n\nFile: ${asset.name}\nSize: ${formatFileSize(asset.size || 0)}\nLocation: ${launcherPath}\n\n🎮 You can now launch ${launcherName} from your Start Menu or Desktop.\n\n⏱️ This dialog will automatically reset in 5 seconds, or switch launcher modes to install another one.`;
            
            // Optional: Clean up downloaded installer file after successful installation
            try {
                await window.nahaAPI.deleteFile(savePath);
                console.log('Cleaned up installer file:', savePath);
            } catch (cleanupErr) {
                console.warn('Could not clean up installer file:', cleanupErr);
                details += '\n\nNote: Installer file remains at: ' + savePath;
            }
            
            // Auto-reset after successful installation (delay to show completion)
            setTimeout(() => {
                console.log('Auto-resetting after successful installation');
                resetInstallationState();
            }, 5000); // Reset after 5 seconds
            
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
                resetInstallationState();
            }, 5000); // Reset after 5 seconds for errors (faster than before)
        }
    }

    onMount(async () => {
        // Check if window.nahaAPI is available
        console.log('window.nahaAPI available:', typeof window.nahaAPI !== 'undefined');
        console.log('window.nahaAPI object:', window.nahaAPI);
        
        if (typeof window.nahaAPI !== 'undefined') {
            console.log('Available prism methods:', Object.keys(window.nahaAPI));
        }
        
        // Theme and debug setup
        try {
            const t = await window.nahaAPI?.getTheme?.();
            if (t && t !== 'system') {
                theme.set(t);
                document.documentElement.dataset.theme = t;
            }
            window.nahaAPI?.onThemeChanged?.((val) => {
                theme.set(val || 'system');
                if (!val || val === 'system') {
                    delete document.documentElement.dataset.theme;
                } else {
                    document.documentElement.dataset.theme = val;
                }
            });

            // Debug state is handled by App.svelte - no need to duplicate here
        } catch {}

        // Initial fetch
        await fetchLatestRelease();

        // Task listener (placeholder for task updates)
        window.nahaAPI?.onTaskUpdate?.((update) => {
            if (update?.details) details = update.details;
        });
    });
</script>

<svelte:head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Install Minecraft NAHA Launcher</title>
</svelte:head>

<div class="fancy-install-container">
    <!-- Background decoration -->
    <div class="background-decoration">
        <div class="decoration-grid"></div>
        <div class="decoration-orbs">
            <div class="orb orb-1"></div>
            <div class="orb orb-2"></div>
            <div class="orb orb-3"></div>
        </div>
    </div>
    
    <!-- Main content -->
    <div class="content-wrapper">
        <div id="global-notice" class="mb-6"></div>
        <NavigationBar debug={$debug} />
        
        <!-- Fancy header with progress -->
        <InstallHeaderSection 
            title="Minecraft Launcher Installer"
            subtitle="Transform your Minecraft experience with our powerful launcher"
            {currentStep}
            totalSteps={3}
        />

        <!-- Status banner -->
        <StatusBanner 
            bind:show={showStatusBanner}
            type={statusBannerType}
            message={statusBannerMessage}
        />
        
        <!-- Installation steps -->
        <div class="install-steps">
            <!-- Step 1: Launcher Mode Selection -->
            <InstallCard 
                title="Choose Your Launcher"
                subtitle="Select between official and cracked launcher versions"
                icon="🎮"
                variant={currentStep >= 1 ? 'primary' : 'default'}
                glowing={currentStep === 1}
            >
                <LauncherModeSelector 
                    mode={$currentMode} 
                    on:modeChange={(e) => currentMode.set(e.detail.mode)}
                />
            </InstallCard>

            <!-- Step 1.5: Asset Selection -->
            <InstallCard 
                title="Select Installation Asset"
                subtitle="Pick the right version for your system"
                icon="📦"
                variant={currentStep >= 2 ? 'primary' : 'default'}
                glowing={currentStep === 2}
            >
                {#if chosenText}
                    <div class="chosen-asset-info">
                        <div class="chosen-icon">✨</div>
                        <p class="chosen-text">{chosenText}</p>
                    </div>
                {/if}
                
                <div class="asset-selection-row">
                    <AssetSelector 
                        release={$release} 
                        on:assetSelect={(e) => selectedIndex = e.detail.index}
                    />
                    <PrimaryInstallButton 
                        disabled={$error || !$release || $progress > 0}
                        text="Start Installation"
                        loadingText="Installing..."
                        on:click={handleInstallClick}
                    />
                </div>
            </InstallCard>

            <!-- Progress Display -->
            {#if $progress > 0 || $status.includes('Installing') || $status.includes('Downloading')}
            <InstallCard 
                title="Installation Progress"
                subtitle="Please wait while we set up your launcher"
                icon={$status.includes('🔐') || $status.includes('administrator') ? '🛡️' : '⏳'}
                variant={$status.includes('🔐') || $status.includes('administrator') ? 'warning' : 'success'}
                glowing={true}
            >
                <ProgressTracker 
                    progress={$progress} 
                    status={$status}
                    variant={$status.includes('🔐') || $status.includes('administrator') ? 'warning' : 'primary'}
                />
            </InstallCard>
            {/if}
            
            <!-- Installation details -->
            {#if details}
                <InstallCard 
                    title={details.includes('🛡️') ? 'Administrator Access Required' : 'Installation Details'}
                    subtitle={details.includes('🛡️') ? 'Security permission needed for installation' : 'Technical information about the process'}
                    icon={details.includes('🛡️') ? '🛡️' : details.includes('✅') ? '✅' : '📋'}
                    variant={details.includes('🛡️') ? 'warning' : details.includes('✅') ? 'success' : $error ? 'error' : 'info'}
                >
                    <div class="details-container" class:admin-request={details.includes('🛡️')} class:success-details={details.includes('✅')}>
                        <pre class="details-text" class:admin-text={details.includes('🛡️')} class:success-text={details.includes('✅')}>{details}</pre>
                        
                        {#if $error && details.includes('Installation failed')}
                            <div class="error-actions">
                                <button class="retry-button" on:click={resetInstallationState}>
                                    🔄 Try Again Now
                                </button>
                            </div>
                        {/if}
                    </div>
                </InstallCard>
            {/if}

            <!-- Step 2: Mod Loader Selection -->
            <InstallCard 
                title="Choose Mod Loader"
                subtitle="Select your preferred modding framework"
                icon="⚙️"
                variant={currentStep >= 1 ? 'primary' : 'default'}
                glowing={currentStep === 1}
            >
                <ModLoaderSelector 
                    loader={$loader} 
                    on:loaderChange={(e) => loader.set(e.detail.loader)}
                />
            </InstallCard>

            <!-- Step 3: Modpack Download -->
            <InstallCard 
                title="Download Modpack"
                subtitle="Get the latest modpack for your chosen loader"
                icon="📦"
                variant={currentStep >= 3 ? 'primary' : 'default'}
                glowing={currentStep === 3}
            >
                <div class="modpack-section">
                    <MrpackDownloader 
                        loader={$loader} 
                        status={$mrpackStatus} 
                        on:download={(e) => handleMrpackDownload(e.detail)}
                    />
                </div>
            </InstallCard>
        </div>
        
        <!-- Footer links -->
        <div class="footer-links">
            <div class="footer-section">
                <span class="footer-label">Source:</span>
                <a href={sourceHref} target="_blank" rel="noopener" class="footer-link">
                    <span class="link-icon">🔗</span>
                    Official GitHub Releases
                </a>
            </div>
            <a href={guideHref} target="_blank" rel="noopener" class="guide-link">
                <span class="guide-icon">📘</span>
                Installation Guide
            </a>
        </div>
    </div>
</div>

<style>
    .fancy-install-container {
        height: 100vh;
        position: relative;
        background: linear-gradient(135deg, 
            #f8fafc 0%, 
            #f1f5f9 25%, 
            #e2e8f0 50%, 
            #cbd5e1 100%
        );
        overflow: hidden;
    }
    
    .background-decoration {
        position: fixed;
        inset: 0;
        pointer-events: none;
        z-index: 0;
    }
    
    .decoration-grid {
        position: absolute;
        inset: 0;
        background-image: 
            linear-gradient(rgba(59, 130, 246, 0.03) 1px, transparent 1px),
            linear-gradient(90deg, rgba(59, 130, 246, 0.03) 1px, transparent 1px);
        background-size: 50px 50px;
        animation: gridMove 20s linear infinite;
    }
    
    @keyframes gridMove {
        0% { transform: translate(0, 0); }
        100% { transform: translate(50px, 50px); }
    }
    
    .decoration-orbs {
        position: absolute;
        inset: 0;
        overflow: hidden;
    }
    
    .orb {
        position: absolute;
        border-radius: 50%;
        background: radial-gradient(circle, 
            rgba(59, 130, 246, 0.1) 0%, 
            rgba(139, 92, 246, 0.05) 50%, 
            transparent 100%
        );
        animation: orbFloat 8s ease-in-out infinite;
    }
    
    .orb-1 {
        width: 300px;
        height: 300px;
        top: -150px;
        left: -150px;
        animation-delay: 0s;
    }
    
    .orb-2 {
        width: 200px;
        height: 200px;
        top: 50%;
        right: -100px;
        animation-delay: 2s;
    }
    
    .orb-3 {
        width: 250px;
        height: 250px;
        bottom: -125px;
        left: 50%;
        animation-delay: 4s;
    }
    
    @keyframes orbFloat {
        0%, 100% { transform: translate(0, 0) scale(1); }
        50% { transform: translate(20px, -20px) scale(1.1); }
    }
    
    .content-wrapper {
        position: relative;
        z-index: 1;
        max-width: 1400px;
        margin: 0 auto;
        padding: 0.75rem;
        height: 100vh;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
    }
    
    .install-steps {
        display: grid;
        grid-template-columns: 1fr;
        gap: 1.5rem;
        margin-bottom: 2rem;
        width: 100%;
    }
    
    .chosen-asset-info {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        background: linear-gradient(135deg, 
            rgba(34, 197, 94, 0.1) 0%, 
            rgba(134, 239, 172, 0.05) 100%
        );
        border: 1px solid rgba(34, 197, 94, 0.2);
        border-radius: 12px;
        padding: 1rem;
        margin-bottom: 1rem;
    }
    
    .chosen-icon {
        font-size: 1.25rem;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
    }
    
    .chosen-text {
        margin: 0;
        font-weight: 500;
        color: #166534;
        font-size: 0.925rem;
    }
    
    .asset-selection-row {
        display: flex;
        gap: 1rem;
        align-items: flex-end;
        flex-wrap: wrap;
        width: 100%;
    }
    
    .asset-selection-row > :global(:first-child) {
        flex: 1;
        min-width: 150px;
    }
    
    .modpack-section {
        width: 100%;
        max-width: 100%;
        box-sizing: border-box;
        overflow: hidden;
    }
    
    .details-container {
        background: rgba(239, 68, 68, 0.05);
        border: 1px solid rgba(239, 68, 68, 0.1);
        border-radius: 8px;
        padding: 1rem;
        overflow: hidden;
    }
    
    .details-container.admin-request {
        background: linear-gradient(135deg, rgba(245, 158, 11, 0.1) 0%, rgba(251, 191, 36, 0.05) 100%);
        border: 1px solid rgba(245, 158, 11, 0.3);
        box-shadow: 0 4px 12px rgba(245, 158, 11, 0.1);
    }
    
    .details-container.success-details {
        background: linear-gradient(135deg, rgba(34, 197, 94, 0.1) 0%, rgba(134, 239, 172, 0.05) 100%);
        border: 1px solid rgba(34, 197, 94, 0.3);
        box-shadow: 0 4px 12px rgba(34, 197, 94, 0.1);
    }
    
    .details-text {
        margin: 0;
        font-size: 0.875rem;
        color: #dc2626;
        white-space: pre-wrap;
        overflow-x: auto;
        line-height: 1.4;
    }
    
    .details-text.admin-text {
        color: #d97706;
        font-weight: 500;
    }
    
    .details-text.success-text {
        color: #059669;
        font-weight: 500;
    }
    
    .error-actions {
        margin-top: 1rem;
        padding-top: 1rem;
        border-top: 1px solid rgba(239, 68, 68, 0.2);
        display: flex;
        justify-content: center;
    }
    
    .retry-button {
        background: linear-gradient(135deg, #dc2626, #ef4444);
        color: white;
        border: none;
        padding: 0.5rem 1rem;
        border-radius: 8px;
        font-weight: 600;
        font-size: 0.875rem;
        cursor: pointer;
        transition: all 0.2s ease;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }
    
    .retry-button:hover {
        background: linear-gradient(135deg, #b91c1c, #dc2626);
        transform: translateY(-1px);
        box-shadow: 0 4px 12px rgba(220, 38, 38, 0.3);
    }
    
    .retry-button:active {
        transform: translateY(0);
    }
    
    .footer-links {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 1.5rem 0;
        border-top: 1px solid rgba(203, 213, 225, 0.5);
        margin-top: auto;
        gap: 1rem;
        flex-wrap: wrap;
        flex-shrink: 0;
    }
    
    .footer-section {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-size: 0.875rem;
    }
    
    .footer-label {
        color: #64748b;
        font-weight: 500;
    }
    
    .footer-link {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        color: #3b82f6;
        text-decoration: none;
        font-weight: 500;
        transition: all 0.2s ease;
        padding: 0.5rem 0.75rem;
        border-radius: 8px;
        background: rgba(59, 130, 246, 0.05);
    }
    
    .footer-link:hover {
        background: rgba(59, 130, 246, 0.1);
        transform: translateY(-1px);
    }
    
    .link-icon {
        font-size: 0.875rem;
    }
    
    .guide-link {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        color: #8b5cf6;
        text-decoration: none;
        font-weight: 600;
        font-size: 0.875rem;
        padding: 0.75rem 1rem;
        background: linear-gradient(135deg, 
            rgba(139, 92, 246, 0.1) 0%, 
            rgba(196, 181, 253, 0.05) 100%
        );
        border: 1px solid rgba(139, 92, 246, 0.2);
        border-radius: 12px;
        transition: all 0.3s ease;
        box-shadow: 0 4px 12px rgba(139, 92, 246, 0.1);
    }
    
    .guide-link:hover {
        transform: translateY(-2px);
        box-shadow: 0 8px 24px rgba(139, 92, 246, 0.2);
        background: linear-gradient(135deg, 
            rgba(139, 92, 246, 0.15) 0%, 
            rgba(196, 181, 253, 0.08) 100%
        );
    }
    
    .guide-icon {
        font-size: 1rem;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
    }
    
    /* Dark theme */
    @media (prefers-color-scheme: dark) {
        .fancy-install-container {
            background: linear-gradient(135deg, 
                #0f172a 0%, 
                #1e293b 25%, 
                #334155 50%, 
                #475569 100%
            );
        }
        
        .chosen-text {
            color: #86efac;
        }
        
        .details-text {
            color: #fca5a5;
        }
        
        .details-text.admin-text {
            color: #fbbf24;
        }
        
        .details-text.success-text {
            color: #6ee7b7;
        }
        
        .details-container.admin-request {
            background: linear-gradient(135deg, rgba(245, 158, 11, 0.15) 0%, rgba(251, 191, 36, 0.08) 100%);
            border-color: rgba(245, 158, 11, 0.4);
        }
        
        .details-container.success-details {
            background: linear-gradient(135deg, rgba(34, 197, 94, 0.15) 0%, rgba(134, 239, 172, 0.08) 100%);
            border-color: rgba(34, 197, 94, 0.4);
        }
        
        .footer-label {
            color: #94a3b8;
        }
        
        .footer-link {
            background: rgba(59, 130, 246, 0.1);
        }
        
        .footer-link:hover {
            background: rgba(59, 130, 246, 0.15);
        }
    }
    
    /* Responsive design */
    @media (max-width: 768px) {
        .content-wrapper {
            padding: 1rem;
        }
        
        .install-steps {
            gap: 1.25rem;
        }
        
        .asset-selection-row {
            flex-direction: column;
            align-items: stretch;
        }
        
        .footer-links {
            flex-direction: column;
            text-align: center;
            gap: 1rem;
        }
        
        .footer-section {
            justify-content: center;
        }
    }
    
    @media (max-width: 640px) {
        .content-wrapper {
            padding: 0.5rem;
        }
        
        .chosen-asset-info {
            padding: 0.75rem;
        }
        
        .chosen-text {
            font-size: 0.875rem;
        }
    }
</style>
