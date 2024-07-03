<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { getRead } from '../global.js';
	import { randomColor } from 'randomcolor';
	import SideBar from '$lib/components/Sidebar.svelte';
	import prettyBytes from 'pretty-bytes';

	let selectedDisk = 0;
	let partitionDetail = [];

	const getStorageJSON = async () => {
		let json = await getRead();

		return json.disk;
	};

	const getFilesystemJSON = async () => {
		let filesystems = await invoke('get_filesystem_json');

		return JSON.parse(filesystems);
	};

	const handlePartitionDetail = async (disks, selectedDisk) => {
		partitionDetail = [];

		for (let i of disks[selectedDisk].partitions.keys()) {
			let partitionPath = disks[selectedDisk].partitions[i].partitionPath;
			let startSector = disks[selectedDisk].partitions[i].start.slice(0, -1);
			let endSector = disks[selectedDisk].partitions[i].end.slice(0, -1);
			let size = disks[selectedDisk].partitions[i].size.slice(0, -1);

			partitionDetail.push({
				path: partitionPath,
				format: null,
				mountpoint: null,
				start: parseInt(startSector),
				end: parseInt(endSector),
				size: parseInt(size)
			});
		}
	};

	const handleSetPartition = async () => {
		let partition = JSON.stringify(partitionDetail);

		await invoke('blueprint_set_partition', { partition });
	};

	let partitionColors = [];

	const getColors = (disks) => {
		let length = disks[selectedDisk].partitions.length;

		let colors = [];

		for (let i = 0; i < length; i++) {
			colors.push(
				randomColor({
					luminosity: 'bright',
					hue: 'random'
				})
			);
		}

		partitionColors = colors;
	};

	let activeTab = 'Efi';
	let bootLoaderLocation = ['Efi of ATA VBOX HARDISK (/dev/sdb1)'];

	const setTab = (tab) => {
		activeTab = tab;
		// Update bootLoaderLocation based on the active tab
		if (tab === 'Efi') {
			bootLoaderLocation = ['Efi of ATA VBOX HARDISK (/dev/sdb1)'];
		} else {
			bootLoaderLocation = ['Mbr of ATA VBOX HARDISK (/dev/sdb2)'];
		}
	};

	onMount(() => {
		getStorageJSON().then((disks) => {
			getColors(disks);
			console.log(disks);
		});
	});
</script>

