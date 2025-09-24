import { writable } from 'svelte/store';

// Main installer state
export const currentMode = writable<string>('cracked');
export const release = writable<any>(null);
export const status = writable<string>('');
export const progress = writable<number>(0);
export const error = writable<string | null>(null);
export const loader = writable<string>('fabric');
export const mrpackStatus = writable<string>('Idle');

// Create installer store object for easier import
export const installerStore = {
  currentMode,
  release,
  status,
  progress,
  error,
  loader,
  mrpackStatus
};
