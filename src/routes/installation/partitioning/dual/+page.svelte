<script>
	import { getRead, getBlueprint } from '/src/routes/installation/global.js';
	import CardTextArea from './../components/CardTextArea.svelte';
	import { onMount } from 'svelte';
	import { writable } from 'svelte/store';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import GlowingText from '$lib/components/ui/GlowingText.svelte';
	import Navigation from '$lib/components/Navigation.svelte';
	import DiskPreview from '$lib/components/DiskPreview.svelte';
	import PreviewButton from '../components/PreviewButton.svelte';
	import { getDiskAfter } from '../utils.js';
	import { invoke } from '@tauri-apps/api/core';
	import { goto } from '$app/navigation';

	const {
		osName = 'Another Operating System',
		osDescription = 'windows10 on path /path/path/path'
	} = $props();

	const Method = {
		SINGLE: 'single',
		DUAL: 'dual',
		MANUAL: 'manual'
	};

	const Preview = {
		BEFORE: 'Before',
		AFTER: 'After'
	};

	// State declarations
	let blueprint = $state(null);
	let diskAfter = $state(null);
	let selectedDisk = $state(null);
	let selectedFilesystem = $state('ext4');
	let selectedPreview = $state(Preview.AFTER);
	let partitionTable = $state(null);
	let swapSize = $state(2048);
	let otherOs = $state(null);
	let isRefreshing = $state(false);
	let useSwap = $state(false);

	const getBlueprintJSON = async () => {
		let blueprint = await getBlueprint();
		return blueprint;
	};

	const getStorageJSON = async (selected) => {
		let json = await getRead();
		selectedDisk = json.disk.find((disk) => disk.diskPath === selected);
		return selectedDisk;
	};

	const getOtherOsJSON = async (path) => {
		let response = await invoke('get_other_os_json');
		let json = JSON.parse(response);

		let others = json.length ? json.filter((os) => os.path.includes(path)) : null;

		console.log(others);

		return others;
	};

	const refreshOtherOs = async (path) => {
		isRefreshing = true;
		otherOs = await getOtherOsJSON(path);
		isRefreshing = false;
	};

	function updateDiskPreview(disk) {
		if (!disk) return;

		diskAfter = getDiskAfter(disk, selectedFilesystem, partitionTable, swapSize);
	}

	const selectDisk = (disk) => {
		selectedDisk = disk;
		updateDiskPreview(disk);
	};

	const decideFilesystem = (filesystem) => {
		selectedFilesystem = filesystem;
		if (selectedDisk) {
			updateDiskPreview(selectedDisk);
		}
	};

	// const decideSwap = (swap) => {
	// 	swapSize = swap;
	// 	if (selectedDisk) {
	// 		updateDiskPreview(selectedDisk);
	// 	}
	// };
	const decideSwap = (swap) => {
		useSwap = swap;
		console.log(`use swap state: ${useSwap}`);
	};

	const handlePartitioning = async () => {
		let blueprint = await getBlueprintJSON();

		let diskPath = blueprint.storage.diskPath;
		let installMethod = blueprint.storage.installMethod;

		console.log('Invoking autogen_partition_select_disk');

		await invoke('autogen_partition_select_disk', {
			blkname: diskPath,
			mode: `${installMethod}boot`,
			partitionTable: partitionTable,
			fs: selectedFilesystem,
			useSwap: useSwap
		});

		goto('/installation/account');
	};
  
    const decideFilesystem = (filesystem) => {
        selectedFilesystem = filesystem;
        if (selectedDisk) {
            updateDiskPreview(selectedDisk);
        }
    };

    const decideSwap = (swap) => {
        swapSize = swap;
        if (selectedDisk) {
            updateDiskPreview(selectedDisk);
        }
    };

    const handlePartitioning = async () => {
        let blueprint = await getBlueprintJSON();

        let diskPath = blueprint.storage.diskPath;
        let installMethod = blueprint.storage.installMethod;

        console.log("Invoking autogen_partition_select_disk with:", {
            blkname: diskPath,
            mode: `${installMethod}boot`,
            partitionTable: partitionTable,
            fs: selectedFilesystem
        });

        await invoke('autogen_partition_select_disk', {
            blkname: diskPath,
            mode: `${installMethod}boot`,
            partitionTable: partitionTable,
            fs: selectedFilesystem
        });
    }

	$effect(() => {
		console.log('Selected disk changed:', selectedDisk);
		if (selectedDisk) {
			updateDiskPreview(selectedDisk);
			selectedPreview = Preview.AFTER;
		}
	});

	onMount(async () => {
		try {
			blueprint = await getBlueprintJSON();
			if (blueprint?.storage?.diskPath) {
				await getStorageJSON(blueprint.storage.diskPath);
			}

			let read = await getRead();
			partitionTable = read.firmware ? 'gpt' : 'mbr';

			if (selectedDisk) {
				diskAfter = getDiskAfter(
					selectedDisk,
					selectedFilesystem,
					partitionTable,
					swapSize ? 1 : 0
				);
				await refreshOtherOs(selectedDisk.diskPath);
			}
		} catch (error) {
			console.error('Error in onMount:', error);
		}
	});
