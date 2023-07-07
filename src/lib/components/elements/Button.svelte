<script lang="ts">
  export let href = '';
  export let type = 'a' as 'button' | 'a';
  export let title = '';
  export let text = '';
  export let className = '';
  export let icon = '';

  export let defaultClassName =
    'flex items-center rounded-md border border-zinc-500 bg-zinc-600 p-2 shadow-main hover:border-main';
</script>

{#if type === 'a'}
  <div class={'cursor-pointer'} title={title || text}>
    <a href="#{href}" class={defaultClassName + ' ' + className}>
      <p>{text}</p>
      <i class={icon + ' text-xl'} />
    </a>
  </div>
{:else}
  <button title={title || text} type="submit" class={defaultClassName + ` ${className}`}>
    <p>{text}</p>
    <i class={icon + ' text-xl'} />
  </button>
{/if}
