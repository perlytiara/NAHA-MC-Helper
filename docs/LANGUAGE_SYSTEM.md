# Language System (i18n) - English / Français

## 🌍 Overview

NAHA MC Helper now supports **English** and **French** with a complete internationalization (i18n) system.

## 📁 File Structure

```text
src/
├── shared/
│   ├── i18n/
│   │   ├── en.json                    # English translations
│   │   └── fr.json                    # French translations
│   ├── stores/
│   │   ├── appStore.ts                # Language store: language writable('en')
│   │   └── i18nStore.ts               # Reactive translation function: $t(key)
│   ├── utils/
│   │   └── i18n.ts                    # Utility functions: setLanguage(), loadSavedLanguage()
│   └── components/ui/settings/
│       └── LanguageSelector.svelte    # Language switcher component
└── features/
    └── onboarding/components/
        └── OnboardingLanguageSelection.svelte  # Language selection during onboarding
```

## 🚀 Quick Start

### In Svelte Components

```svelte
<script>
  import { t } from '../../../shared/stores/i18nStore';
</script>

<h1>{$t('homepage.title')}</h1>
<p>{$t('onboarding.welcome.description')}</p>
```

### With Parameters

```svelte
<p>{$t('onboarding.stepOf', { current: 2, total: 7 })}</p>
<!-- English: "Step 2 of 7" -->
<!-- French: "Étape 2 sur 7" -->
```

### In TypeScript/JavaScript

```typescript
import { t, setLanguage, getCurrentLanguage } from './shared/utils/i18n';

// Get translation
const title = t('homepage.title');

// Change language
setLanguage('fr'); // Switch to French
setLanguage('en'); // Switch to English

// Get current language
const current = getCurrentLanguage(); // 'en' or 'fr'
```

## 📝 Translation Keys

### Common

| Key | English | Français |
|-----|---------|----------|
| `common.yes` | Yes | Oui |
| `common.no` | No | Non |
| `common.back` | Back | Retour |
| `common.next` | Next | Suivant |
| `common.continue` | Continue | Continuer |
| `common.cancel` | Cancel | Annuler |

### Navigation

| Key | English | Français |
|-----|---------|----------|
| `navigation.home` | Home | Accueil |
| `navigation.install` | Install | Installer |
| `navigation.update` | Update | Mettre à jour |
| `navigation.servers` | Servers | Serveurs |

### Homepage

| Key | English | Français |
|-----|---------|----------|
| `homepage.title` | Minecraft Helper | Assistant Minecraft |
| `homepage.subtitle` | Your complete toolkit... | Votre boîte à outils complète... |
| `homepage.installModpack` | Install Modpack | Installer un Modpack |

### Onboarding

| Key | English | Français |
|-----|---------|----------|
| `onboarding.welcome.title` | Welcome to NAHA... | Bienvenue dans NAHA... |
| `onboarding.minecraftCheck.title` | Do you have Minecraft installed? | Avez-vous Minecraft installé? |
| `onboarding.minecraftCheck.yesHaveIt` | Yes, I have it | Oui, je l'ai |

[See full translations in `en.json` and `fr.json`]

## 🎨 Language Selection

### During Onboarding (First Time)

#### Step 1: Language Selection

```text
┌─────────────────────────────────┐
│          ⚡ NAHA                │
│                                 │
│  Choose Your Language /         │
│  Choisissez Votre Langue        │
│                                 │
│  🇬🇧 English     🇫🇷 Français  │
└─────────────────────────────────┘
```

### Via Menu (Anytime)

```text
Help → Language → [English / Français]
```

Or use the `LanguageSelector` component anywhere:

```svelte
<LanguageSelector />
```

## 💾 Persistence

Language preference is saved to **localStorage**:

```javascript
localStorage.getItem('naha-language') // 'en' or 'fr'
```

Automatically loaded on app start via `loadSavedLanguage()` in `App.svelte`.

## 🔄 Reactive Updates

All components using `$t()` automatically re-render when language changes:

```svelte
<script>
  import { t } from '../stores/i18nStore';
  import { language } from '../stores/appStore';
</script>

<!-- Automatically updates when $language changes -->
<h1>{$t('homepage.title')}</h1>

<!-- Current language: {$language} -->
```

## 🌐 Adding New Translations

### 1. Add to English (`en.json`)

```json
{
  "myFeature": {
    "title": "My Feature Title",
    "description": "Feature description"
  }
}
```

### 2. Add to French (`fr.json`)

```json
{
  "myFeature": {
    "title": "Titre de Ma Fonctionnalité",
    "description": "Description de la fonctionnalité"
  }
}
```

### 3. Use in Component

