<script lang="ts">
  import { page } from '$app/stores';
  import Button from '$lib/components/elements/Button.svelte';
</script>

<div class="mt-3 flex items-center">
  <h1 class="text-2xl font-bold">Read and destroy?</h1>
</div>
<p class="my-2">You're about to read and destroy the note with id {$page.params.id}.</p>
<form method="POST" action="?/getNote" class="flex items-center justify-between">
  <Button text="Yes, show me the note" className="!rounded-none" type="button" icon="i-line-md:clipboard-check" />
  <Button text="No, not now" className="!rounded-none" href="/" type="a" icon="i-line-md:close-circle" />
</form>
