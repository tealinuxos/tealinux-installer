<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	let json;

	async function handleGetJSON() {
		invoke('get_read_json').then((response) => {
			json = JSON.parse(response);
			console.log(json);
			showModelName = true;
		}).await;
	}

	async function handleGetLocale() {
		invoke('get_locale_json').then((locales) => {
			locales = JSON.parse(locales);
			console.log(locales);
		}).await;
	}

	onMount(() => {
		handleGetJSON();
		handleGetLocale();
	});
</script>

<nav class="flex flex-col flex-1 overflow-y-auto bg-white">
	<div class="bg-[#D9D9D9] flex items-center py-4 px-8 w-full font-sans font-medium text-2xl">
		Tealinux Installer
	</div>
</nav>

<main class="p-8">
	<div
		class="bg-[#D9D9D9] w-full grid-cols-3 grid place-items-center py-3 px-16 h-[40vh] rounded-2xl"
	>
		<div class="border border-black flex flex-col items-center gap-y-1">
			<!-- <img src="{laptop}" alt="gambar laptop" class="w-1/2"> -->
			<p class="text-2xl font-medium">82SV</p>
			<p>Yoga Slim 7Pro 14|AP7</p>
			<p>80 %</p>
			<p>online</p>
		</div>
		<div>ini bagian tengah</div>
		<div>ini bagian kanan</div>
	</div>
	<div class="w-full mt-8 py-4 grid grid-cols-2 gap-8 max-h-96 overflow-y-auto">
		{#each [1, 2, 3, 4, 5, 6] as e}
			<div class="aspect-square bg-[#D9D9D9]"></div>
		{/each}
	</div>
	<a href="/installation">Install</a>
</main>
