<script lang="ts">
  import { page } from '$app/stores';
  import { env } from '$env/dynamic/public';
  import type { Note, ResponseBody } from '$lib/@types';
  import Button from '$lib/components/elements/Button.svelte';
  import Input from '$lib/components/elements/Input.svelte';
  import Modal from '$lib/components/elements/Modal.svelte';

  let form: ResponseBody<{ note: Note; secret: string }> = $page.form;

  const note = form.data?.note;
  const secret = form.data?.secret;

  const url = `${note?.id}@${secret}`;
  const link = `${env.PUBLIC_DOMAIN}${url}`;
</script>

<input type="text" value={link} class="bg-olive p-2 font-bold" readonly />

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
    <Button type="a" text="Read note" className="!rounded-none" icon="i-line-md:clipboard-check" href="/{url}" />
  {:else if !note?.destroy_without_confirmation}
    <Button type="a" text="Destroy note now" className="!rounded-none" icon="i-line-md:close-circle" href="#destroy" />
  {:else}
    <Button type="a" text="Destroy note now" className="!rounded-none" icon="i-line-md:close-circle" href="/{url}" />
  {/if}
</div>

{#if !note?.duration_hours && !note?.destroy_without_confirmation}
  <Modal
    buttonText="Yes, show me the note"
    text="You're about to read and destroy the note with id {url}."
    title="Read and destroy?"
    type="a"
    href="/{url}"
    id="destroy"
    icon="i-line-md:clipboard-check"
  />
{/if}

{#if note?.manual_password}
  <h3 class="mb-1 text-xl font-semibold mt-2">Manual password</h3>
  <div class="flex flex-col justify-between space-y-2.5 md:flex-row md:space-x-5 md:space-y-0">
    <Input {form} type="password" name="manual_password" label="Password necessary to read the note" />
  </div>
{/if}
