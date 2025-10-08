/**
 * Internationalization (i18n) Utility
 * Provides translation functionality for the application
 */

import { get } from 'svelte/store';
import { language } from '../stores/appStore';
import enTranslations from '../i18n/en.json';
import frTranslations from '../i18n/fr.json';

type TranslationKey = string;
type Translations = typeof enTranslations;

const translations: Record<string, Translations> = {
  en: enTranslations,
  fr: frTranslations
};

/**
 * Get a translation by key path (e.g., 'homepage.title')
 */
export function t(key: TranslationKey, params?: Record<string, string | number>): string {
  const currentLang = get(language);
  const keys = key.split('.');
  
  let value: any = translations[currentLang];
  
  for (const k of keys) {
    if (value && typeof value === 'object' && k in value) {
      value = value[k];
    } else {
      console.warn(`Translation key not found: ${key} for language: ${currentLang}`);
      // Fallback to English
      value = translations['en'];
      for (const fallbackKey of keys) {
        if (value && typeof value === 'object' && fallbackKey in value) {
          value = value[fallbackKey];
        } else {
          return key; // Return the key itself as fallback
        }
      }
      break;
    }
  }
  
  // Handle parameter substitution
  if (typeof value === 'string' && params) {
    return value.replace(/\{\{(\w+)\}\}/g, (match, paramKey) => {
      return params[paramKey]?.toString() || match;
    });
  }
  
  return typeof value === 'string' ? value : key;
}

/**
 * Get current language
 */
export function getCurrentLanguage(): string {
  return get(language);
}

/**
 * Set language
 */
export function setLanguage(lang: 'en' | 'fr'): void {
  language.set(lang);
  // Save to localStorage
  if (typeof window !== 'undefined' && window.localStorage) {
    localStorage.setItem('naha-language', lang);
  }
}

/**
 * Load saved language from localStorage
 */
export function loadSavedLanguage(): void {
  if (typeof window !== 'undefined' && window.localStorage) {
    const saved = localStorage.getItem('naha-language');
    if (saved === 'en' || saved === 'fr') {
      language.set(saved);
    }
  }
}

/**
 * Get available languages
 */
export function getAvailableLanguages(): Array<{ code: string; name: string; nativeName: string }> {
  return [
    { code: 'en', name: 'English', nativeName: 'English' },
    { code: 'fr', name: 'French', nativeName: 'Français' }
  ];
}


