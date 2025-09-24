<script lang="ts">
  import { counterStore } from '../stores/counter';
</script>

<div class="stats shadow w-full mb-8">
  <div class="stat">
    <div class="stat-title">Button Clicks</div>
    <div class="stat-value text-primary">{$counterStore}</div>
    <div class="stat-desc">Keep clicking!</div>
  </div>
  
  <div class="stat">
    <div class="stat-title">Framework</div>
    <div class="stat-value text-secondary">Svelte 5</div>
    <div class="stat-desc">With TypeScript</div>
  </div>
  
  <div class="stat">
    <div class="stat-title">Build Tool</div>
    <div class="stat-value text-accent">Vite</div>
    <div class="stat-desc">Lightning fast</div>
  </div>
</div>
