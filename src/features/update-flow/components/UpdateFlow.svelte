<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { fade } from 'svelte/transition';
  import { t } from '../../../shared/stores/i18nStore';
  import UpdateLauncherSelection from './UpdateLauncherSelection.svelte';
  import UpdateModpackSelection from './UpdateModpackSelection.svelte';
  import UpdateInstanceSelection from './UpdateInstanceSelection.svelte';
  import UpdateProgress from './UpdateProgress.svelte';
  import type { MinecraftInstance } from '../types';

  const dispatch = createEventDispatcher();

  let currentStep = 1;
  const totalSteps = 4;
  
  let selectedLauncher: string | null = null;
  let selectedModpackType: string | null = null;
  let selectedInstance: MinecraftInstance | null = null;
  let isUpdating = false;

  $: stepTitles = [
    $t('updateFlow.steps.chooseLauncher'),
    $t('updateFlow.steps.selectModpack'),
    $t('updateFlow.steps.pickInstance'),
    $t('updateFlow.steps.updating')
  ];

  function handleLauncherSelected(event: CustomEvent) {
    selectedLauncher = event.detail.launcher;
    currentStep = 2;
  }

  function handleModpackSelected(event: CustomEvent) {
    selectedModpackType = event.detail.modpackType;
    currentStep = 3;
  }

  function handleInstanceSelected(event: CustomEvent) {
    selectedInstance = event.detail.instance;
    currentStep = 4;
    startUpdate();
  }

  async function startUpdate() {
    isUpdating = true;
    // The UpdateProgress component will handle the actual update
  }

  function handleUpdateComplete() {
    dispatch('complete');
  }

  function handleBack() {
    if (currentStep > 1 && currentStep < 4) {
      currentStep--;
    } else {
      dispatch('back');
    }
  }
</script>

