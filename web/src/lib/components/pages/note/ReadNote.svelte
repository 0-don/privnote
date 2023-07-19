<script lang="ts">
  import type { Captcha, NoteResponse, ResponseBody } from '$lib/@types';
  import Button from '$lib/components/elements/Button.svelte';
  import dayjs from 'dayjs';
  import relativeTime from 'dayjs/plugin/relativeTime';

  dayjs.extend(relativeTime);

  export let data: ResponseBody<NoteResponse & Captcha>;
  export let title = 'Note contents';

  const error = data?.messages?.find(({ path }) => path === 'error')?.message || '';
  const alert = data?.data?.alert;
  const note = data?.data?.note;
  const tag = data?.data?.tag;
</script>

<div class="mt-3 flex items-center justify-start">
  <h1 class="my-2 text-2xl font-bold">{title}</h1>
</div>

{#if alert && dayjs(alert).isValid()}
  <div class="bg-alert mb-3 p-1 text-center">
    This note will self-destruct in {dayjs(alert).fromNow()} ({dayjs(alert).toISOString()})
  </div>
{:else if alert}
  <div class="bg-alert mb-3 p-1 text-center">{alert}</div>
{/if}

{#if error}
  <div class="mb-3 break-all text-red-400">{error}</div>
{/if}

{#if note}
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

{#if dayjs(alert).isValid() && note && tag}
  <form method="POST" class="flex justify-end mt-2">
    <input name="id" type="hidden" value={note.id} />
    <input name="tag" type="hidden" value={tag} />
    <Button type="button" text="Destroy note now" className="!rounded-none" icon="i-line-md:close-circle" />
  </form>
{/if}
