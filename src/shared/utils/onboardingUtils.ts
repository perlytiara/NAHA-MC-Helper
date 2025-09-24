// src/shared/utils/onboardingUtils.ts

/**
 * Onboarding utility functions for managing first-time user experience
 */

export interface OnboardingData {
  minecraftInstalled?: boolean;
  preferredLauncher?: string;
  completedAt?: string;
}

/**
 * Check if the user has completed onboarding
 */
export function isOnboardingCompleted(): boolean {
  if (typeof window === 'undefined' || !window.localStorage) {
    return false;
  }
  
  try {
    const completed = localStorage.getItem('naha-onboarding-completed');
    return completed === 'true';
  } catch (error) {
    console.warn('Failed to check onboarding status:', error);
    return false;
  }
}

/**
 * Mark onboarding as completed
 */
export function completeOnboarding(data: Partial<OnboardingData> = {}): void {
  if (typeof window === 'undefined' || !window.localStorage) {
    return;
  }
  
  try {
    localStorage.setItem('naha-onboarding-completed', 'true');
    
    if (data.minecraftInstalled !== undefined) {
      localStorage.setItem('naha-minecraft-installed', data.minecraftInstalled.toString());
    }
    
    if (data.preferredLauncher) {
      localStorage.setItem('naha-preferred-launcher', data.preferredLauncher);
    }
    
    localStorage.setItem('naha-onboarding-completed-at', new Date().toISOString());
  } catch (error) {
    console.warn('Failed to save onboarding data:', error);
  }
}

/**
 * Get onboarding data
 */
export function getOnboardingData(): OnboardingData {
  if (typeof window === 'undefined' || !window.localStorage) {
    return {};
  }
  
  try {
    const minecraftInstalled = localStorage.getItem('naha-minecraft-installed') === 'true';
    const preferredLauncher = localStorage.getItem('naha-preferred-launcher') || undefined;
    const completedAt = localStorage.getItem('naha-onboarding-completed-at') || undefined;
    
    return {
      minecraftInstalled,
      preferredLauncher,
      completedAt
    };
  } catch (error) {
    console.warn('Failed to get onboarding data:', error);
    return {};
  }
}

/**
 * Reset onboarding data (for testing or user request)
 */
export function resetOnboarding(): void {
  if (typeof window === 'undefined' || !window.localStorage) {
    return;
  }
  
  try {
    localStorage.removeItem('naha-onboarding-completed');
    localStorage.removeItem('naha-minecraft-installed');
    localStorage.removeItem('naha-preferred-launcher');
    localStorage.removeItem('naha-onboarding-completed-at');
  } catch (error) {
    console.warn('Failed to reset onboarding data:', error);
  }
}

/**
 * Get user's preferred launcher based on onboarding data
 */
export function getPreferredLauncher(): string | null {
  const data = getOnboardingData();
  return data.preferredLauncher || null;
}

/**
 * Check if user has Minecraft installed (based on onboarding)
 */
export function hasMinecraftInstalled(): boolean | null {
  const data = getOnboardingData();
  return data.minecraftInstalled !== undefined ? data.minecraftInstalled : null;
}
