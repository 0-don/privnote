<script lang="ts">
  import type { HTMLInputTypeAttribute } from 'svelte/elements';
  import type { ResponseBody } from '$lib/@types';

  export let className = '';
  export let name = '';
  export let type = 'text' as HTMLInputTypeAttribute;
  export let label = '';

  export let form: ResponseBody;
  const msg = form?.messages?.find((f) => f.path === name);
</script>

<div class="flex w-full flex-col">
  <label for={name} class="text-sm mb-0.5">{label}</label>
  <input
    id={name}
    {type}
    {name}
    value={msg?.value || ''}
    class="w-full bg-zinc-600 p-2 focus:bg-zinc-500 {className}"
  />
  {#if msg?.message}
    <p class="text-red-400">{msg.message}</p>
  {/if}
</div>
