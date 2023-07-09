<script lang="ts">
  import { dev } from '$app/environment';
  import type { CaptchLoad, CreateNoteResponse } from '$lib/@types';
  import Button from '$lib/components/elements/Button.svelte';
  import FormOptions from './FormOptions.svelte';

  export let form: CreateNoteResponse[] = [];
  export let data: CaptchLoad;
  const note = form?.find(({ path }) => path === 'note');
  const errorForm = form?.find((f) => f.path === 'error')?.message;
  const errorCaptcha = form?.find((f) => f.path === 'error')?.message;
</script>

<form method="POST">
  {#if dev}
    {JSON.stringify(form)}
  {/if}

  {#if errorForm || errorCaptcha}
    <p class="text-red-400">{errorForm + ' ' + errorCaptcha}</p>
  {/if}

  <section id="content" class="mt-4">
    <textarea
      rows="13"
      name="note"
      placeholder="Write your note here..."
      class="w-full !bg-yellow-100 !bg-opacity-75 p-5 text-black outline-none placeholder:text-black"
    />
    {#if note}
      <p class="text-red-400">{note.message}</p>
    {/if}
  </section>

  {#if 'captch' in data}
    <input type="hidden" name="captcha" value={data.captch} />
  {/if}

  <FormOptions {form} />

  <section class="mt-4 flex justify-between">
    <Button type="button" className="!rounded-none" text="Create Note" icon="i-line-md:document-add" />
    <Button href="options" className="!rounded-none" text="Show Options" icon="i-line-md:edit-twotone" />
  </section>
</form>
