<script>
  import 'uno.css';
  import '../app.css';
</script>

<header class="bg-main py-3">
  <section class="container flex items-center space-x-5 px-2">
    <img alt="logo" src="/privnote-logo.svg" class="cursor-progress" />
    <p class="w-64 text-sm">Send notes will self-destruct after being read.</p>
  </section>
</header>
<hr class="border-8 border-alt" />
<section class="container relative">
  <slot />
</section>
