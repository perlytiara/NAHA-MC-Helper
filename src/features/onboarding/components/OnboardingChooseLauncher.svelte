<!-- src/features/onboarding/components/OnboardingChooseLauncher.svelte -->
<script>
  import { createEventDispatcher } from "svelte";
  import { debug } from "../../../shared/stores/appStore";
  import { t } from '../../../shared/stores/i18nStore';

  const dispatch = createEventDispatcher();

  export let hasMinecraft = null; // Will be passed from parent component

  let selectedLauncher = null;

  const launchers = [
    {
      id: "prism",
      name: "Prism Launcher",
      descriptionKey: "prism",
      icon: "🎮",
      gradient: "from-blue-500 to-indigo-600",
      accent: "#3b82f6",
      badgeKey: "cracked",
      badgeColor: "blue",
      featureKeys: ["free", "modSupport", "multiPlatform"],
    },
    {
      id: "astralrinth",
      name: "AstralRinth",
      descriptionKey: "astralrinth",
      icon: "🌟",
      gradient: "from-orange-500 to-red-600",
      accent: "#f97316",
      badgeKey: "cracked",
      badgeColor: "orange",
      featureKeys: ["free", "modrinth", "multiPlatform"],
    },
    {
      id: "modrinth",
      name: "ModRinth",
      descriptionKey: "modrinth",
      icon: "🔮",
      gradient: "from-green-500 to-teal-600",
      accent: "#10b981",
      badgeKey: "official",
      badgeColor: "green",
      featureKeys: ["paid", "official", "modrinth"],
    },
    {
      id: "xmcl",
      name: "XMCL",
      descriptionKey: "xmcl",
      icon: "⚡",
      gradient: "from-purple-500 to-pink-600",
      accent: "#8b5cf6",
      badgeKey: "official",
      badgeColor: "purple",
      featureKeys: ["paid", "modern", "multiPlatform"],
    },
    {
      id: "other",
      name: "Other (Experimental)",
      descriptionKey: "other",
      icon: "🔧",
      gradient: "from-gray-500 to-slate-600",
      accent: "#6b7280",
      badgeKey: "official",
      badgeColor: "gray",
      featureKeys: ["custom", "experimental", "other"],
    },
  ];

  // Determine layout based on Minecraft status
  $: layoutConfig = hasMinecraft ? {
    // Yes Minecraft: 3 on first row, 2 on second row (centered)
    firstRow: launchers.slice(0, 3), // Prism, AstralRinth, ModRinth
    secondRow: launchers.slice(3, 5) // XMCL, Other
  } : {
    // No Minecraft: 2 on first row, 2 on second row (including ModRinth)
    firstRow: [launchers[0], launchers[1]], // Prism, AstralRinth
    secondRow: [launchers[2], launchers[3]] // ModRinth, XMCL
  };



  function handleLauncherSelect(launcherId) {
    selectedLauncher = launcherId;
  }

  function handleNext() {
    if (selectedLauncher) {
      dispatch("launcher-checked", { launcher: selectedLauncher });
    }
  }

  function handleBack() {
    dispatch("back");
  }
</script>

