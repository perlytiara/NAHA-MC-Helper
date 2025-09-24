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
        // Try multiple paths to find minecraft-installer.exe
        const possiblePaths = [
            // 1. In resources directory (packaged app)
            path.join(process.resourcesPath, 'minecraft-installer.exe'),
            // 2. In app directory (development)
            path.join(app.getAppPath(), 'minecraft-installer.exe'),
            // 3. In tools directory (development)
            path.join(app.getAppPath(), 'tools', 'minecraft-installer', 'target', 'release', 'minecraft-installer.exe'),
            // 4. In current working directory
            path.join(process.cwd(), 'minecraft-installer.exe'),
            // 5. In resourcesPath/tools (alternative packaging)
            path.join(process.resourcesPath, 'tools', 'minecraft-installer', 'target', 'release', 'minecraft-installer.exe')
        ];
        
        console.log('🔍 [INSTALLER] App path:', app.getAppPath());
        console.log('🔍 [INSTALLER] Resources path:', process.resourcesPath);
        console.log('🔍 [INSTALLER] Current working directory:', process.cwd());
        
        // Find the first existing path
        for (const testPath of possiblePaths) {
            console.log('🔍 [INSTALLER] Checking path:', testPath);
            if (fs.existsSync(testPath)) {
                this.installerPath = testPath;
                console.log('✅ [INSTALLER] Found installer at:', this.installerPath);
                break;
            }
        }
        
        // If no path found, use the first one as default
        if (!this.installerPath) {
            this.installerPath = possiblePaths[0];
            console.log('⚠️ [INSTALLER] No installer found, using default path:', this.installerPath);
        }
        
        console.log('✅ [INSTALLER] Final installer path:', this.installerPath);
        console.log('✅ [INSTALLER] File exists:', fs.existsSync(this.installerPath));
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
            console.log('🚀 [INSTALLER] Running command:', this.installerPath, args);
            console.log('🚀 [INSTALLER] File exists:', fs.existsSync(this.installerPath));

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
