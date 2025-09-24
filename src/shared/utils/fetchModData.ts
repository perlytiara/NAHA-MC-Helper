// Utility for fetching mod data
export interface ModData {
  name: string;
  mc: string;
  mods: number | null;
}

export interface ModLoaderData {
  fab: ModData;
  neo: ModData;
}

/**
 * Fetches mod data from the backend or API
 * This is a placeholder implementation - replace with actual API calls
 */
export async function fetchModData(): Promise<ModLoaderData> {
  // Mock implementation for development
  // Replace with actual API calls when backend is available
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve({
        fab: {
          name: 'Fabric',
          mc: '1.20.1',
          mods: 150
        },
        neo: {
          name: 'NeoForge', 
          mc: '1.20.1',
          mods: 120
        }
      });
    }, 500);
  });
}
