<script lang="ts">
  export let name: string;
  export let className = '';
</script>

<fieldset class="w-full {className}">
  <legend class="sr-only">{name.charAt(0).toUpperCase() + name.slice(1, name.length)}</legend>
  <div class="space-y-5">
    <div class="flex items-start">
      <div class="flex h-6 items-center">
        <input id={name} {name} type="checkbox" class="h-4 w-4 rounded border-gray-300 text-main focus:ring-main" />
      </div>
      <div class="ml-3 text-sm leading-6">
        <label for="comments" class="font-medium text-white"
          >Do not ask for confirmation before showing and destroying the note.</label
        >
      </div>
    </div>
  </div>
</fieldset>
