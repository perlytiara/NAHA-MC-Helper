// Installer utilities for cross-platform launcher installation
export interface ReleaseAsset {
    name: string;
    browser_download_url: string;
    size: number;
    content_type?: string;
    id: number;
}

export interface GitHubRelease {
    tag_name: string;
    name: string;
    body: string;
    assets: ReleaseAsset[];
    published_at: string;
    html_url: string;
}

export interface ProcessedRelease {
    releaseName: string;
    tag: string;
    assets: ReleaseAsset[];
    best?: ReleaseAsset;
    publishedAt: string;
    url: string;
}

// OS Detection and Asset Matching
export function detectOS(): 'windows' | 'macos' | 'linux' {
    if (typeof window !== 'undefined' && window.navigator) {
        const userAgent = window.navigator.userAgent.toLowerCase();
        if (userAgent.includes('win')) return 'windows';
        if (userAgent.includes('mac')) return 'macos';
        return 'linux';
    }
    
    // Fallback to process.platform if available (Electron)
    if (typeof process !== 'undefined' && process.platform) {
        switch (process.platform) {
            case 'win32': return 'windows';
            case 'darwin': return 'macos';
            default: return 'linux';
        }
    }
    
    return 'windows'; // Default fallback
}

export function detectArchitecture(): 'x64' | 'arm64' | 'x86' {
    if (typeof process !== 'undefined' && process.arch) {
        switch (process.arch) {
            case 'x64': return 'x64';
            case 'arm64': return 'arm64';
            case 'ia32': return 'x86';
            default: return 'x64';
        }
    }
    
    // Fallback detection via navigator
    if (typeof window !== 'undefined' && window.navigator) {
        const userAgent = window.navigator.userAgent.toLowerCase();
        if (userAgent.includes('arm') || userAgent.includes('aarch64')) return 'arm64';
        if (userAgent.includes('wow64') || userAgent.includes('x64')) return 'x64';
    }
    
    return 'x64'; // Default fallback
}

// Asset matching logic for different OS/arch combinations
export function findBestAsset(assets: ReleaseAsset[]): ReleaseAsset | null {
    if (!assets || assets.length === 0) return null;
    
    const os = detectOS();
    const arch = detectArchitecture();
    
    // Define priority patterns for each OS
    const patterns = {
        windows: [
            // Windows patterns in order of preference
            // Special case for XMCL: prefer .appinstaller over .exe
            /xmcl.*\.appinstaller$/i,
            /x-minecraft-launcher.*\.appinstaller$/i,
            /\.appinstaller$/i,
            // ModRinth specific patterns
            /modrinth.*app.*setup.*\.exe$/i,
            /modrinth.*app.*x64.*setup.*\.exe$/i,
            // Then regular executables
            /windows.*x64.*\.exe$/i,
            /win.*x64.*\.exe$/i,
            /x64-setup\.exe$/i,
            /setup.*\.exe$/i,
            /windows.*64.*\.exe$/i,
            /win.*64.*\.exe$/i,
            /windows.*\.exe$/i,
            /win.*\.exe$/i,
            /\.exe$/i,
            /windows.*\.msi$/i,
            /win.*\.msi$/i,
            /\.msi$/i
        ],
        macos: [
            // macOS patterns in order of preference
            // ModRinth specific patterns
            /modrinth.*app.*universal.*\.dmg$/i,
            /modrinth.*app.*\.dmg$/i,
            /macos.*\.dmg$/i,
            /mac.*\.dmg$/i,
            /darwin.*\.dmg$/i,
            /osx.*\.dmg$/i,
            /\.dmg$/i,
            /macos.*\.pkg$/i,
            /mac.*\.pkg$/i,
            /\.pkg$/i
        ],
        linux: [
            // Linux patterns in order of preference
            // ModRinth specific patterns
            /modrinth.*app.*amd64.*\.deb$/i,
            /modrinth.*app.*\.deb$/i,
            /modrinth.*app.*amd64.*\.rpm$/i,
            /modrinth.*app.*\.rpm$/i,
            /modrinth.*app.*amd64.*\.AppImage$/i,
            /modrinth.*app.*\.AppImage$/i,
            // Regular patterns
            /linux.*x64.*\.deb$/i,
            /linux.*amd64.*\.deb$/i,
            /linux.*64.*\.deb$/i,
            /\.deb$/i,
            /linux.*x64.*\.rpm$/i,
            /linux.*64.*\.rpm$/i,
            /\.rpm$/i,
            /linux.*\.tar\.gz$/i,
            /linux.*\.tar\.xz$/i,
            /appimage$/i,
            /\.AppImage$/i
        ]
    };
    
    // Architecture-specific patterns
    const archPatterns = {
        x64: [/x64/i, /amd64/i, /x86_64/i, /64/i],
        arm64: [/arm64/i, /aarch64/i, /armv8/i],
        x86: [/x86/i, /i386/i, /32/i]
    };
    
    const osPatterns = patterns[os];
    const currentArchPatterns = archPatterns[arch];
    
    // First, try to find assets that match both OS and architecture
    for (const pattern of osPatterns) {
        for (const asset of assets) {
            if (pattern.test(asset.name)) {
                // Check if it also matches architecture
                const matchesArch = currentArchPatterns.some(archPattern => 
                    archPattern.test(asset.name)
                );
                if (matchesArch || arch === 'x64') { // x64 is often implicit
                    return asset;
                }
            }
        }
    }
    
    // If no arch-specific match, try OS-only patterns
    for (const pattern of osPatterns) {
        for (const asset of assets) {
            if (pattern.test(asset.name)) {
                return asset;
            }
        }
    }
    
    // Fallback: return the first asset that seems like an installer
    const installerPatterns = [/\.exe$/i, /\.dmg$/i, /\.deb$/i, /\.pkg$/i, /\.msi$/i, /\.rpm$/i, /\.AppImage$/i];
    for (const pattern of installerPatterns) {
        const asset = assets.find(a => pattern.test(a.name));
        if (asset) return asset;
    }
    
    // Last resort: return first asset
    return assets[0] || null;
}

