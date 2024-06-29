<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { json } from '../stores.js';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { fly } from 'svelte/transition';

	let timezones = [];
	let showTimezone = false;
	let selectedTimezone = null;
	let selectedCity = null;
	let filteredTimezones = [];
	let showOptions = false;
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

	$: if (selectedTimezone) {
		const parts = selectedTimezone.split('/');
		selectedCity = parts.length > 1 ? parts[1] : null;
		showOptions = false; // Hide options when an option is selected
	}

	onMount(() => {
		getTimezone();
	});

	function toggleOptions() {
		showOptions = !showOptions;
	}
</script>

<Sidebar />
<section class="flex flex-col items-center justify-center h-[85dvh]">
	{#if showTimezone}
		<form class="text-center p-8 rounded-md min-h-[50dvh]">
			<div>
				<h1 class="text-center mb-6 font-bold text-[32px] font-archivo">Select Timezone</h1>
			</div>

			<div class="max-w-md mx-auto relative">
				<h2 class="font-poppin text-left mb-2">Region</h2>
				<div
					class="relative flex items-center mb-1 h-[45px] {showOptions
						? 'rounded-t-lg'
						: 'rounded-lg'} bg-white overflow-hidden border border-greyBorder"
				>
					<input
						type="text"
						placeholder="Select Region"
						class="h-full w-full outline-none text-sm text-gray-700 pr-2 pl-[12px] font-poppin"
						on:focus={toggleOptions}
						bind:value={searchTerm}
					/>
					<div class="inset-y-0 left-0 flex items-center pr-4">
						<svg
							width="14"
							height="9"
							viewBox="0 0 14 9"
							fill="none"
							xmlns="http://www.w3.org/2000/svg"
						>
							<path
								d="M1 1.5L7 7.5L13 1.5"
								stroke="black"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"
							/>
						</svg>
					</div>
				</div>
				{#if showOptions}
					<div
						class="absolute z-10 w-full bg-white border border-greyBorder rounded-b-xl max-h-[30vh] overflow-y-auto"
						in:fly={{ y: -10, duration: 1000 }}
						out:fly={{ y: 10, duration: 300 }}
					>
						{#each filteredTimezones.sort((a, b) => a.region.localeCompare(b.region)) as timezone}
							{@const region = timezone.region}
							{#if timezone.city}
								{#each timezone.city as city}
									<div
										class="flex flex-row-reverse w-full items-center justify-between py-4 px-4 border border-b-grayBorder last:border-none bg-white hover:bg-slate-100 transition-all"
										on:click={() => {
											selectedTimezone = `${region}/${city}`;
											toggleOptions();
										}}
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
									class="flex flex-row-reverse w-full items-center justify-between py-4 px-4 border border-b-slate-400 last:border-none bg-white hover:bg-slate-100 transition-all"
									on:click={() => {
										selectedTimezone = region;
										toggleOptions();
									}}
								>
									<input
										type="radio"
										id={region + 'ID'}
										value={region}
										bind:group={selectedTimezone}
									/>
									<label for={region + 'ID'}>{region}</label>
								</div>
							{/if}
						{/each}
					</div>
				{/if}
			</div>
			<div class="max-w-md mx-auto mb-4">
				<h2 class="font-poppin text-left mb-2 font-medium">Zone</h2>
				<div
					class="relative flex items-center h-[45px] rounded-lg bg-white overflow-hidden border border-greyBorder"
				>
					<input
						type="text"
						placeholder="select zone"
						value={selectedCity}
						class="h-full w-full outline-none text-sm text-gray-700 pr-2 pl-[12px] font-poppin"
					/>
				</div>
			</div>
			<div class="max-w-md mx-auto mt-8">
				<h2 class="font-poppin text-left mb-2 font-medium">Preview</h2>
				<div
					class="relative flex items-center w-[451px] h-[66px] rounded-lg bg-white overflow-hidden border border-greyBorder"
				>
					<svg
						class="ml-[12px]"
						width="22"
						height="22"
						viewBox="0 0 22 22"
						fill="none"
						xmlns="http://www.w3.org/2000/svg"
					>
						<path
							d="M11 5V11L15 13M21 11C21 16.5228 16.5228 21 11 21C5.47715 21 1 16.5228 1 11C1 5.47715 5.47715 1 11 1C16.5228 1 21 5.47715 21 11Z"
							stroke="black"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						/>
					</svg>

					<input
						type="text"
						placeholder="21:26:21"
						class="h-full w-full outline-none text-sm text-gray-700 pr-2 pl-8 font-poppin"
					/>
				</div>
			</div>
			<div class="max-w-md mx-auto fixed bottom-0 mb-12">
				<div class="grid grid-cols-2 gap-[295px]">
					<a
						href="/installation/keyboard"
						class="text-white bg-greyButton hover:bg-gray-500 focus:ring-4 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 h-[44px] w-[76px]"
						>Back</a
					>
					<a
						href="/installation/locale"
						on:click={handleSetTimezone}
						class="text-white bg-greyButton hover:bg-gray-500 focus:ring-4 focus:ring-gray-900 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 focus:outline-none h-[44px] w-[76px]"
						>Next</a
					>
				</div>
			</div>
		</form>
	{/if}
</section>
