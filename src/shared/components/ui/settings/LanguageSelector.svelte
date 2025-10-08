<!-- src/shared/components/ui/settings/LanguageSelector.svelte -->
<script>
  import { language } from '../../../stores/appStore';
  import { t } from '../../../stores/i18nStore';
  import { setLanguage, getAvailableLanguages } from '../../../utils/i18n.ts';
  
  const languages = getAvailableLanguages();
  
  function handleLanguageChange(langCode) {
    setLanguage(langCode);
  }
</script>

<div class="language-selector">
  <div class="selector-label" role="heading" aria-level="3">{$t('settings.language')}</div>
  
  <div class="language-buttons" role="radiogroup" aria-label={$t('settings.language')}>
    {#each languages as lang}
      <button
        class="language-button"
        class:active={$language === lang.code}
        on:click={() => handleLanguageChange(lang.code)}
        role="radio"
        aria-checked={$language === lang.code}
      >
        <span class="language-flag">
          {#if lang.code === 'en'}🇬🇧{:else if lang.code === 'fr'}🇫🇷{/if}
        </span>
        <span class="language-name">{lang.nativeName}</span>
      </button>
    {/each}
  </div>
</div>

<style>
  .language-selector {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .selector-label {
    font-size: 0.9rem;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.8);
  }
  
  .language-buttons {
    display: flex;
    gap: 0.75rem;
  }
  
  .language-button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1.25rem;
    background: rgba(255, 255, 255, 0.05);
    border: 2px solid rgba(255, 255, 255, 0.2);
    border-radius: 10px;
    color: rgba(255, 255, 255, 0.8);
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
  }
  
  .language-button:hover {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.4);
    transform: translateY(-2px);
  }
  
  .language-button.active {
    background: linear-gradient(135deg, #22c55e, #16a34a);
    border-color: #22c55e;
    color: white;
    box-shadow: 0 4px 16px rgba(34, 197, 94, 0.3);
  }
  
  .language-button.active:hover {
    background: linear-gradient(135deg, #16a34a, #15803d);
  }
  
  .language-flag {
    font-size: 1.5rem;
  }
  
  .language-name {
    font-weight: 600;
  }
  
  @media (max-width: 640px) {
    .language-buttons {
      flex-direction: column;
      width: 100%;
    }
    
    .language-button {
      width: 100%;
      justify-content: center;
    }
  }
</style>

