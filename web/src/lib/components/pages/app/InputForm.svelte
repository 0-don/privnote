<script lang="ts">
  import type { Messages, ResponseBody } from '$lib/@types';
  import Button from '$lib/components/elements/Button.svelte';
  import FormOptions from './FormOptions.svelte';

  export let form: ResponseBody;
  export let data: ResponseBody;
  const note = form?.messages?.find(({ path }) => path === 'note')?.message || '';
  const tag = form?.messages?.find(({ path }) => path === 'tag')?.message || '';

  const errorsForm = form?.messages
    ?.filter((f) => f.path === 'error')
    .map(({ message }) => message)
    .join(', ');
  const errorsData = (Array.isArray(data) ? data : []).map(({ message }) => message).join(', ');
</script>

<form method="POST">
  {#if errorsForm || errorsData || tag}
    <p class="break-all text-red-400">{errorsForm}</p>
    <p class="break-all text-red-400">{errorsData}</p>
    <p class="break-all text-red-400">{tag}</p>
  {/if}

  <section id="content" class="mt-4">
    <textarea
      rows="13"
      name="note"
      placeholder="Write your note here..."
      class="w-full !bg-yellow-100 !bg-opacity-75 p-5 text-black outline-none placeholder:text-black"
    />
    {#if note}
      <p class="text-red-400">{note}</p>
    {/if}
  </section>

  {#if typeof data?.data === 'object' && !Array.isArray(data?.data)}
    <input type="hidden" name="tag" value={data.data?.config} />
  {/if}

  <FormOptions {form} />

  <section class="mt-4 flex justify-between">
    <Button type="button" className="!rounded-none" text="Create Note" icon="i-line-md:document-add" />
    <Button href="options" className="!rounded-none" text="Show Options" icon="i-line-md:edit-twotone" />
  </section>
</form>
