import { writable } from 'svelte/store';
import type { User } from '../types/user';

export const users = writable<User[]>([]);
export const selectedUser = writable<User | null>(null);

export const addUser = (user: User) => {
  users.update(currentUsers => [...currentUsers, user]);
};

export const removeUser = (userId: string) => {
  users.update(currentUsers => currentUsers.filter(user => user.id !== userId));
};

export const selectUser = (user: User | null) => {
  selectedUser.set(user);
};
