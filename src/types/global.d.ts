// Global type declarations for the application

interface Window {
  prism: {
    detectLaunchers: () => Promise<{ success: boolean; launchers: any[]; error?: string }>;
    validateMinecraftInstallation: (path: string) => Promise<boolean>;
    openFileDialog: (options: { properties: string[] }) => Promise<string[]>;
    onShowAboutDialog: (callback: () => void) => () => void;
    getFileHash: (filePath: string, algorithm?: string) => Promise<string>;
  };
}

// Node.js globals for renderer process
declare const process: NodeJS.Process;
declare const setTimeout: (callback: () => void, ms: number) => NodeJS.Timeout;
declare const clearTimeout: (timeoutId: NodeJS.Timeout) => void;