// GitHub API utilities
export const LAUNCHER_REPOS = {
    prism: 'Diegiwg/PrismLauncher-Cracked',
    xmcl: 'Voxelum/x-minecraft-launcher', 
    astralrinth: 'astralrinth-app', // Special case: uses direct download, not API
    modrinth: 'modrinth-app' // Special case: uses CDN, not GitHub
} as const;

export function getRepoUrl(mode: keyof typeof LAUNCHER_REPOS): string | null {
    const repo = LAUNCHER_REPOS[mode];
    if (mode === 'astralrinth') {
        // AstralRinth uses direct download - return null to indicate special handling
        return null;
    }
    if (mode === 'modrinth') {
        // ModRinth uses direct CDN - return null to indicate special handling
        return null;
    }
    return `https://api.github.com/repos/${repo}/releases/latest`;
}

// Get ModRinth download URL for current platform
export function getModRinthDownloadUrl(): string {
    const platform = detectOS();
    const version = '0.10.7'; // This could be fetched from latest.json if needed
    
    const baseUrl = `https://launcher-files.modrinth.com/versions/${version}`;
    
    if (platform === 'windows') {
        return `${baseUrl}/windows/Modrinth%20App_${version}_x64-setup.exe`;
    } else if (platform === 'macos') {
        return `${baseUrl}/macos/Modrinth%20App_${version}_universal.dmg`;
    } else if (platform === 'linux') {
        // Default to AppImage for Linux
        return `${baseUrl}/linux/Modrinth%20App_${version}_amd64.AppImage`;
    }
    
    // Fallback
    return `${baseUrl}/windows/Modrinth%20App_${version}_x64-setup.exe`;
}

