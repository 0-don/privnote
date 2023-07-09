<script lang="ts">
  import type { CaptchaLoad, Notification } from '$lib/@types';
  import Header from '$lib/components/pages/app/Header.svelte';
  import InputForm from '$lib/components/pages/app/InputForm.svelte';

  export let form: Notification[] = [];
  export let data: CaptchaLoad;
</script>

<svelte:head>
  <title>Priv-Note</title>
</svelte:head>

<Header />
<InputForm {form} {data} />
