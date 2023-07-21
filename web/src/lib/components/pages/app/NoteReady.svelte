<script lang="ts">
  import { page } from '$app/stores';
  import { env } from '$env/dynamic/public';
  import type { Note, ResponseBody } from '$lib/@types';
  import Button from '$lib/components/elements/Button.svelte';
  import Modal from '$lib/components/elements/Modal.svelte';

  let form: ResponseBody<{ note: Note; secret: string }> = $page.form;

  const note = form.data?.note;
  const secret = form.data?.secret;

  const url = `${note?.id}@${secret}`
  const link = `${env.PUBLIC_DOMAIN}note/${url}`;
</script>

<!-- svelte-ignore a11y-autofocus -->
<input type="text" value="{link}" class="bg-olive p-2 font-bold" autofocus readonly />

<p class="italic bg-darkGold mb-4 p-2 font-light">The note will self-destruct after reading it.</p>
<div class="flex items-center justify-between">
  <Button
    type="a"
    text="E-mail link"
    className="!rounded-none"
    icon="i-line-md:email-twotone"
    href="mailto:?body={link}"
  />
  {#if note?.duration_hours}
    <Button
      type="a"
      text="Read note"
      className="!rounded-none"
      icon="i-line-md:clipboard-check"
      href="/note/{url}"
    />
  {:else}
    <Button type="a" text="Destroy note now" className="!rounded-none" icon="i-line-md:close-circle" href="#destroy" />
  {/if}
</div>

{#if !note?.duration_hours}
  <Modal
    buttonText="Yes, show me the note"
    text="You're about to read and destroy the note with id {url}."
    title="Read and destroy?"
    type="a"
    href="/note/{url}"
    id="destroy"
    icon="i-line-md:clipboard-check"
  />
{/if}