```svelte
<h1>{$t('myFeature.title')}</h1>
<p>{$t('myFeature.description')}</p>
```

## 🎯 Complete Translation Coverage

### ✅ Completed

1. **Language Store** (`appStore.ts`)
2. **Translation Files** (`en.json`, `fr.json`)
3. **Utility Functions** (`i18n.ts`)
4. **Reactive Store** (`i18nStore.ts`)
5. **Language Selector Component** (`LanguageSelector.svelte`)
6. **Onboarding Language Selection** (`OnboardingLanguageSelection.svelte`)
7. **NavigationBar** (translated)
8. **Language persistence** (localStorage)
9. **Auto-load on startup**

### 🔄 To Translate (Easy to add)

Components can be translated by:

1. Importing `{ t }` from `i18nStore`
2. Replacing hardcoded text with `{$t('key')}`

Example components to translate:

- HomePage
- OnboardingWelcome
- OnboardingMinecraftCheck  
- UpdateFlow components
- Dialogs and modals

## 🧪 Testing

### Switch Language

```javascript
// In browser console
await window.nahaAPI.setLanguage('fr') // Switch to French
await window.nahaAPI.setLanguage('en') // Switch to English
```

### Check Current Language

```javascript
localStorage.getItem('naha-language') // Returns 'en' or 'fr'
```

## 📊 Onboarding Flow (Updated)

### Full Onboarding

```text
Step 1: Language Selection (NEW!)
   ↓
Step 2: Welcome  
   ↓
Step 3: Minecraft Check (✅ Yes, 📥 No, 🔄 Update)
   ↓
... continue with setup
```

### From Homepage Install Modpack

```text
Step 3: Minecraft Check (skips language & welcome)
   ↓
... continue with install
```

## 🎨 Language Flags

- 🇬🇧 English
- 🇫🇷 Français

## 🔧 Technical Details

### Store Structure

```typescript
// appStore.ts
export const language = writable('en'); // 'en' or 'fr'
```

### Translation Function

```typescript
// i18nStore.ts
export const t = derived(language, ($language) => 
  (key: string, params?: Record<string, string | number>): string => {
    // Returns translated string
    // Supports {{placeholder}} substitution
    // Falls back to English if key not found
  }
);
```

### Utility Functions

```typescript
// i18n.ts
export function setLanguage(lang: 'en' | 'fr'): void;
export function loadSavedLanguage(): void;
export function getCurrentLanguage(): string;
export function getAvailableLanguages(): Array<{...}>;
```

## 📚 Translation File Structure

```json
{
  "common": { ... },          // Shared buttons/labels
  "navigation": { ... },      // Nav bar items
  "homepage": { ... },        // Homepage content
  "onboarding": {             // Onboarding flow
    "welcome": { ... },
    "minecraftCheck": { ... },
    "chooseLauncher": { ... },
    ...
  },
  "updateFlow": { ... },      // Update instance flow
  "settings": { ... },        // Settings labels
  "menu": { ... },            // Menu items
  "dialogs": { ... },         // Dialog content
  "notifications": { ... }    // Notification messages
}
```

## 🚧 Next Steps

To complete full translation:

1. **Add menu option** for language switching
2. **Translate remaining components:**
   - HomePage → Use `$t('homepage.*')`
   - OnboardingWelcome → Use `$t('onboarding.welcome.*')`
   - OnboardingMinecraftCheck → Use `$t('onboarding.minecraftCheck.*')`
   - UpdateFlow components → Use `$t('updateFlow.*')`
3. **Test both languages** thoroughly
4. **Add language icons** to settings

## 🌟 Features

- ✅ Two languages (English & French)
- ✅ Reactive updates (instant language switch)
- ✅ LocalStorage persistence
- ✅ Fallback to English if key missing
- ✅ Parameter substitution (`{{param}}`)
- ✅ Beautiful language selector UI
- ✅ Onboarding integration
- ✅ Type-safe (TypeScript)

## 💡 Pro Tips

### 1. Always use translation keys

```svelte
<!-- ❌ Bad -->
<h1>Welcome to NAHA</h1>

<!-- ✅ Good -->
<h1>{$t('onboarding.welcome.title')}</h1>
```

### 2. Keep keys organized

Use dot notation: `feature.section.element`

### 3. Test both languages

Switch between languages to ensure all text is translated.

### 4. Use parameters for dynamic content

```svelte
{$t('updateFlow.instanceSelection.modCount', { count: 53 })}
<!-- English: "53 mods" -->
<!-- French: "53 mods" -->
```

---

**Version:** 1.0.2  
**Languages:** English (🇬🇧), Français (🇫🇷)  
**Status:** ✅ Infrastructure Complete - Ready for Full Translation
