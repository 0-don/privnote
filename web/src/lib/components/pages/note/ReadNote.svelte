<script lang="ts">
  import type { ResponseBody } from '$lib/@types';
  import Button from '$lib/components/elements/Button.svelte';

  export let data: ResponseBody;
  export let title = 'Note contents';
  export let alert = '';
</script>

<div class="mt-3 flex items-center justify-start">
  <h1 class="my-2 text-2xl font-bold">{title}</h1>
</div>
<div class="bg-alert text-center">{alert}</div>
