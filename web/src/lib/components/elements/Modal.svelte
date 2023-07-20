<script lang="ts">
  import Button from './Button.svelte';
  export let buttonText: string;
  export let text: string;
  export let title: string;
  export let type: 'a' | 'button';
  export let id = '';
  export let href = '';
  export let icon = '';
</script>

<div
  {id}
  class="pointer-events-none fixed bottom-0 left-0 right-0 top-0 text-ellipsis opacity-0 transition-all target:pointer-events-auto target:opacity-100 z-40"
>
  <!-- svelte-ignore a11y-missing-content -->
  <!-- svelte-ignore a11y-invalid-attribute -->
  <a href="#" class="close hover:opacity-40" />
  <div
    id="target-inner"
    class="bg-container absolute left-1/2 top-1/2 block w-1/2 -translate-x-1/2 -translate-y-1/2 p-5 shadow-xl"
  >
    <h2 class="text-3xl font-bold">{title}</h2>
    <p class="my-2 text-gray-300">{text}</p>
    <div class="flex justify-between">
      <Button {type} {icon} {href} text={buttonText} className="!rounded-none !bg-main" />
      <Button type="a" text="Cancel" className="!rounded-none" icon="i-line-md:close-circle" href="#" />
    </div>
  </div>
</div>
