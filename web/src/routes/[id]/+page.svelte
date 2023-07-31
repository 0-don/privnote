<script lang="ts">
  import Debug from '$lib/components/utils/Debug.svelte';
  import ReadNote from '$lib/components/pages/note/ReadNote.svelte';
  import ConfirmReadNote from '$lib/components/pages/note/ConfirmReadNote.svelte';
  import type { ResponseBody } from '$lib/@types';

  export let data: ResponseBody<Boolean>;
</script>

<Debug />

{#if !data?.data}
  <ReadNote />
{:else}
  <ConfirmReadNote />
{/if}
