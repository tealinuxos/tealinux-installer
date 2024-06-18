<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { json } from '../stores.js';
	import { writable } from 'svelte/store';

	let isOpen = false;
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

<section class="flex flex-col items-center">
	<h1 class="text-center mb-16">Search Locales</h1>
	{#if showLocales}
		<form class="bg-slate-300 text-center w-[50dvw] p-8 rounded-md min-h-[50dvh]">
			<input
				type="text"
				placeholder="Search..."
				class="p-2 mb-8 max-w-full rounded-md border border-gray-300 focus:border-gray-500 focus:outline-none focus:ring-1 focus:ring-gray-500"
				bind:value={searchTerm}
				on:click={handleFocusIn}
				on:focus={handleFocusIn}
			/>
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
		</form>
		<a href="/installation/timezone" on:click={handleSetLocale}>Next</a>
	{/if}
</section>