// Create a fake release object for ModRinth
export function createModRinthRelease(): { 
    tag_name: string; 
    name: string;
    body: string;
    published_at: string;
    assets: Array<{ name: string; browser_download_url: string }>;
    html_url: string;
} {
    const platform = detectOS();
    const version = '0.10.7';
    const baseUrl = `https://launcher-files.modrinth.com/versions/${version}`;
    
    // Create platform-specific assets
    const assets = [];
    
    if (platform === 'windows') {
        assets.push({
            name: `Modrinth App_${version}_x64-setup.exe`,
            browser_download_url: `${baseUrl}/windows/Modrinth%20App_${version}_x64-setup.exe`,
            size: 50000000, // Estimated size
            content_type: 'application/octet-stream'
        });
    } else if (platform === 'macos') {
        assets.push({
            name: `Modrinth App_${version}_universal.dmg`,
            browser_download_url: `${baseUrl}/macos/Modrinth%20App_${version}_universal.dmg`,
            size: 45000000, // Estimated size
            content_type: 'application/octet-stream'
        });
    } else if (platform === 'linux') {
        assets.push({
            name: `Modrinth App_${version}_amd64.AppImage`,
            browser_download_url: `${baseUrl}/linux/Modrinth%20App_${version}_amd64.AppImage`,
            size: 55000000, // Estimated size
            content_type: 'application/octet-stream'
        });
        assets.push({
            name: `Modrinth App_${version}_amd64.deb`,
            browser_download_url: `${baseUrl}/linux/Modrinth%20App_${version}_amd64.deb`,
            size: 48000000, // Estimated size
            content_type: 'application/octet-stream'
        });
        assets.push({
            name: `Modrinth App-${version}-1.x86_64.rpm`,
            browser_download_url: `${baseUrl}/linux/Modrinth%20App-${version}-1.x86_64.rpm`,
            size: 49000000, // Estimated size
            content_type: 'application/octet-stream'
        });
    }
    
    return {
        tag_name: version,
        tag: version,
        name: `Modrinth App ${version}`,
        releaseName: `Modrinth App ${version}`,
        body: 'The official Modrinth launcher - the best way to discover, download, and manage Minecraft mods and modpacks',
        published_at: new Date().toISOString(),
        assets: assets,
        html_url: 'https://modrinth.com/app'
    };
}

// Create a fake release object for AstralRinth
export function createAstralRinthRelease(): { 
    tag_name: string; 
    name: string;
    body: string;
    published_at: string;
    assets: Array<{ name: string; browser_download_url: string }>;
    html_url: string;
} {
    const platform = detectOS();
    const version = '0.10.601';
    
    // AstralRinth direct download URLs
    let downloadUrl: string;
    let fileName: string;
    
    if (platform === 'windows') {
        fileName = `AstralRinth App_${version}_x64-setup.exe`;
        // Use correct download URL format from the releases page
        downloadUrl = `https://git.astralium.su/didirus/AstralRinth/releases/download/AR-${version}/AstralRinth%20App_${version}_x64-setup.exe`;
    } else if (platform === 'macos') {
        fileName = `AstralRinth App_${version}_aarch64.dmg`;
        downloadUrl = `https://git.astralium.su/didirus/AstralRinth/releases/download/AR-${version}/AstralRinth%20App_${version}_aarch64.dmg`;
    } else if (platform === 'linux') {
        fileName = `AstralRinth App_${version}_amd64.AppImage`;
        downloadUrl = `https://git.astralium.su/didirus/AstralRinth/releases/download/AR-${version}/AstralRinth%20App_${version}_amd64.AppImage`;
    } else {
        // Fallback to Windows
        fileName = `AstralRinth App_${version}_x64-setup.exe`;
        downloadUrl = `https://git.astralium.su/didirus/AstralRinth/releases/download/AR-${version}/AstralRinth%20App_${version}_x64-setup.exe`;
    }
    
    return {
        tag_name: `v${version}`,
        tag: `v${version}`,
        name: `AstralRinth ${version}`,
        releaseName: `AstralRinth ${version}`,
        body: 'AstralRinth launcher - a powerful alternative Minecraft launcher with advanced features and mod support',
        published_at: new Date().toISOString(),
        assets: [
            {
                name: fileName,
                browser_download_url: downloadUrl
            }
        ],
        html_url: 'https://git.astralium.su/didirus/AstralRinth/releases/latest'
    };
}

