<script lang="ts">
  import type { HTMLInputTypeAttribute } from 'svelte/elements';
  import type { CreateNoteResponse } from '$lib/@types';

  export let className = '';
  export let name = '';
  export let type = 'text' as HTMLInputTypeAttribute;
  export let label = '';

  export let form: CreateNoteResponse[] = [];
  const error = form?.find((f) => f.path === name)?.message;
</script>

<div class="flex w-full flex-col">
  <label for={name} class="text-sm">{label}</label>
  <input id={name} {type} {name} class="w-full bg-zinc-600 p-2 focus:bg-zinc-500 {className}" />
  {#if error}
    <p class="text-red-400">{error}</p>
  {/if}
</div>
