<script>
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { getBlueprint } from '../global.js';

	let timezone;
	let mainLocale;
	let locales;
	let formattedPartitions;
	let assignedPartitions;

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

	const printJson = async () => {
		await invoke('print_json');
	};
</script>

<Sidebar />

{#await getBlueprint() then blueprint}
	{console.log(blueprint)}
	<section class="flex flex-col items-center justify-center h-auto">
		<header class="flex items-center justify-center w-full gap-[10px]">
			<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
			<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
			<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
			<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
			<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
			<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		</header>
		<form class=" text-center w-[50dvw] p-8 rounded-md min-h-[50dvh]">
			<h1 class="text-center mb-6 font-bold text-[32px] font-archivo">Summary</h1>
			<div class="mb-4">
				<div class="flex justify-between">
					<h2 class="font-poppin mb-2 font-semibold">Keyboard layout</h2>
					<img src="/green-pencil.svg" alt="" class="text-left mb-2" />
				</div>
				<div
					class="relative flex items-center w-full h-[45px] rounded-[10px] bg-grayTealinux border-2 border-black overflow-hidden mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
				>
					<span class="ml-[12px] font-poppin text-gray-500 text-[14px]"
						>{blueprint.keyboard.layout} - {blueprint.keyboard.variant}</span
					>
				</div>
			</div>

			<div class="mx-auto mb-4">
				<div class="flex justify-between">
					<h2 class="font-poppin text-left mb-2 font-semibold">Timezone</h2>
					<img src="/green-pencil.svg" alt="" class="mb-2" />
				</div>
				<div
					class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
				>
					<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
						Region:
					</h2>
					<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"
						>{blueprint.timezone.region}</span
					>
				</div>
				<div
					class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
				>
					<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
						City:
					</h2>
					<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"
						>{blueprint.timezone.city}</span
					>
				</div>
			</div>

			<div class="mx-auto mb-4">
				<div class="flex justify-between">
					<h2 class="font-poppin text-left mb-2 font-semibold">Locale</h2>
					<img src="/green-pencil.svg" alt="" class="mb-2" />
				</div>
				<div
					class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
				>
					<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
						Main locale:
					</h2>
					<span class="ml-[4px] font-poppin text-gray-500 text-[14px]">{blueprint.locale.main}</span
					>
				</div>
			</div>

			<div class="mx-auto mb-4">
				<div class="flex justify-between">
					<h2 class="font-poppin text-left mb-2 font-semibold">User</h2>
					<img src="/green-pencil.svg" alt="" class="mb-2" />
				</div>
				<div
					class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
				>
					<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
						Computer name:
					</h2>
					<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"
						>{blueprint.account.fullname}</span
					>
				</div>
				<div
					class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
				>
					<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
						Username:
					</h2>
					<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"
						>{blueprint.account.hostname}</span
					>
				</div>
				<div
					class="relative flex items-center w-full h-[45px] rounded-[10px] overflow-hidden bg-grayTealinux border-2 border-black mb-2 font-poppin text-[14px] mx-auto shadow-2xl"
				>
					<h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">
						Password:
					</h2>
					<span class="ml-[4px] font-poppin text-gray-500 text-[14px]"
						>{blueprint.account.password}</span
					>
					<img src="/eyeSlash.svg" alt="" class="mr-[17.18px] ml-auto" />
				</div>
			</div>

			<div class="w-full mx-auto mb-4">
				<div class="flex relative items-center justify-between">
					<h2 class="font-poppin font-semibold text-[15px]">Partition installation</h2>
					<img src="/green-pencil.svg" alt="" />
				</div>

				<h2 class="font-poppin font-medium text-[16px] text-center">After</h2>
				<div class="flex items-center justify-center">
					<h2 class="font-poppin font-medium text-[17px] mr-[10px]">/dev/sda</h2>
					<div class="w-full h-[27px] bg-[#C85036] rounded-[128px]">
						<div class="bg-[#3293C8] h-[27px] w-[50%] rounded-[128px]"></div>
					</div>
				</div>
				<div class="mt-[15px] flex flex-col items-start">
					<div class="flex ml-[92px]">
						<div class="flex items-start gap-x-4 mr-[69px]">
							<div class="flex items-center">
								<div class="bg-[#3293C8] w-[16px] h-[16px] rounded-[3px]"></div>
							</div>
							<div class="text-start">
								<p class="font-poppin font-medium text-[15px]">sdb1</p>
								<p class="font-poppin font-medium text-[16px] whitespace-nowrap">13.5 GiB LUKS</p>
							</div>
						</div>
						<div class="flex items-start gap-x-4">
							<div class="flex items-center">
								<div class="bg-[#C85036] w-[16px] h-[16px] rounded-[3px]"></div>
							</div>
							<div class="text-start">
								<p class="font-poppin font-medium text-[15px]">sdb2</p>
								<p class="font-poppin font-medium text-[16px] whitespace-nowrap">8.5 GiB LUKS</p>
							</div>
						</div>
					</div>
				</div>
				<!-- partisi v -->
				<div class="mt-[8px]">
					<div
						class="relative flex items-center w-full h-[50px] rounded-tl-lg rounded-tr-lg bg-white overflow-hidden border border-greyBorder font-poppin text-[14px] mx-auto"
					>
						<div class="flex flex-wrap gap-x-[180px] ml-[10px]">
							<h2>Partition</h2>
							<h2>File system</h2>
							<h2>Used as</h2>
							<h2>Format</h2>
						</div>
					</div>
					<div
						class="relative flex flex-col md:flex-row items-center w-full h-[65px] bg-white overflow-hidden border border-greyBorder font-poppin text-[14px] mx-auto"
					>
						<div class="pl-[10px]">
							<h2>/dev/sda1</h2>
							<h2 class="text-gray-500">33.55 GB</h2>
						</div>
						<div class="flex flex-wrap pl-[10px] md:pl-[100px] gap-4 md:gap-12">
							<div
								class="bg-gray-500 w-full md:w-[200px] h-[42px] rounded-xl flex justify-center items-center flex-wrap"
							>
								<span class="text-white">lalal</span>
							</div>
							<div
								class="bg-gray-500 w-full md:w-[200px] h-[42px] rounded-xl flex justify-center items-center flex-wrap"
							>
								<span class="text-white">lalal</span>
							</div>
							<div
								class="bg-gray-500 w-full md:w-[200px] h-[42px] rounded-xl flex justify-center items-center flex-wrap"
							>
								<span class="text-white">lalal</span>
							</div>
						</div>
					</div>
					<div
						class="relative flex flex-col md:flex-row items-center w-full h-[65px] bg-white overflow-hidden border border-greyBorder font-poppin text-[14px] mx-auto"
					>
						<div class="pl-[10px]">
							<h2>/dev/sda1</h2>
							<h2 class="text-gray-500">33.55 GB</h2>
						</div>
						<div class="flex flex-wrap pl-[10px] md:pl-[100px] gap-4 md:gap-12">
							<div
								class="bg-gray-500 w-full md:w-[200px] h-[42px] rounded-xl flex justify-center items-center flex-wrap"
							>
								<span class="text-white">lalal</span>
							</div>
							<div
								class="bg-gray-500 w-full md:w-[200px] h-[42px] rounded-xl flex justify-center items-center flex-wrap"
							>
								<span class="text-white">lalal</span>
							</div>
							<div
								class="bg-gray-500 w-full md:w-[200px] h-[42px] rounded-xl flex justify-center items-center flex-wrap"
							>
								<span class="text-white">lalal</span>
							</div>
						</div>
					</div>
					<div
						class="relative flex flex-col md:flex-row items-center w-full h-[65px] rounded-bl-lg rounded-br-lg bg-white overflow-hidden border border-greyBorder font-poppin text-[14px] mx-auto"
					>
						<div class="pl-[10px]">
							<h2>/dev/sda1</h2>
							<h2 class="text-gray-500">33.55 GB</h2>
						</div>
						<div class="flex flex-wrap pl-[10px] md:pl-[100px] gap-4 md:gap-12">
							<div
								class="bg-gray-500 w-full md:w-[200px] h-[42px] rounded-xl flex justify-center items-center flex-wrap"
							>
								<span class="text-white">lalal</span>
							</div>
							<div
								class="bg-gray-500 w-full md:w-[200px] h-[42px] rounded-xl flex justify-center items-center flex-wrap"
							>
								<span class="text-white">lalal</span>
							</div>
							<div
								class="bg-gray-500 w-full md:w-[200px] h-[42px] rounded-xl flex justify-center items-center flex-wrap"
							>
								<span class="text-white">lalal</span>
							</div>
						</div>
					</div>
				</div>
			</div>
		</form>
	</section>
{/await}

<!-- {#await setSummary()}
    Loading...
{:then}
    <h2 class="font-bold">Timezone</h2>
    <p>Set timezone to {timezone}.</p>

    <h2 class="font-bold">Locale</h2>
    <p>Set locale to ({mainLocale})</p>

    <h2 class="font-bold">Partition</h2>
    {#if formattedPartitions !== null}
        {#each formattedPartitions as partition}
            {@const path = partition.path}
            {@const filesystem = partition.format}
            <p>Format {path} as {filesystem}</p>
        {/each}
    {/if}

    {#if assignedPartitions !== null}
        {#each assignedPartitions as partition}
            {@const path = partition.path}
            {@const mountPoint = partition.mountpoint}
            <p>Assign {path} as {mountPoint}</p>
        {/each}
    {/if}

    <a class="border-2 border-black p-2" href="/installation/install">Start Install (Nginstal tenan)</a>
    <button class="border-2 border-black p-2" on:click={printJson}>Print JSON</button>
{/await}  -->
