<script lang="ts">
  import { dev } from '$app/environment';
  import { page } from '$app/stores';
  import type { ResponseBody } from '$lib/@types';
  import { debugLog } from '$lib/utils/client/constants';

  let data: ResponseBody = $page.data;
  let form: ResponseBody = $page.form;

  const entries = Object.entries(debugLog(data, form)).filter(([, messages]) => messages.length);
</script>

{#if dev}
  <div class="absolute left-[5%] top-[5%] z-50 w-2/12 rounded-md bg-slate-900 px-5 py-4 shadow-xl">
    <h1 class="mb-2 mt-1 text-lg font-semibold leading-3 text-green-600">Debug</h1>
    {#each entries as [title, messages]}
      <div class="flex flex-col space-y-0.5">
        <p class="text-sm font-bold text-gray-300">{title}</p>
        {#each messages as message}
          <p class="break-words text-sm text-gray-400">
            <span class="text-gray-500">${message.path}: </span><span class="text-yellow-200"
              >{message?.message ? JSON.stringify(message.message) : ''} {message?.value || ''}
            </span>
          </p>
        {/each}
      </div>
    {/each}
  </div>
{/if}
