import { writable } from 'svelte/store';
import type { NotificationData } from '../types/common';

// Main application state stores
export const currentPage = writable('homepage');
export const currentParams = writable({});
export const debug = writable(false);
export const onboardingCompleted = writable(false);

// Navigation state
export const isNavigationOpen = writable(false);

// Application settings
export const theme = writable('light');
export const isLoading = writable(false);

// Error handling
export const error = writable(null);
export const notification = writable<NotificationData | null>(null);