<div class="update-flow-container" transition:fade>
  <!-- Decorative background elements -->
  <div class="decoration-pattern"></div>
  <div class="orb orb-1"></div>
  <div class="orb orb-2"></div>
  <div class="orb orb-3"></div>
  
  <!-- Progress Bar -->
  <div class="progress-bar-container">
    <div class="progress-steps">
      {#each Array(totalSteps) as _, i}
        <div class="step-indicator" class:active={i + 1 <= currentStep} class:completed={i + 1 < currentStep}>
          <div class="step-circle">
            {#if i + 1 < currentStep}
              ✓
            {:else}
              {i + 1}
            {/if}
          </div>
          <div class="step-label">{stepTitles[i]}</div>
        </div>
        {#if i < totalSteps - 1}
          <div class="step-connector" class:active={i + 1 < currentStep}></div>
        {/if}
      {/each}
    </div>
  </div>

  <!-- Content Area -->
  <div class="content-area">
    {#if currentStep === 1}
      <div class="step-wrapper" in:fade={{ duration: 200 }}>
        <UpdateLauncherSelection on:launcherSelected={handleLauncherSelected} on:back={handleBack} />
      </div>
    {:else if currentStep === 2}
      <div class="step-wrapper" in:fade={{ duration: 200 }}>
        <UpdateModpackSelection 
          launcher={selectedLauncher} 
          on:modpackSelected={handleModpackSelected} 
          on:back={handleBack} 
        />
      </div>
    {:else if currentStep === 3}
      <div class="step-wrapper" in:fade={{ duration: 200 }}>
        <UpdateInstanceSelection 
          launcher={selectedLauncher}
          modpackType={selectedModpackType}
          on:instanceSelected={handleInstanceSelected}
          on:back={handleBack}
        />
      </div>
    {:else if currentStep === 4}
      <div class="step-wrapper" in:fade={{ duration: 200 }}>
        <UpdateProgress 
          instance={selectedInstance}
          modpackType={selectedModpackType}
          on:complete={handleUpdateComplete}
        />
      </div>
    {/if}
  </div>
</div>

<style>
  .update-flow-container {
    position: relative;
    width: 100%;
    height: 100vh;
    overflow: hidden;
    background: linear-gradient(
      135deg,
      #0f172a 0%,
      #1e293b 25%,
      #334155 50%,
      #475569 100%
    );
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 2rem;
    padding: 2rem;
  }
  
  /* Decorative grid pattern */
  .decoration-pattern {
    position: absolute;
    inset: 0;
    background-image: 
      linear-gradient(rgba(255, 255, 255, 0.03) 1px, transparent 1px),
      linear-gradient(90deg, rgba(255, 255, 255, 0.03) 1px, transparent 1px);
    background-size: 50px 50px;
    pointer-events: none;
    z-index: 0;
  }
  
  /* Floating orbs */
  .orb {
    position: absolute;
    border-radius: 50%;
    background: radial-gradient(
      circle,
      rgba(34, 197, 94, 0.08) 0%,
      rgba(16, 185, 129, 0.04) 50%,
      transparent 100%
    );
    pointer-events: none;
    animation: float 20s ease-in-out infinite;
    z-index: 0;
  }
  
  .orb-1 {
    width: 400px;
    height: 400px;
    top: -100px;
    left: -100px;
    animation-delay: 0s;
  }
  
  .orb-2 {
    width: 300px;
    height: 300px;
    top: 50%;
    right: -50px;
    animation-delay: -7s;
  }
  
  .orb-3 {
    width: 350px;
    height: 350px;
    bottom: -100px;
    left: 30%;
    animation-delay: -14s;
  }
  
  @keyframes float {
    0%, 100% {
      transform: translate(0, 0) scale(1);
    }
    33% {
      transform: translate(30px, -30px) scale(1.1);
    }
    66% {
      transform: translate(-20px, 20px) scale(0.9);
    }
  }

  .progress-bar-container {
    position: relative;
    z-index: 1;
    background: rgba(16, 185, 129, 0.1);
    border: 2px solid rgba(16, 185, 129, 0.3);
    border-radius: 12px;
    padding: 0.75rem;
    backdrop-filter: blur(10px);
    flex-shrink: 0;
    max-width: 900px;
    margin: 0 auto;
    width: 100%;
  }

  .progress-steps {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .step-indicator {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    flex: 0 0 auto;
  }

  .step-circle {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.1);
    border: 2px solid rgba(255, 255, 255, 0.3);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    font-size: 1rem;
    color: rgba(255, 255, 255, 0.6);
    transition: all 0.3s ease;
  }

  .step-indicator.active .step-circle {
    background: linear-gradient(135deg, #10b981, #059669);
    border-color: #10b981;
    color: white;
    box-shadow: 0 0 20px rgba(16, 185, 129, 0.5);
  }

  .step-indicator.completed .step-circle {
    background: linear-gradient(135deg, #059669, #047857);
    border-color: #059669;
    color: white;
  }

  .step-label {
    font-size: 0.7rem;
    color: rgba(255, 255, 255, 0.6);
    text-align: center;
    max-width: 80px;
    line-height: 1.2;
  }

  .step-indicator.active .step-label {
    color: #10b981;
    font-weight: 600;
  }

  .step-connector {
    flex: 1;
    height: 2px;
    background: rgba(255, 255, 255, 0.2);
    margin: 0 0.5rem;
    margin-bottom: 2rem;
    transition: all 0.3s ease;
  }

  .step-connector.active {
    background: linear-gradient(90deg, #059669, #10b981);
  }

  .content-area {
    position: relative;
    z-index: 1;
    flex: 1;
    overflow: hidden;
    background: rgba(255, 255, 255, 0.05);
    border: 2px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 0;
    backdrop-filter: blur(10px);
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 0;
  }
  
  .step-wrapper {
    width: 100%;
    height: 100%;
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .step-wrapper > :global(div) {
    width: 100%;
    max-height: 100%;
  }
  
  /* Dark theme adjustments */
  @media (prefers-color-scheme: dark) {
    .update-flow-container {
      background: linear-gradient(
        135deg,
        #0f172a 0%,
        #1e293b 25%,
        #0f172a 50%,
        #1e293b 100%
      );
    }
    
    .decoration-pattern {
      background-size: 40px 40px;
      background-position: 0 0;
    }
    
    .orb {
      background: radial-gradient(
        circle,
        rgba(34, 197, 94, 0.12) 0%,
        rgba(16, 185, 129, 0.06) 50%,
        transparent 100%
      );
    }
  }

  @media (max-width: 640px) {
    .update-flow-container {
      padding: 1rem;
      gap: 1.5rem;
    }

    .step-circle {
      width: 32px;
      height: 32px;
      font-size: 0.85rem;
    }

    .step-label {
      font-size: 0.6rem;
      max-width: 60px;
    }

    .orb-1,
    .orb-2,
    .orb-3 {
      opacity: 0.5;
    }
  }
</style>

