<script>
	import { run } from 'svelte/legacy';

	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { getRead } from '../global.js';
	import { randomColor } from 'randomcolor';
	import SideBar from '$lib/components/Sidebar.svelte';
	import prettyBytes from 'pretty-bytes';

	let show = $state(true);
	let selectedDisk = $state(0);
	let partitionDetail = $state([]);
	let bootloader = $state({
		firmwareType: null,
		path: null
	});
	let bootloaderPartitionIndex = $state(null);

	const getStorageJSON = async () => {
		let json = await getRead();
		json = json.disk.filter((disk) => disk.partitions !== null);

		return json;
	};

	const getFirmwareType = async () => {
		let read = await getRead();

		return read.firmware;
	};

	const getFilesystemJSON = async () => {
		let filesystems = await invoke('get_filesystem_json');

		return JSON.parse(filesystems);
	};

	const filesystems = ['btrfs', 'ext4', 'fat32', 'fat16', 'exfat', 'linux-swap(v1)'];

	const handlePartitionDetail = async (disks, selectedDisk) => {
		partitionDetail = [];
		bootloaderPartitionIndex = null;

		for (let i of disks[selectedDisk].partitions.keys()) {
			let partitionPath = disks[selectedDisk].partitions[i].partitionPath;
			let filesystemType = disks[selectedDisk].partitions[i].filesystem;
			let startSector = disks[selectedDisk].partitions[i].start.slice(0, -1);
			let endSector = disks[selectedDisk].partitions[i].end.slice(0, -1);
			let size = disks[selectedDisk].partitions[i].size.slice(0, -1);
			let partitionNumber = disks[selectedDisk].partitions[i].number;
			let partitionDiskPath = disks[selectedDisk].diskPath;
			let mountpoint =
				partitionPath === bootloader.path
					? bootloader.firmwareType === 'UEFI'
						? '/boot/efi'
						: '/boot'
					: null;

			if (mountpoint) {
				bootloaderPartitionIndex = i;
			}

			partitionDetail.push({
				number: parseInt(partitionNumber),
				diskPath: partitionDiskPath,
				path: partitionPath,
				mountpoint,
				filesystem: filesystemType,
				format: false,
				start: parseInt(startSector),
				end: parseInt(endSector),
				size: parseInt(size)
			});
		}
	};

	const handleSetStorage = async () => {
		let disks = await getRead();
		disks = disks.disk;

		let storage = {
			diskPath: disks[selectedDisk].diskPath,
			partitionTable: disks[selectedDisk].label,
			newPartitionTable: false,
			layoutChanged: false,
			partitions: null
		};

		await invoke('blueprint_set_storage', {
			storage: JSON.stringify(storage),
			partition: JSON.stringify(partitionDetail)
		});
	};

	const handleSetBootloader = async () => {
		let bootloaderJSON = JSON.stringify(bootloader);

		await invoke('blueprint_set_bootloader', { bootloader: bootloaderJSON });
	};

	let partitionColors = $state([]);

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

	let activeTab = $state('Efi');
	let bootLoaderLocation = $state([]);

	const setTab = async (tab) => {
		activeTab = tab;
		// Update bootLoaderLocation based on the active tab

		let bootloaderPartition = await getBootloaderPartition(activeTab);

		if (tab === 'Efi') {
			let bootloader = [];

			for (let i of bootloaderPartition.keys()) {
				let diskModel = bootloaderPartition[i].diskModel;
				let path = bootloaderPartition[i].partitionPath;

				bootloader.push('EFI of ' + diskModel + ' (' + path + ')');
			}

			bootLoaderLocation = bootloader;
		} else {
			let bootloader = [];

			for (let i of bootloaderPartition.keys()) {
				let model = bootloaderPartition[i].model;
				let path = bootloaderPartition[i].diskPath;

				bootloader.push('MBR of ' + model + ' (' + path + ')');
			}

			bootLoaderLocation = bootloader;
		}
	};

	const refreshEfi = () => {
		getBootloaderPartition(activeTab).then((efi) => {
			bootloader.firmwareType = 'UEFI';
			efi === null ? (bootloader.path = null) : (bootloader.path = efi[0].partitionPath);
			handleSetBootloader();
		});
	};

	const refreshMbr = () => {
		getBootloaderPartition(activeTab).then((disk) => {
			bootloader.firmwareType = 'BIOS';
			disk === null ? (bootloader.path = null) : (bootloader.path = disk[0].diskPath);
			handleSetBootloader();
		});
	};

	const getBootloaderPartition = async (activeTab) => {
		let disk = await getStorageJSON();

		let partition = [];

		if (activeTab === 'Efi') {
			for (let i = 0; i < disk.length; i++) {
				let p = disk[i].partitions.filter(
					(partition) => partition.flags !== null && partition.flags.includes('esp')
				);
				p = p.map((partition) => Object.assign(partition, { diskModel: disk[i].model }));

				if (p.length) {
					partition.push(p[0]);
				}
			}
		} else {
			partition = disk;
		}

		return partition;
	};

	const spawnGparted = async () => {
		await invoke('spawn_gparted');
	};

	const spawnTerminal = async () => {
		await invoke('spawn_terminal');
	};

	const setReadJSON = async () => {
		await invoke('set_read_json');
	};

	const selectAutoInstall = async () => {
		window.location.href = '/installation/partition_auto';
	};

	const setup = () => {
		getFirmwareType().then((firmware) => {
			if (firmware === 'UEFI') {
				setTab('Efi');
				refreshEfi();
			} else {
				setTab('Mbr');
				refreshMbr();
			}
			getStorageJSON().then((disks) => {
				getColors(disks);
				handlePartitionDetail(disks, selectedDisk);
				handleSetStorage();
			});
		});
	};

	const refresh = () => {
		show = false;
		selectedDisk = 0;
		setReadJSON().then(() => (show = true));
		setup();
	};

	let partitionChecked = $state(false);

	const checkPartition = () => {
		let root = partitionDetail.filter((obj) => obj.mountpoint === '/');
		partitionChecked =
			root.length > 0 ? (bootloader.firmwareType && bootloader.path ? true : false) : false;
	};

	onMount(() => {
		setup();
	});
	run(() => {
		partitionDetail, checkPartition();
	});
