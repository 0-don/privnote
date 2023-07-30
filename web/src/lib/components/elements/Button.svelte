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
  <div class={'cursor-pointer group'} title={title || text}>
    <a {href} class={`${defaultClassName} ${className}`} aria-label={title || text || href}>
      <p>{text}</p>
      <i class={icon + ' text-xl group-hover:bg-purple-300 transition-all duration-1000'} />
    </a>
  </div>
{:else}
  <button title={title || text} type="submit" class={`${defaultClassName} ${className} group`}>
    <p>{text}</p>
    <i class={icon + ' text-xl group-hover:bg-purple-300 transition-all duration-1000'} />
  </button>
{/if}
{#if error}
  <p class="text-red-400">{error}</p>
{/if}
