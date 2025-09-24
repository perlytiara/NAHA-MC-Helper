import './input.css';
import { mount } from 'svelte';
import App from './app/App.svelte';

const appElement = document.getElementById('app');

if (appElement) {
  mount(App, {
    target: appElement,
  });
} else {
  console.error('Could not find app element');
}
