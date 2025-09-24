<script lang="ts">
  import { counterStore, increment } from '../stores/counter';
</script>

<div class="flex flex-wrap gap-4 justify-center">
  <button class="btn btn-primary" on:click={increment}>
    Increment Counter ({$counterStore})
  </button>
  
  <button class="btn btn-secondary">
    Learn More
  </button>
  
  <button class="btn btn-accent">
    Settings
  </button>
</div>
