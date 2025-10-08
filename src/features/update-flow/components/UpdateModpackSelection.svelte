<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  
  export let launcher: string | null;
  
  const dispatch = createEventDispatcher();

  const modpacks = [
    {
      type: 'neoforge',
      name: 'NeoForge',
      icon: '⚒️',
      description: 'Modern modding platform',
      color: '#f97316'
    },
    {
      type: 'fabric',
      name: 'Fabric',
      icon: '🧵',
      description: 'Lightweight and fast',
      color: '#8b5cf6'
    }
  ];

  function selectModpack(modpackType: string) {
    dispatch('modpackSelected', { modpackType });
  }

  function goBack() {
    dispatch('back');
  }
</script>

<div class="modpack-selection">
  <h2 class="title">Choose Your Modpack Type</h2>
  <p class="subtitle">Select the type of modpack you want to update in {launcher}</p>

  <div class="modpack-grid">
    {#each modpacks as modpack}
      <button 
        class="modpack-card" 
        style="--accent-color: {modpack.color}"
        on:click={() => selectModpack(modpack.type)}
      >
        <div class="modpack-icon">{modpack.icon}</div>
        <h3 class="modpack-name">{modpack.name}</h3>
        <p class="modpack-description">{modpack.description}</p>
        <div class="select-indicator">
          <span class="arrow">→</span>
          <span>Select</span>
        </div>
      </button>
    {/each}
  </div>

  <div class="actions">
    <button class="back-button" on:click={goBack}>
      ← Back
    </button>
  </div>
</div>

<style>
  .modpack-selection {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .title {
    font-size: 1.5rem;
    font-weight: 800;
    text-align: center;
    background: linear-gradient(45deg, #10b981, #06b6d4);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin: 0;
  }

  .subtitle {
    font-size: 0.9rem;
    color: rgba(255, 255, 255, 0.7);
    text-align: center;
    margin: 0;
  }

  .modpack-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1.5rem;
    margin: 2rem 0;
  }

  .modpack-card {
    background: rgba(255, 255, 255, 0.08);
    border: 2px solid rgba(255, 255, 255, 0.15);
    border-radius: 16px;
    padding: 2rem 1.5rem;
    cursor: pointer;
    transition: all 0.3s ease;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    text-align: center;
    position: relative;
    overflow: hidden;
  }

  .modpack-card::before {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(135deg, 
      var(--accent-color, #10b981) 0%, 
      transparent 70%
    );
    opacity: 0;
    transition: opacity 0.3s ease;
  }

  .modpack-card:hover {
    transform: translateY(-6px);
    border-color: var(--accent-color, #10b981);
    box-shadow: 0 12px 32px rgba(16, 185, 129, 0.3);
  }

  .modpack-card:hover::before {
    opacity: 0.1;
  }

  .modpack-icon {
    font-size: 3rem;
    filter: drop-shadow(0 4px 12px rgba(0, 0, 0, 0.3));
    position: relative;
    z-index: 1;
  }

  .modpack-name {
    font-size: 1.25rem;
    font-weight: 700;
    color: white;
    margin: 0;
    position: relative;
    z-index: 1;
  }

  .modpack-description {
    font-size: 0.85rem;
    color: rgba(255, 255, 255, 0.6);
    margin: 0;
    position: relative;
    z-index: 1;
  }

  .select-indicator {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.9rem;
    font-weight: 600;
    color: var(--accent-color, #10b981);
    margin-top: 0.5rem;
    opacity: 0;
    transform: translateY(10px);
    transition: all 0.3s ease;
    position: relative;
    z-index: 1;
  }

  .modpack-card:hover .select-indicator {
    opacity: 1;
    transform: translateY(0);
  }

  .arrow {
    font-size: 1.2rem;
    animation: bounce 1s infinite;
  }

  @keyframes bounce {
    0%, 100% { transform: translateX(0); }
    50% { transform: translateX(4px); }
  }

  .actions {
    display: flex;
    justify-content: center;
    margin-top: 1rem;
  }

  .back-button {
    background: rgba(255, 255, 255, 0.1);
    border: 2px solid rgba(255, 255, 255, 0.2);
    border-radius: 8px;
    padding: 0.75rem 1.5rem;
    font-size: 0.9rem;
    font-weight: 600;
    color: white;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .back-button:hover {
    background: rgba(255, 255, 255, 0.15);
    border-color: rgba(255, 255, 255, 0.3);
    transform: translateY(-2px);
  }

  @media (max-width: 640px) {
    .title {
      font-size: 1.25rem;
    }

    .modpack-grid {
      grid-template-columns: 1fr;
      gap: 1rem;
      margin: 1.5rem 0;
    }

    .modpack-card {
      padding: 1.5rem 1rem;
    }

    .modpack-icon {
      font-size: 2.5rem;
    }
  }
</style>

