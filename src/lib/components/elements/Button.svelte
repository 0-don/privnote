<script lang="ts">
  export let id = '';
  export let type = 'a' as 'button' | 'a';
  export let text = '';
  export let className = '';
  export let icon = '';
</script>

{#if type === 'a'}
  <div class={className + ' cursor-pointer'}>
    <a
      href="#{id}"
      class="flex items-center rounded-md border border-zinc-500 bg-zinc-600 p-2 shadow-main hover:border-main"
    >
      <p>{text}</p>
      <i class={icon + ' text-xl'} />
    </a>
  </div>
{:else}
  <button
    type="submit"
    class={className +
      ' flex items-center rounded-md border border-zinc-500 bg-zinc-600 p-2 shadow-main hover:border-main'}
  >
    <p>{text}</p>
    <i class={icon + ' text-xl'} />
  </button>
{/if}
