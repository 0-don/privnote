<script lang="ts">
  import { env } from '$env/dynamic/public';
  import type { Note, ResponseBody } from '$lib/@types';
  import Button from '$lib/components/elements/Button.svelte';

  export let form: ResponseBody;

  const note = form.data as Note;

  const url = `${env.PUBLIC_DOMAIN}${note.id}`;
</script>

<p class="bg-olive p-2 font-bold">
  {url}
</p>
<p class="bg-darkGold mb-4 p-2 font-light italic">The note will self-destruct after reading it.</p>
<div class="flex items-center justify-between">
  <Button type="a" text="E-mail link"  className="!rounded-none" icon="i-line-md:email-twotone" href="mailto:?body={url}" />
</div>
