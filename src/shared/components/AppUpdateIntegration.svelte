<script>
  import { onMount } from 'svelte';
  import UpdateManager from './updates/UpdateManager.svelte';

  let updateManager;

  onMount(() => {
    // Make update manager available globally for settings
    window.updateManager = updateManager;
  });
</script>

<!-- This component integrates the update manager into the app -->
<!-- It should be included in the main App.svelte file -->

<UpdateManager bind:this={updateManager} />
