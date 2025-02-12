<script>
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { getRead, getBlueprint } from '../global.js';
	import prettyBytes from 'pretty-bytes';
	import { randomColor } from 'randomcolor';

	let timezone;
	let mainLocale;
	let locales;
	let formattedPartitions;
	let assignedPartitions;
	let passwordVisible = false;

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

	function togglePasswordVisibility() {
    passwordVisible = !passwordVisible;
  }

	const printJson = async () => {
		await invoke('print_json');
	};
</script>

<Sidebar />
<div class="relative w-full">
	<header
		class="flex items-center justify-center w-full gap-[10px] py-10 fixed top-0 bg-whiteTealinux z-30"
	>
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
	</header>
	{#await getBlueprint() then blueprint}
        {@const keyboard = blueprint.keyboard === null ? 'To be filled' : blueprint.keyboard.layout + ' - ' + blueprint.keyboard.variant}
        {@const timezoneRegion = blueprint.timezone === null ? 'To be filled' : blueprint.timezone.region}
        {@const timezoneCity = blueprint.timezone === null ? 'To be filled' : blueprint.timezone.city}
        {@const locale = blueprint.locale === null ? 'To be filled' : blueprint.locale.main}
        {@const userFullname = blueprint.account === null ? 'To be filled' : blueprint.account.fullname}
        {@const userUsername = blueprint.account === null ? 'To be filled' : blueprint.account.username}
        {@const userHostname = blueprint.account === null ? 'To be filled' : blueprint.account.hostname}
        {@const userPassword = blueprint.account === null ? 'To be filled' : blueprint.account.password}

		<section class="flex flex-col items-center justify-center h-auto mt-32">
			<form class=" text-center w-[50dvw] p-8 rounded-md min-h-[50dvh] pb-[12dvh]">
				<h1 class="text-center mb-6 font-bold text-[32px] font-archivo">Summary</h1>
				<div class="mb-4">
					<div class="flex justify-between">
						<h2 class="font-poppin mb-2 font-semibold">Keyboard layout</h2>
						<img
							on:click={() => (window.location.href = '/installation/keyboard')}
							src="/green-pencil.svg"
							alt=""
							class="text-left mb-2"
						/>
					</div>
					<div
						class="relative flex items-center w-full h-[45px] rounded-[10px] bg-grayTealinux border-2 border-black overflow-hidden mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
					>
						<span class="ml-[12px] font-poppin text-gray-500 text-[14px]"
							>{keyboard}</span
						>
					</div>
				</div>

				<div class="mx-auto mb-4">
					<div class="flex justify-between">
						<h2 class="font-poppin text-left mb-2 font-semibold">Timezone</h2>
						<img
							on:click={() => (window.location.href = '/installation/timezone')}
							src="/green-pencil.svg"
							alt=""
							class="text-left mb-2"
						/>
					</div>
					<div
						class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
					>
						<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
							Region:
						</h2>
						<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"
							>{timezoneRegion}</span
						>
					</div>
					<div
						class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
					>
						<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
							City:
						</h2>
						<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"
							>{timezoneCity}</span
						>
					</div>
				</div>

				<div class="mx-auto mb-4">
					<div class="flex justify-between">
						<h2 class="font-poppin text-left mb-2 font-semibold">Locale</h2>
						<img
							on:click={() => (window.location.href = '/installation/locale')}
							src="/green-pencil.svg"
							alt=""
							class="text-left mb-2"
						/>
					</div>
					<div
						class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
					>
						<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
							Main locale:
						</h2>
						<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"
							>{locale}</span
						>
					</div>
				</div>

				<div class="mx-auto mb-4">
					<div class="flex justify-between">
						<h2 class="font-poppin text-left mb-2 font-semibold">User</h2>
						<img
							on:click={() => (window.location.href = '/installation/account')}
							src="/green-pencil.svg"
							alt=""
							class="text-left mb-2"
						/>
					</div>
					<div
						class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
					>
						<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
							Computer name:
						</h2>
						<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"
							>{userFullname}</span
						>
					</div>
					<div
						class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
					>
						<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
							Username:
						</h2>
						<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"
							>{userHostname}</span
						>
					</div>
					<div
						class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
					>
						<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
							Password:
						</h2>
						<span class="ml-[4px] font-poppin text-gray-500 text-[14px] {passwordVisible ? '' : 'password-hidden'}">
							{userPassword}
						  </span>
						  <img
						  src="/eyeSlash.svg"
						  alt="Toggle Visibility"
						  class="mr-[17.18px] ml-auto cursor-pointer"
						  on:click={togglePasswordVisibility}
						/>
					</div>
				</div>

                {#await getDisk() then disks}
                    <div class="w-full mx-auto mb-4">
                        <div class="flex relative items-center justify-between">
                            <h2 class="font-poppin font-semibold text-[15px]">Partition installation</h2>
                            <img
                                on:click={() => (window.location.href = '/installation/partition')}
                                src="/green-pencil.svg"
                                alt=""
                                class="text-left mb-2"
                            />
                        </div>
                        {#if disks !== null}
                            <h1 class="p-4 text-[18px] font-bold">After</h1>
                        {/if}

                        {#if disks !== null}
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
                                            {@const path = partition.path == null ? 'Unallocated' : partition.path.slice(5)}
                                            {@const filesystem =
                                                partition.filesystem == null
                                                    ? path == 'Unallocated'
                                                        ? 'Unallocated'
                                                        : 'Unknown'
                                                    : partition.filesystem}
                                            {@const prettySize = prettyBytes(size)}
                                            <div class="flex pr-2 gap-x-2">
                                                <div style="background-color: {color}" class="w-4 h-4 rounded-xs"></div>
                                                <div class="flex flex-col text-sm font-poppinmedium font-medium">
                                                    <span class="pl-1">{path}</span>
                                                    <span class="pl-1">{prettySize} {filesystem}</span>
                                                </div>
                                            </div>
                                        {/each}
                                    </div>
                                </div>
                            {/await}
                        {:else}
                            To be filled
                        {/if}

                        {#if disks !== null}
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
                            {#each disks as partition}
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
                                    <div class="pl-[10px] text-left">
                                        <h2>{path}</h2>
                                        <h2 class="text-black">{size}</h2>
                                    </div>
                                    <div
                                        class="bg-[#E7EDED] w-full h-[42px] rounded-xl flex justify-center items-center flex-wrap"
                                    >
                                        <span class="text-black">{filesystem}</span>
                                    </div>
                                    <div
                                        class="bg-[#E7EDED] w-full h-[42px] rounded-xl flex justify-center items-center flex-wrap"
                                    >
                                        <span class="text-black">{mountpoint}</span>
                                    </div>
                                    <div
                                        class="bg-[#E7EDED] w-full h-[42px] rounded-xl flex justify-center items-center flex-wrap"
                                    >
                                        <span class="text-black">{format}</span>
                                    </div>
                                </div>
                            {/each}
                        </div>
                        {/if}
                    </div>
                {/await}
			</form>
			<div class="flex justify-between items-center h-[12dvh] w-[80dvw] fixed bottom-0 bg-white">
				<a
					href="/installation/partition"
					class="text-white bg-greenTealinux focus:ring-4 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 ml-[100px]"
					>Back</a
				>
				<a
					href="/installation/install"
					class="text-white bg-greenTealinux focus:ring-4 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 mr-[100px]"
					>Install</a
				>
			</div>
		</section>
	{/await}
</div>

<style>
	.password-hidden {
	  -webkit-text-security: disc;
	  -moz-text-security: disc;
	  text-security: disc;
	}
  </style>