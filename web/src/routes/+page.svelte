<script lang="ts">
  import Header from '$lib/components/pages/app/Header.svelte';
  import InputForm from '$lib/components/pages/app/InputForm.svelte';
  import type { ActionData } from './$types';

  export let form: ActionData;
</script>

<svelte:head>
  <title>Priv-Note</title>
</svelte:head>

<Header />
<InputForm {form} />
