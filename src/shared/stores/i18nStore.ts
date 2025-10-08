/**
 * i18n Store
 * Reactive translation store for Svelte components
 */

import { derived } from 'svelte/store';
import { language } from './appStore';
import enTranslations from '../i18n/en.json';
import frTranslations from '../i18n/fr.json';

type Translations = typeof enTranslations;

const translations: Record<string, Translations> = {
  en: enTranslations,
  fr: frTranslations
};

/**
 * Reactive translation function
 * Usage in Svelte: $t('homepage.title')
 */
export const t = derived(
  language,
  ($language) => (key: string, params?: Record<string, string | number>): string => {
    const keys = key.split('.');
    let value: any = translations[$language];
    
    for (const k of keys) {
      if (value && typeof value === 'object' && k in value) {
        value = value[k];
      } else {
        // Fallback to English
        value = translations['en'];
        for (const fallbackKey of keys) {
          if (value && typeof value === 'object' && fallbackKey in value) {
            value = value[fallbackKey];
          } else {
            return key;
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
);