<SideBar />
<section class=" items-center justify-center w-[80dvw] mx-auto">
	<div class="flex flex-col gap-y-10 min-h-[85dvh] mb-[15dvh]">
		<h1 class="font-poppinsemibold font-bold text-4xl text-center mt-16">
			Set Installation Partition
		</h1>

		{#await getStorageJSON()}
			Loading...
		{:then disks}
			<!-- option -->
			<div
				class="relative flex items-center w-full g-whiteTealinux text-blue-black border-2 border-greyBorder rounded-lg shadow-md shadow-black/50"
			>
				<select
					class="w-full b appearance-none p-4 py-2"
					id="diskSelect"
					bind:value={selectedDisk}
					on:change={() => getColors(disks)}
				>
					{#each disks as disk, i}
						{@const model = disk.model}
						{@const path = disk.diskPath}
						<option value={i}>{model + ' (' + path + ')'}</option>
					{/each}
				</select>
				<img src="/dropDownMain.svg" alt="arr" class="absolute right-4" />
			</div>
			<div class="flex gap-x-4">
				<!-- Partition Bar -->
				<div class="flex-[1] self-start">
					<div class="flex mb-4 h-8 w-full overflow-hidden rounded-full">
						<div class="h-full flex rounded-full overflow-hidden w-full">
							{#each disks[selectedDisk].partitions as partition, i}
								{@const diskSize = disks[selectedDisk].size.slice(0, -1)}
								{@const partitionSize = partition.size.slice(0, -1)}
								{@const percentage = (partitionSize / diskSize) * 100}
								{@const color = partitionColors[i]}

								<div style="width: {percentage}%; background-color: {color}" class="h-full"></div>
							{/each}
						</div>
					</div>

					<div class="flex gap-y-4 flex-wrap mb-4">
						{#each disks[selectedDisk].partitions as partition, i}
							{@const color = partitionColors[i]}
							{@const size = partition.size.slice(0, -1) * 512}
							{@const path =
								partition.partitionPath == null ? 'Unallocated' : partition.partitionPath.slice(5)}
							{@const filesystem =
								partition.filesystem == null
									? path == 'Unallocated'
										? 'Unallocated'
										: 'Unknown'
									: partition.filesystem}
							{@const prettySize = prettyBytes(size)}
							<div class="flex pr-2 gap-x-2">
								<div style="background-color: {color}" class="w-4 h-4 rounded-sm"></div>
								<div class="flex flex-col text-sm font-poppinmedium font-medium">
									<span class="pl-1">{path}</span>
									<span class="pl-1">{prettySize} {filesystem}</span>
								</div>
							</div>
						{/each}
					</div>

					<div class="flex mb-4 h-8 w-full overflow-hidden rounded-full">
						<div class="h-full flex rounded-full overflow-hidden w-full">
							{#each partitionDetail as partition, i}
								{@const diskSize = disks[selectedDisk].size.slice(0, -1)}
								{@const partitionSize = partition.size}
								{@const percentage = (partitionSize / diskSize) * 100}
								{@const color = partitionColors[i]}

								<div style="width: {percentage}%; background-color: {color}" class="h-full"></div>
							{/each}
						</div>
					</div>

					<div class="flex flex-wrap gap-y-4 gap-x-1 mb-4">
						{#each partitionDetail as partition, i}
							{@const color = partitionColors[i]}
							{@const size = partition.size * 512}
							{@const path = partition.path == null ? 'Unallocated' : partition.path.slice(5)}
							{@const mountpoint =
								partition.mountpoint == null ? 'Not assigned' : partition.mountpoint}
							{@const format = partition.format == null ? "Don't format" : partition.format}
							{@const prettySize = prettyBytes(size)}

							<div class="flex pr-2 gap-x-2">
								<div style="background-color: {color}" class="w-4 h-4 rounded-sm"></div>
								<div class="flex flex-col text-sm font-poppinmedium font-medium">
									<span>{path}</span>
									<span>{prettySize} {format}</span>
								</div>
							</div>
						{/each}
					</div>
				</div>
				{#await handlePartitionDetail(disks, selectedDisk)}
					Loading...
				{:then}
					<div
						class="flex flex-col flex-[1] bg-white-tealinux border-2 h-fit rounded-lg border-greyBorder"
					>
						{#each partitionDetail as partition, i}
							{@const path = partition.path}

							{#if path == null}
								<div class="flex gap-x-4 items-center border border-b-greyBorder p-2">
									<span>Unallocated</span>
									<select bind:value={partitionDetail[i].format} on:change={handleSetPartition}>
										<option value={null}>Do not format</option>
										{#await getFilesystemJSON()}
											<option disabled={true}>Loading...</option>
										{:then filesystems}
											{#each filesystems as filesystem}
												<option value={filesystem}>{filesystem}</option>
											{/each}
										{/await}
									</select>
									<select bind:value={partitionDetail[i].mountpoint} on:change={handleSetPartition}>
										<option value={null}>No Mountpoint</option>
										<option value="/">/</option>
										<option value="/boot/efi">/boot/efi</option>
										<option value="/home">/home</option>
									</select>
								</div>
							{:else}
								<div
									class="flex justify-between items-center border border-b-greyBorder p-2 py-3 font-poppinmedium font-medium text-sm"
								>
									<span>{path}</span>
									<div class="relative flex items-center">
										<select
											bind:value={partitionDetail[i].format}
											on:change={handleSetPartition}
											class=" appearance-none flex gap-x-6 items-center bg-greenTealinux text-white py-2 px-4 pr-8 rounded-lg"
										>
											<option value={null}>Do not format</option>
											{#await getFilesystemJSON()}
												<option disabled={true}>Loading...</option>
											{:then filesystems}
												{#each filesystems as filesystem}
													<option value={filesystem}>{filesystem}</option>
												{/each}
											{/await}
										</select>
										<svg
											width="14"
											height="9"
											viewBox="0 0 14 9"
											class=" absolute right-2"
											fill="none"
											xmlns="http://www.w3.org/2000/svg"
										>
											<path
												d="M1 1.5L7 7.5L13 1.5"
												stroke="white"
												stroke-width="2"
												stroke-linecap="round"
												stroke-linejoin="round"
											/>
										</svg>
									</div>
									<div class="relative flex items-center">
										<select
											bind:value={partitionDetail[i].mountpoint}
											on:change={handleSetPartition}
											class=" appearance-none flex gap-x-6 items-center bg-greenTealinux text-white py-2 px-4 pr-8 rounded-lg"
										>
											<option value={null}>No Mountpoint</option>
											<option value="/">/</option>
											<option value="/boot/efi">/boot/efi</option>
											<option value="/home">/home</option>
										</select>
										<svg
											width="14"
											height="9"
											viewBox="0 0 14 9"
											class=" absolute right-2"
											fill="none"
											xmlns="http://www.w3.org/2000/svg"
										>
											<path
												d="M1 1.5L7 7.5L13 1.5"
												stroke="white"
												stroke-width="2"
												stroke-linecap="round"
												stroke-linejoin="round"
											/>
										</svg>
									</div>
									<div
										class="flex gap-x-6 items-center bg-greenTealinux text-white py-2 px-4 rounded-lg"
									>
										<label for="format-{i}">Format</label>
										<input
											type="checkbox"
											name="format-{i}"
											id="format-{i}"
											class="rounded-checkbox"
										/>
									</div>
								</div>
							{/if}
						{/each}
					</div>
				{/await}
			</div>
		{/await}
		<div class="flex flex-col items-center justify-center p-4 font-poppinmedium">
			<div class="flex items-center justify-center space-x-2 bg-greenTealinux rounded-t-md">
				<button
					class={`py-2 px-4 rounded-t-md ${
						activeTab === 'Efi' ? 'bg-greenTealinux text-white' : 'bg-gray-200 text-black'
					}`}
					on:click={() => setTab('Efi')}
				>
					Efi
				</button>
				<button
					class={`py-2 px-4 rounded-t-md ${
						activeTab === 'Mbr' ? 'bg-greenTealinux text-white' : 'bg-gray-200 text-black'
					}`}
					on:click={() => setTab('Mbr')}
				>
					Mbr
				</button>
			</div>
			<div
				class="bg-greenTealinux text-black flex items-center justify-between gap-x-4 px-8 py-4 rounded-full w-3/5"
			>
				<p class="text-white font-bold">Boot loader location</p>
				<div class="relative flex items-center justify-between w-2/3 bg-gray-200 rounded-full">
					<select class="w-full b appearance-none p-4 py-2 rounded-full" id="diskSelect">
						{#each bootLoaderLocation as location, i}
							<option value={i}>{location}</option>
						{/each}
					</select>
					<img src="/dropDownMain.svg" alt="arr" class="absolute right-4" />
				</div>
			</div>
		</div>
	</div>
	<div class="flex justify-between items-center h-[15dvh] w-[80dvw] fixed bottom-0 bg-white">
		<a
			href="/installation/account"
			class="text-white bg-greenTealinux focus:ring-4 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2"
			>back</a
		>
		<a
			href="/installation/summary"
			on:click={handleSetPartition}
			class="text-white bg-greenTealinux focus:ring-4 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2"
			>Next</a
		>
	</div>
</section>

<style>
	/* Apply the same styles to option elements for better compatibility */
	select option {
		background-color: black !important;
		color: white !important;
	}
	.rounded-checkbox {
		@apply appearance-none h-5 w-5 bg-white rounded-full cursor-pointer;
	}
	.rounded-checkbox:checked {
		@apply bg-blue-500;
	}
	.rounded-checkbox:checked::before {
		content: '';
		@apply block h-5 w-5 bg-blue-500 border border-greyBorder rounded-full m-auto;
	}
</style>
