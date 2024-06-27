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

<main class="ax-h-dvh">
	<div class="p-8 overflow-auto max-h-[85dvh]">
		<div class=" bg-[#D9D9D9] bg-opacity-45 w-full p-5 rounded-2xl mb-6">
			<div class="bg-[#DCDCDC] grid-cols-3 grid place-items-center py-3 px-16 h-[35vh] rounded-3xl">
				<div class="border border-black flex flex-col items-center gap-y-1">
					<p class="text-2xl font-medium">82SV</p>
					<p>Yoga Slim 7Pro 14|AP7</p>
					<p>80 %</p>
					<p>online</p>
				</div>
				<div>ini bagian tengah</div>
				<div>ini bagian kanan</div>
			</div>
		</div>
		{#each [1, 2] as x}
			<div class=" bg-[#D9D9D9] bg-opacity-45 w-full p-5 rounded-2xl mb-6">
				<div
					class="bg-[#DCDCDC] grid-cols-3 grid place-items-center py-3 px-16 h-[35vh] rounded-3xl"
				></div>
			</div>
		{/each}
	</div>

	<div class="flex justify-between h-[15dvh] items-center w-full px-8">
		<button
			on:click={() => {
				window.location.href = '/';
			}}
			type="button"
			class="py-[10px] px-[20px] bg-[#929AAB] text-white font-poppin text-base rounded-lg"
			>Back</button
		>
		<button
			on:click={() => {
				window.location.href = '/installation/keyboard';
			}}
			type="button"
			class="py-[10px] px-[20px] bg-[#929AAB] text-white font-poppin text-base rounded-lg"
			>Next</button
		>
	</div>
</main>
