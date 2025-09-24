<!-- src/features/onboarding/components/OnboardingInstallLauncher.svelte -->
<script>
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  let selectedLauncher = null;

  const launchers = [
    {
      id: "cracked",
      name: "Prism Launcher",
      description: "A user-friendly launcher with mod support.",
      icon: "🎮",
      gradient: "from-blue-500 to-indigo-600",
      accent: "#3b82f6",
      badge: "Cracked",
      badgeColor: "orange",
      features: ["Free", "Mod Support", "Multi-Platform"],
    },
    {
      id: "official",
      name: "XMCL",
      description: "A lightweight and fast launcher.",
      icon: "⚡",
      gradient: "from-purple-500 to-pink-600",
      accent: "#8b5cf6",
      badge: "Official",
      badgeColor: "blue",
      features: ["Paid", "Modern", "Multi-Platform"],
    },
    {
      id: "astralrinth",
      name: "AstralRinth",
      description: "A launcher with advanced features.",
      icon: "🌟",
      gradient: "from-orange-500 to-red-600",
      accent: "#f97316",
      badge: "Cracked",
      badgeColor: "orange",
      features: ["Free", "Modrinth", "Multi-Platform"],
    },
  ];

  function handleLauncherSelect(launcherId) {
    selectedLauncher = launcherId;
  }

  function handleNext() {
    if (selectedLauncher) {
      dispatch("launcher-selected", { launcher: selectedLauncher });
    }
  }

  function handleBack() {
    dispatch("back");
  }
</script>

