<script lang="ts">
  import { page } from '$app/stores';
  import type { Captcha, Note, ResponseBody } from '$lib/@types';
  import Input from '$lib/components/elements/Input.svelte';
  import Button from '$lib/components/elements/Button.svelte';
  import Modal from '$lib/components/elements/Modal.svelte';
  import dayjs from 'dayjs';
  import relativeTime from 'dayjs/plugin/relativeTime';

  dayjs.extend(relativeTime);

  export let title = 'Note contents';

  let data: ResponseBody<Note & Captcha> = $page.data;
  let form: ResponseBody<Note & Captcha> = $page.form;

  const error = data?.messages?.find(({ path }) => path === 'error')?.message || '';
  const body = data?.messages?.find(({ path }) => path === 'body')?.message || '';

  const note = data?.data?.id ? data.data : form?.data;
  const tag = data?.data?.tag || form?.data?.tag;
  const url = $page.params.id;
</script>

<div class="mt-3 flex items-center justify-start">
  <h1 class="my-2 text-2xl font-bold">{title}</h1>
</div>

{#if note?.id && dayjs(note?.delete_at).isValid()}
  <div class="bg-alert mb-3 p-1 text-center">
    This note will self-destruct in {dayjs(note?.delete_at).fromNow()} ({dayjs(note?.delete_at).toISOString()})
  </div>
{:else if note?.id}
  <div class="bg-alert mb-3 p-1 text-center">
    This note was destroyed. If you need to keep it, copy it before closing this window.
  </div>
{/if}

{#if error}
  <div class="mb-3 break-all text-red-400">{error}</div>
{/if}

{#if note?.note}
  <section id="content">
    <textarea
      rows="13"
      name="note"
      value={note.note}
      readonly
      placeholder="Write your note here..."
      class="w-full !bg-yellow-100 !bg-opacity-75 p-5 text-black outline-none placeholder:text-black"
    />
  </section>
{/if}

{#if body && !note}
  <form method="POST" action="?/password">
    <input name="id" type="hidden" value={url} />
    <input name="tag" type="hidden" value={tag} />

    <h3 class="mb-1 text-xl font-semibold">Manual password</h3>

    <div class="flex flex-col justify-between w-full md:w-1/2 space-y-2.5 md:flex-row md:space-x-5 md:space-y-0">
      <Input {form} type="password" name="manual_password" label="Enter a custom password to encrypt the note" />
    </div>
    <Button type="button" text="Proceed" className="!rounded-none mt-4" icon="i-line-md:play-filled" />
  </form>
{/if}

{#if dayjs(note?.delete_at).isValid() && tag && note?.id}
  <form method="POST" class="flex justify-end mt-2" action="?/delete">
    <input name="id" type="hidden" value={url} />
    <input name="tag" type="hidden" value={tag} />
    {#if note?.manual_password}
      <input name="manual_password" type="hidden" value={note?.manual_password} />
    {/if}

    {#if note?.destroy_without_confirmation}
      <Button type="button" text="Destroy note now" className="!rounded-none" icon="i-line-md:close-circle" />
    {:else}
      <Button
        type="a"
        href="#destroy"
        text="Destroy note now"
        className="!rounded-none"
        icon="i-line-md:close-circle"
      />
      <Modal
        buttonText="Yes, destroy the note"
        text="You're about to read and destroy the note with id {url}. This action is irreversible."
        title="Destroy the Note?"
        type="button"
        id="destroy"
        icon="i-line-md:clipboard-check"
      />
    {/if}
  </form>
{/if}
