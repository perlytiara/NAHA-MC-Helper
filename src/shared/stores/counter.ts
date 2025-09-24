import { writable } from 'svelte/store';

export const counterStore = writable(0);

export const increment = () => {
  counterStore.update(n => n + 1);
};

export const reset = () => {
  counterStore.set(0);
};
