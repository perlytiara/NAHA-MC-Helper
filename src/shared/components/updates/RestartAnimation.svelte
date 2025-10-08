<script>
  export let isVisible = false;
  export let message = 'Downloading update...';
  export let showRestartPrompt = false;
  export let onRestart = () => {};
  export let onLater = () => {};
</script>

{#if isVisible}
  <div class="fixed inset-0 bg-base-300/95 backdrop-blur-sm z-50 flex items-center justify-center">
    <div class="text-center">
      <!-- Quirky animated cat SVG -->
      <div class="mb-8 animate-bounce">
        <svg class="w-48 h-48 mx-auto cat-animation" viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
          <!-- Cat head -->
          <circle cx="100" cy="100" r="60" fill="#10b981" class="pulse-animation"/>
          
          <!-- Cat ears -->
          <path d="M 60 60 L 40 20 L 80 50 Z" fill="#10b981" class="ear-wiggle-left"/>
          <path d="M 140 60 L 160 20 L 120 50 Z" fill="#10b981" class="ear-wiggle-right"/>
          
          <!-- Inner ears -->
          <path d="M 65 55 L 50 30 L 75 50 Z" fill="#6ee7b7"/>
          <path d="M 135 55 L 150 30 L 125 50 Z" fill="#6ee7b7"/>
          
          <!-- Eyes -->
          <g class="blink-animation">
            <ellipse cx="80" cy="95" rx="8" ry="12" fill="#2D3748"/>
            <ellipse cx="120" cy="95" rx="8" ry="12" fill="#2D3748"/>
            <!-- Eye shine -->
            <circle cx="82" cy="92" r="3" fill="white"/>
            <circle cx="122" cy="92" r="3" fill="white"/>
          </g>
          
          <!-- Nose -->
          <path d="M 100 105 L 95 110 L 105 110 Z" fill="#FF1493"/>
          
          <!-- Mouth - happy smile -->
          <path d="M 85 115 Q 100 125 115 115" stroke="#2D3748" stroke-width="3" fill="none" stroke-linecap="round"/>
          
          <!-- Whiskers -->
          <g class="whiskers-wiggle">
            <line x1="40" y1="100" x2="70" y2="98" stroke="#2D3748" stroke-width="2" stroke-linecap="round"/>
            <line x1="40" y1="110" x2="70" y2="108" stroke="#2D3748" stroke-width="2" stroke-linecap="round"/>
            <line x1="160" y1="100" x2="130" y2="98" stroke="#2D3748" stroke-width="2" stroke-linecap="round"/>
            <line x1="160" y1="110" x2="130" y2="108" stroke="#2D3748" stroke-width="2" stroke-linecap="round"/>
          </g>
        </svg>
      </div>
      
      <!-- Update message -->
      <h2 class="text-4xl font-bold mb-4 gradient-text animate-pulse">
        {#if showRestartPrompt}
          ✨ Update Ready!
        {:else}
          🔄 Updating...
        {/if}
      </h2>
      
      <p class="text-xl mb-6 opacity-80">
        {message}
      </p>
      
      {#if showRestartPrompt}
        <!-- Restart Prompt Card -->
        <div class="restart-prompt-card">
          <p class="text-lg mb-6 text-white">
            The update has been installed successfully!
          </p>
          
          <div class="flex gap-4 justify-center">
            <button 
              class="restart-btn"
              on:click={onRestart}
            >
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
              Restart Now
            </button>
            
            <button 
              class="later-btn"
              on:click={onLater}
            >
              Later
            </button>
          </div>
        </div>
      {:else}
        <!-- Loading spinner -->
        <div class="loading-spinner"></div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .cat-animation {
    animation: float 3s ease-in-out infinite;
  }
  
  @keyframes float {
    0%, 100% { transform: translateY(0px) rotate(-5deg); }
    50% { transform: translateY(-20px) rotate(5deg); }
  }
  
  .pulse-animation {
    animation: pulse-scale 2s ease-in-out infinite;
  }
  
  @keyframes pulse-scale {
    0%, 100% { transform: scale(1); }
    50% { transform: scale(1.05); }
  }
  
  .ear-wiggle-left {
    animation: wiggle-left 1s ease-in-out infinite;
    transform-origin: 70px 55px;
  }
  
  .ear-wiggle-right {
    animation: wiggle-right 1s ease-in-out infinite;
    transform-origin: 130px 55px;
  }
  
  @keyframes wiggle-left {
    0%, 100% { transform: rotate(0deg); }
    50% { transform: rotate(-10deg); }
  }
  
  @keyframes wiggle-right {
    0%, 100% { transform: rotate(0deg); }
    50% { transform: rotate(10deg); }
  }
  
  .blink-animation {
    animation: blink 3s ease-in-out infinite;
  }
  
  @keyframes blink {
    0%, 90%, 100% { opacity: 1; }
    95% { opacity: 0; }
  }
  
  .whiskers-wiggle {
    animation: whisker-move 2s ease-in-out infinite;
  }
  
  @keyframes whisker-move {
    0%, 100% { transform: translateX(0); }
    50% { transform: translateX(3px); }
  }
  
  .gradient-text {
    background: linear-gradient(135deg, #10b981 0%, #059669 50%, #0d9488 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }
  
  .loading-spinner {
    width: 60px;
    height: 60px;
    border: 6px solid rgba(16, 185, 129, 0.2);
    border-top-color: #10b981;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin: 0 auto;
  }
  
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  
  .restart-prompt-card {
    background: linear-gradient(135deg, rgba(16, 185, 129, 0.2) 0%, rgba(5, 150, 105, 0.2) 100%);
    border: 2px solid rgba(16, 185, 129, 0.3);
    border-radius: 16px;
    padding: 2rem;
    backdrop-filter: blur(10px);
    animation: slide-up 0.3s ease-out;
  }
  
  @keyframes slide-up {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
  
  .restart-btn {
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
    color: white;
    padding: 0.75rem 2rem;
    border-radius: 8px;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: all 0.3s ease;
    border: none;
    cursor: pointer;
  }
  
  .restart-btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 10px 25px rgba(16, 185, 129, 0.4);
  }
  
  .later-btn {
    background: rgba(255, 255, 255, 0.1);
    color: white;
    padding: 0.75rem 2rem;
    border-radius: 8px;
    font-weight: 600;
    border: 2px solid rgba(255, 255, 255, 0.2);
    transition: all 0.3s ease;
    cursor: pointer;
  }
  
  .later-btn:hover {
    background: rgba(255, 255, 255, 0.15);
    border-color: rgba(255, 255, 255, 0.3);
  }
</style>


