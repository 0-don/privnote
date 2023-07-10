<script lang="ts">
  import type { ResponseBody } from '$lib/@types';

  export let name: string;
  export let className = '';

  export let form: ResponseBody;
  const error = form?.messages?.find((f) => f.path === name)?.message;
</script>

<fieldset class="w-full {className}">
  <legend class="sr-only">{name.charAt(0).toUpperCase() + name.slice(1, name.length)}</legend>
  <div class="space-y-5">
    <div class="flex items-start">
      <div class="flex h-6 items-center">
        <input
          id={name}
          {name}
          checked
          value="1"
          type="checkbox"
          class="text-main focus:ring-main h-4 w-4 rounded border-gray-300"
        />
      </div>
      <div class="ml-3 text-sm leading-6">
        <label for="comments" class="font-medium text-white"
          >Do not ask for confirmation before showing and destroying the note.</label
        >
      </div>
    </div>
  </div>
  {#if error}
    <p class="text-red-400">{error}</p>
  {/if}
</fieldset>
