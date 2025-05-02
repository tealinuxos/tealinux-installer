<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { fly } from 'svelte/transition';
	import { getBlueprint } from '../global.js';

	let timezones = [];
	let showTimezone = $state(false);
	let selectedTimezone = $state(null);
	let selectedCity = $state(null);
	let filteredTimezones = $state([]);
	let formatedTimezones = [];
	let showOptions = $state(false);
	let searchTerm = $state('');

	const getTimezone = async () => {
		invoke('get_timezone_json').then((response) => {
			timezones = JSON.parse(response);
			filteredTimezones = setFilteredTimezones(timezones);
			formatedTimezones = filteredTimezones;
			console.log(timezones);
			showTimezone = true;
		});
	};

	//set filtered timezones to region/city from invoke
	const setFilteredTimezones = (timezones) => {
		let timezoneOptions = [];

		timezones.forEach((timezone) => {
			const region = timezone.region;
			if (timezone.city) {
				timezone.city.forEach((city) => {
					timezoneOptions.push(`${region}/${city}`);
				});
			} else {
				timezoneOptions.push(region);
			}
		});

		console.log(timezoneOptions);
		return timezoneOptions;
	};

	const handleSetTimezone = async () => {
		let region = selectedTimezone.split('/')[0];
		let city = selectedTimezone.split('/')[1] || null;

		await invoke('blueprint_set_timezone', { region: region, city: city });
	};

	function filterOptions() {
		const term = searchTerm.toLowerCase();
		filteredTimezones = formatedTimezones.filter((timezone) =>
			timezone.toLowerCase().includes(term)
		);
	}

	const selectTimezone = (value) => {
		selectedTimezone = value;
		searchTerm = value.split('/')[0];
	};

	run(() => {
		searchTerm, filterOptions();
	});

	run(() => {
		if (selectedTimezone) {
			const parts = selectedTimezone.split('/');
			selectedCity = parts.length > 1 ? parts[1] : null;
			showOptions = false; // Hide options when an option is selected
		}
	});

	onMount(() => {
		getTimezone();
		getBlueprint().then((blueprint) => {
			if (blueprint.timezone !== null) {
				selectedTimezone = blueprint.timezone.region + '/' + blueprint.timezone.city;
				searchTerm = blueprint.timezone.region + '/' + blueprint.timezone.city;
			}
		});
	});

	function openOptions() {
		showOptions = true;
	}
	function closeOptions(value) {
		searchTerm = value;
		showOptions = false;
	}

	let date = new Date();
	let datePreview = $state('');
	let timePreview = $state('Select Timezone');

	const handlePreview = () => {
		if (selectedTimezone !== null) {
			let timeFormat = new Intl.DateTimeFormat([], {
				timeZone: selectedTimezone,
				hour: 'numeric',
				minute: 'numeric',
				second: 'numeric',
				hour12: false
			});
			let dateFormat = new Intl.DateTimeFormat([], {
				timeZone: selectedTimezone,
				year: 'numeric',
				month: 'long',
				day: 'numeric'
			});
			datePreview = dateFormat.format(date);
			timePreview = timeFormat.format(date);
		}
	};

	run(() => {
		selectedTimezone, handlePreview();
	});
	run(() => {
		console.log(showOptions);
	});
</script>

<div class="relative w-full">
	<header class="flex items-center justify-center w-full gap-[10px] mt-[40px]">
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-grayTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-grayTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-grayTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-grayTealinux rounded-full"></div>
	</header>
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
							: 'rounded-lg'} overflow-hidden bg-grayTealinux border-2 border-black"
					>
						<input
							type="text"
							placeholder="Select Region"
							class="h-full w-full outline-hidden text-sm text-gray-700 bg-grayTealinux pr-2 pl-[12px] font-poppin"
							onfocus={openOptions}
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
							{#each filteredTimezones as timezone}
								<button
									class="flex flex-row-reverse w-full items-center justify-between py-4 px-4 border border-b-grayBorder last:border-none bg-white hover:bg-slate-100 transition-all"
									for="timezone-{timezone}"
									onclick={() => console.log('lll')}
								>
									<input
										required
										type="radio"
										name="timezone"
										id="timezone-{timezone}"
										value={timezone}
										bind:group={selectedTimezone}
										class="w-5 h-5"
									/>
									<p>{timezone}</p>
								</button>
							{/each}
						</div>
					{/if}
				</div>
				<div class="max-w-md mx-auto mb-4">
					<h2 class="font-poppin text-left mb-2 font-medium">City</h2>
					<div
						class="relative flex items-center h-[45px] rounded-lg overflow-hidden bg-grayTealinux border-2 border-black"
					>
						<input
							type="text"
							placeholder="Select city"
							disabled
							value={selectedCity}
							class="h-full w-full outline-hidden text-sm text-gray-700 pr-2 pl-[12px] font-poppin bg-grayTealinux"
						/>
					</div>
				</div>
				<!-- TIME PREVIEW -->
				<div class="max-w-md mx-auto mt-8">
					<h2 class="font-poppin text-left mb-2 font-medium">Preview</h2>
					<div
						class="relative flex items-center w-[451px] h-[66px] rounded-lg overflow-hidden bg-grayTealinux border-2 border-black"
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

						<span class="ml-4">{timePreview}</span>
						<span class="absolute right-0 mr-4">{datePreview}</span>
					</div>
				</div>
				<div class="max-w-md mx-auto fixed bottom-0 mb-12">
					<div class="grid grid-cols-2 gap-[295px]">
						<a
							href="/installation/keyboard"
							class="text-white bg-greenTealinux focus:ring-4 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 h-[44px] w-[76px]"
							>Back</a
						>
						<a
							href="/installation/locale"
							onclick={handleSetTimezone}
							class="text-white bg-greenTealinux {selectedTimezone
								? ''
								: ' brightness-75 pointer-events-none'}  focus:ring-4 focus:ring-gray-900 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 focus:outline-hidden"
							>Next</a
						>
					</div>
				</div>
			</form>
		{/if}
	</section>
</div>
