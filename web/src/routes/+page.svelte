<script lang="ts">
  import { dev } from '$app/environment';
  import type { Messages, ResponseBody, Tag } from '$lib/@types';
  import Header from '$lib/components/pages/app/Header.svelte';
  import InputForm from '$lib/components/pages/app/InputForm.svelte';
  import Debug from '$lib/components/pages/debug/Debug.svelte';

  export let form: ResponseBody;
  export let data: ResponseBody;
  const messages: Messages[] = dev
    ? ([
        ...(form?.messages || []).map((m) => ({ ...m, key: 'form' })),
        ...(data?.messages || []).map((m) => ({ ...m, key: 'captcha' })),
        ...(data?.data ? [{ key: 'data', path: 'error', message: data?.data }] : [])
      ] as Messages[])
    : [];
</script>

<svelte:head>
  <title>Priv-Note</title>
</svelte:head>

<Debug {messages} />
<Header />
<InputForm {form} {data} />
