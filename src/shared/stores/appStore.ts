import { writable } from 'svelte/store';
import type { NotificationData } from '../types/common';

// Main application state stores
export const currentPage = writable('homepage');
export const currentParams = writable({});
export const debug = writable(false);
export const onboardingCompleted = writable(false);
export const onboardingStartStep = writable(1);
export const onboardingCurrentStep = writable(1);
export const hideUpdateButton = writable(false);
export const isInOnboarding = writable(false);

// Navigation state
export const isNavigationOpen = writable(false);

// Application settings
export const theme = writable('light');
export const isLoading = writable(false);
export const language = writable('en'); // 'en' or 'fr'

// Error handling
export const error = writable(null);
export const notification = writable<NotificationData | null>(null);
