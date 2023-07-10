<script lang="ts">
  import { dev } from '$app/environment';
  import type { NotificationEvent } from '$lib/@types';
  export let events: NotificationEvent[] = [];
</script>

{#if dev}
  <div class="absolute left-[5%] top-[5%] w-2/12 rounded-md bg-slate-900 p-5 shadow-xl">
    <h1 class="font-semibold text-green-600">Debug</h1>
    {#each events as event}
      <div class="mt-1 flex flex-col space-y-0.5">
        <p class="text-sm text-gray-300">{event.key}</p>
        <p class="text-sm text-gray-400">{event.message}</p>
      </div>
    {/each}
  </div>
{/if}
