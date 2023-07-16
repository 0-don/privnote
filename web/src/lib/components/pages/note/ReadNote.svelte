<script lang="ts">
  import type { NoteResponse, ResponseBody } from '$lib/@types';
  import Button from '$lib/components/elements/Button.svelte';

  export let data: ResponseBody<NoteResponse>;
  export let title = 'Note contents';
</script>

<div class="mt-3 flex items-center justify-start">
  <h1 class="my-2 text-2xl font-bold">{title}</h1>
</div>
<div class="bg-alert text-center">{data.data?.alert}</div>

<section id="content">
  <textarea
    rows="13"
    name="note"
    value={data.data?.note.note}
    placeholder="Write your note here..."
    class="w-full !bg-yellow-100 !bg-opacity-75 p-5 text-black outline-none placeholder:text-black"
  />
</section>
