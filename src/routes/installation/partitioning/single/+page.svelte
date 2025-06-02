<script>
	import { invoke } from '@tauri-apps/api/core';
	import { goto } from '$app/navigation';
	import { getRead, getBlueprint } from './../../global.js';
	import { onMount } from 'svelte';
	import { writable } from 'svelte/store';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import GlowingText from '$lib/components/ui/GlowingText.svelte';
	import Navigation from '$lib/components/Navigation.svelte';
	import DiskPreview from '$lib/components/DiskPreview.svelte';
	import CardTextArea from '../components/CardTextArea.svelte';
	import PreviewButton from '../components/PreviewButton.svelte';
	import { getDiskAfter, getIdealSwapSize } from '../utils.js';

	const Method = {
		SINGLE: 'single',
		DUAL: 'dual',
		MANUAL: 'manual'
	};

	const Preview = {
		BEFORE: 'Before',
		AFTER: 'After'
	};

	// const disks = writable([]);
	// const selectedDisk = writable(null);
	// const selectedMethod = writable(null);
	// const diskAfter = writable(null);
	// const selectedPreview = writable(Preview.BEFORE);

	let blueprint = $state(null);
	let diskBefore = $state(null);
	let diskAfter = $state(null);
	let selectedDisk = $state(null);
	let selectedFilesystem = $state('ext4');
	let selectedPreview = $state(Preview.BEFORE);
	let partitionTable = $state(null);
	let useSwap = $state(false);
	let memorySize = $state(null);

	const getBlueprintJSON = async () => {
		let blueprint = await getBlueprint();
		return blueprint;
	};

	const getStorageJSON = async (selected) => {
		let json = await getRead();
		selectedDisk = json.disk.find((disk) => disk.diskPath === selected);

		return selectedDisk;
	};

	function updateDiskPreview(disk) {
		if (!disk) return;

		// Simulasi preview AFTER dengan menambah partisi dummy
		diskAfter.set({
			...disk,
			partitions: [...disk.partitions, { name: 'New Partition', size: '500MB' }]
		});
	}

	const selectDisk = (disk) => {
		console.log(`Selected Disk: ${disk.name}`);
		selectedDisk = disk;
		updateDiskPreview(disk);
	};

	const decideFilesystem = (filesystem) => {
		selectedFilesystem = filesystem;
	};

	const decideSwap = (swap) => {
		useSwap = swap;
	};

	const handlePartitioning = async () => {
		let blueprint = await getBlueprintJSON();

		let diskPath = blueprint.storage.diskPath;
		let installMethod = blueprint.storage.installMethod;

		console.log(diskPath, installMethod, partitionTable);

		console.log('Invoking autogen_partition_select_disk');

		await invoke('autogen_partition_select_disk', {
			blkname: diskPath,
			mode: `${installMethod}boot`,
			partitionTable: partitionTable,
			fs: selectedFilesystem,
			useSwap: useSwap
		})
			.then(() => {
				// NOP
				goto('/installation/account');
			})
			.catch((error) => {
				alert('Error: ' + error);
			});
	};

	$effect(async () => {
		if (diskBefore && selectedFilesystem && partitionTable && memorySize) {
			let swapSize = useSwap ? await getIdealSwapSize(memorySize) : 0;
			diskAfter = getDiskAfter(diskBefore, selectedFilesystem, partitionTable, swapSize);
		}
		selectedPreview = Preview.AFTER;
	});

	onMount(async () => {
		blueprint = await getBlueprintJSON();
		diskBefore = await getStorageJSON(blueprint.storage.diskPath);
		let read = await getRead();
		partitionTable = read.firmware == 'BIOS' ? 'mbr' : 'gpt';

		memorySize = read.memory.capacity;

		diskAfter = getDiskAfter(diskBefore, selectedFilesystem, partitionTable, 0);
	});
</script>

{#if blueprint && diskBefore}
	<TwoSide>
		{#snippet left()}
			<div class="mx-[35px] space-y-[15px]">
				<h1 class="font-jakarta font-[800] text-[28px]">
					Configure <span class="text-green-tealinux">Single Boot</span><br />
				</h1>
				<p class="font-jakarta text-sm font-[200]">
                    Install TealinuxOS as the only operating system on your disk by erasing existing partitions and setting up a new structure.
				</p>
			</div>
		{/snippet}

		{#snippet right()}
			<div
				class="flex flex-col h-[562px] p-3 space-y-[15px] mb-[15px] bg-black/30 border-[0.5px] border-gray-900 rounded-[10px] font-jakarta"
			>
				<div class="flex flex-col gap-2">
					<GlowingText size="[11]" text="Selected Disk" />

					{#key diskBefore}
						<CardTextArea
							initialDevice={blueprint.storage.diskPath}
							initialDescription={diskBefore ? diskBefore.model : 'Unknown'}
							showCaption={false}
							showIcon={true}
							isSelected={true}
						/>
					{/key}
				</div>

				<GlowingText size="[11]" text="File System" />
				<div class="flex flex-col gap-2">
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

				<GlowingText size="[11]" text="Swap Option" />
				<div class="flex flex-col gap-2">
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
					<div class="space-y-[10px] w-full">
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
/>
