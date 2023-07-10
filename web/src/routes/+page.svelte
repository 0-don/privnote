<script lang="ts">
  import { dev } from '$app/environment';
  import type { DebugMessages, Messages, ResponseBody, Tag } from '$lib/@types';
  import Header from '$lib/components/pages/app/Header.svelte';
  import InputForm from '$lib/components/pages/app/InputForm.svelte';
  import Debug from '$lib/components/pages/debug/Debug.svelte';

  export let form: ResponseBody;
  export let data: ResponseBody;
  const debug: DebugMessages = dev
    ? {
        data: [
          ...(data?.data ? [{ key: 'response', path: 'data', message: data.data }] : []),
          ...(form.data ? [{ key: 'request', path: 'data', message: form.data }] : [])
        ],
        form: (form?.messages || []).map((m) => ({ ...m, key: 'form' })),
        captcha: (data?.messages || []).map((m) => ({ ...m, key: 'captcha' }))
      }
    : // ? ([
      //     ...(form?.messages || []).map((m) => ({ ...m, key: 'form' })),
      //     ...(data?.messages || []).map((m) => ({ ...m, key: 'captcha' })),
      //     ...(data?.data ? [{ key: 'data', path: 'data', message: data?.data }] : [])
      //   ] as Messages[])
      {};
</script>

<svelte:head>
  <title>Priv-Note</title>
</svelte:head>

<Debug {debug} />
<Header />
<InputForm {form} {data} />
