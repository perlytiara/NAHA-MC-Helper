<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { fade, fly } from 'svelte/transition';
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

  const stepTitles = [
    'Choose Your Launcher',
    'Select Modpack Type',
    'Pick Your Instance',
    'Updating...'
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
      <div in:fly={{ x: -50, duration: 300 }} out:fly={{ x: 50, duration: 300 }}>
        <UpdateLauncherSelection on:launcherSelected={handleLauncherSelected} on:back={handleBack} />
      </div>
    {:else if currentStep === 2}
      <div in:fly={{ x: -50, duration: 300 }} out:fly={{ x: 50, duration: 300 }}>
        <UpdateModpackSelection 
          launcher={selectedLauncher} 
          on:modpackSelected={handleModpackSelected} 
          on:back={handleBack} 
        />
      </div>
    {:else if currentStep === 3}
      <div in:fly={{ x: -50, duration: 300 }} out:fly={{ x: 50, duration: 300 }}>
        <UpdateInstanceSelection 
          launcher={selectedLauncher}
          modpackType={selectedModpackType}
          on:instanceSelected={handleInstanceSelected}
          on:back={handleBack}
        />
      </div>
    {:else if currentStep === 4}
      <div in:fly={{ x: -50, duration: 300 }}>
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
    width: 100%;
    max-width: 700px;
    margin: 0 auto;
    padding: 1rem;
    height: 100vh;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .progress-bar-container {
    background: rgba(16, 185, 129, 0.1);
    border: 2px solid rgba(16, 185, 129, 0.3);
    border-radius: 12px;
    padding: 1rem;
    backdrop-filter: blur(10px);
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
    flex: 1;
    overflow-y: auto;
    background: rgba(255, 255, 255, 0.05);
    border: 2px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 1.5rem;
    backdrop-filter: blur(10px);
  }

  @media (max-width: 640px) {
    .update-flow-container {
      padding: 0.5rem;
      gap: 0.5rem;
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

    .content-area {
      padding: 1rem;
    }
  }
</style>

