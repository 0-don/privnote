<script>
  import Button from '$lib/components/elements/Button.svelte';
</script>

<form>
  <section id="content" class="mt-4">
    <textarea
      rows="15"
      placeholder="Write your note here..."
      class="w-full !bg-yellow-100 !bg-opacity-75 p-5 text-black outline-none placeholder:text-black"
    />
  </section>

  <section class="flex justify-between">
    <Button type="button" text="Create Note" icon="i-line-md:document-add" />
    <Button id="#options" text="Show Options" icon="i-line-md:edit-twotone" />
  </section>
</form>
