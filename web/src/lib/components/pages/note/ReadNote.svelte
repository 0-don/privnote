<script lang="ts">
  import type { NoteResponse, ResponseBody } from '$lib/@types';
  import Button from '$lib/components/elements/Button.svelte';

  export let data: ResponseBody<NoteResponse>;
  export let title = 'Note contents';

  const error = data?.messages?.find(({ path }) => path === 'error')?.message || '';
</script>

<div class="mt-3 flex items-center justify-start">
  <h1 class="my-2 text-2xl font-bold">{title}</h1>
</div>

{#if data.data?.alert}
  <div class="bg-alert mb-3 p-1 text-center">{data.data?.alert}</div>
{/if}
{#if error}
  <div class="mb-3 break-all text-red-400">{error}</div>
{/if}

{#if data.data?.note}
  <section id="content">
    <textarea
      rows="13"
      name="note"
      value={data.data?.note.note}
      readonly
      placeholder="Write your note here..."
      class="w-full !bg-yellow-100 !bg-opacity-75 p-5 text-black outline-none placeholder:text-black"
    />
  </section>
{/if}
