<script lang="ts">
  import { page } from '$app/stores';
  import type { ResponseBody, Tag } from '$lib/@types';
  import Button from '$lib/components/elements/Button.svelte';
  import FormOptions from './FormOptions.svelte';

  let form: ResponseBody = $page.form;
  let data: ResponseBody = $page.data;
  const note = form?.messages?.find(({ path }) => path === 'note');

  const errorsForm = form?.messages
    ?.filter((f) => f.path === 'error')
    .map(({ message }) => message)
    .join(', ');
  const errorsData = (Array.isArray(data.messages) ? data.messages : []).map(({ message }) => message).join(', ');
  const tag = (data.data as Tag)?.tag;

  console.log(form,data);
</script>

<form method="POST">
  {#if errorsForm || errorsData}
    <p class="break-all text-red-400">{errorsForm}</p>
    <p class="break-all text-red-400">{errorsData}</p>
  {/if}

  <section id="content">
    <textarea
      rows="13"
      name="note"
      value="{String(note?.value || '')}"
      placeholder="Write your note here..."
      class="w-full !bg-yellow-100 !bg-opacity-75 p-5 text-black outline-none placeholder:text-black"
    />
    {#if note?.message}
      <p class="text-red-400">{note.message}</p>
    {/if}
  </section>

  {#if tag}
    <input type="hidden" name="tag" value={tag} />
  {/if}

  <FormOptions />

  <section class="mt-4 flex justify-between">
    <Button type="button" className="!rounded-none !bg-main" text="Create Note" icon="i-line-md:document-add" />
    <Button href="#options" className="!rounded-none" text="Show Options" icon="i-line-md:edit-twotone" />
  </section>
</form>
