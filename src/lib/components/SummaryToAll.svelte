<script>
	import { summaryActive } from './store';
	import { onDestroy } from 'svelte';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { getRead, getBlueprint } from '../../routes/installation/global';
	import prettyBytes from 'pretty-bytes';
	import { randomColor } from 'randomcolor';

	let itActive = false;
	let timezone;
	let mainLocale;
	let locales;
	let formattedPartitions;
	let assignedPartitions;

	const unsubscribe = summaryActive.subscribe((isValue) => {
		itActive = isValue;
	});

	onDestroy(() => {
		unsubscribe();
	});

	function handleClickDButton() {
		summaryActive.set(false);
	}

	// ====================================

	const getDisk = async () => {
		let blueprint = await getBlueprint();
		return blueprint.disk;
	};

	const getDiskSize = async () => {
		let disk = await getDisk();
		let size = 0;

		for (let i of disk.keys()) {
			size += disk[i].size;
		}

		return size;
	};

	const getStorageJSON = async () => {
		let json = await getRead();
		json = json.disk.filter((disk) => disk.partitions !== null);

		return json;
	};

	const setSummary = async () => {
		let json = await getBlueprint();
		console.log(json);

		timezone = json.timezone.region + '/' + json.timezone.city;
		mainLocale = json.locale.main;

		let partitions = json.disk.filter(
			(partition) => partition.format !== false || partition.mountpoint !== null
		);

		formattedPartitions = partitions.filter((partition) => partition.format !== null);
		assignedPartitions = partitions.filter((partition) => partition.mountpoint !== null);
	};

	const getColors = (disk) => {
		let length = disk.length;

		let colors = [];

		for (let i = 0; i < length; i++) {
			colors.push(
				randomColor({
					luminosity: 'bright',
					hue: 'random'
				})
			);
		}
		return colors;
	};

	const printJson = async () => {
		await invoke('print_json');
	};
</script>

