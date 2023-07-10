<script lang="ts">
  import { dev } from '$app/environment';
  import type { DebugMessages } from '$lib/@types';

  export let debug: DebugMessages;
</script>

{#if dev}
  <div class="absolute left-[5%] top-[5%] w-2/12 rounded-md bg-slate-900 p-5 shadow-xl">
    <h1 class="font-semibold text-green-600">Debug</h1>
    {#each Object.entries(debug) as [title, messages]}
      {#each messages as message}
        <div class="mt-1 flex flex-col space-y-0.5">
          <p class="text-sm text-gray-300">{title}</p>
          <p class="break-words text-sm text-gray-400">
            <span class="text-gray-500">${message.path}: </span><span class="text-yellow-200"
              >{JSON.stringify(message.message)}</span
            >
          </p>
        </div>
      {/each}
    {/each}
  </div>
{/if}
