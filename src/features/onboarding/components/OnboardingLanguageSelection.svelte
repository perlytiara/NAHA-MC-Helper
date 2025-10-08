<!-- src/features/onboarding/components/OnboardingLanguageSelection.svelte -->
<script>
  import { createEventDispatcher } from 'svelte';
  import { language } from '../../../shared/stores/appStore';
  import { setLanguage } from '../../../shared/utils/i18n.ts';
  
  const dispatch = createEventDispatcher();
  
  function handleEnglish() {
    setLanguage('en');
    dispatch('next');
  }
  
  function handleFrench() {
    setLanguage('fr');
    dispatch('next');
  }
</script>

<div class="language-content">
  <!-- Header -->
  <div class="header-section">
    <div class="logo">
      <div class="logo-icon">⚡</div>
      <span class="logo-text">NAHA</span>
    </div>
  </div>
  
  <!-- Main content -->
  <div class="main-content">
    <h1 class="question-title">Choose Your Language / Choisissez Votre Langue</h1>
    
    <p class="description-text">
      Select your preferred language • Sélectionnez votre langue préférée
    </p>
    
    <div class="language-cards">
      <button class="language-card english-card" on:click={handleEnglish}>
        <div class="card-flag">🇬🇧</div>
        <div class="card-title">English</div>
        <div class="card-subtitle">United States / United Kingdom</div>
      </button>
      
      <button class="language-card french-card" on:click={handleFrench}>
        <div class="card-flag">🇫🇷</div>
        <div class="card-title">Français</div>
        <div class="card-subtitle">France / Canada</div>
      </button>
    </div>
  </div>
</div>

<style>
  .language-content {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    text-align: center;
    width: 100%;
    max-width: 600px;
    margin: 0 auto;
  }
  
  .header-section {
    display: flex;
    justify-content: center;
  }
  
  .logo {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 1rem;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }
  
  .logo-icon {
    width: 24px;
    height: 24px;
    background: linear-gradient(135deg, #22c55e, #16a34a);
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    color: white;
    box-shadow: 0 2px 8px rgba(34, 197, 94, 0.3);
  }
  
  .logo-text {
    font-size: 1.5rem;
    font-weight: 800;
    color: white;
    letter-spacing: -0.5px;
  }
  
  .main-content {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    align-items: center;
  }
  
  .question-title {
    font-size: 2rem;
    font-weight: 700;
    color: white;
    margin: 0;
    line-height: 1.3;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  }
  
  .description-text {
    font-size: 1rem;
    color: rgba(255, 255, 255, 0.7);
    line-height: 1.6;
    margin: 0;
    max-width: 500px;
  }
  
  .language-cards {
    display: flex;
    gap: 1.5rem;
    width: 100%;
    max-width: 550px;
  }
  
  .language-card {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    padding: 2rem 1.5rem;
    border: 2px solid rgba(255, 255, 255, 0.2);
    border-radius: 16px;
    background: rgba(255, 255, 255, 0.05);
    backdrop-filter: blur(10px);
    cursor: pointer;
    transition: all 0.3s ease;
  }
  
  .language-card:hover {
    transform: translateY(-4px);
    border-color: rgba(255, 255, 255, 0.4);
    background: rgba(255, 255, 255, 0.1);
  }
  
  .language-card:active {
    transform: translateY(-2px);
  }
  
  .english-card:hover {
    border-color: #3b82f6;
    box-shadow: 0 8px 32px rgba(59, 130, 246, 0.3);
  }
  
  .french-card:hover {
    border-color: #8b5cf6;
    box-shadow: 0 8px 32px rgba(139, 92, 246, 0.3);
  }
  
  .card-flag {
    font-size: 4rem;
    margin-bottom: 0.5rem;
  }
  
  .card-title {
    font-size: 1.5rem;
    font-weight: 700;
    color: white;
    margin: 0;
  }
  
  .card-subtitle {
    font-size: 0.85rem;
    color: rgba(255, 255, 255, 0.6);
    line-height: 1.5;
    margin: 0;
  }
  
  @media (max-width: 768px) {
    .language-cards {
      flex-direction: column;
      gap: 1rem;
    }
    
    .language-card {
      padding: 1.5rem 1rem;
    }
    
    .card-flag {
      font-size: 3rem;
    }
  }
  
  @media (max-width: 640px) {
    .language-content {
      gap: 1.5rem;
    }
    
    .question-title {
      font-size: 1.5rem;
    }
    
    .description-text {
      font-size: 0.9rem;
    }
  }
</style>

