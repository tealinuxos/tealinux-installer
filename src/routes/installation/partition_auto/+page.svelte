<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { fly } from 'svelte/transition';
	import { getBlueprint } from '../global.js';

	let timezones = [];
	let showTimezone = false;
	let selectedTimezone = null;
	let selectedCity = null;
	let filteredTimezones = [];
	let formatedTimezones = [];
	let showOptions = false;
	let searchTerm = '';
	let isDisabled = true;
	let currentDisk = '';

	let diskLists = [];

	const get_disks_data = async () => {
		invoke('get_disk_lists_key_val').then((response) => {
			diskLists = JSON.parse(response);
		});
	};

	const handleSetPartitionAuto = async () => {
		console.log('invoking autogen_partition_select_disk' + currentDisk);
		await invoke('autogen_partition_select_disk', { blkname: currentDisk });
	};

	const setStateSelected = async (str) => {
		isDisabled = false;
		currentDisk = str;

		console.log(`debug: selected ${str}`);
	};

	onMount(() => {
		get_disks_data();
	});

	$: console.log(showOptions);
	$: console.log(diskLists);
</script>

<Sidebar />
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
		<form class="text-center p-8 rounded-md min-h-[50dvh]">
			<div>
				<h1 class="text-center mb-6 font-bold text-[32px] font-archivo">Select Disk</h1>
			</div>
			<p>
				Let's decide installer to choose best settings for your partition, first choose the disk
			</p>

			<div class="bg-white shadow-md rounded-lg p-6 mb-4">
				<div class="flex flex-col space-y-4">
					{#each diskLists as diskList}
						<label class="radio-card">
							<input
								type="radio"
								class="peer sr-only"
								on:click={() => {
									setStateSelected(diskList['blkname']);
								}}
							/>
							<div
								class="group flex items-center rounded-lg border border-gray-200 bg-white p-4 shadow-sm hover:border-blue-500 peer-checked:border-blue-500 peer-checked:ring-2 peer-checked:ring-blue-500"
							>
								<div>
									<h5 class="text-lg font-medium text-gray-900">
										<b>
											{diskList['blkname']} ({diskList['blksize']})
										</b>
									</h5>
								</div>
								<svg
									class="ml-auto h-6 w-6 text-gray-300 group-hover:text-blue-500 peer-checked:text-blue-500"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
									xmlns="http://www.w3.org/2000/svg"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M5 13l4 4L19 7"
									></path>
								</svg>
							</div>
						</label>
					{/each}
				</div>
			</div>
			<div class="max-w-md mx-auto fixed bottom-0 mb-12">
				<div class="grid grid-cols-2 gap-[295px]">
					<a
						href="/installation/partition"
						class="text-black bg-greenTealinux focus:ring-4 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 h-[44px] w-[76px]"
						>Back</a
					>

					{#if isDisabled == true}
						<a
							href="#x"
							class="text-white bg-greenTealinux focus:ring-4 font-medium
                    rounded-lg text-sm px-5 py-2.5 me-2 mb-2 brightness-75 pointer-events-none"
							>Next</a
						>
					{:else}
						<a
							href="/installation/summary"
							on:click={handleSetPartitionAuto}
							class="disabled:opacity-50 disabled:pointer-events-none disabled:cursor-not-allowed text-black disabled bg-greenTealinux focus:ring-4 focus:ring-gray-900 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 focus:outline-hidden"
							>Next</a
						>
					{/if}
				</div>
			</div>
		</form>
	</section>
</div>
