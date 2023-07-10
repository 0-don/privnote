<script lang="ts">
  import { dev } from '$app/environment';
  import type { CaptchaLoad, Notification, NotificationEvent } from '$lib/@types';
  import Header from '$lib/components/pages/app/Header.svelte';
  import InputForm from '$lib/components/pages/app/InputForm.svelte';
  import Debug from '$lib/components/pages/debug/Debug.svelte';

  export let form: Notification[] = [];
  export let data: CaptchaLoad;
  const events: NotificationEvent[] = dev
    ? [
        ...(form || []).map((m) => ({ ...m, key: 'form' })),
        ...(Array.isArray(data) ? data : []).map((m) => ({ ...m, key: 'captcha' }))
      ]
    : [];
</script>

<svelte:head>
  <title>Priv-Note</title>
</svelte:head>

<Debug {events} />
<Header />
<InputForm {form} {data} />
