/**
 * Minecraft Installer Service
 * Interfaces with the minecraft-installer.exe tool for modpack installation
 */

import { spawn } from 'child_process';
import path from 'path';
import { app } from 'electron';
import fs from 'fs';

export class MinecraftInstallerService {
    constructor() {
        // Determine platform-specific executable name
        this.platform = process.platform;
        this.arch = process.arch;
        this.executableName = this.getPlatformExecutableName();
        
        // Path to the platform-specific minecraft-installer executable
        // In production, it will be in extraResources/installer/
        // In development, it will be in the tools/minecraft-installer-releases/ directory
        if (app.isPackaged) {
            // Production: executable is in extraResources/installer/
            this.installerPath = path.join(
                process.resourcesPath,
                'installer',
                this.executableName
            );
        } else {
            // Development: executable is in tools/minecraft-installer-releases/ directory
            this.installerPath = path.join(
                app.getAppPath(),
                'tools',
                'minecraft-installer-releases',
                this.executableName
            );
        }
        
        console.log('MinecraftInstallerService: Platform:', this.platform, 'Architecture:', this.arch);
        console.log('MinecraftInstallerService: Executable name:', this.executableName);
        console.log('MinecraftInstallerService: Looking for executable at:', this.installerPath);
        console.log('MinecraftInstallerService: App is packaged:', app.isPackaged);
        console.log('MinecraftInstallerService: Resources path:', process.resourcesPath);
        
        // Check if the file exists and log the result
        const exists = fs.existsSync(this.installerPath);
        console.log('MinecraftInstallerService: Executable exists:', exists);
        
        if (!exists) {
            console.warn('MinecraftInstallerService: Executable not found! This will cause legacy installation mode.');
            
            // Try alternative paths for debugging
            const altPaths = [
                path.join(process.resourcesPath, this.executableName),
                path.join(process.resourcesPath, 'installer', this.executableName),
                path.join(app.getAppPath(), 'tools', 'minecraft-installer-releases', this.executableName),
                // Fallback to old path for backward compatibility
                path.join(app.getAppPath(), 'tools', 'minecraft-installer', 'target', 'release', this.executableName)
            ];
            
            console.log('MinecraftInstallerService: Checking alternative paths:');
            altPaths.forEach((altPath, index) => {
                const altExists = fs.existsSync(altPath);
                console.log(`  ${index + 1}. ${altPath} - ${altExists ? 'EXISTS' : 'NOT FOUND'}`);
            });
        }
    }

    /**
     * Get the platform-specific executable name
     */
    getPlatformExecutableName() {
        const platform = this.platform;
        const arch = this.arch;
        
        // Map platform and architecture to executable names
        if (platform === 'win32') {
            // Windows executables
            if (arch === 'x64') {
                return 'minecraft-installer-windows-x86_64.exe';
            } else {
                return 'minecraft-installer-windows-gnu-x86_64.exe';
            }
        } else if (platform === 'darwin') {
            // macOS executables
            if (arch === 'arm64') {
                return 'minecraft-installer-macos-apple-silicon-aarch64';
            } else if (arch === 'x64') {
                return 'minecraft-installer-macos-intel-x86_64';
            } else {
                // Fallback to Intel x64
                return 'minecraft-installer-macos-intel-x86_64';
            }
        } else if (platform === 'linux') {
            // Linux executable
            if (arch === 'x64') {
                return 'minecraft-installer-linux-x86_64';
            } else {
                // Fallback to x86_64
                return 'minecraft-installer-linux-x86_64';
            }
        } else {
            // Fallback to Windows x64 for unknown platforms
            console.warn(`MinecraftInstallerService: Unknown platform ${platform}, falling back to Windows x64 executable`);
            return 'minecraft-installer-windows-x86_64.exe';
        }
    }

    /**
     * List detected launchers
     */
    async listLaunchers() {
        return this.runCommand(['--list-launchers']);
    }

    /**
     * Download and install NeoForge modpack from NAHA API
     */
    async downloadNeoForge(targetLauncher = null, createInstance = true, onProgress = null, customPath = null) {
        const args = ['--download-neoforge'];
        
        // If no target launcher specified, let the installer auto-detect
        if (targetLauncher) {
            args.push('--target-launcher', targetLauncher);
        }
        
        // Add custom path for "other" launcher
        if (targetLauncher === 'other') {
            if (!customPath || !customPath.trim()) {
                throw new Error('Custom path is required for Other launcher type');
            }
            args.push('--custom-path', customPath);
        }
        
        if (createInstance) {
            args.push('--create-instance');
        }

        return this.runCommand(args, { onProgress });
    }

