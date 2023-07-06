<script>
	import 'uno.css';
	import '../app.postcss';

</script>

<main>
<header class="bg-rose-600 flex justify-center items-center py-1 space-x-2">
	<img alt="logo" src="/privnote-logo.svg" >
	<p class="w-64 text-sm">Send notes will self-destruct after being read.</p>
</header>
<slot />
</main>
