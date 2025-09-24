import daisyui from 'daisyui';

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{svelte,html,js,ts}', './index.html'],
  theme: {
    extend: {},
  },
  plugins: [daisyui],
};
