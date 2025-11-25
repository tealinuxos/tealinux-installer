<script lang="ts">
	import type { Disk, ReadFromOpt } from '$types/read-from-opt.js';
	import { commands } from '$types/commands.js';
	import { onMount } from 'svelte';
	import prettyBytes from 'pretty-bytes';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import GlowingText from '$lib/components/ui/GlowingText.svelte';
	import Navigation from '$lib/components/Navigation.svelte';
	import DiskSlider from '$lib/components/DiskSlider.svelte';

	let systemInfo = $state<ReadFromOpt | null>(null);
	let totalStorage = $state<number>(0);
	let isLoading = $state<boolean>(true);

	const getSystemInfo = async (): Promise<ReadFromOpt | null> => {
		try {
			const data = await commands.getReadFromOpt();
			const json: ReadFromOpt = typeof data === 'string' ? JSON.parse(data) : data;
			return json;
		} catch (error) {
			console.error('Error fetching system info:', error);
			return null;
		}
	};

	const calculateTotalStorage = (disks: Disk[] | undefined): number => {
		if (!disks || disks.length === 0) return 0;

		return disks.reduce((total, disk) => {
			let sizeStr = disk.size.replace(/\D/g, '');
			let size = Number(sizeStr);
			return (total += size);
		}, 0);
	};

	const checkUnknown = (s: string | undefined): string => {
		return !s || s.trim() === '' ? 'Unknown' : s;
	};

	onMount(async () => {
		const data = await getSystemInfo();

		if (data) {
			systemInfo = data;
			totalStorage = calculateTotalStorage(systemInfo.disk);
		}
		isLoading = false;
	});
</script>

{#if isLoading}
	<div class="flex h-full items-center justify-center">
		<p>Loading System Information...</p>
	</div>
{:else if systemInfo}
	<TwoSide>
		{#snippet left()}
			<div class="w-[288px] space-y-[15px]">
				<div class="flex space-x-3.5">
					<div class="w-[58px]">
						<img src="/logo-tealinux.svg" class="w-full" alt="logo" />
					</div>
					<h1 class="font-archivo font-semibold text-[40px] tracking-[-1.8px]">TeaLinux OS</h1>
				</div>
				<p class="font-jakarta text-sm font-extralight tracking-[-0.56px] text-center">
					<i>"Nikmatnya sebuah racikan"</i>
				</p>
			</div>
		{/snippet}

		{#snippet right()}
			<div class="flex space-x-2 mb-2 mt-[5px]">
				<div
					class="w-1/2 bg-[#101010] border-[1.3px] border-[#3C6350] p-[15px] rounded-[14px] space-y-4"
				>
					<GlowingText size="[15]" text="Hardware" />
					<div class="space-y-4 text-[15px]">
						<div class="leading-none space-y-2.5">
							<p class="font-medium">Hardware model</p>
							<p class="font-extralight">
								{checkUnknown(systemInfo?.model.systemVersion)}-{checkUnknown(
									systemInfo?.model.systemProductName
								)}
							</p>
						</div>
						<div class="leading-none space-y-2.5">
							<p class="font-medium">Memory</p>
							<p class="font-extralight">
								{((systemInfo?.memory?.capacity ?? 0) / 1024).toFixed(2)} GiB
							</p>
						</div>
						<div class="leading-none space-y-2.5">
							<p class="font-medium">Processor</p>
							<p class="font-extralight">
								{checkUnknown(systemInfo?.lspci.cpu)}
							</p>
						</div>
						<div class="leading-none space-y-2.5">
							<p class="font-medium">Primary Graphic Card</p>
							<p class="font-extralight">{checkUnknown(systemInfo?.lspci.vga[0])}</p>
						</div>

						<div class="leading-none space-y-2.5">
							<p class="font-medium">Secondary Graphic Card</p>
							<p class="font-extralight">{systemInfo?.lspci.vga[1] || '-'}</p>
						</div>

						<div class="leading-none space-y-2.5">
							<p class="font-medium">Disk Capacity</p>
							<p class="font-extralight">
								{totalStorage === 0 ? '-' : prettyBytes(totalStorage * 512)}
							</p>
						</div>
					</div>
				</div>

				<div
					class="w-1/2 bg-[#101010] border-[1.3px] border-[#3C6350] p-[15px] rounded-[14px] space-y-5"
				>
					<GlowingText size="[15]" text="Operating System" />
					<div class="space-y-4 text-[15px]">
						<div class="leading-none space-y-2.5">
							<p class="font-medium">Operating System</p>
							<p class="font-extralight">{systemInfo?.operatingSystem.name}</p>
						</div>
						<div class="leading-none space-y-2.5">
							<p class="font-medium">Operating System Architecture</p>
							<p class="font-extralight">{systemInfo?.operatingSystem.architecture}</p>
						</div>
						<div class="leading-none space-y-2.5">
							<p class="font-medium">Kernel</p>
							<p class="font-extralight">{systemInfo?.kernel.name} {systemInfo?.kernel.version}</p>
						</div>
						<div class="leading-none space-y-2.5">
							<p class="font-medium">Boot Mode</p>
							<p class="font-extralight">{checkUnknown(systemInfo?.firmware)}</p>
						</div>
						<div class="leading-none space-y-2.5">
							<p class="font-medium">Desktop Environment</p>
							<p class="font-extralight capitalize">{systemInfo?.desktopEnvironment.name}</p>
						</div>
						<div class="leading-none space-y-2.5">
							<p class="font-medium">Display Server</p>
							<p class="font-extralight capitalize">{systemInfo?.displayServer.name}</p>
						</div>
					</div>
				</div>
			</div>

			<DiskSlider disks={systemInfo.disk ?? []} />
		{/snippet}
	</TwoSide>
{:else}
	<div class="flex h-full items-center justify-center text-red-500">
		<p>Failed to load system information.</p>
	</div>
{/if}

<Navigation
	currentStep={1}
	currentTitle="System Information"
	prevPath="/installation"
	nextPath="/installation/localization"
	nextAction={null}
/>
