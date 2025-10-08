<script>
  export let isVisible = false;
  export let onRestart = () => {};
  
  let countdown = 5;
  let countdownInterval;
  
  $: if (isVisible && !countdownInterval) {
    startCountdown();
  }
  
  function startCountdown() {
    countdownInterval = setInterval(() => {
      countdown--;
      if (countdown <= 0) {
        clearInterval(countdownInterval);
        onRestart();
      }
    }, 1000);
  }
  
  function handleRestartNow() {
    if (countdownInterval) {
      clearInterval(countdownInterval);
    }
    onRestart();
  }
</script>

{#if isVisible}
  <div class="fixed inset-0 bg-base-300/95 backdrop-blur-sm z-50 flex items-center justify-center">
    <div class="text-center">
      <!-- Quirky animated cat SVG -->
      <div class="mb-8 animate-bounce">
        <svg class="w-48 h-48 mx-auto cat-animation" viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
          <!-- Cat head -->
          <circle cx="100" cy="100" r="60" fill="#FF6B9D" class="pulse-animation"/>
          
          <!-- Cat ears -->
          <path d="M 60 60 L 40 20 L 80 50 Z" fill="#FF6B9D" class="ear-wiggle-left"/>
          <path d="M 140 60 L 160 20 L 120 50 Z" fill="#FF6B9D" class="ear-wiggle-right"/>
          
          <!-- Inner ears -->
          <path d="M 65 55 L 50 30 L 75 50 Z" fill="#FFB6D9"/>
          <path d="M 135 55 L 150 30 L 125 50 Z" fill="#FFB6D9"/>
          
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
      <h2 class="text-4xl font-bold mb-4 text-primary animate-pulse">
        🎉 Update Downloaded!
      </h2>
      
      <p class="text-xl mb-6 opacity-80">
        The installer has been opened automatically.
      </p>
      
      <div class="alert alert-info mb-6">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
        <div>
          <p class="font-bold">Please complete the installation</p>
          <p class="text-sm">The setup wizard will guide you through the update process.</p>
        </div>
      </div>
      
      <div class="text-6xl font-bold mb-4 countdown-number">
        {countdown}
      </div>
      
      <p class="text-lg mb-8">
        This window will close in <span class="font-bold text-primary">{countdown}</span> seconds...
      </p>
      
      <button 
        class="btn btn-primary btn-lg gap-2"
        on:click={handleRestartNow}
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
        </svg>
        Close Now
      </button>
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
  
  .countdown-number {
    animation: countdown-pulse 1s ease-in-out infinite;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }
  
  @keyframes countdown-pulse {
    0%, 100% { transform: scale(1); }
    50% { transform: scale(1.2); }
  }
</style>

