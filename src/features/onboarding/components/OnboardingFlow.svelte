<!-- src/features/onboarding/components/OnboardingFlow.svelte -->
<script>
  import { onMount } from "svelte";
  import { currentPage, debug, onboardingCompleted, onboardingStartStep, onboardingCurrentStep, hideUpdateButton, isInOnboarding } from "../../../shared/stores/appStore";
  import { completeOnboarding as saveOnboardingData } from "../../../shared/utils/onboardingUtils";
  import OnboardingLanguageSelection from "./OnboardingLanguageSelection.svelte";
  import OnboardingWelcome from "./OnboardingWelcome.svelte";
  import OnboardingMinecraftCheck from "./OnboardingMinecraftCheck.svelte";
  import OnboardingInstallLauncher from "./OnboardingInstallLauncher.svelte";
  import OnboardingChooseLauncher from "./OnboardingChooseLauncher.svelte";
  import OnboardingInstall from "./OnboardingInstall.svelte";
  import OnboardingMinecraftSetup from "./OnboardingMinecraftSetup.svelte";
  import OnboardingCreateInstance from "./OnboardingCreateInstance.svelte";
  import OnboardingInstanceProgress from "./OnboardingInstanceProgress.svelte";
  import OnboardingCompletion from "./OnboardingCompletion.svelte";

  let currentStep = 1;
  let totalSteps = 6; // Will be updated based on Minecraft status
  let minecraftInstalled = null;
  let selectedLauncher = null;
  let selectedServer = null;
  let hasInitialized = false;
  
  // Immediately sync with store before rendering
  $: if ($onboardingStartStep && !hasInitialized) {
    console.log('🔧 [FLOW] Initial sync - setting currentStep to:', $onboardingStartStep);
    currentStep = $onboardingStartStep;
  }
  
  // Set current step from store on mount (restore preserved step)
  onMount(() => {
    console.log('🔧 [FLOW] onMount - savedStep:', $onboardingCurrentStep, 'startStep:', $onboardingStartStep);
    // Use the preserved current step if available
    currentStep = $onboardingCurrentStep;
    hasInitialized = true;
    console.log('🔧 [FLOW] After mount - currentStep:', currentStep);
  });
  
  // Save current step to store whenever it changes (preserve on navigation)
  $: if (hasInitialized) {
    onboardingCurrentStep.set(currentStep);
  }
  
  // Debug selectedServer changes
  $: if (selectedServer) {
    console.log('🔧 [FLOW] selectedServer changed to:', selectedServer, 'currentStep:', currentStep);
  }
  let skipInstall = false;
  let useMinecraftSetup = false;
  let skipInstanceCreation = false;
  let createdInstances = [];
  let showInstallationProgress = false;
  let installationData = null;

  // Check if this is first time user
  $: isFirstTime = !localStorage.getItem("naha-onboarding-completed");

  function nextStep() {
    if (currentStep < totalSteps) {
      console.log('🔧 [FLOW] nextStep: advancing from', currentStep, 'to', currentStep + 1);
      currentStep++;
    } else {
      console.log('🔧 [FLOW] nextStep: already at max step', currentStep, '/', totalSteps);
    }
  }

  function previousStep() {
    console.log('🔧 [FLOW] previousStep called. currentStep:', currentStep, 'startStep:', $onboardingStartStep);
    
    // If we're at step 3 (Minecraft check) and came from homepage install button, go back to homepage
    if (currentStep === 3 && $onboardingStartStep === 3) {
      console.log('🔧 [FLOW] Going back to homepage (came from install button)');
      onboardingStartStep.set(1);
      onboardingCurrentStep.set(1);
      hideUpdateButton.set(false);
      isInOnboarding.set(false);
      // Check if onboarding was previously completed
      const wasCompleted = localStorage.getItem('naha-onboarding-completed');
      if (wasCompleted) {
        onboardingCompleted.set(true);
      }
      currentPage.set('homepage');
      return;
    }
    
    if (currentStep > 1) {
      console.log('🔧 [FLOW] Going to previous step:', currentStep - 1);
      currentStep--;
    }
  }

  function handleMinecraftResponse(event) {
    const { hasMinecraft, wantsUpdate } = event.detail;
    minecraftInstalled = hasMinecraft;
    console.log('🔧 [FLOW] minecraftInstalled set to:', hasMinecraft, 'wantsUpdate:', wantsUpdate);

    if (wantsUpdate) {
      // Redirect to update flow
      console.log('🔧 [FLOW] Redirecting to update flow, keeping step at 2');
      // Keep the step at 2 so when we come back, we're still at step 2
      // Don't change currentStep, it stays at 2
      currentPage.set('update-instance');
      return;
    }

    if (hasMinecraft) {
      // If user has Minecraft, they still need to select their launcher first
      useMinecraftSetup = true;
      totalSteps = 7; // 7 steps for users who have Minecraft: Language -> Welcome -> Check -> Launcher -> Server Selection -> Instance Creation -> Completion
    } else {
      // If user doesn't have Minecraft, they need to install a launcher
      useMinecraftSetup = false;
      totalSteps = 8; // 8 steps for users who need to install: Language -> Welcome -> Check -> Launcher -> Install -> Server Selection -> Instance Creation -> Completion
    }
    nextStep();
  }

  function handleLauncherSelected(event) {
    selectedLauncher = event.detail.launcher;
    nextStep();
  }

  function handleLauncherChecked(event) {
    selectedLauncher = event.detail.launcher;
    nextStep();
  }

  function handleServerSelected(event) {
    selectedServer = event.detail.server;
    console.log('🔧 [FLOW] handleServerSelected called, server:', event.detail.server, 'currentStep:', currentStep);
    // Don't auto-advance - let user click Next button
  }

  function handleInstallComplete() {
    // After launcher installation, advance to server selection step
    console.log('🔧 [FLOW] handleInstallComplete called, currentStep:', currentStep, 'minecraftInstalled:', minecraftInstalled);
    nextStep();
    console.log('🔧 [FLOW] After nextStep, currentStep:', currentStep);
  }

  function handleSkipInstall() {
    skipInstall = true;
    nextStep();
  }

  function handleContinue() {
    nextStep();
  }

  function handleInstanceCreated(event) {
    createdInstances.push(event.detail);
    nextStep();
  }

  function handleSkipInstanceCreation() {
    skipInstanceCreation = true;
    nextStep();
  }

  function handleCompleteOnboarding() {
    // Save completion state using utility function
    saveOnboardingData({
      minecraftInstalled,
      preferredLauncher: selectedLauncher,
      selectedServer,
      skipInstall,
      skipInstanceCreation,
      createdInstances,
    });

    // Mark onboarding as completed in the store
    onboardingCompleted.set(true);
    
    // Reset onboarding control stores
    onboardingStartStep.set(1);
    onboardingCurrentStep.set(1);
    hideUpdateButton.set(false);
    isInOnboarding.set(false);
    
    // Navigate to homepage
    currentPage.set("homepage");
  }

  function skipOnboarding() {
    handleCompleteOnboarding();
  }

  function handleStartInstallation(event) {
    installationData = event.detail;
    showInstallationProgress = true;
  }

  function handleInstallationComplete() {
    showInstallationProgress = false;
    // Mark instance as created
    if (installationData) {
      createdInstances.push({
        launcher: installationData.selectedLauncher,
        server: installationData.selectedServer,
        completed: true
      });
    }
    installationData = null;
    // Go to completion step
    nextStep();
  }

  function handleInstallationBack() {
    showInstallationProgress = false;
    installationData = null;
  }

  function handleCompletionBack() {
    previousStep();
  }

  function handleCompletionFinish() {
    handleCompleteOnboarding();
  }
