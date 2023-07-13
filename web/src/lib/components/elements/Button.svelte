<script lang="ts">
  import type { ResponseBody } from '$lib/@types';

  export let href = '';
  export let type = 'a' as 'button' | 'a';
  export let title = '';
  export let text = '';
  export let className = '';
  export let icon = '';

  export let form: ResponseBody | undefined = undefined;
  const error = form?.messages?.find((f) => f.path === text)?.message;

  let defaultClassName = `flex items-center rounded-md border border-zinc-500 bg-zinc-600 p-2 shadow-main hover:border-main ${
    text.length ? 'space-x-1' : ''
  }`;
</script>

{#if type === 'a'}
  <div class={'cursor-pointer'} title={title || text}>
    <a {href} class={`${defaultClassName} ${className}`}>
      <p>{text}</p>
      <i class={icon + ' text-xl'} />
    </a>
  </div>
{:else}
  <button name={text || title} title={title || text} type="submit" class={`${defaultClassName} ${className}`}>
    <p>{text}</p>
    <i class={icon + ' text-xl'} />
  </button>
{/if}
{#if error}
  <p class="text-red-400">{error}</p>
{/if}
