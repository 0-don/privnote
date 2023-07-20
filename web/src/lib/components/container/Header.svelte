<script lang="ts">
  import Button from '$lib/components/elements/Button.svelte';

  export let title = 'New note';
  export let info: ConstructorOfATypedSvelteComponent;
</script>

<div class="mt-3 flex items-center justify-between">
  <h1 class="text-2xl font-bold">{title}</h1>
  <Button className="mr-1 !mt-[0.35rem]" href="#question" icon="i-line-md:question-circle" />
</div>
<div id="question" class="bg-container relative mt-4 hidden p-2 target:block">
  <div class="absolute top-[-3.35rem] right-1"><Button title="question" href="#" icon="i-line-md:close-circle" /></div>
  <svelte:component this={info} />
</div>
<hr class="mb-4 border-transparent" />