</script>

<div class="onboarding-container">
  <!-- Background decoration -->
  <div class="background-decoration">
    <div class="decoration-grid"></div>
    <div class="decoration-orbs">
      <div class="orb orb-1"></div>
      <div class="orb orb-2"></div>
      <div class="orb orb-3"></div>
    </div>
  </div>

  <!-- Main content -->
  <div
    class="content-wrapper"
    class:launcher-step={currentStep === 3 ||
      (currentStep === 4 && minecraftInstalled === true) ||
      currentStep === 5}
  >
    <!-- Progress indicator -->
    <div class="progress-indicator">
      <div class="progress-bar">
        <div
          class="progress-fill"
          style="width: {(currentStep / totalSteps) * 100}%"
        ></div>
      </div>
      <div class="progress-text">Step {currentStep} of {totalSteps}</div>
    </div>

    <!-- Step content -->
    <div
      class="step-content"
      class:launcher-step={currentStep === 3 ||
        (currentStep === 4 && minecraftInstalled === true) ||
        currentStep === 5}
    >
      {#if $debug}
        <!-- Debug info -->
        <div class="debug-panel">
          <span class="debug-icon">🐛</span>
          <span class="debug-text">
            Step:{currentStep} | Minecraft:{minecraftInstalled === null
              ? "null"
              : minecraftInstalled
                ? "Y"
                : "N"} | Total:{totalSteps} | Setup:{useMinecraftSetup
              ? "Y"
              : "N"} | Launcher:{selectedLauncher || "none"} | Server:{selectedServer ||
              "none"} | Skip:{skipInstall ? "Y" : "N"} | SkipInst:{skipInstanceCreation
              ? "Y"
              : "N"} | Instances:{createdInstances.length}
          </span>
        </div>
      {/if}

      {#if showInstallationProgress && installationData}
        <OnboardingInstanceProgress
          selectedLauncher={installationData.selectedLauncher}
          selectedServer={installationData.selectedServer}
          targetLauncher={installationData.targetLauncher}
          customPath={installationData.customPath}
          on:complete={handleInstallationComplete}
          on:back={handleInstallationBack}
        />
      {:else if currentStep === 1}
        <OnboardingLanguageSelection on:next={nextStep} />
      {:else if currentStep === 2}
        <OnboardingWelcome on:next={nextStep} on:skip={skipOnboarding} />
      {:else if currentStep === 3}
        <OnboardingMinecraftCheck
          on:minecraft-response={handleMinecraftResponse}
          on:back={previousStep}
        />
      {:else if currentStep === 4}
        <!-- Launcher selection for ALL users at step 4 -->
        <OnboardingChooseLauncher
          hasMinecraft={minecraftInstalled}
          on:launcher-checked={handleLauncherChecked}
          on:back={previousStep}
        />
      {:else if currentStep === 5 && minecraftInstalled === false}
        <!-- Install instructions for users who don't have Minecraft -->
        <OnboardingInstall
          {selectedLauncher}
          on:install-complete={handleInstallComplete}
          on:skip-install={handleSkipInstall}
          on:continue={handleContinue}
          on:back={previousStep}
        />
      {:else if currentStep === 5 && minecraftInstalled === true}
        <!-- Server selection for users who have Minecraft -->
        <OnboardingCreateInstance
          hasMinecraftInstalled={true}
          isInstanceCreationMode={false}
          selectedLauncherFromFlow={selectedLauncher}
          selectedServerFromFlow={selectedServer}
          on:server-selected={handleServerSelected}
          on:next={nextStep}
          on:back={previousStep}
        />
      {:else if currentStep === 6 && minecraftInstalled === false}
        <!-- Server selection for users who don't have Minecraft (after install step) -->
        <OnboardingCreateInstance
          hasMinecraftInstalled={false}
          isInstanceCreationMode={false}
          selectedLauncherFromFlow={selectedLauncher}
          selectedServerFromFlow={selectedServer}
          on:server-selected={handleServerSelected}
          on:next={nextStep}
          on:back={previousStep}
        />
      {:else if currentStep === 6 && minecraftInstalled === true}
        <!-- Instance creation for users who have Minecraft -->
        <OnboardingCreateInstance
          hasMinecraftInstalled={true}
          isInstanceCreationMode={true}
          selectedLauncherFromFlow={selectedLauncher}
          selectedServerFromFlow={selectedServer}
          on:instanceCreated={handleInstanceCreated}
          on:continue={handleSkipInstanceCreation}
          on:back={previousStep}
          on:start-installation={handleStartInstallation}
        />
      {:else if currentStep === 7 && minecraftInstalled === false}
        <!-- Instance creation for users who don't have Minecraft -->
        <OnboardingCreateInstance
          hasMinecraftInstalled={false}
          isInstanceCreationMode={true}
          selectedLauncherFromFlow={selectedLauncher}
          selectedServerFromFlow={selectedServer}
          on:instanceCreated={handleInstanceCreated}
          on:continue={handleSkipInstanceCreation}
          on:back={previousStep}
          on:start-installation={handleStartInstallation}
        />
      {:else if currentStep === 6 && minecraftInstalled === false}
        <!-- Instance creation for users who don't have Minecraft -->
        <OnboardingCreateInstance
          hasMinecraftInstalled={false}
          selectedLauncherFromFlow={selectedLauncher}
          selectedServerFromFlow={selectedServer}
          isInstanceCreationMode={true}
          on:instanceCreated={handleInstanceCreated}
          on:continue={handleSkipInstanceCreation}
          on:back={previousStep}
          on:start-installation={handleStartInstallation}
        />
      {:else if currentStep === 7 && minecraftInstalled === true}
        <!-- Completion step for users WITH Minecraft -->
        <OnboardingCompletion
          {selectedServer}
          {selectedLauncher}
          hasMinecraftInstalled={minecraftInstalled}
          on:back={handleCompletionBack}
          on:finish={handleCompletionFinish}
        />
      {:else if currentStep === 8}
        <!-- Completion step for users WITHOUT Minecraft -->
        <OnboardingCompletion
          {selectedServer}
          {selectedLauncher}
          hasMinecraftInstalled={minecraftInstalled}
          on:back={handleCompletionBack}
          on:finish={handleCompletionFinish}
        />
      {:else}
        <!-- Fallback for any other case -->
        <div style="padding: 20px; text-align: center;">
          <h2>Unknown step</h2>
          <p>currentStep: {currentStep}</p>
          <p>minecraftInstalled: {minecraftInstalled}</p>
          <p>totalSteps: {totalSteps}</p>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .onboarding-container {
    height: 100vh;
    position: relative;
    background: linear-gradient(
      135deg,
      #0f172a 0%,
      #1e293b 25%,
      #334155 50%,
      #475569 100%
    );
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1rem;
    box-sizing: border-box;
  }

  .background-decoration {
    position: fixed;
    inset: -200px;
    pointer-events: none;
    z-index: 0;
  }

  .decoration-grid {
    position: absolute;
    inset: -200px;
    background-image:
      linear-gradient(rgba(59, 130, 246, 0.02) 1px, transparent 1px),
      linear-gradient(90deg, rgba(59, 130, 246, 0.02) 1px, transparent 1px);
    background-size: 40px 40px;
    background-position: 0 0;
    animation: gridDrift 30s linear infinite;
  }

  @keyframes gridDrift {
    0% {
      transform: translate(0, 0);
    }
    100% {
      transform: translate(40px, 40px);
    }
  }

  .decoration-orbs {
    position: absolute;
    inset: 0;
    overflow: visible;
  }

  .orb {
    position: absolute;
    border-radius: 50%;
    background: radial-gradient(
      circle,
      rgba(34, 197, 94, 0.08) 0%,
      rgba(16, 185, 129, 0.04) 50%,
      transparent 100%
    );
    animation: orbDance 15s ease-in-out infinite;
  }

  .orb-1 {
    width: 300px;
    height: 300px;
    top: 10%;
    left: 10%;
    animation-delay: 0s;
  }

  .orb-2 {
    width: 200px;
    height: 200px;
    top: 60%;
    right: 10%;
    animation-delay: 5s;
  }

  .orb-3 {
    width: 250px;
    height: 250px;
    bottom: 10%;
    left: 60%;
    animation-delay: 10s;
  }

  @keyframes orbDance {
    0%,
    100% {
      transform: translate(0, 0) scale(1) rotate(0deg);
      opacity: 0.6;
    }
    33% {
      transform: translate(30px, -30px) scale(1.1) rotate(120deg);
      opacity: 0.8;
    }
    66% {
      transform: translate(-20px, 20px) scale(0.9) rotate(240deg);
      opacity: 0.4;
    }
  }

  .content-wrapper {
    position: relative;
    z-index: 1;
    width: 100%;
    max-width: 700px;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: center;
    min-height: 100vh;
    padding-top: 2rem;
  }

  .content-wrapper.launcher-step {
    max-width: none;
    overflow: visible;
    padding-top: 2rem;
  }

  .progress-indicator {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    align-items: center;
  }

  .progress-bar {
    width: 600%;
    height: 3px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 2px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #22c55e, #16a34a);
    border-radius: 2px;
    transition: width 0.3s ease;
  }

  .progress-text {
    color: rgba(255, 255, 255, 0.7);
    font-size: 0.875rem;
    font-weight: 500;
  }

  .step-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    min-height: 300px;
    max-height: 90vh;
    overflow: visible;
    width: 100%;
  }

  .step-content.launcher-step {
    overflow: visible;
  }

  /* Dark theme adjustments */
  @media (prefers-color-scheme: dark) {
    .onboarding-container {
      background: linear-gradient(
        135deg,
        #0f172a 0%,
        #1e293b 25%,
        #0f172a 50%,
        #1e293b 100%
      );
    }

    .decoration-grid {
      background-image:
        linear-gradient(rgba(59, 130, 246, 0.03) 1px, transparent 1px),
        linear-gradient(90deg, rgba(59, 130, 246, 0.03) 1px, transparent 1px);
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

  /* Responsive design */
  @media (max-width: 768px) {
    .content-wrapper {
      padding: 1.5rem;
      gap: 1.5rem;
    }

    .step-content {
      min-height: 350px;
    }
  }

  @media (max-width: 640px) {
    .content-wrapper {
      padding: 1rem;
      gap: 1rem;
    }

    .step-content {
      min-height: 300px;
    }

    .progress-text {
      font-size: 0.8rem;
    }
  }

  /* Debug Panel Styles */
  .debug-panel {
    background: rgba(30, 41, 59, 0.9);
    border: 1px solid rgba(59, 130, 246, 0.4);
    border-radius: 4px;
    padding: 0.3rem 0.5rem;
    margin: 0.3rem auto;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    gap: 0.2rem;
    font-size: 0.6rem;
    backdrop-filter: blur(8px);
    max-width: 100%;
    width: 100%;
    box-sizing: border-box;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
    overflow: visible;
  }

  .debug-icon {
    font-size: 0.7rem;
  }

  .debug-text {
    color: rgba(255, 255, 255, 0.9);
    font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
    font-weight: 500;
    letter-spacing: 0.5px;
    white-space: normal;
    word-wrap: break-word;
    word-break: break-all;
    flex: 1;
    min-width: 0;
    line-height: 1.2;
  }
  
  /* Responsive debug panel */
  @media (max-width: 768px) {
    .debug-panel {
      font-size: 0.5rem;
      padding: 0.25rem 0.4rem;
      gap: 0.15rem;
    }
    
    .debug-text {
      letter-spacing: 0.25px;
    }
  }
  
  @media (max-width: 640px) {
    .debug-panel {
      font-size: 0.45rem;
      padding: 0.2rem 0.3rem;
      gap: 0.1rem;
    }
    
    .debug-icon {
      font-size: 0.6rem;
    }
  }
</style>