// Get fallback URLs for AstralRinth
export function getAstralRinthFallbackUrls(repo: string): string[] {
    return [
        // Try different API formats
        `https://git.astralium.su/api/v4/projects/${encodeURIComponent(repo)}/releases`,
        `https://git.astralium.su/api/v4/projects/${repo.replace('/', '%2F')}/releases`,
        // Try with project ID if we knew it (but we don't, so these are examples)
        // `https://git.astralium.su/api/v4/projects/123/releases`,
    ];
}

// Installation command generation
export function getInstallCommand(filePath: string, os: ReturnType<typeof detectOS>): {
    command: string;
    args: string[];
    requiresElevation: boolean;
} {
    switch (os) {
        case 'windows':
            if (filePath.endsWith('.appinstaller')) {
                // Windows App Installer for XMCL - use direct file association
                return {
                    command: filePath, // Let Windows handle .appinstaller files directly
                    args: [],
                    requiresElevation: false // App Installer doesn't require elevation
                };
            } else if (filePath.endsWith('.exe')) {
                return {
                    command: filePath,
                    args: [], // Remove silent flag so user can see the installer
                    requiresElevation: true
                };
            } else if (filePath.endsWith('.msi')) {
                return {
                    command: 'msiexec',
                    args: ['/i', filePath, '/quiet', '/norestart'],
                    requiresElevation: true
                };
            }
            break;
            
        case 'macos':
            if (filePath.endsWith('.dmg')) {
                return {
                    command: 'hdiutil',
                    args: ['attach', filePath, '-nobrowse', '-mountpoint', '/tmp/installer_mount'],
                    requiresElevation: false
                };
            } else if (filePath.endsWith('.pkg')) {
                return {
                    command: 'installer',
                    args: ['-pkg', filePath, '-target', '/'],
                    requiresElevation: true
                };
            }
            break;
            
        case 'linux':
            if (filePath.endsWith('.deb')) {
                return {
                    command: 'dpkg',
                    args: ['-i', filePath],
                    requiresElevation: true
                };
            } else if (filePath.endsWith('.rpm')) {
                return {
                    command: 'rpm',
                    args: ['-i', filePath],
                    requiresElevation: true
                };
            } else if (filePath.endsWith('.AppImage')) {
                return {
                    command: 'chmod',
                    args: ['+x', filePath],
                    requiresElevation: false
                };
            }
            break;
    }
    
    // Default: try to execute the file directly
    return {
        command: filePath,
        args: [],
        requiresElevation: false
    };
}

// Validate download integrity (if checksums are available)
export async function validateDownload(filePath: string, expectedHash?: string): Promise<boolean> {
    if (!expectedHash || typeof window === 'undefined') return true;
    
    try {
        // This would require the main process to compute file hash
        const actualHash = await window.nahaAPI?.getFileHash?.(filePath, 'sha256');
        return actualHash === expectedHash;
    } catch (error) {
        console.warn('Could not validate download hash:', error);
        return true; // Don't block installation if hash validation fails
    }
}

// Format file size for display
export function formatFileSize(bytes: number): string {
    if (bytes === 0) return '0 B';
    
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
}

// Get the appropriate cache directory for downloads
export function getCacheDirectory(): string {
    if (typeof process !== 'undefined' && process.platform) {
        switch (process.platform) {
            case 'win32':
                return process.env.LOCALAPPDATA || process.env.TEMP || 'C:\\Temp';
            case 'darwin':
                return process.env.HOME + '/Library/Caches' || '/tmp';
            case 'linux':
                return process.env.XDG_CACHE_HOME || process.env.HOME + '/.cache' || '/tmp';
            default:
                return '/tmp';
        }
    }
    
    // Fallback for browser environment
    return 'Downloads';
}

// Generate default download path for installers
export function getDefaultDownloadPath(fileName: string, launcherMode: string): string {
    const cacheDir = getCacheDirectory();
    const sanitizedFileName = fileName.replace(/[<>:"/\\|?*]/g, '-');
    
    if (typeof process !== 'undefined' && process.platform === 'win32') {
        return `${cacheDir}\\NAHA-MC-Helper\\${launcherMode}\\${sanitizedFileName}`;
    } else {
        return `${cacheDir}/NAHA-MC-Helper/${launcherMode}/${sanitizedFileName}`;
    }
}