<div class="install-launcher-content">
  <!-- Header -->
  <div class="header-section">
    <div class="logo">
      <div class="logo-icon">⚡</div>
      <span class="logo-text">NAHA</span>
    </div>
  </div>

  <!-- Main content -->
  <div class="main-content">
    <h1 class="install-title">Choose your Minecraft Launcher</h1>

    <p class="install-subtitle">
      Select one of the following launchers to proceed with the installation.
    </p>

    <div class="launcher-options">
      {#each launchers as launcher}
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
                class:green={launcher.badgeColor === "green"}
                class:blue={launcher.badgeColor === "blue"}
                class:orange={launcher.badgeColor === "orange"}
              >
                {launcher.badge}
              </div>
            </div>
            <div class="option-info">
              <h3 class="option-title">{launcher.name}</h3>
              <p class="option-description">{launcher.description}</p>
              <div class="option-features">
                {#each launcher.features as feature}
                  <span class="feature-tag">{feature}</span>
                {/each}
              </div>
            </div>
          </div>
          <div class="option-background {launcher.gradient}"></div>
        </label>
      {/each}
    </div>
  </div>

  <!-- Navigation -->
  <div class="navigation-section">
    <button class="back-button" on:click={handleBack}> ← Back </button>

    <button
      class="next-button"
      class:disabled={!selectedLauncher}
      disabled={!selectedLauncher}
      on:click={handleNext}
    >
      Next
    </button>
  </div>
</div>

<style>
  .install-launcher-content {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    width: 100%;
    max-width: none;
    margin: 0 auto;
    max-height: 100vh;
    overflow: visible;
    padding: 1rem 0.5rem;
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
    overflow: hidden;
  }

  .install-title {
    font-size: 1.25rem;
    font-weight: 700;
    color: white;
    margin: 0;
    line-height: 1.2;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
    text-align: center;
  }

  .install-subtitle {
    font-size: 0.8rem;
    color: rgba(255, 255, 255, 0.7);
    line-height: 1.4;
    margin: 0;
    text-align: center;
    max-width: 400px;
  }

  .launcher-options {
    display: flex;
    flex-direction: row;
    gap: 0.75rem;
    width: 100%;
    max-width: none;
    flex: 1;
    overflow-x: visible;
    overflow-y: visible;
    justify-content: center;
    align-items: stretch;
    padding: 0.5rem 0;
    min-width: fit-content;
  }

  .launcher-option {
    position: relative;
    cursor: pointer;
    border: 2px solid rgba(255, 255, 255, 0.2);
    border-radius: 8px;
    padding: 0.8rem;
    background: linear-gradient(
      145deg,
      rgba(255, 255, 255, 0.1) 0%,
      rgba(255, 255, 255, 0.05) 100%
    );
    backdrop-filter: blur(10px);
    transition: all 0.3s ease;
    overflow: hidden;
    min-height: 140px;
    width: 280px;
    flex: 0 0 280px;
    display: flex;
    flex-direction: column;
  }

  .launcher-option:hover {
    background: rgba(255, 255, 255, 0.15);
    border-color: rgba(255, 255, 255, 0.3);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }

  .launcher-option.selected {
    border-color: var(--accent-color);
    background: rgba(255, 255, 255, 0.2);
    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.2);
    transform: translateY(-1px);
  }

  .launcher-option.recommended {
    border-color: rgba(34, 197, 94, 0.5);
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
    padding: 0.3rem 0.8rem;
    border-radius: 16px;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
    white-space: nowrap;
  }

  .option-badge.green {
    background: linear-gradient(135deg, #22c55e, #16a34a);
    color: white;
  }

  .option-badge.blue {
    background: linear-gradient(135deg, #3b82f6, #2563eb);
    color: white;
  }

  .option-badge.orange {
    background: linear-gradient(135deg, #f97316, #ea580c);
    color: white;
  }

  .option-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
    margin-bottom: 0.25rem;
  }

  .option-title {
    font-size: 0.95rem;
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
    font-size: 0.75rem;
    color: rgba(255, 255, 255, 0.7);
    line-height: 1.3;
    transition: color 0.3s ease;
    margin: 0;
  }

  .launcher-option.selected .option-description {
    color: rgba(255, 255, 255, 0.9);
  }

  .option-features {
    display: flex;
    flex-wrap: nowrap;
    gap: 0.1rem;
    margin-top: auto;
    justify-content: center;
    overflow-x: auto;
  }

  .feature-tag {
    background: rgba(255, 255, 255, 0.1);
    color: rgba(255, 255, 255, 0.8);
    padding: 0.08rem 0.25rem;
    border-radius: 4px;
    font-size: 0.6rem;
    font-weight: 500;
    border: 1px solid rgba(255, 255, 255, 0.2);
    white-space: nowrap;
    flex-shrink: 0;
  }

  .launcher-option.selected .feature-tag {
    background: rgba(255, 255, 255, 0.15);
    color: rgba(255, 255, 255, 0.9);
    border-color: rgba(255, 255, 255, 0.3);
  }

  .navigation-section {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    margin-top: 0.75rem;
    flex-shrink: 0;
    padding-top: 0.5rem;
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
    .install-launcher-content {
      gap: 0.5rem;
      max-width: none;
      padding: 0.75rem 0.25rem;
    }

    .install-title {
      font-size: 1.1rem;
    }

    .install-subtitle {
      font-size: 0.75rem;
      max-width: 350px;
    }

    .launcher-options {
      flex-direction: row;
      gap: 0.5rem;
      max-width: none;
      padding: 0.4rem 0;
    }

    .launcher-option {
      padding: 0.6rem 0.7rem;
      min-height: 120px;
      width: 220px;
      flex: 0 0 220px;
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
      font-size: 0.65rem;
      padding: 0.2rem 0.5rem;
    }

    .feature-tag {
      font-size: 0.55rem;
      padding: 0.06rem 0.2rem;
    }
  }

  @media (max-width: 640px) {
    .install-launcher-content {
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

    .install-title {
      font-size: 1rem;
    }

    .install-subtitle {
      font-size: 0.7rem;
      max-width: 300px;
    }

    .launcher-options {
      flex-direction: row;
      gap: 0.3rem;
      max-width: 380px;
      padding: 0.3rem 0;
    }

    .launcher-option {
      padding: 0.4rem 0.5rem;
      min-height: 90px;
      min-width: 100px;
      max-width: 120px;
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