<div class="choose-launcher-content" class:minecraft-layout={hasMinecraft}>
  <!-- Header -->
  <div class="header-section">
    <div class="logo">
      <div class="logo-icon">⚡</div>
      <span class="logo-text">NAHA</span>
    </div>
  </div>

  <!-- Main content -->
  <div class="main-content">
    <h1 class="choose-title">{$t('onboarding.chooseLauncher.title')}</h1>

    <p class="choose-subtitle">
      {$t('onboarding.chooseLauncher.description')}
    </p>

    <!-- Debug info -->
    {#if $debug}
      <div style="background: rgba(255,0,0,0.1); padding: 0.5rem; margin: 0.5rem 0; border-radius: 4px; font-size: 0.8rem; color: white;">
        <strong>🔍 Debug:</strong> hasMinecraft = {hasMinecraft}<br>
        <strong>First Row:</strong> {layoutConfig.firstRow.map(l => l.name).join(', ')}<br>
        <strong>Second Row:</strong> {layoutConfig.secondRow.map(l => l.name).join(', ')}
      </div>
    {/if}


    <div class="launcher-options" class:minecraft-layout={hasMinecraft}>
      <!-- First Row -->
      <div class="launcher-row first-row">
        {#each layoutConfig.firstRow as launcher}
          <label
            class="launcher-option"
            class:selected={selectedLauncher === launcher.id}
          >
            <input
              type="radio"
              name="launcher"
              value={launcher.id}
              checked={selectedLauncher === launcher.id}
              on:change={() => handleLauncherSelect(launcher.id)}
            />
            <div class="option-content">
              <div class="option-header">
                <div class="option-icon">{launcher.icon}</div>
                <div
                  class="option-badge"
                  class:blue={launcher.badgeColor === "blue"}
                  class:orange={launcher.badgeColor === "orange"}
                  class:purple={launcher.badgeColor === "purple"}
                  class:green={launcher.badgeColor === "green"}
                  class:gray={launcher.badgeColor === "gray"}
                >
                  {$t(`onboarding.chooseLauncher.badges.${launcher.badgeKey}`)}
                </div>
              </div>
              <div class="option-info">
                <h3 class="option-title">{launcher.name}</h3>
                <p class="option-description">{$t(`onboarding.chooseLauncher.descriptions.${launcher.descriptionKey}`)}</p>
                <div class="option-features">
                  {#each launcher.featureKeys as featureKey}
                    <span class="feature-tag">{$t(`onboarding.chooseLauncher.features.${featureKey}`)}</span>
                  {/each}
                </div>
              </div>
            </div>
            <div class="option-background {launcher.gradient}"></div>
          </label>
        {/each}
      </div>

      <!-- Second Row -->
      <div class="launcher-row second-row">
        {#each layoutConfig.secondRow as launcher}
          <label
            class="launcher-option"
            class:selected={selectedLauncher === launcher.id}
          >
            <input
              type="radio"
              name="launcher"
              value={launcher.id}
              checked={selectedLauncher === launcher.id}
              on:change={() => handleLauncherSelect(launcher.id)}
            />
            <div class="option-content">
              <div class="option-header">
                <div class="option-icon">{launcher.icon}</div>
                <div
                  class="option-badge"
                  class:blue={launcher.badgeColor === "blue"}
                  class:orange={launcher.badgeColor === "orange"}
                  class:purple={launcher.badgeColor === "purple"}
                  class:green={launcher.badgeColor === "green"}
                  class:gray={launcher.badgeColor === "gray"}
                >
                  {$t(`onboarding.chooseLauncher.badges.${launcher.badgeKey}`)}
                </div>
              </div>
              <div class="option-info">
                <h3 class="option-title">{launcher.name}</h3>
                <p class="option-description">{$t(`onboarding.chooseLauncher.descriptions.${launcher.descriptionKey}`)}</p>
                <div class="option-features">
                  {#each launcher.featureKeys as featureKey}
                    <span class="feature-tag">{$t(`onboarding.chooseLauncher.features.${featureKey}`)}</span>
                  {/each}
                </div>
              </div>
            </div>
            <div class="option-background {launcher.gradient}"></div>
          </label>
        {/each}
      </div>
    </div>
  </div>

  <!-- Navigation -->
  <div class="navigation-section">
    <button class="back-button" on:click={handleBack}> ← {$t('common.back')} </button>

    <button
      class="next-button"
      class:disabled={!selectedLauncher}
      disabled={!selectedLauncher}
      on:click={handleNext}
    >
      {$t('common.next')}
    </button>
  </div>
</div>

<style>
  .choose-launcher-content {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    width: 100%;
    max-width: 650px;
    margin: 0 auto;
    max-height: 100vh;
    overflow: visible;
    padding: 1rem 1rem;
  }

  /* Wider content for Minecraft users */
  .choose-launcher-content.minecraft-layout {
    max-width: 950px;
  }

  /* Square content for non-Minecraft users */
  .choose-launcher-content:not(.minecraft-layout) {
    max-width: 650px;
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
    gap: 0.75rem;
    align-items: center;
    flex: 1;
    overflow: visible;
    width: 100%;
  }

  .choose-title {
    font-size: 1.25rem;
    font-weight: 700;
    color: white;
    margin: 0;
    line-height: 1.2;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    text-align: center;
  }

  .choose-subtitle {
    font-size: 0.8rem;
    color: rgba(255, 255, 255, 0.7);
    line-height: 1.4;
    margin: 0;
    text-align: center;
    max-width: 400px;
  }

  .launcher-options {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    width: 100%;
    max-width: 100%;
    flex: 0 0 auto;
    overflow: visible;
    padding: 0.5rem 0;
  }

  /* Wider container for Minecraft users (3 launchers per row) */
  .launcher-options.minecraft-layout {
    max-width: 100%;
  }

  /* Square container for non-Minecraft users (2x2 grid) */
  .launcher-options:not(.minecraft-layout) {
    max-width: 100%;
  }

  .launcher-row {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: stretch;
    gap: 0.85rem;
    width: 100%;
    flex-wrap: nowrap;
  }

  /* First row: 3 launchers for Minecraft users, 2 for non-Minecraft users */
  /* Second row: 2 launchers for Minecraft users, 2 for non-Minecraft users */

  .launcher-option {
    position: relative;
    cursor: pointer;
    border: 2px solid rgba(255, 255, 255, 0.2);
    border-radius: 8px;
    padding: 0.75rem;
    background: linear-gradient(
      145deg,
      rgba(255, 255, 255, 0.1) 0%,
      rgba(255, 255, 255, 0.05) 100%
    );
    backdrop-filter: blur(10px);
    transition: all 0.3s ease;
    overflow: hidden;
    min-height: 150px;
    display: flex;
    flex-direction: column;
  }

  /* Wider cards for non-Minecraft users (2x2 grid) */
  .launcher-options:not(.minecraft-layout) .launcher-option {
    width: 285px;
    height: 150px;
    min-height: 150px;
    max-height: 150px;
    flex: 0 0 285px;
  }

  .launcher-option:hover {
    background: rgba(255, 255, 255, 0.15);
    border-color: rgba(255, 255, 255, 0.3);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .launcher-option.selected {
    border-color: #22c55e;
    background: rgba(255, 255, 255, 0.2);
    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.2);
    transform: translateY(-1px);
  }

  .option-background {
    position: absolute;
    inset: 0;
    opacity: 0.1;
    transition: opacity 0.3s ease;
  }

  .launcher-option.selected .option-background {
    opacity: 0.2;
  }

  .from-blue-500.to-indigo-600 {
    background: linear-gradient(135deg, #3b82f6, #4f46e5);
  }

  .from-purple-500.to-pink-600 {
    background: linear-gradient(135deg, #8b5cf6, #db2777);
  }

  .from-orange-500.to-red-600 {
    background: linear-gradient(135deg, #f97316, #dc2626);
  }

  .from-gray-500.to-slate-600 {
    background: linear-gradient(135deg, #6b7280, #475569);
  }

  .from-green-500.to-teal-600 {
    background: linear-gradient(135deg, #10b981, #0d9488);
  }

  .launcher-option input[type="radio"] {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }

  .option-content {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    position: relative;
    z-index: 2;
    height: 100%;
    justify-content: space-between;
  }

  .option-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 0.25rem;
  }

  .option-icon {
    font-size: 1.25rem;
    filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
    transition: transform 0.3s ease;
  }

  .launcher-option:hover .option-icon {
    transform: scale(1.1);
  }

  .option-badge {
    padding: 0.2rem 0.6rem;
    border-radius: 16px;
    font-size: 0.7rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
  }

  .option-badge.blue {
    background: linear-gradient(135deg, #3b82f6, #2563eb);
    color: white;
  }

  .option-badge.orange {
    background: linear-gradient(135deg, #f97316, #ea580c);
    color: white;
  }

  .option-badge.purple {
    background: linear-gradient(135deg, #8b5cf6, #7c3aed);
    color: white;
  }

  .option-badge.green {
    background: linear-gradient(135deg, #10b981, #059669);
    color: white;
  }

  .option-badge.gray {
    background: linear-gradient(135deg, #6b7280, #4b5563);
    color: white;
    border: 1px dashed rgba(255, 255, 255, 0.3);
  }

  .option-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
    margin-bottom: 0.25rem;
  }

  .option-title {
    font-size: 0.9rem;
    font-weight: 700;
    color: white;
    margin: 0;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
    transition: color 0.3s ease;
    line-height: 1.2;
  }

  .launcher-option.selected .option-title {
    color: #60a5fa;
  }

  .option-description {
    font-size: 0.7rem;
    color: rgba(255, 255, 255, 0.7);
    line-height: 1.2;
    transition: color 0.3s ease;
    margin: 0;
  }

  .launcher-option.selected .option-description {
    color: rgba(255, 255, 255, 0.9);
  }

  .option-features {
    display: flex;
    flex-wrap: wrap;
    gap: 0.15rem;
    margin-top: auto;
    justify-content: center;
  }

  /* Add spacing between description and badges for Minecraft users */
  .launcher-options.minecraft-layout .option-features {
    margin-top: 0.5rem;
  }

  .feature-tag {
    background: rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.8);
    padding: 0.1rem 0.3rem;
    border-radius: 4px;
    font-size: 0.65rem;
    font-weight: 500;
    border: 1px solid rgba(255, 255, 255, 0.2);
    white-space: nowrap;
  }

  .launcher-option.selected .feature-tag {
    background: rgba(255, 255, 255, 0.15);
    color: rgba(255, 255, 255, 0.9);
    border-color: rgba(255, 255, 255, 0.3);
  }

  .navigation-section {
    display: inline-flex !important;
    justify-content: center !important;
    align-items: center !important;
    gap: 1.5rem !important;
    margin: 0.75rem auto 0 auto !important;
    flex-shrink: 0;
    padding: 0.5rem 1rem;
    width: auto !important;
    max-width: 300px !important;
  }

  /* More specific selector to override any conflicts */
  div.navigation-section {
    display: inline-flex !important;
    justify-content: center !important;
    align-items: center !important;
    gap: 1.5rem !important;
    margin: 0.75rem auto 0 auto !important;
    width: auto !important;
    max-width: 300px !important;
  }

  /* Global style to force the layout */
  :global(.navigation-section) {
    display: inline-flex !important;
    justify-content: center !important;
    align-items: center !important;
    gap: 1.5rem !important;
    margin: 0.75rem auto 0 auto !important;
    width: auto !important;
    max-width: 300px !important;
  }

  .back-button {
    background: transparent;
    color: rgba(255, 255, 255, 0.6);
    border: 1px solid rgba(255, 255, 255, 0.2);
    padding: 0.5rem 0.75rem;
    border-radius: 6px;
    font-size: 0.8rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .back-button:hover {
    background: rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.8);
    border-color: rgba(255, 255, 255, 0.3);
  }

  .next-button {
    background: linear-gradient(135deg, #22c55e, #16a34a);
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-size: 0.8rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
    box-shadow: 0 4px 16px rgba(34, 197, 94, 0.3);
  }

  .next-button:hover:not(.disabled) {
    background: linear-gradient(135deg, #16a34a, #15803d);
    transform: translateY(-1px);
    box-shadow: 0 6px 20px rgba(34, 197, 94, 0.4);
  }

  .next-button.disabled {
    background: rgba(255, 255, 255, 0.2);
    color: rgba(255, 255, 255, 0.4);
    cursor: not-allowed;
    box-shadow: none;
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .choose-launcher-content {
      gap: 0.5rem;
      max-width: 500px;
      padding: 0.75rem 0.25rem;
    }

    .choose-title {
      font-size: 1.1rem;
    }

    .choose-subtitle {
      font-size: 0.75rem;
      max-width: 350px;
    }

    .launcher-options {
      gap: 0.5rem;
      max-width: 400px;
      padding: 0.4rem 0;
    }

    .launcher-options.minecraft-layout {
      max-width: 600px;
    }

    .launcher-options:not(.minecraft-layout) {
      max-width: 400px;
    }

    .launcher-options:not(.minecraft-layout) .launcher-option {
      width: 220px;
      height: 120px;
      min-height: 120px;
      max-height: 120px;
    }

    .launcher-row {
      flex-direction: column;
      gap: 0.5rem;
    }

    .launcher-option {
      padding: 0.6rem 0.75rem;
      min-height: 120px;
    }

    .option-icon {
      font-size: 1.1rem;
    }

    .option-title {
      font-size: 0.8rem;
    }

    .option-description {
      font-size: 0.65rem;
    }

    .option-badge {
      font-size: 0.6rem;
      padding: 0.1rem 0.4rem;
    }

    .feature-tag {
      font-size: 0.55rem;
      padding: 0.06rem 0.2rem;
    }
  }

  @media (max-width: 640px) {
    .choose-launcher-content {
      gap: 0.4rem;
      max-width: 400px;
      padding: 0.5rem 0.25rem;
    }

    .logo {
      padding: 0.3rem 0.6rem;
      gap: 0.4rem;
    }

    .logo-icon {
      width: 18px;
      height: 18px;
      font-size: 9px;
    }

    .logo-text {
      font-size: 1.1rem;
    }

    .choose-title {
      font-size: 1rem;
    }

    .choose-subtitle {
      font-size: 0.7rem;
      max-width: 300px;
    }

    .launcher-options {
      gap: 0.4rem;
      max-width: 350px;
      padding: 0.3rem 0;
    }

    .launcher-options:not(.minecraft-layout) {
      max-width: 350px;
    }

    .launcher-options:not(.minecraft-layout) .launcher-option {
      width: 200px;
      height: 100px;
      min-height: 100px;
      max-height: 100px;
    }

    .launcher-option {
      padding: 0.5rem 0.6rem;
      min-height: 100px;
    }

    .option-icon {
      font-size: 1rem;
    }

    .option-title {
      font-size: 0.75rem;
    }

    .option-description {
      font-size: 0.6rem;
    }

    .option-badge {
      font-size: 0.55rem;
      padding: 0.08rem 0.3rem;
    }

    .feature-tag {
      font-size: 0.5rem;
      padding: 0.05rem 0.15rem;
    }

    .back-button,
    .next-button {
      padding: 0.4rem 0.6rem;
      font-size: 0.7rem;
    }
  }
</style>