</script>

<SideBar />
<div class="relative w-full">
	<header
		class="flex items-center justify-center w-full gap-[10px] py-10 fixed top-0 bg-whiteTealinux z-30"
	>
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-grayTealinux rounded-full"></div>
	</header>
	<section class=" items-center justify-center w-[80dvw] mx-auto mt-28">
		<div class="flex flex-col gap-y-10 min-h-[85dvh] mb-[15dvh]">
			<h1 class="font-poppinsemibold font-bold text-4xl text-center mt-8">
				Set Installation Partition
			</h1>
			<div class=" font-poppinmedium font-medium flex gap-x-8 justify-center">
				<button class=" bg-grayTealinux text-black py-2 px-4 rounded-lg" onclick={spawnGparted}>
					Gparted
				</button>
				<button class=" bg-grayTealinux text-black py-2 px-4 rounded-lg" onclick={spawnTerminal}>
					Terminal
				</button>
				<button class=" bg-grayTealinux text-black py-2 px-4 rounded-lg" onclick={refresh}>
					Refresh
				</button>
				<button
					class=" bg-grayTealinux text-black py-2 px-4 rounded-lg"
					onclick={selectAutoInstall}
				>
					Auto partition
				</button>
			</div>

			{#if !show}
				<div class="flex flex-col items-center justify-center p-4 font-poppinmedium">
					Loading partition...
				</div>
			{/if}
			{#if show}
				{#await getStorageJSON() then disks}
					<!-- option -->
					<div
						class="relative flex items-center w-full g-whiteTealinux text-blue-black border-2 border-greyBorder rounded-lg shadow-md shadow-black/50"
					>
						<select
							class="w-full b appearance-none p-4 py-2"
							id="diskSelect"
							bind:value={selectedDisk}
							onchange={() => getColors(disks)}
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
							<h1 class="p-4 text-[18px] font-bold">Current</h1>
							<div class="flex mb-4 h-8 w-full overflow-hidden rounded-full">
								<div class="h-full flex rounded-full overflow-hidden w-full">
									{#each disks[selectedDisk].partitions as partition, i}
										{@const diskSize = disks[selectedDisk].size.slice(0, -1)}
										{@const partitionSize = partition.size.slice(0, -1)}
										{@const percentage = (partitionSize / diskSize) * 100}
										{@const color = partitionColors[i]}

										<div
											style="width: {percentage}%; background-color: {color}"
											class="h-full"
										></div>
									{/each}
								</div>
							</div>

							<div class="flex gap-y-4 flex-wrap mb-4">
								{#each disks[selectedDisk].partitions as partition, i}
									{@const color = partitionColors[i]}
									{@const size = partition.size.slice(0, -1) * 512}
									{@const path =
										partition.partitionPath == null
											? 'Unallocated'
											: partition.partitionPath.slice(5)}
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

							<h1 class="p-4 text-[18px] font-bold">After</h1>

							<div class="flex mb-4 h-8 w-full overflow-hidden rounded-full">
								<div class="h-full flex rounded-full overflow-hidden w-full">
									{#each partitionDetail as partition, i}
										{@const diskSize = disks[selectedDisk].size.slice(0, -1)}
										{@const partitionSize = partition.size}
										{@const percentage = (partitionSize / diskSize) * 100}
										{@const color = partitionColors[i]}

										<div
											style="width: {percentage}%; background-color: {color}"
											class="h-full"
										></div>
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
									{@const filesystem =
										partition.filesystem == null ? 'Unknown' : partition.filesystem}
									{@const prettySize = prettyBytes(size)}

									<div class="flex pr-2 gap-x-2">
										<div style="background-color: {color}" class="w-4 h-4 rounded-sm"></div>
										<div class="flex flex-col text-sm font-poppinmedium font-medium">
											<span>{path}</span>
											<span>{prettySize} {filesystem}</span>
										</div>
									</div>
								{/each}
							</div>
						</div>
						{#await handlePartitionDetail(disks, selectedDisk) then}
							<div
								class="flex flex-col flex-[1] bg-white-tealinux border-2 h-fit rounded-lg border-greyBorder"
							>
								{#each partitionDetail as partition, i}
									{@const path = partition.path}

									{#if path == null}
										<div
											class="flex justify-between items-center border border-b-greyBorder p-2 py-3 font-poppinmedium font-medium text-sm"
										>
											<span>Unallocated</span>
											<div class="relative flex items-center">
												<select
													bind:value={partitionDetail[i].filesystem}
													onchange={handleSetStorage}
													class=" appearance-none flex gap-x-6 items-center bg-grayTealinux text-black py-2 px-4 pr-8 rounded-lg"
												>
													<option disabled={true} value={null}>Unallocated</option>

													{#each filesystems as filesystem}
														<option value={filesystem}>{filesystem}</option>
													{/each}
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
														stroke="black"
														stroke-width="2"
														stroke-linecap="round"
														stroke-linejoin="round"
													/>
												</svg>
											</div>
											<div class="relative flex items-center">
												<select
													bind:value={partitionDetail[i].mountpoint}
													onchange={handleSetStorage}
													class=" appearance-none flex gap-x-6 items-center bg-grayTealinux text-black py-2 px-4 pr-8 rounded-lg"
												>
													<option value={null}>No Mountpoint</option>
													<option value="/">/</option>
													<option value="/home">/home</option>
													<option value="swap">swap</option>
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
														stroke="black"
														stroke-width="2"
														stroke-linecap="round"
														stroke-linejoin="round"
													/>
												</svg>
											</div>
											<div
												class="flex gap-x-6 items-center bg-grayTealinux text-black py-2 px-4 rounded-lg"
											>
												<label for="format-{i}">Format</label>
												<input
													type="checkbox"
													name="format-{i}"
													id="format-{i}"
													class="rounded-checkbox"
													bind:checked={partitionDetail[i].format}
													onchange={handleSetStorage}
												/>
											</div>
										</div>
									{:else}
										<div
											class="flex justify-between items-center border border-b-greyBorder p-2 py-3 font-poppinmedium font-medium text-sm"
										>
											<span>{path}</span>
											<div class="relative flex items-center">
												<select
													bind:value={partitionDetail[i].filesystem}
													onchange={handleSetStorage}
													class=" appearance-none flex gap-x-6 items-center bg-grayTealinux text-black py-2 px-4 pr-8 rounded-lg"
												>
													{#if partitionDetail[i].filesystem === 'ntfs'}
														<option disabled={true} value="ntfs">ntfs</option>
													{:else if partitionDetail[i].filesystem === null}
														<option disabled={true} value={null}>Unknown</option>
													{/if}

													{#each filesystems as filesystem}
														<option value={filesystem}>{filesystem}</option>
													{/each}
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
														stroke="black"
														stroke-width="2"
														stroke-linecap="round"
														stroke-linejoin="round"
													/>
												</svg>
											</div>
											<div class="relative flex items-center">
												<select
													bind:value={partitionDetail[i].mountpoint}
													onchange={handleSetStorage}
													class=" appearance-none flex gap-x-6 items-center bg-grayTealinux text-black py-2 px-4 pr-8 rounded-lg"
												>
													{#if partitionDetail[i].mountpoint === '/boot/efi'}
														<option disabled={true} value="/boot/efi"> /boot/efi </option>
													{:else}
														<option value={null}>No Mountpoint</option>
														<option value="/">/</option>
														<option value="/home">/home</option>
														<option value="swap">swap</option>
													{/if}
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
														stroke="black"
														stroke-width="2"
														stroke-linecap="round"
														stroke-linejoin="round"
													/>
												</svg>
											</div>
											<div
												class="flex gap-x-6 items-center bg-grayTealinux text-black py-2 px-4 rounded-lg"
											>
												<label for="format-{i}">Format</label>
												<input
													type="checkbox"
													name="format-{i}"
													id="format-{i}"
													class="rounded-checkbox"
													bind:checked={partitionDetail[i].format}
													onchange={handleSetStorage}
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
					<div class="flex items-center justify-center space-x-2 bg-black/25 rounded-t-md">
						{#await getFirmwareType() then firmwareType}
							{#if firmwareType === 'UEFI'}
								<button
									class={`py-2 px-4 rounded-t-md ${
										activeTab === 'Efi' ? 'bg-transparent' : 'bg-gray-200 text-black'
									}`}
									onclick={async () => {
										await setTab('Efi');
										refreshEfi();
										partitionDetail[bootloaderPartitionIndex].mountpoint = '/boot/efi';
										handleSetStorage();
									}}
								>
									Efi
								</button>
							{/if}
						{/await}
						<button
							class={`py-2 px-4 rounded-t-md ${
								activeTab === 'Mbr' ? 'bg-transparent' : 'bg-gray-200 text-black'
							}`}
							onclick={async () => {
								await setTab('Mbr');
								refreshMbr();
								partitionDetail[bootloaderPartitionIndex].mountpoint = null;
								handleSetStorage();
							}}
						>
							Mbr
						</button>
					</div>
					<div
						class="bg-black/25 text-black flex items-center justify-between gap-x-4 px-8 py-4 rounded-full w-4/5"
					>
						<p class="font-bold w-1/4">Boot loader location</p>
						<div class="relative flex items-center justify-between w-full bg-gray-200 rounded-full">
							<select
								class="w-full b appearance-none p-4 py-2 rounded-full"
								id="diskSelect"
								bind:value={bootloader.path}
								onchange={handleSetBootloader}
							>
								{#each bootLoaderLocation as location, i}
									{#if activeTab === 'Efi'}
										{#await getBootloaderPartition(activeTab) then partition}
											<option value={partition[i].partitionPath}>{location}</option>
										{/await}
									{:else}
										{#await getBootloaderPartition(activeTab) then partition}
											<option value={partition[i].diskPath}>{location}</option>
										{/await}
									{/if}
								{/each}
							</select>
							<img src="/dropDownMain.svg" alt="arr" class="absolute right-4" />
						</div>
					</div>
				</div>
			{/if}
		</div>
		<div class="flex justify-between items-center h-[15dvh] w-[80dvw] fixed bottom-0 bg-white">
			<a
				href="/installation/account"
				class="text-white bg-greenTealinux focus:ring-4 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2"
				>back</a
			>
			<a
				href="/installation/summary"
				onclick={handleSetStorage}
				class="text-white bg-greenTealinux focus:ring-4 font-medium
                    rounded-lg text-sm px-5 py-2.5 me-2 mb-2
                    {partitionChecked ? '' : 'brightness-75 pointer-events-none'}
                ">Next</a
			>
		</div>
	</section>
</div>

<style>
	/* Apply the same styles to option elements for better compatibility */
	select option {
		background-color: black !important;
		color: white !important;
	}
	.rounded-checkbox {
		@apply appearance-none h-5 w-5 bg-white rounded-full border-2 border-black cursor-pointer;
	}
	.rounded-checkbox:checked {
		@apply bg-black;
	}
	.rounded-checkbox:checked::before {
		content: '';
		@apply block h-5 w-5 bg-black border border-black/25 rounded-full m-auto;
	}
</style>
