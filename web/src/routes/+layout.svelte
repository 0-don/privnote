<script>
  import Footer from '$lib/components/container/Footer.svelte';
  import Discord from '$lib/svg/Discord.svelte';
  import Github from '$lib/svg/Github.svelte';
  import 'uno.css';
  import '../app.css';
</script>

<header class="bg-main">
  <div class="container py-3 items-center flex justify-between h-full">
    <section class="container flex items-center space-x-5 px-2 w-full">
      <a aria-label="Go To Homepage" href="/"
        ><img alt="logo" width="200" height="50" src="/privnote-logo.svg" class="cursor-cell" /></a
      >
      <p class="w-32 md:w-60 text-xs md:text-sm">Sent notes will self-destruct after being read.</p>
    </section>
    <section class="flex flex-col justify-between items-center h-full space-y-4 md:flex-row md:space-y-0 md:space-x-4">
      <a aria-label="Our Discord Server" href="https://discord.gg/coding" target="_blank"><Discord /></a>
      <a aria-label="Github Repository" href="https://github.com/0-don/privnote" target="_blank"><Github /></a>
    </section>
  </div>
</header>
<hr class="border-alt border-8" />
<section class="container flex flex-col justify-between">
  <slot />
</section>
<Footer />
