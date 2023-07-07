<script lang="ts">
  export let className = '';
  export let name = '';
  export let type = 'text';
  export let label = '';
</script>

<div class="flex w-full flex-col">
  <label for={name} class="text-sm">{label}</label>
  <input
    id={name}
    {type}
    {name}
    class="w-full bg-zinc-600 p-2 focus:border-transparent focus:bg-zinc-500 focus:ring focus:ring-transparent {className}"
  />
</div>
