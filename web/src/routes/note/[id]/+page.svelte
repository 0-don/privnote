<script lang="ts">
  import type { ResponseBody } from '$lib/@types';
  import ReadNote from '$lib/components/pages/note/ReadNote.svelte';

  export let data: ResponseBody;
</script>

<div>{JSON.stringify(data)}</div>

<ReadNote {data} />
