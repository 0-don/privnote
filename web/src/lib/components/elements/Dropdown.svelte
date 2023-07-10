<script lang="ts">
  import type { Message } from '$lib/@types';

  export let className = '';
  export let name = '';
  export let options = [] as { value: string | number; text: string | number }[];

  export let form: Message[] = [];
  const error = form?.find((f) => f.path === name)?.message;
</script>

<label class="w-full" for={name}>
  <select id={name} {name} class="{className} w-full bg-zinc-600 p-2 focus:bg-zinc-500">
    {#each options as { value, text }, i}
      <option {value}>{text}</option>
    {/each}
  </select>
  {#if error}
    <p class="text-red-400">{error}</p>
  {/if}
</label>