{#await getBlueprint() then blueprint}
	<aside
		class="fixed z-[50] bg-black/50 w-screen transition-transform duration-500 ease-in-out top-0"
		class:translate-x-0={itActive}
		class:-translate-x-full={!itActive}
	>
		<div
			class="h-screen bg-whiteTealinux w-[60vw] flex p-8 pr-4 transition-transform duration-700 ease-in-out overflow-y-auto"
			class:translate-x-0={itActive}
			class:-translate-x-full={!itActive}
		>
			<div class=" w-[95%] px-2">
				<div class="flex gap-x-6 mb-8 font-archivo font-semibold text-4xl">
					<img src="/summary.svg" alt="" />

					<h1>Summary</h1>
				</div>
				<div class="flex justify-between gap-x-4 items-center py-8 px-8 h-fit rounded-3xl">
					<div class="mb-4 w-full">
						<div class="flex justify-between">
							<h2 class="font-poppin mb-2 font-semibold">Keyboard layout</h2>
							<a href="/installation/keyboard">
								<img src="/green-pencil.svg" alt="" class="mb-2" />
							</a>
						</div>
						<div
							class="relative flex items-center w-full h-[45px] rounded-[10px] bg-grayTealinux border-2 border-black overflow-hidden mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
						>
							{#if blueprint.keyboard}
								<span class="ml-[12px] font-poppin text-gray-500 text-[14px]"
									>{blueprint.keyboard.layout} - {blueprint.keyboard.variant}</span
								>
							{:else}
								<span class="ml-[12px] font-poppin text-gray-500 text-[14px]"></span>
							{/if}
						</div>
						<div class="mb-4 w-full">
							<div class="flex justify-between">
								<h2 class="font-poppin text-left mb-2 font-semibold">Timezone</h2>
								<a href="/installation/timezone">
									<img src="/green-pencil.svg" alt="" class="mb-2" />
								</a>
							</div>
							<div
								class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
							>
								<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
									Region:
								</h2>
								{#if blueprint.timezone}
									<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"
										>{blueprint.timezone.region}</span
									>
								{:else}
									<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"></span>
								{/if}
							</div>
							<div
								class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
							>
								<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
									City:
								</h2>
								{#if blueprint.timezone}
									<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"
										>{blueprint.timezone.city}</span
									>
								{:else}
									<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"></span>
								{/if}
							</div>
						</div>

						<div class="mb-4 w-full">
							<div class="flex justify-between">
								<h2 class="font-poppin text-left mb-2 font-semibold">Locale</h2>
								<a href="/installation/locale">
									<img src="/green-pencil.svg" alt="" class="mb-2" />
								</a>
							</div>
							<div
								class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
							>
								<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
									Main locale:
								</h2>
								{#if blueprint.locale}
									<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"
										>{blueprint.locale.main}</span
									>
								{:else}
									<span class="ml-[4px] font-poppin text-gray-500 text-[14px]`"></span>
								{/if}
							</div>
						</div>

						<div class="mb-4 w-full">
							<div class="flex justify-between">
								<h2 class="font-poppin text-left mb-2 font-semibold">User</h2>
								<a href="/installation/account">
									<img src="/green-pencil.svg" alt="" class="mb-2" />
								</a>
							</div>
							<div
								class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
							>
								<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
									Computer name:
								</h2>
								{#if blueprint.account}
									<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"
										>{blueprint.account.fullname}</span
									>
								{:else}
									<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"></span>
								{/if}
							</div>
							<div
								class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
							>
								<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
									Username:
								</h2>
								{#if blueprint.account}
									<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"
										>{blueprint.account.hostname}</span
									>
								{:else}
									<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"></span>
								{/if}
							</div>
							<div
								class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
							>
								<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
									Password:
								</h2>
								{#if blueprint.account}
									<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"
										>{blueprint.account.password}</span
									>
								{:else}
									<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"></span>
								{/if}
								<img src="/eyeSlash.svg" alt="" class="mr-[17.18px] ml-auto" />
							</div>
						</div>

						<div class="w-full mx-auto mb-4">
							<div class="flex relative items-center justify-between">
								<h2 class="font-poppin font-semibold text-[15px]">Partition installation</h2>
								<a href="/installation/partition">
									<img src="/green-pencil.svg" alt="" class="" />
								</a>
							</div>
							<h1 class="p-4 text-[20px] font-bold text-center">After</h1>
						</div>
						{#await getDisk() then disks}
							{@const colors = getColors(disks)}
							{#await getDiskSize() then diskSize}
								<div class="w-full">
									<div class="flex mb-4 h-8 w-full overflow-hidden rounded-full">
										<div class="h-full flex rounded-full overflow-hidden w-full">
											{#each disks as partition, i}
												{@const partitionSize = partition.size}
												{@const percentage = (partitionSize / diskSize) * 100}

												{@const color = colors[i]}

												<div
													style="width: {percentage}%; background-color: {color}"
													class="h-full"
												></div>
											{/each}
										</div>
									</div>
									<div class="flex gap-y-4 flex-wrap mb-4">
										{#each disks as partition, i}
											{@const color = colors[i]}
											{@const size = partition.size * 512}
											{@const path =
												partition.path == null ? 'Unallocated' : partition.path.slice(5)}
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
								</div>
							{/await}
						{/await}

						<div class="mt-[8px]">
							<div
								class="relative flex items-center w-full h-[50px] rounded-tl-lg rounded-tr-lg bg-white overflow-hidden border border-greyBorder font-poppin text-[14px] mx-auto"
							>
								<div class="grid grid-cols-4 text-center w-full">
									<h2>Partition</h2>
									<h2>File system</h2>
									<h2>Used as</h2>
									<h2>Format</h2>
								</div>
							</div>
							{#await getDisk() then disk}
								{#each disk as partition}
									{@const mountpoint =
										partition.mountpoint === null ? 'No mountpoint' : partition.mountpoint}
									{@const path = partition.path === null ? 'Unallocated' : partition.path}
									{@const filesystem =
										path === 'Unallocated'
											? 'Unallocated'
											: partition.filesystem === null
												? 'Unknown'
												: partition.filesystem}
									{@const format = partition.format ? 'Yes' : 'No'}
									{@const size = prettyBytes(partition.size * 512)}

									<div
										class="relative grid grid-cols-4 gap-x-1 md:flex-row items-center w-full h-[65px] bg-white overflow-hidden border border-greyBorder font-poppin text-[14px] mx-auto"
									>
										<div class="pl-[10px]">
											<h2>{path}</h2>
											<h2 class="text-black">{size}</h2>
										</div>
										<div
											class="bg-grayTealinux w-full h-[42px] rounded-xl flex justify-center items-center flex-wrap"
										>
											<span class="text-black">{filesystem}</span>
										</div>
										<div
											class="bg-grayTealinux w-full h-[42px] rounded-xl flex justify-center items-center flex-wrap"
										>
											<span class="text-black">{mountpoint}</span>
										</div>
										<div
											class="bg-grayTealinux w-full h-[42px] rounded-xl flex justify-center items-center flex-wrap"
										>
											<span class="text-black">{format}</span>
										</div>
									</div>
								{/each}
							{/await}
						</div>
					</div>
				</div>
				<div class="flex justify-center items-center mx-auto bg-white">
					<a
						href="/installation/summary/"
						on:click={handleClickDButton}
						class="text-white bg-greenTealinux focus:ring-4 w-[224px] text-center font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-8"
						>Back to summary page</a
					>
				</div>
			</div>
			<div
				class="w-[5%] grid justify-end items-center cursor-pointer"
				on:click={handleClickDButton}
			>
				<img src="/dropDownMain.svg" alt="" class="rotate-90 h-6" />
			</div>
		</div>
	</aside>
{/await}
