<script lang="ts">
  import type { Captcha, NoteResponse, ResponseBody } from '$lib/@types';
  import Debug from '$lib/components/utils/Debug.svelte';
  import ReadNote from '$lib/components/pages/note/ReadNote.svelte';
  import { debugLog } from '$lib/utils/client/constants';

  export let form: ResponseBody;
  export let data: ResponseBody<NoteResponse & Captcha>;
</script>

<Debug debug={debugLog(data, form)} />

<ReadNote {data} />
