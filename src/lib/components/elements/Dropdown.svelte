<script lang="ts">
  export let className = '';
  export let name = '';
  export let options = [] as { value: string | number; text: string | number }[];
</script>

<label class="w-full" for={name}>
  <select id={name} {name} class="{className} w-full bg-zinc-600">
    {#each options as { value, text }, i}
      <option {value}>{text}</option>
    {/each}
  </select>
</label>
