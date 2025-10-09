<script>
  export let isVisible = false;
  export let message = 'Downloading update...';
  export let showRestartPrompt = false;
  export let onRestart = () => {};
  export let onLater = () => {};
</script>

{#if isVisible}
  <div class="update-popup-overlay">
    <div class="update-popup">
      <!-- Dancing cat animation -->
      <div class="cat-container">
        <svg class="dancing-cat" viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
          <!-- Cat body -->
          <ellipse cx="100" cy="140" rx="40" ry="35" fill="#10b981" class="body-bounce"/>
          
          <!-- Cat head -->
          <circle cx="100" cy="90" r="45" fill="#10b981" class="head-bob"/>
          
          <!-- Cat ears -->
          <g class="ears-dance">
            <path d="M 65 60 L 50 25 L 80 50 Z" fill="#059669"/>
            <path d="M 135 60 L 150 25 L 120 50 Z" fill="#059669"/>
            <!-- Inner ears -->
            <path d="M 68 55 L 58 35 L 77 50 Z" fill="#6ee7b7"/>
            <path d="M 132 55 L 142 35 L 123 50 Z" fill="#6ee7b7"/>
          </g>
          
          <!-- Cat eyes - happy/excited -->
          <g class="eyes-sparkle">
            <circle cx="85" cy="85" r="6" fill="#1f2937"/>
            <circle cx="115" cy="85" r="6" fill="#1f2937"/>
            <!-- Sparkles -->
            <circle cx="87" cy="83" r="2" fill="white" class="sparkle"/>
            <circle cx="117" cy="83" r="2" fill="white" class="sparkle"/>
          </g>
          
          <!-- Cat nose -->
          <circle cx="100" cy="100" r="4" fill="#0d9488"/>
          
          <!-- Cat smile -->
          <path d="M 85 105 Q 100 115 115 105" stroke="#1f2937" stroke-width="2.5" fill="none" stroke-linecap="round" class="smile"/>
          
          <!-- Whiskers -->
          <g class="whiskers-move">
            <line x1="50" y1="95" x2="75" y2="93" stroke="#1f2937" stroke-width="2" stroke-linecap="round"/>
            <line x1="50" y1="102" x2="75" y2="100" stroke="#1f2937" stroke-width="2" stroke-linecap="round"/>
            <line x1="150" y1="95" x2="125" y2="93" stroke="#1f2937" stroke-width="2" stroke-linecap="round"/>
            <line x1="150" y1="102" x2="125" y2="100" stroke="#1f2937" stroke-width="2" stroke-linecap="round"/>
          </g>
          
          <!-- Dancing arms -->
          <g class="arms-wave">
            <ellipse cx="65" cy="130" rx="12" ry="30" fill="#10b981" transform="rotate(-20 65 130)"/>
            <ellipse cx="135" cy="130" rx="12" ry="30" fill="#10b981" transform="rotate(20 135 130)"/>
          </g>
          
          <!-- Tail -->
          <path d="M 135 155 Q 170 140 165 110" stroke="#10b981" stroke-width="10" fill="none" stroke-linecap="round" class="tail-wag"/>
        </svg>
      </div>
      
      {#if showRestartPrompt}
        <!-- Restart Prompt -->
        <div class="update-content restart-ready">
          <h3 class="update-title">✨ All Done!</h3>
          <p class="update-message">Update installed successfully</p>
          
          <div class="button-group">
            <button class="btn-restart" on:click={onRestart}>
              <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2"/>
              </svg>
              Restart Now
            </button>
            <button class="btn-later" on:click={onLater}>
              Later
            </button>
          </div>
        </div>
      {:else}
        <!-- Download/Install Progress -->
        <div class="update-content">
          <h3 class="update-title">Updating...</h3>
          <p class="update-message">{message}</p>
          <div class="progress-bar">
            <div class="progress-fill"></div>
          </div>
        </div>
      {/if}
    </div>
  </div>
{/if}

<style>
  .update-popup-overlay {
    position: fixed;
    bottom: 24px;
    right: 24px;
    z-index: 9999;
    pointer-events: none;
  }
  
  .update-popup {
    background: linear-gradient(135deg, rgba(31, 41, 55, 0.98) 0%, rgba(17, 24, 39, 0.98) 100%);
    backdrop-filter: blur(20px);
    border-radius: 20px;
    padding: 24px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5), 
                0 0 0 1px rgba(16, 185, 129, 0.2);
    min-width: 320px;
    max-width: 380px;
    pointer-events: auto;
    animation: slide-in 0.4s cubic-bezier(0.16, 1, 0.3, 1);
  }
  
  @keyframes slide-in {
    from {
      opacity: 0;
      transform: translateY(30px) scale(0.95);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }
  
  .cat-container {
    display: flex;
    justify-content: center;
    margin-bottom: 20px;
  }
  
  .dancing-cat {
    width: 120px;
    height: 120px;
    animation: cat-dance 1.2s ease-in-out infinite;
  }
  
  @keyframes cat-dance {
    0%, 100% { transform: translateY(0) rotate(-3deg); }
    25% { transform: translateY(-8px) rotate(3deg); }
    50% { transform: translateY(0) rotate(-3deg); }
    75% { transform: translateY(-8px) rotate(3deg); }
  }
  
  .body-bounce {
    animation: body-bounce 0.6s ease-in-out infinite;
    transform-origin: 100px 140px;
  }
  
  @keyframes body-bounce {
    0%, 100% { transform: scaleY(1); }
    50% { transform: scaleY(0.95); }
  }
  
  .head-bob {
    animation: head-bob 0.6s ease-in-out infinite;
    transform-origin: 100px 90px;
  }
  
  @keyframes head-bob {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-5px); }
  }
  
  .ears-dance {
    animation: ears-wiggle 0.4s ease-in-out infinite;
    transform-origin: 100px 60px;
  }
  
  @keyframes ears-wiggle {
    0%, 100% { transform: rotate(0deg); }
    25% { transform: rotate(-5deg); }
    75% { transform: rotate(5deg); }
  }
  
  .eyes-sparkle {
    animation: blink 3s ease-in-out infinite;
  }
  
  @keyframes blink {
    0%, 93%, 100% { opacity: 1; }
    95% { opacity: 0; }
  }
  
  .sparkle {
    animation: sparkle 1.5s ease-in-out infinite;
  }
  
  @keyframes sparkle {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.5; transform: scale(0.8); }
  }
  
  .smile {
    animation: smile-pulse 1.5s ease-in-out infinite;
  }
  
  @keyframes smile-pulse {
    0%, 100% { stroke-width: 2.5; }
    50% { stroke-width: 3; }
  }
  
  .whiskers-move {
    animation: whiskers-twitch 0.8s ease-in-out infinite;
  }
  
  @keyframes whiskers-twitch {
    0%, 100% { transform: translateX(0); }
    50% { transform: translateX(2px); }
  }
  
  .arms-wave {
    animation: arms-wave 0.6s ease-in-out infinite alternate;
    transform-origin: 100px 130px;
  }
  
  @keyframes arms-wave {
    0% { transform: rotate(-3deg); }
    100% { transform: rotate(3deg); }
  }
  
  .tail-wag {
    animation: tail-wag 0.5s ease-in-out infinite;
    transform-origin: 135px 155px;
  }
  
  @keyframes tail-wag {
    0%, 100% { transform: rotate(0deg); }
    50% { transform: rotate(15deg); }
  }
  
  .update-content {
    text-align: center;
  }
  
  .update-title {
    font-size: 1.5rem;
    font-weight: 700;
    color: #10b981;
    margin-bottom: 8px;
  }
  
  .update-message {
    font-size: 0.95rem;
    color: rgba(255, 255, 255, 0.8);
    margin-bottom: 16px;
    line-height: 1.4;
  }
  
  .progress-bar {
    width: 100%;
    height: 6px;
    background: rgba(16, 185, 129, 0.2);
    border-radius: 10px;
    overflow: hidden;
  }
  
  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #10b981, #059669, #0d9488);
    background-size: 200% 100%;
    border-radius: 10px;
    animation: progress-flow 1.5s ease-in-out infinite;
  }
  
  @keyframes progress-flow {
    0% { width: 20%; background-position: 0% 0%; }
    50% { width: 80%; background-position: 100% 0%; }
    100% { width: 20%; background-position: 0% 0%; }
  }
  
  .restart-ready {
    animation: ready-pulse 0.5s ease-out;
  }
  
  @keyframes ready-pulse {
    0% { transform: scale(0.95); opacity: 0; }
    50% { transform: scale(1.02); }
    100% { transform: scale(1); opacity: 1; }
  }
  
  .button-group {
    display: flex;
    gap: 10px;
    margin-top: 20px;
    justify-content: center;
  }
  
  .btn-restart {
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
    color: white;
    padding: 12px 24px;
    border-radius: 12px;
    font-weight: 600;
    font-size: 0.95rem;
    border: none;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 8px;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
  }
  
  .btn-restart:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(16, 185, 129, 0.5);
  }
  
  .btn-restart:active {
    transform: translateY(0);
  }
  
  .btn-later {
    background: rgba(255, 255, 255, 0.08);
    color: rgba(255, 255, 255, 0.9);
    padding: 12px 24px;
    border-radius: 12px;
    font-weight: 600;
    font-size: 0.95rem;
    border: 1.5px solid rgba(255, 255, 255, 0.15);
    cursor: pointer;
    transition: all 0.3s ease;
  }
  
  .btn-later:hover {
    background: rgba(255, 255, 255, 0.12);
    border-color: rgba(255, 255, 255, 0.25);
  }
</style>


