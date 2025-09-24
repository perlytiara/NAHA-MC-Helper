<script lang="ts">
  import { increment } from '../stores/counter';

  export let name: string;
</script>

<div class="hero bg-base-100 rounded-lg shadow-xl mb-8">
  <div class="hero-content text-center">
    <div class="max-w-md">
      <h1 class="text-5xl font-bold text-primary">Welcome to {name}!</h1>
      <p class="py-6">
        A modern Electron application built with Svelte, Tailwind CSS, DaisyUI, and TypeScript.
      </p>
      <button class="btn btn-primary btn-lg" on:click={increment}>
        Get Started
      </button>
    </div>
  </div>
</div>
