<script lang="ts">
  import type { User } from '../types/user';
  import { removeUser, selectUser } from '../stores/userStore';

  export let user: User;

  const handleSelect = () => {
    selectUser(user);
  };

  const handleRemove = () => {
    removeUser(user.id);
  };
</script>

<div class="card bg-base-100 shadow-md">
  <div class="card-body">
    <h3 class="card-title">{user.username}</h3>
    <p class="text-sm text-gray-600">{user.email}</p>
    <p class="text-xs text-gray-400">Created: {user.createdAt.toLocaleDateString()}</p>
    
    <div class="card-actions justify-end">
      <button class="btn btn-sm btn-primary" on:click={handleSelect}>
        Select
      </button>
      <button class="btn btn-sm btn-error" on:click={handleRemove}>
        Remove
      </button>
    </div>
  </div>
</div>