</script>

{#if blueprint && selectedDisk}
	<TwoSide>
		{#snippet left()}
			<div class="mx-[35px] space-y-[15px]">
				<h1 class="font-jakarta font-[800] text-[28px]">
					Configure Dual Boot<br />
				</h1>
				<p class="font-jakarta text-sm font-[200]">
					Qorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam eu turpis molestie, dictum
					est a, mattis tellus.
				</p>
			</div>
		{/snippet}

		{#snippet right()}
			<div
				class="flex flex-col h-[562px] p-6 space-y-[15px] mb-[15px] bg-black/30 border-[0.5px] border-gray-900 rounded-[10px] font-jakarta"
			>
				<div class="flex flex-col gap-2 mt-8">
					<!-- OS Info Section -->
					<div class="flex items-center justify-between w-full">
						<div class="flex flex-col">
							<span
								class="text-[#4CDA95] font-['Plus_Jakarta_Sans'] text-[16px] font-bold leading-[140%]"
							>
								{osName}
							</span>
							{#key otherOs}
								{#if !isRefreshing}
									{#each otherOs as os}
										<span
											class="text-white font-['Plus_Jakarta_Sans'] text-[13px] font-normal leading-[140%] mt-1"
										>
											{os.name}
										</span>
									{:else}
										<span
											class="text-white font-['Plus_Jakarta_Sans'] text-[13px] font-normal leading-[140%] mt-1"
										>
											No other operating system found!
										</span>
									{/each}
								{:else}
									<span
										class="text-white font-['Plus_Jakarta_Sans'] text-[13px] font-normal leading-[140%] mt-1"
									>
										Refreshing...
									</span>
								{/if}
							{/key}
						</div>

						<button
							onclick={() => refreshOtherOs(selectedDisk.diskPath)}
							class="flex w-[131.389px] h-[43px] justify-center items-center gap-[7.963px] rounded-[14px] border-[0.239px] border-[#3C6350] bg-[#101010]"
						>
							<svg
								width="17"
								height="17"
								viewBox="0 0 17 17"
								fill="none"
								xmlns="http://www.w3.org/2000/svg"
							>
								<path
									id="Vector"
									d="M8.71289 16.2334C6.47956 16.2334 4.58789 15.4584 3.03789 13.9084C1.48789 12.3584 0.712891 10.4667 0.712891 8.2334C0.712891 6.00007 1.48789 4.1084 3.03789 2.5584C4.58789 1.0084 6.47956 0.233402 8.71289 0.233402C9.86289 0.233402 10.9629 0.470735 12.0129 0.945402C13.0629 1.42007 13.9629 2.0994 14.7129 2.9834V1.2334C14.7129 0.950069 14.8089 0.712735 15.0009 0.521402C15.1929 0.330069 15.4302 0.234069 15.7129 0.233402C15.9956 0.232735 16.2332 0.328735 16.4259 0.521402C16.6186 0.714069 16.7142 0.951402 16.7129 1.2334V6.2334C16.7129 6.51674 16.6169 6.7544 16.4249 6.9464C16.2329 7.1384 15.9956 7.23407 15.7129 7.2334H10.7129C10.4296 7.2334 10.1922 7.1374 10.0009 6.9454C9.80956 6.7534 9.71356 6.51607 9.71289 6.2334C9.71222 5.95074 9.80822 5.7134 10.0009 5.5214C10.1936 5.3294 10.4309 5.2334 10.7129 5.2334H13.9129C13.3796 4.30007 12.6506 3.56674 11.7259 3.0334C10.8012 2.50007 9.79689 2.2334 8.71289 2.2334C7.04622 2.2334 5.62956 2.81674 4.46289 3.9834C3.29622 5.15007 2.71289 6.56674 2.71289 8.2334C2.71289 9.90007 3.29622 11.3167 4.46289 12.4834C5.62956 13.6501 7.04622 14.2334 8.71289 14.2334C9.84622 14.2334 10.8839 13.9461 11.8259 13.3714C12.7679 12.7967 13.4969 12.0257 14.0129 11.0584C14.1462 10.8251 14.3339 10.6627 14.5759 10.5714C14.8179 10.4801 15.0636 10.4757 15.3129 10.5584C15.5796 10.6417 15.7712 10.8167 15.8879 11.0834C16.0046 11.3501 15.9962 11.6001 15.8629 11.8334C15.1796 13.1667 14.2046 14.2334 12.9379 15.0334C11.6712 15.8334 10.2629 16.2334 8.71289 16.2334Z"
									fill="#26A768"
								/>
							</svg>
							<span
								class="text-[#4CDA95] font-['Plus_Jakarta_Sans'] text-[13px] font-bold leading-[140%]"
							>
								Refresh
							</span>
						</button>
					</div>
					<div class="flex flex-col gap-2">
						<GlowingText size="[11]" text="Selected Disk" />

						{#if selectedDisk}
							<CardTextArea
								initialDevice={selectedDisk.diskPath}
								initialDescription={selectedDisk.model}
								showCaption={false}
								showIcon={true}
								isSelected={true}
							/>
						{:else}
							<CardTextArea
								initialDevice="No Disk Selected"
								initialDescription="Please select a disk"
								showCaption={false}
								showIcon={true}
								isSelected={false}
							/>
						{/if}
					</div>
				</div>

				<GlowingText size="[11]" text="File System" />
				<div class="flex gap-2">
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
				<div class="flex gap-2">
					{#key swapSize}
						<CardTextArea
							initialDevice="SWAP"
							caption="Recommended"
							showCaption={true}
							showIcon={false}
							onclick={() => decideSwap(true)}
							isSelected={!useSwap}
						/>
						<CardTextArea
							initialDevice="NO SWAP"
							caption="No problem"
							showCaption={true}
							showIcon={false}
							onclick={() => decideSwap(false)}
							isSelected={useSwap}
						/>
					{/key}
				</div>

				<div
					class="flex flex-col p-[15px] gap-[10px] self-stretch rounded-[10.267px] border border-[#3C6350] bg-[#101010]"
				>
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
						{#if selectedPreview === Preview.BEFORE && selectedDisk}
							{#key selectedDisk}
								<DiskPreview disk={selectedDisk} />
							{/key}
						{:else if selectedPreview === Preview.AFTER && diskAfter}
							{#key diskAfter}
								<DiskPreview disk={diskAfter} />
							{/key}
						{:else}
							<div class="text-center py-4 text-gray-400">No disk data available</div>
						{/if}
					</div>
				</div>
			</div>
		{/snippet}
	</TwoSide>
{/if}
<Navigation
	currentStep={4}
	currentTitle="Dual Boot"
	prevPath="/installation/partitioning"
	nextPath="/installation/account"
    nextAction={handlePartitioning}
/>
