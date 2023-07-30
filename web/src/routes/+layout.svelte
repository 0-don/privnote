<script>
  import 'uno.css';
  import '../app.css';
  import Footer from '$lib/components/container/Footer.svelte';
  import Discord from '$lib/svg/Discord.svelte';
  import Github from '$lib/svg/Github.svelte';
  import PrivnoteLogo from '$lib/svg/PrivnoteLogo.svelte';
</script>

<header class="bg-main">
  <div class="container py-3 items-center flex justify-between">
    <section class="container flex items-center space-x-5 px-2">
      <a href="/"><PrivnoteLogo /></a>
      <p class="w-64 text-sm">Send notes will self-destruct after being read.</p>
    </section>
    <section class="flex space-x-2">
      <a href="https://discord.gg/coding" target="_blank"><Discord /></a>
      <a href="https://github.com/Don-Cryptus/privnote" target="_blank"><Github /></a>
    </section>
  </div>
</header>
<hr class="border-alt border-8" />
<section class="container flex flex-col justify-between">
  <slot />
</section>
<Footer />
