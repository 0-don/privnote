<script>
  import ClosePopup from '$lib/components/elements/ClosePopup.svelte';
  import Popup from '$lib/components/elements/Popup.svelte';
</script>

<div class="flex items-center justify-between">
  <h1 class="my-2 text-2xl font-bold">New note</h1>
  <Popup className="mr-2" id="question" />
</div>
<div id="question" class="relative mt-4 hidden bg-container p-2 target:block">
  <div class="absolute right-2 top-2"><ClosePopup /></div>
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
