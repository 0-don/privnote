<script lang="ts">
  import { dev } from '$app/environment';
  import type { DebugMessages, Messages, ResponseBody, Tag } from '$lib/@types';
  import InputForm from '$lib/components/pages/app/InputForm.svelte';
  import Header from '$lib/components/container/Header.svelte';
  import Debug from '$lib/components/pages/debug/Debug.svelte';
  import NewNoteInfo from '$lib/components/pages/app/info/NewNoteInfo.svelte';
  import NoteLinReadyInfo from '$lib/components/pages/app/info/NoteLinReadyInfo.svelte';

  export let form: ResponseBody;
  export let data: ResponseBody;

  const debug: DebugMessages = dev
    ? {
        data: [
          ...(data?.data ? [{ key: 'response', path: 'data', message: data.data }] : []),
          ...(form?.data ? [{ key: 'request', path: 'form', message: form.data }] : [])
        ] as Messages[],
        form: (form?.messages || []).map((m) => ({ ...m, key: 'form' })),
        captcha: (data?.messages || []).map((m) => ({ ...m, key: 'captcha' }))
      }
    : {};
</script>

<svelte:head>
  <title>Priv-Note</title>
</svelte:head>

<Debug {debug} />
<Header title={form?.data ? 'Note link ready' : 'New note'} info={form?.data ? NoteLinReadyInfo : NewNoteInfo} />
<InputForm {form} {data} />
