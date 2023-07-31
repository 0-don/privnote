<script lang="ts">
  import type { ResponseBody } from '$lib/@types';
  import InputForm from '$lib/components/pages/app/InputForm.svelte';
  import Header from '$lib/components/container/Header.svelte';
  import Debug from '$lib/components/utils/Debug.svelte';
  import NewNoteInfo from '$lib/components/pages/app/info/NewNoteInfo.svelte';
  import NoteLinReadyInfo from '$lib/components/pages/app/info/NoteLinReadyInfo.svelte';
  import NoteReady from '$lib/components/pages/app/NoteReady.svelte';

  export let form: ResponseBody;
</script>

<Debug />
<Header title={form?.data ? 'Note link ready' : 'New note'} info={form?.data ? NoteLinReadyInfo : NewNoteInfo} />

{#if form?.data}
  <NoteReady />
{:else}
  <InputForm />
{/if}
