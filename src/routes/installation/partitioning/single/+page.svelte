<script lang="ts">
	import { commands, type BluePrint } from '$types/commands.js';
	import type { Disk } from '$types/read-from-opt.js';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import GlowingText from '$lib/components/ui/GlowingText.svelte';
	import Navigation from '$lib/components/Navigation.svelte';
	import DiskPreview from '$lib/components/DiskPreview.svelte';
	import CardTextArea from '../components/CardTextArea.svelte';
	import PreviewButton from '../components/PreviewButton.svelte';
	import { getDiskAfter, getIdealSwapSize } from '../utils';
	import { Preview } from '$types/installation/partitioning/partitioning.types.js';
	import { getBlueprintInfo, getSystemInfo } from '$lib/utils/read_utils.js';
	import { resolve } from '$app/paths';

	let blueprint = $state<BluePrint | null>(null);
	let diskBefore = $state<Disk | null>(null);
	let diskAfter = $state<Disk | null>(null);
	let selectedDisk = $state<Disk | null>(null);
	let partitionTable = $state<'mbr' | 'gpt' | null>(null);
	let memorySize = $state<number | null>(null);
	let selectedFilesystem = $state('ext4');
	let selectedPreview = $state(Preview.BEFORE);
	let useSwap = $state(false);

	const getStorageJSON = async (diskPath: string | null) => {
		let json = await getSystemInfo();
		selectedDisk = json?.disk.find((disk) => disk.diskPath === diskPath) || null;

		return selectedDisk;
	};

	const decideFilesystem = (filesystem: string) => {
		selectedFilesystem = filesystem;
	};

	const decideSwap = (swap: boolean) => {
		useSwap = swap;
	};

	const handlePartitioning = async () => {
		let diskPath = blueprint?.storage?.diskPath;
		let installMethod = blueprint?.storage?.installMethod;

		try {
			await commands.autogenPartitionSelectDisk(
				diskPath!,
				`${installMethod}boot`,
				partitionTable!,
				selectedFilesystem,
				useSwap,
				null,
				null
			);

			goto(resolve('/installation/account'));
		} catch (error) {
			alert('Error: ' + error);
		}
	};

	$effect(() => {
		if (diskBefore && selectedFilesystem && partitionTable && memorySize) {
			let swapSize = useSwap ? getIdealSwapSize(memorySize) : 0;
			diskAfter = getDiskAfter(diskBefore, selectedFilesystem, partitionTable, swapSize);
		}
		selectedPreview = Preview.AFTER;
	});

	onMount(async () => {
		blueprint = await getBlueprintInfo();
		diskBefore = await getStorageJSON(blueprint?.storage?.diskPath || null);
		let read = await getSystemInfo();

		partitionTable = read?.firmware == 'BIOS' ? 'mbr' : 'gpt';

		memorySize = read?.memory.capacity || null;

		diskAfter = getDiskAfter(diskBefore!, selectedFilesystem, partitionTable, 0);
	});
</script>

{#if blueprint && diskBefore}
	<TwoSide>
		{#snippet left()}
			<div class="mx-[35px] space-y-[15px]">
				<h1 class="font-archivo font-semibold text-[28px]">
					Configure <span class="text-green-tealinux">Single Boot</span><br />
				</h1>
				<p class="font-jakarta text-sm font-extralight">
					Install TealinuxOS as the only operating system on your disk by erasing existing
					partitions and setting up a new structure.
				</p>
			</div>
		{/snippet}

		{#snippet right()}
			<div
				class="flex flex-col h-[562px] p-4 space-y-[15px] mb-[15px] bg-black/30 border-[0.5px] border-gray-900 rounded-[10px] font-jakarta"
			>
				<div class="flex flex-col gap-1">
					<GlowingText size="[11]" text="Selected Disk" />

					{#key diskBefore}
						<CardTextArea
							initialDevice={blueprint?.storage?.diskPath || ''}
							description={diskBefore ? diskBefore.model : 'Unknown'}
							showCaption={false}
							showIcon={true}
							isSelected={true}
						/>
					{/key}
				</div>

				<div class="flex flex-col gap-2">
					<GlowingText size="[11]" text="File System" />
					{#key selectedFilesystem}
						<CardTextArea
							initialDevice="EXT4"
							caption="Stable and widely used!"
							showCaption={true}
							showIcon={false}
							onclick={() => decideFilesystem('ext4')}
							isSelected={selectedFilesystem === 'ext4'}
						/>
						<CardTextArea
							initialDevice="BTRFS"
							caption="Support snapshots (Advanced)"
							showCaption={true}
							showIcon={false}
							onclick={() => decideFilesystem('btrfs')}
							isSelected={selectedFilesystem === 'btrfs'}
						/>
					{/key}
				</div>

				<div class="flex flex-col gap-2">
					<GlowingText size="[11]" text="Swap Option" />
					{#key useSwap}
						<CardTextArea
							initialDevice="NO SWAP"
							caption="No problem"
							showCaption={true}
							showIcon={false}
							onclick={() => decideSwap(false)}
							isSelected={!useSwap}
						/>
						<CardTextArea
							initialDevice="SWAP"
							caption="Recommended"
							showCaption={true}
							showIcon={false}
							onclick={() => decideSwap(true)}
							isSelected={useSwap}
						/>
					{/key}
				</div>

				<div
					class="flex flex-col p-[15px] gap-2px] self-stretch rounded-[10.267px] border border-[#3C6350] bg-[#101010]"
				>
					<!-- {#if $selectedDisk} -->
					<div class="flex flex-row space-x-2">
						<PreviewButton
							title={Preview.BEFORE}
							selected={selectedPreview === Preview.BEFORE}
							onclick={() => (selectedPreview = Preview.BEFORE)}
						/>
						<PreviewButton
							title={Preview.AFTER}
							selected={selectedPreview === Preview.AFTER}
							onclick={() => (selectedPreview = Preview.AFTER)}
						/>
					</div>
					<div class="space-y-2.5 w-full">
						{#key diskAfter}
							{#if selectedPreview === Preview.BEFORE}
								<DiskPreview disk={diskBefore} />
							{:else if diskAfter}
								<DiskPreview disk={diskAfter} />
							{/if}
						{/key}
					</div>
				</div>
			</div>
		{/snippet}
	</TwoSide>
{/if}

<Navigation
	currentStep={4}
	currentTitle="Single Boot"
	prevPath="/installation/partitioning"
	nextAction={handlePartitioning}
	nextPath="/installation/account"
/>
