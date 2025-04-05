<script>
	// import { onMount } from 'svelte';
	import { getRead } from './global.js';
	import prettyBytes from 'pretty-bytes';
	import { randomColor } from 'randomcolor';
	import Navigation from '$lib/components/Navigation.svelte';
	import TwoSide from '../../lib/components/layouts/TwoSide.svelte';
	import GlowingText from '../../lib/components/ui/GlowingText.svelte';

	const getStorageJSON = async () => {
		let json = await getRead();

		json = json.disk.filter((disk) => disk.partitions !== null);

		return json;
	};

	const getTotalStorage = async () => {
		let storage = await getStorageJSON();
		let total = 0;

		for (let i of storage.keys()) {
			let size = storage[i].size.slice(0, -1);

			total += parseInt(size);
		}

		return total;
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

	const checkUnknown = (s) => {
		if (!s || s.length === 0) {
			return 'Unknown';
		} else {
			return s;
		}
	};

	// onMount(() => {
	// 	getStorageJSON().then((disks) => {
	// 		getColors(disks, 0);
	// 	});
	// });
</script>

<main class="min-h-dvh min-w-dvw grid place-items-center bg-tealinux font-jakarta">
	{#await getRead() then json}
		<!-- <pre>
		{JSON.stringify(json, null, 2)}
	</pre> -->
		<div class="flex flex-col justify-between max-h-[720px] min-w-full min-h-full max-w-[1080px]">
			<TwoSide>
				<div slot="left">
					<div class="w-[288px] space-y-[15px]">
						<div class="flex space-x-[14px]">
							<div class="w-[58px]">
								<img src="/logo-tealinux.svg" class="w-full" alt="logo" />
							</div>
							<h1 class="font-archivo font-[600] text-[40px] tracking-[-1.8px]">TeaLinux OS</h1>
						</div>
						<p class="font-jakarta text-sm font-[200] tracking-[-0.56px] text-center">
							Qorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam eu turpis molestie,
							dictum est a, mattis tellus.
						</p>
					</div>
				</div>
				<div slot="right">
					<!-- information system -->
					<div class="flex space-x-5 mb-[15px]">
						<div
							class="w-1/2 bg-[#101010] border-[1.3px] border-[#3C6350] p-[15px] rounded-[14px] space-y-5"
						>
							<GlowingText size="[15]" text="Hardware" />
							<div class="space-y-5 text-[15px] tracking-[-0.6px]">
								<div class="leading-none space-y-[10px]">
									<p class="font-[500]">Hardware model</p>
									<p class="font-[200]">
										{checkUnknown(json.model.systemVersion)}-{checkUnknown(
											json.model.systemProductName
										)}
									</p>
								</div>
								<div class="leading-none space-y-[10px]">
									<p class="font-[500]">Memory</p>
									<p class="font-[200]">
										{parseFloat((json.memory.capacity / 1024).toFixed(2))} GiB
									</p>
								</div>
								<div class="leading-none space-y-[10px]">
									<p class="font-[500]">Processor</p>
									<p class="font-[200]">
										{checkUnknown(json.lspci.cpu)}
									</p>
								</div>
								<div class="leading-none space-y-[10px]">
									<p class="font-[500]">Primary Graphic Card</p>
									<p class="font-[200]">{checkUnknown(json.lspci.vga[0])}</p>
								</div>

								<div class="leading-none space-y-[10px]">
									<p class="font-[500]">Secondary Graphic Card</p>
									<p class="font-[200]">{checkUnknown(json.lspci.vga[1])}</p>
								</div>
								{#await getTotalStorage() then totalSize}
									{@const storage = totalSize * 512}
									{@const storageGB = storage === 0 ? 'No disk' : prettyBytes(totalSize * 512)}
									<div class="leading-none space-y-[10px]">
										<p class="font-[500]">Disk Capacity</p>
										<p class="font-[200]">{storageGB}</p>
									</div>
								{/await}
							</div>
						</div>

						<div
							class="w-1/2 bg-[#101010] border-[1.3px] border-[#3C6350] p-[15px] rounded-[14px] space-y-5"
						>
							<GlowingText size="[15]" text="Operating System" />
							<div class="space-y-5 text-[15px] tracking-[-0.6px]">
								<div class="leading-none space-y-[10px]">
									<p class="font-[500]">Operating System</p>
									<p class="font-[200]">Arch Linux</p>
								</div>
								<div class="leading-none space-y-[10px]">
									<p class="font-[500]">Operating System Architecture</p>
									<p class="font-[200]">x86_64</p>
								</div>
								<div class="leading-none space-y-[10px]">
									<p class="font-[500]">Desktop Environment</p>
									<p class="font-[200]">COSMIC</p>
								</div>
								<div class="leading-none space-y-[10px]">
									<p class="font-[500]">Windowing System</p>
									<p class="font-[200]">Wayland</p>
								</div>
							</div>
						</div>
					</div>

					<!-- partition -->
					<div class="bg-[#101010] border-[1.3px] border-[#3C6350] p-[15px] rounded-[14px]">
						<GlowingText size="lg" text="Disk" />

						{#await getStorageJSON()}
							Loading...
						{:then disks}
							{#each disks as disk, idx}
								{@const size = parseInt(disks[idx].size.replace('s', ' '))}
								{@const prettySize = size === 0 ? 'No disk' : prettyBytes(size * 512)}
								{@const colors = getColors(disks, idx)}

								<div class="mb-2 mt-1 flex justify-between items-center uppercase">
									<p>
										<span class="text-sm font-[500]">{disk.diskPath}</span>
										{' '}
										<span class="text-xs font-[400]">{disk.model}</span>
									</p>
									<p class="text-xs font-[200]">
										{disk.label}
									</p>
								</div>

								<div class=" flex gap-x-4 items-start">
									<div class="w-full">
										<div class="flex mb-4 h-7 w-full overflow-hidden">
											<div class="h-full flex overflow-hidden w-full">
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
												<div class="flex items-center pr-2 gap-x-[2px]">
													<div style="background-color: {color}" class="w-2 h-2 rounded-full"></div>
													<div class="flex flex-col text-[11px] font-jakarta">
														<span class="pl-1">{filesystem}</span>
														<!-- <span class="pl-1">{prettySize} {filesystem}</span> -->
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
			</TwoSide>
			<Navigation />
		</div>
	{/await}
</main>

<style>
	@reference "tailwindcss/theme";
</style>
