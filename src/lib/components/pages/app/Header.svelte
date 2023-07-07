<script>
  import Button from '$lib/components/elements/Button.svelte';
</script>

<div class="mt-3 flex items-center justify-between">
  <h1 class="my-2 text-2xl font-bold">New note</h1>
  <Button className="mr-1 !mt-[0.35rem]" href="question" icon="i-line-md:question-circle" />
</div>
<div id="question" class="relative mt-4 hidden bg-container p-2 target:block">
  <div class="absolute -top-14 right-1"><Button href="" icon="i-line-md:close-circle" /></div>
  <div>
    With Privnote you can send notes that will self-destruct after being read.<br />
    1. Write the note below, encrypt it and get a link.<br />
    2. Send the link to whom you want to read the note.<br />
    3. The note will self-destruct after being read by the recipient.<br />
    <br />
    By clicking the options button, you can specify a manual password to encrypt the note, set an expiration date and be
    notified when the note is destroyed.<br />
    <br />
    To know more about how Privnote works, check out the
    <a href="/info/about">about page</a>.
  </div>
</div>
