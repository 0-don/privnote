<script lang="ts">
  import type { NoteResponse, ResponseBody } from '$lib/@types';
  import Debug from '$lib/components/pages/debug/Debug.svelte';
  import ReadNote from '$lib/components/pages/note/ReadNote.svelte';
  import { debugLog } from '$lib/utils/client/constants';

  export let form: ResponseBody;
  export let data: ResponseBody<NoteResponse>;
</script>

<Debug debug={debugLog(data, form)} />

<ReadNote {data} />
