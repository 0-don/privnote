<script lang="ts">
  import { env } from '$env/dynamic/public';
  import type { Note, ResponseBody } from '$lib/@types';

  export let form: ResponseBody;

  const note = form.data as Note;
</script>

<div class="bg-olive p-2">
  {env.PUBLIC_DOMAIN}{note.id}
</div>
