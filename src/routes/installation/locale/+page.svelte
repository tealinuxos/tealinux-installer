<script>
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { json } from '../stores.js';
	import { writable } from 'svelte/store';

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
<Sidebar />
<section class="flex flex-col items-center justify-center h-screen">
	{#if showLocales}
	<!-- bg-slate-300 -->
		<form class=" text-center w-[50dvw] p-8 rounded-md min-h-[50dvh]">
			<h1 class="text-center mb-6 font-bold text-[32px] font-archivo">Select Locales</h1>

			<div class="max-w-md mx-auto mb-4">
				<h2 class="font-poppin text-left mb-2 font-medium">Main Locale</h2>
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
					<!-- <img src={mag_glass} alt="" class="h-4 w-4" /> -->
					<svg width="20" height="21" viewBox="0 0 20 21" fill="none" xmlns="http://www.w3.org/2000/svg">
						<path d="M19 19.5L14.65 15.15M17 9.5C17 13.9183 13.4183 17.5 9 17.5C4.58172 17.5 1 13.9183 1 9.5C1 5.08172 4.58172 1.5 9 1.5C13.4183 1.5 17 5.08172 17 9.5Z" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
						</svg>
						
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
				<h2 class="font-poppin text-left mb-2 font-medium">Date and time</h2>
				<div class="relative flex items-center w-[421px] h-[45px] rounded-lg bg-white overflow-hidden border border-greyBorder">
					<input type="text" id="" class="peer h-full w-full outline-none text-sm text-gray-700 pr-2 pl-[12px]">
				</div>
			</div>

			<div class="max-w-md mx-auto mb-4">
				<h2 class="font-poppin text-left mb-2 font-medium">Number and currency</h2>
				<div class="relative flex items-center w-[421px] h-[45px] rounded-lg bg-white overflow-hidden border border-greyBorder">
					<input type="text" id="" class="peer h-full w-full outline-none text-sm text-gray-700 pr-2 pl-[12px]">
				</div>
			</div>

			<div class="max-w-md mx-auto mt-[208px]">
				<div class="grid grid-cols-2 gap-[295px]">
					<a href="/installation/timezone" class="text-white bg-greyButton hover:bg-gray-500 focus:ring-4 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 h-[44px] w-[76px]">Back</a>
					<a href="/installation/account" on:click={handleSetLocale} class="text-white bg-greyButton hover:bg-gray-500 focus:ring-4 focus:ring-gray-900 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 focus:outline-none h-[44px] w-[76px]">Next</a>
				</div>
			</div>
		</form>		
	{/if}
</section>

