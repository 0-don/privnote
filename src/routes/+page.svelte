<script>
  import Popup from '$lib/components/elements/Popup.svelte';
</script>

<div class="flex items-center justify-between">
  <h1 class="my-2 text-2xl font-bold">New note</h1>
  <Popup id="question" />
</div>
<div id="question" class="hidden target:block">asd</div>
