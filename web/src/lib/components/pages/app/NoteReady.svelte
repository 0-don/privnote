<script lang="ts">
  import { page } from '$app/stores';
  import { env } from '$env/dynamic/public';
  import type { Note, ResponseBody } from '$lib/@types';
  import Button from '$lib/components/elements/Button.svelte';

  let form: ResponseBody = $page.form;

  const note = form.data as Note;

  const url = `${env.PUBLIC_DOMAIN}${note.id}`;
</script>

<p class="bg-olive p-2 font-bold">
  {url}
</p>
<p class="italic bg-darkGold mb-4 p-2 font-light">The note will self-destruct after reading it.</p>
<div class="flex items-center justify-between">
  <Button
    type="a"
    text="E-mail link"
    className="!rounded-none"
    icon="i-line-md:email-twotone"
    href="mailto:?body={url}"
  />
  <Button type="a" text="Destroy note now" className="!rounded-none" icon="i-line-md:close-circle" href="#destroy" />
</div>

<div
  id="destroy"
  class="pointer-events-none fixed bottom-0 left-0 right-0 top-0 text-ellipsis opacity-0 transition-all target:pointer-events-auto target:opacity-100 z-40"
>
  <!-- svelte-ignore a11y-missing-content -->
  <!-- svelte-ignore a11y-invalid-attribute -->
  <a href="#" class="close hover:opacity-40" />
  <div
    id="target-inner"
    class="bg-container absolute left-1/2 top-1/2 block w-1/2 -translate-x-1/2 -translate-y-1/2 p-5 shadow-xl"
  >
    <h2 class="text-3xl font-bold">Read and destroy?</h2>
    <p class="my-2 text-gray-300">You're about to read and destroy the note with id {note.id}.</p>
    <div class="flex justify-between">
      <Button
        type="a"
        text="Yes, show me the note"
        className="!rounded-none !bg-main"
        icon="i-line-md:clipboard-check"
        href="/note/{note.id}"
      />
      <Button type="a" text="No, not now" className="!rounded-none" icon="i-line-md:close-circle" href="#" />
    </div>
  </div>
</div>
