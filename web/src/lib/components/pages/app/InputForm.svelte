<script lang="ts">
  import { dev } from '$app/environment';
  import type { CaptchaLoad, Notification } from '$lib/@types';
  import Button from '$lib/components/elements/Button.svelte';
  import FormOptions from './FormOptions.svelte';

  export let form: Notification[] = [];
  export let data: CaptchaLoad;
  const note = form?.find(({ path }) => path === 'note');
  const tag = form?.find(({ path }) => path === 'tag');

  const errorForm = form?.find((f) => f.path === 'error')?.message;
  const errorCaptcha = form?.find((f) => f.path === 'error')?.message;
</script>

<form method="POST">
  {#if dev}
    <h5 class="text-green-400">Form</h5>
    <p class="break-all text-green-400">{JSON.stringify(form, null, 4)}</p>
    <h5 class="text-blue-400">Load</h5>
    <p class="break-all text-blue-400">{JSON.stringify(data, null, 4)}</p>
  {/if}

  {#if errorForm || errorCaptcha || tag}
    <p class="break-all text-red-400">{errorForm + ' ' + errorCaptcha + ' ' + tag}</p>
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

  {#if 'tag' in data}
    <input type="hidden" name="tag" value={data.tag} />
  {/if}

  <FormOptions {form} />

  <section class="mt-4 flex justify-between">
    <Button type="button" className="!rounded-none" text="Create Note" icon="i-line-md:document-add" />
    <Button href="options" className="!rounded-none" text="Show Options" icon="i-line-md:edit-twotone" />
  </section>
</form>
