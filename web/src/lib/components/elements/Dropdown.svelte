<script lang="ts">
  import type { ResponseBody } from '$lib/@types';

  export let className = '';
  export let name = '';
  export let options = [] as { value: string | number; text: string | number }[];

  export let form: ResponseBody;
  const msg = form?.messages?.find((f) => f.path === name);
</script>

<label class="w-full" for={name}>
  <select id={name} {name} class="{className} w-full bg-zinc-600 p-2 focus:bg-zinc-500">
    {#each options as { value, text }, i}
      <option selected={msg?.value === value} {value}>{text}</option>
    {/each}
  </select>
  {#if msg?.message}
    <p class="text-red-400">{msg.message}</p>
  {/if}
</label>