    /**
     * Download and install Fabric modpack from NAHA API
     */
    async downloadFabric(targetLauncher = null, createInstance = true, onProgress = null, customPath = null) {
        const args = ['--download-fabric'];
        
        if (targetLauncher) {
            args.push('--target-launcher', targetLauncher);
        }
        
        // Add custom path for "other" launcher
        if (targetLauncher === 'other') {
            if (!customPath || !customPath.trim()) {
                throw new Error('Custom path is required for Other launcher type');
            }
            args.push('--custom-path', customPath);
        }
        
        if (createInstance) {
            args.push('--create-instance');
        }

        return this.runCommand(args, { onProgress });
    }

    /**
     * Install local mrpack file
     */
    async installMrpack(mrpackPath, targetLauncher = null, createInstance = true, customPath = null) {
        const args = ['--mrpack', mrpackPath];
        
        if (targetLauncher) {
            args.push('--target-launcher', targetLauncher);
        }
        
        // Add custom path for "other" launcher
        if (targetLauncher === 'other' && customPath) {
            args.push('--custom-path', customPath);
        }
        
        if (createInstance) {
            args.push('--create-instance');
        }

        return this.runCommand(args);
    }


    /**
     * List available Minecraft versions
     */
    async listVersions(versionType = null) {
        const args = ['--list-versions'];
        
        if (versionType) {
            args.push('--version-type', versionType);
        }

        return this.runCommand(args);
    }

    /**
     * Install specific Minecraft version
     */
    async installMinecraft(version, loader = 'vanilla', loaderVersion = 'stable', force = false) {
        const args = [
            '--version', version,
            '--loader', loader,
            '--loader-version', loaderVersion
        ];
        
        if (force) {
            args.push('--force');
        }

        return this.runCommand(args);
    }

    /**
     * Run installer command with real-time output
     */
    async runCommand(args, options = {}) {
        return new Promise((resolve, reject) => {
            // Debug logging removed for cleaner output

            const process = spawn(this.installerPath, args, {
                stdio: ['pipe', 'pipe', 'pipe'],
                ...options
            });

            let stdout = '';
            let stderr = '';
            let isResolved = false;
            
            // Set a timeout to prevent hanging (5 minutes for installation)
            const timeout = setTimeout(() => {
                if (isResolved) return;
                isResolved = true;
                
                // Installer timeout - killing process
                process.kill('SIGTERM');
                
                reject({
                    success: false,
                    error: 'Installation timed out after 5 minutes',
                    code: -1,
                    stdout,
                    stderr,
                    args
                });
            }, 5 * 60 * 1000); // 5 minutes

            // Handle real-time output
            process.stdout.on('data', (data) => {
                const output = data.toString();
                stdout += output;
                
                // Emit progress events for real-time updates
                if (options.onProgress) {
                    options.onProgress({
                        type: 'stdout',
                        data: output,
                        timestamp: new Date().toISOString()
                    });
                }
            });

            process.stderr.on('data', (data) => {
                const output = data.toString();
                stderr += output;
                
                // Emit progress events for real-time updates
                if (options.onProgress) {
                    options.onProgress({
                        type: 'stderr',
                        data: output,
                        timestamp: new Date().toISOString()
                    });
                }
            });

            process.on('close', (code) => {
                if (isResolved) return;
                isResolved = true;
                
                clearTimeout(timeout);

                const result = {
                    success: code === 0,
                    code,
                    stdout,
                    stderr,
                    args
                };

                if (code === 0) {
                    resolve(result);
                } else {
                    reject(result);
                }
            });

            process.on('error', (error) => {
                if (isResolved) return;
                isResolved = true;
                
                clearTimeout(timeout);

                reject({
                    success: false,
                    error: error.message,
                    code: -1,
                    stdout,
                    stderr,
                    args
                });
            });

            // Handle process termination
            process.on('exit', (code, signal) => {
                if (isResolved) return;
                isResolved = true;
                
                clearTimeout(timeout);

                if (signal) {
                    reject({
                        success: false,
                        error: `Process terminated with signal: ${signal}`,
                        code: -1,
                        stdout,
                        stderr,
                        args
                    });
                }
            });
        });
    }

    /**
     * Check if installer is available
     */
    async isAvailable() {
        try {
            return fs.existsSync(this.installerPath);
        } catch {
            return false;
        }
    }

    /**
     * Get installer version info
     */
    async getVersion() {
        try {
            const result = await this.runCommand(['-V']);
            return {
                available: true,
                version: result.stdout.trim()
            };
        } catch (error) {
            return {
                available: false,
                error: error.message || 'Installer not available'
            };
        }
    }
}

export default MinecraftInstallerService;
