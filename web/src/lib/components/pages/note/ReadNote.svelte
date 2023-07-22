<script lang="ts">
  import { page } from '$app/stores';
  import type { Captcha, Note, NoteResponse, ResponseBody } from '$lib/@types';
  import Button from '$lib/components/elements/Button.svelte';
  import Modal from '$lib/components/elements/Modal.svelte';
  import dayjs from 'dayjs';
  import relativeTime from 'dayjs/plugin/relativeTime';

  dayjs.extend(relativeTime);

  export let title = 'Note contents';
  let data: ResponseBody<Note & Captcha> = $page.data;

  const error = data?.messages?.find(({ path }) => path === 'error')?.message || '';

  const note = data?.data;
  const tag = data?.data?.tag;
  const url = $page.params.id;

  console.log(note?.id);
</script>

<div class="mt-3 flex items-center justify-start">
  <h1 class="my-2 text-2xl font-bold">{title}</h1>
</div>

{#if note?.id && dayjs(note?.deleted_at).isValid()}
  <div class="bg-alert mb-3 p-1 text-center">
    This note will self-destruct in {dayjs(note?.deleted_at).fromNow()} ({dayjs(note?.deleted_at).toISOString()})
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

{#if dayjs(note?.deleted_at).isValid() && note?.id && tag}
  <form method="POST" class="flex justify-end mt-2">
    <input name="id" type="hidden" value={note.id} />
    <input name="tag" type="hidden" value={tag} />
    <Button type="a" href="#destroy" text="Destroy note now" className="!rounded-none" icon="i-line-md:close-circle" />
    <Modal
      buttonText="Yes, destroy the note"
      text="You're about to read and destroy the note with id {url}. This action is irreversible."
      title="Destroy the Note?"
      type="button"
      id="destroy"
      icon="i-line-md:clipboard-check"
    />
  </form>
{/if}
