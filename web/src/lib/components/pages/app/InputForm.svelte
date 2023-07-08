<script lang="ts">
  import Button from '$lib/components/elements/Button.svelte';
  import type { ActionData } from '../../../../routes/$types';
  import FormOptions from './FormOptions.svelte';

  export let form: ActionData;
  const note = form?.find(({ path }) => path === 'note');
</script>

<form method="POST">
  {JSON.stringify(note)}

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

  <FormOptions />

  <section class="mt-4 flex justify-between">
    <Button type="button" className="!rounded-none" text="Create Note" icon="i-line-md:document-add" />
    <Button href="options" className="!rounded-none" text="Show Options" icon="i-line-md:edit-twotone" />
  </section>
</form>
