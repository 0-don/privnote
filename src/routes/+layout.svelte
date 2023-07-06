<script>
  import 'uno.css';
  import '../app.postcss';
</script>

<header class="absolute left-0 right-0 top-0 bg-rose-600 py-1">
  <section class="sm:container flex items-center space-x-2 px-2">
    <img alt="logo" src="/privnote-logo.svg" />
    <p class="w-64 text-sm">Send notes will self-destruct after being read.</p>
  </section>
</header>
<slot />
