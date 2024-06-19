<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { json } from '../stores.js';

	let timezones = [];
	let showTimezone = false;
	let selectedTimezone = null;
	let filteredTimezones = [];
	let searchTerm = '';

	const getTimezone = async () => {
		invoke('get_timezone_json').then((response) => {
			timezones = JSON.parse(response);
			filteredTimezones = timezones;
			console.log(timezones);
			showTimezone = true;
		});
	};

	const handleSetTimezone = async () => {
		json.timezone.region = selectedTimezone.split('/')[0];
		json.timezone.city = selectedTimezone.split('/')[1] || null;
		console.log(json);
	};

	function filterOptions() {
		const term = searchTerm.toLowerCase();
		filteredTimezones = timezones.filter((timezone) =>
			timezone.region.toLowerCase().includes(term)
		);
	}

	$: searchTerm, filterOptions();

	onMount(() => {
		getTimezone();
	});
</script>

<section class="flex flex-col items-center">
	<h1>Select Timezone</h1>
	{#if showTimezone}
		<form class="bg-slate-300 text-center w-[50dvw] p-8 rounded-md min-h-[50dvh]">
			<input
				type="text"
				placeholder="Search..."
				class="p-2 mb-8 max-w-full rounded-md border border-gray-300 focus:border-gray-500 focus:outline-none focus:ring-1 focus:ring-gray-500"
				bind:value={searchTerm}
			/>
			<div class="h-[50dvh] overflow-y-auto rounded-xl border border-slate-400">
				{#each filteredTimezones.sort((a, b) => a.region.localeCompare(b.region)) as timezone}
					{@const region = timezone.region}
					{#if timezone.city}
						{#each timezone.city as city}
							<div
								class="flex flex-row-reverse w-full items-center justify-between py-4 px-4 border border-b-slate-400 last:border-none bg-slate-200 hover:bg-slate-100 transition-all"
							>
								<input
									required
									type="radio"
									id={region + 'ID'}
									value="{region}/{city}"
									bind:group={selectedTimezone}
								/>
								<label for={region + 'ID'}>{region}/{city}</label>
							</div>
						{/each}
					{:else}
						<div
							class="flex flex-row-reverse w-full items-center justify-between py-4 px-4 border border-b-slate-400 last:border-none bg-slate-200 hover:bg-slate-100 transition-all"
						>
							<input type="radio" id={region + 'ID'} value={region} bind:group={selectedTimezone} />
							<label for={region + 'ID'}>{region}</label>
						</div>
					{/if}
				{/each}
			</div>
			<a href="/installation/account" on:click={handleSetTimezone}>Next</a>
		</form>
	{/if}
</section>
