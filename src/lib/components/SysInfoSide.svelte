<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount, onDestroy } from 'svelte';
	import { getRead } from '../../routes/installation/global.js';
	import { sysInfoActivated } from './store.js'; // Path ke file store.js
	import prettyBytes from 'pretty-bytes';
	import { randomColor } from 'randomcolor';

	let isActivated;

	const unsubscribe = sysInfoActivated.subscribe((value) => {
		isActivated = value;
	});

	const getStorageJSON = async () => {
		let json = await getRead();

		return json.disk;
	};

	const getColors = (disks, partIdx) => {
		let length = disks[partIdx].partitions.length;

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

	onMount(() => {
		getStorageJSON().then((disks) => {
			getColors(disks);
		});
	});

	onDestroy(() => {
		unsubscribe();
	});

	function handleButtonClick() {
		sysInfoActivated.set(false);
	}
</script>

<aside
	class="fixed z-40 bg-black/50 w-screen transition-transform duration-500 ease-in-out"
	class:translate-x-0={isActivated}
	class:-translate-x-full={!isActivated}
>
	<div
		class="h-dvh bg-[#C8E8D6] w-[60dvw] flex p-8 pr-4 transition-transform duration-700 ease-in-out"
		class:translate-x-0={isActivated}
		class:-translate-x-full={!isActivated}
	>
		<div class=" w-[95%] px-2">
			<div class="flex gap-x-6 mb-8 font-archivo font-semibold text-4xl">
				<svg
					width="40"
					height="37"
					viewBox="0 0 40 37"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						d="M20 27.808V35.2546M12.6 35.2546H27.4M5.2 1.74512H34.8C36.8435 1.74512 38.5 3.41208 38.5 5.46839V24.0847C38.5 26.1411 36.8435 27.808 34.8 27.808H5.2C3.15655 27.808 1.5 26.1411 1.5 24.0847V5.46839C1.5 3.41208 3.15655 1.74512 5.2 1.74512Z"
						stroke="black"
						stroke-width="3"
						stroke-linecap="round"
						stroke-linejoin="round"
					/>
				</svg>

				<h1>System information</h1>
			</div>
			{#await getRead() then json}
				{@const memoryPercent = (json.memory.used / json.memory.capacity) * 100}
				{@const storageGB = prettyBytes(parseInt(json.disk[0].size.replace('s', ' ')) * 512)}
				<div class=" mx-auto overflow-auto scrollbar-none font-poppinmedium font-medium">
					<div class=" bg-white w-full rounded-[43px] mb-6">
						<div class="bg-white flex justify-between items-center py-8 px-8 h-fit rounded-3xl">
							<div class="flex flex-[1] flex-col items-center">
								<img src="/windows.svg" alt="" />
								<p class="text-2xl font-medium mt-[8px]">82SV</p>
								<p>{json.model.systemProductName + ' - ' + json.model.systemVersion}</p>
								<h2 class="font-medium font-poppin text-[16px] flex items-center">
									<img src="/battrey.svg" alt="" class="pr-[8px]" />
									{json.battery.capacity}%
								</h2>
								<div class="flex items-center">
									<img src="/Connection.svg" alt="" class="pr-[8px]" />
									<h2 class="font-medium font-poppin text-[16px] flex items-center">
										Online
										{#if json.online.status}
											<span
												class="w-3 ml-2 aspect-square rounded-full bg-green-400 inline-block border border-slate-600 pl-[6px]"
											></span>
										{:else}
											<span>false</span>
										{/if}
									</h2>
								</div>
							</div>
							<div class="flex flex-[1] flex-col items-center gap-y-4">
								<!-- RAM -->
								<div class="flex flex-col items-center h-full">
									<div class="flex items-center justify-center h-full">
										<h2 class="font-archivo font-bold text-[20px]">Ram</h2>
									</div>
									<div class="w-[241px] h-[16px] bg-grayTealinux rounded-[128px]">
										<div
											class="bg-[#F1C21B] h-[16px] rounded-[128px]"
											style="width: {memoryPercent.toFixed()}%"
										></div>
									</div>
									<h2 class="font-medium font-poppin text-[16px] mt-2">
										{memoryPercent.toFixed(2)}% of 100%
									</h2>
								</div>
								<!-- Storage -->
								<div class="flex flex-col items-center h-full">
									<div class="flex items-center justify-center h-full">
										<h2 class="font-archivo font-bold text-[20px]">Storage</h2>
									</div>
									<div class="w-[241px] h-[16px] bg-grayTealinux rounded-[128px]">
										<div
											class="bg-[#F1C21B] h-[16px] rounded-[128px] flex items-center"
											style="width: 90%"
										></div>
									</div>
									<h2 class="font-medium font-poppin text-[16px] mt-2">{storageGB}</h2>
								</div>
								<!-- GPU -->
								<div class="flex flex-col items-center h-full">
									<div class="flex items-center justify-center h-full">
										<h2 class="font-archivo font-bold text-[20px] text-center">GPU</h2>
									</div>
									<ul class="list-disc">
										{#each json.lspci.vga as vga}
											<li>
												<h2 class="font-poppin font-medium text-[16px]">{vga}</h2>
											</li>
										{/each}
									</ul>
								</div>
								<!-- CPU -->
								<div class="flex flex-col items-center h-full">
									<div class="flex items-center justify-center h-full">
										<h2 class="font-archivo font-bold text-[20px] text-center">CPU</h2>
									</div>
									<h2 class="font-poppin font-medium text-[16px]">{json.lspci.cpu}</h2>
								</div>
							</div>
						</div>
					</div>
					<!-- ============================================================================================ -->
					<div class=" bg-white w-full rounded-[43px] mb-6 flex justify-center overflow-y-auto">
						<div
							class="bg-white place-items-center py-2 px-8 max-h-[35dvh] min-w-full rounded-3xl overflow-y-auto"
						>
							{#await getStorageJSON()}
								Loading...
							{:then disks}
								{#each disks as disk, idx}
									{@const sizeGB = prettyBytes(parseInt(disks[idx].size.replace('s', ' ')) * 512)}
									{@const colors = getColors(disks, idx)}
									<div
										class="flex items-center justify-between bg-gray-300 h-[45px] rounded-[10px] mt-[30.05px] w-full"
									>
										<p
											class="font-poppin font-medium text-[#0D1814] text-[14px] mt-[12px] ml-[12px] mb-[12px]"
										>
											{disk.model + ' (' + disk.diskPath + ')'}
										</p>
										<p class="font-poppin text-[14px] text-[#0D1814] mt-[12px] mr-[12px] mb-[12px]">
											Disk Size : {sizeGB}
										</p>
									</div>
									<div class="mt-[33px] flex gap-x-4 items-start">
										<p class="font-poppin font-medium text-[18px]">Current:</p>

										<div class="w-full">
											<div class="flex flex-[1] mb-4 h-8 w-full overflow-hidden rounded-full">
												<div class="h-full flex rounded-full overflow-hidden w-full">
													{#each disk.partitions as partition, i}
														{@const diskSize = disk.size.slice(0, -1)}
														{@const partitionSize = partition.size.slice(0, -1)}
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
												{#each disk.partitions as partition, i}
													{@const color = colors[i]}
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
										</div>
									</div>
								{/each}
							{/await}
						</div>
					</div>
				</div>
			{/await}
		</div>
		<div class="w-[5%] grid justify-end items-center cursor-pointer" on:click={handleButtonClick}>
			<svg viewBox="0 0 14 9" fill="none" xmlns="http://www.w3.org/2000/svg" class=" rotate-90 h-6">
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
</aside>

<style>
	/* .scrollbar-none::-webkit-scrollbar {
		display: none;
	} */

	/* Hide scrollbar for IE, Edge and Firefox */
	.scrollbar-none {
		-ms-overflow-style: none; /* IE and Edge */
		scrollbar-width: none; /* Firefox */
	}
</style>
