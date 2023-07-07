<script>
  export let id = '';
  export let className = '';
  export let icon = '';
</script>

<div class={className + ' cursor-pointer'}>
  <a
    href="#{id}"
    class="inline-flex items-center rounded-md border border-zinc-500 bg-zinc-600 p-2 shadow-2xl shadow-main hover:border-main"
  >
    <i class={icon + ' text-xl'} />
  </a>
</div>
