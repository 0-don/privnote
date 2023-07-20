<script lang="ts">
  import { page } from '$app/stores';
  import { env } from '$env/dynamic/public';
  import type { Note, ResponseBody } from '$lib/@types';
  import Button from '$lib/components/elements/Button.svelte';
  import Modal from '$lib/components/elements/Modal.svelte';

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
  {#if note.duration_hours}
    <Button
      type="a"
      text="Read note"
      className="!rounded-none"
      icon="i-line-md:clipboard-check"
      href="/note/{note.id}"
    />
  {:else}
    <Button type="a" text="Destroy note now" className="!rounded-none" icon="i-line-md:close-circle" href="#destroy" />
  {/if}
</div>

{#if !note.duration_hours}
  <Modal
    buttonText="Yes, show me the note"
    text="You're about to read and destroy the note with id {note.id}."
    title="Read and destroy?"
    type="a"
    href="/note/{note.id}"
    id="destroy"
    icon="i-line-md:clipboard-check"
  />
{/if}
