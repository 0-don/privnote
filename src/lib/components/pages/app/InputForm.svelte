<script>
  import Button from '$lib/components/elements/Button.svelte';
  import Dropdown from '$lib/components/elements/Dropdown.svelte';
</script>

<form>
  <section id="content" class="mt-4">
    <textarea
      rows="15"
      placeholder="Write your note here..."
      class="w-full !bg-yellow-100 !bg-opacity-75 p-5 text-black outline-none placeholder:text-black"
    />
  </section>

  <section id="options" class="relative mt-4 hidden target:block">
    <h3>Note self-destructs</h3>
    <div class="flex justify-between space-x-5">
      <Dropdown
        options={[
          { text: 'after reading it', value: 0 },
          { text: '1 hour from now', value: 1 },
          { text: '24 hours from now', value: 24 },
          { text: '7 days from now', value: 168 },
          { text: '30 days from now', value: 720 }
        ]}
      />

      <label class="w-full" for="no-confirm"
        ><input id="no-confirm" name="no-confirm" type="checkbox" /> Do not ask for confirmation before showing and destroying
        the note.</label
      >
    </div>
    <div class="flex justify-end"><Button href="" icon="i-line-md:close-circle" /></div>
  </section>

  <section class="mt-4 flex justify-between">
    <Button type="button" text="Create Note" icon="i-line-md:document-add" />
    <Button href="options" text="Show Options" icon="i-line-md:edit-twotone" />
  </section>
</form>
