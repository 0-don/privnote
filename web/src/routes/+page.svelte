<script lang="ts">
  import type { CaptchLoad, CreateNoteResponse } from '$lib/@types';
  import Header from '$lib/components/pages/app/Header.svelte';
  import InputForm from '$lib/components/pages/app/InputForm.svelte';

  export let form: CreateNoteResponse[] = [];
  export let data: CaptchLoad;
</script>

<svelte:head>
  <title>Priv-Note</title>
</svelte:head>

<Header />
<InputForm {form} {data} />
