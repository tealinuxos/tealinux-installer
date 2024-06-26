<script>
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { json } from '../stores.js';
	import { writable } from 'svelte/store';
	import mag_glass from '$lib/assets/magnifying-glass.png';

	let searchTerm = '';

	let locales = [];
	let showLocales = false;
	let selectedLocales = [];
	let filteredLocales = writable([]);

	const getLocale = async () => {
		invoke('get_locale_json').then((response) => {
			locales = JSON.parse(response);
			filteredLocales.set(locales);
			showLocales = true;
		});
	};

	const handleSetLocale = async () => {
		json.locale.locales = selectedLocales;
		json.locale.main = selectedLocales[0];
		console.log(json);
	};

	function filterOptions() {
		const term = searchTerm.toLowerCase();
		filteredLocales.set(locales.filter((locale) => locale.name.toLowerCase().includes(term)));
	}

	function handleFocusIn() {
		isOpen = true;
	}

	function handleOptionClick(locale) {
		if (!selectedLocales.includes(locale.name)) {
			selectedLocales = [...selectedLocales, locale.name];
		}
		console.log(selectedLocales);
	}

	$: searchTerm, filterOptions();

	onMount(() => {
		getLocale();
	});
</script>

<section class="flex flex-col items-center justify-center h-screen">
	<h1 class="text-center mb-16 font-bold text-[32px] font-archivo">Select Locales</h1>
	{#if showLocales}
	<!-- bg-slate-300 -->
		<form class=" text-center w-[50dvw] p-8 rounded-md min-h-[50dvh]">
			<div class="max-w-md mx-auto mb-4">
				<h2 class="font-poppin text-left mb-2">Main Locale</h2>
				<div class="relative flex items-center w-[421px] h-[45px] rounded-lg bg-white overflow-hidden border border-greyBorder">
				  <input
					type="text"
					placeholder="Search..."
					class="peer h-full w-full outline-none text-sm text-gray-700 pr-2 pl-8"
					bind:value={searchTerm}
					on:click={handleFocusIn}
					on:focus={handleFocusIn}
				  />
				  <div class="inset-y-0 left-0 flex items-center pr-4">
					<img src={mag_glass} alt="" class="h-4 w-4" />
				  </div>
				</div>
			  </div>
			  
			<div class="h-[50dvh] overflow-y-auto rounded-xl border border-slate-400">
				{#each $filteredLocales as locale}
					<div
						class="flex flex-row-reverse w-full items-center justify-between py-4 px-4 border border-b-slate-400 last:border-none bg-slate-200 hover:bg-slate-100 transition-all"
					>
						<input
							type="checkbox"
							name={locale.name}
							class="h-6 w-6"
							checked={selectedLocales.includes(locale.name)}
							on:click={handleOptionClick(locale)}
							value={locale.name}
						/>
						<label for={locale.name}>{locale.name}</label>
					</div>
				{/each}
			</div>

			<div class="max-w-md mx-auto mb-4">
				<h2 class="font-poppin text-left mb-2">Date and time</h2>
				<div class="relative flex items-center w-[421px] h-[45px] rounded-lg bg-white overflow-hidden border border-greyBorder">
					<input type="text" id="" class="peer h-full w-full outline-none text-sm text-gray-700 pr-2 pl-8">
				</div>
			</div>

			<div class="max-w-md mx-auto mb-4">
				<h2 class="font-poppin text-left mb-2">Number and currency</h2>
				<div class="relative flex items-center w-[421px] h-[45px] rounded-lg bg-white overflow-hidden border border-greyBorder">
					<input type="text" id="" class="peer h-full w-full outline-none text-sm text-gray-700 pr-2 pl-8">
				</div>
			</div>
			
			<div class="max-w-md mx-auto">
				<div class="grid grid-cols-2 gap-52 p-4">
					<a href="#" class="text-white bg-greyButton hover:bg-gray-500 focus:ring-4 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 h-[44px] w-[76px]">Back</a>
					<a href="/installation/timezone" on:click={handleSetLocale} class="text-white bg-greyButton hover:bg-gray-500 focus:ring-4 focus:ring-gray-900 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 focus:outline-none h-[44px] w-[76px]">Next</a>
				</div>
			</div>
		</form>		
	{/if}
</section>

