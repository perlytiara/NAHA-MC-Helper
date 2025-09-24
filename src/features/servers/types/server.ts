// Server-related types for the servers feature

export type ServerLoader = 'fabric' | 'btw' | 'neoforge';

export interface Server {
    id: string;
    name: string;
    loader: ServerLoader;
    mcVersion: string;
    packVersion: string;
    online: number;
    max: number;
    image: string;
    updateAvailable: boolean;
}

export interface ServerStatus {
    online: boolean;
    playerCount: number;
    maxPlayers: number;
    ping?: number;
}

