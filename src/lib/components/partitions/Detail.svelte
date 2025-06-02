<script>
	import { prettySize } from '$lib/essentials.js';
	import { onMount } from 'svelte';
	import SelectComponent from '../SelectComponent.svelte';
	import ComponentSelect from './ComponentSelect.svelte';

	let {
		showEdit = $bindable(),
		modifiedPartition = $bindable(),
		tempModifiedPartition = $bindable(),
		selectedPartition = $bindable(),
		newPartition = $bindable(),
		readOnly = false,
		bootPartitionIndex = $bindable(),
		firmwareType = 'BIOS',
		highestNumber = $bindable(),
		partitionTable
	} = $props();

	let index = selectedPartition;
	let currentIndex = $state(selectedPartition);
	let newPartitionIndex = $state(1);

	const espSize = 2097152;

	let inputtedSize = $state(0);
	let actualSize = $state(0);
    let filesystem = $state(modifiedPartition[index].filesystem || null);
    let mountpoint = $state(modifiedPartition[index].mountpoint || null);
    let format = $state(modifiedPartition[index].format || false);
    let label = $state(modifiedPartition[index].label || null);
    let flags = $state(modifiedPartition[index].flags || []);

	let newAllocated = $state(null);
	let newEspPartition = $state(null);
	let newGap = $state(null);

	let flagList = $state(['hidden', 'boot', 'efi', 'esp', 'bios_grub']);

	const getSectorFromMB = (size) => {
		return Math.floor((Number(size) * 1024 * 1024) / 512);
	};

	const getFlagList = (existFlags) => {
		let flags = existFlags ? existFlags : [];

		let newList = flagList.concat(flags);

		let s = new Set(newList);

		flagList = [...s];
	};

	const isArrayIdentical = (a, b) => {
		return JSON.stringify(a) === JSON.stringify(b);
	};

	const applyModifiedPartition = () => {
		modifiedPartition = [];
		modifiedPartition = JSON.parse(JSON.stringify(tempModifiedPartition));
		showEdit = false;
	};

	const cancelModifiedPartition = () => {
		tempModifiedPartition = [];
		tempModifiedPartition = JSON.parse(JSON.stringify(modifiedPartition));

		showEdit = false;
	};

	const createPartition = () => {
		modifiedPartition = [];

		let inputtedSizeSector = getSectorFromMB(inputtedSize);

		let remainderSize = actualSize - inputtedSizeSector;

		if (remainderSize >= 0) {
			if (mountpoint === '/') {
				if (firmwareType === 'UEFI') {
					if (bootPartitionIndex === null) {
						newEspPartition = {
							...newAllocated,
							number: newAllocated.number,
							path: `#${newPartitionIndex}`,
							size: espSize,
							end: newAllocated.start + espSize - 1,
							filesystem: 'fat32',
							label: 'EFI',
							format: true,
							mountpoint: '/boot/efi',
							flags: ['boot', 'esp']
						};

						highestNumber += 1;
						newPartitionIndex += 1;
						tempModifiedPartition[index] = newEspPartition;

						newAllocated = {
							...newAllocated,
							number: newEspPartition.number + 1,
							path: `#${newPartitionIndex}`,
							size: inputtedSizeSector - espSize - 4096,
							start: newEspPartition.end + 1,
							end: newAllocated.size - espSize + newEspPartition.end - 4096,
							format: true,
							filesystem,
							mountpoint,
							label,
							flags
						};

						highestNumber += 1;
						newPartitionIndex += 1;

						tempModifiedPartition.splice(index + 1, 0, newAllocated);
						bootPartitionIndex = index;
						currentIndex += 1;

						if (partitionTable === 'gpt') {
							newGap = {
								...newAllocated,
								path: null,
								number: 0,
								size: actualSize - newAllocated.size - newEspPartition.size - 2048,
								start: newAllocated.end + 1,
								end: actualSize - newAllocated.size - newEspPartition.size + newAllocated.end - 2048,
								format: false,
								filesystem: null,
								mountpoint: null,
								label: null,
								flags: null
							};

							tempModifiedPartition.splice(index + 2, 0, newGap);
							currentIndex += 1;
						} else {
							tempModifiedPartition[index + 1] = {
								...tempModifiedPartition[index + 1],
								size: tempModifiedPartition[index + 1].size + 2048,
								end: tempModifiedPartition[index + 1].end + 2048
							};
						}
					} else {
						tempModifiedPartition[bootPartitionIndex].mountpoint = '/boot/efi';

						tempModifiedPartition[index] = {
							...newAllocated,
							number: highestNumber + 1,
							path: `#${newPartitionIndex}`,
							size: inputtedSizeSector - 4096,
							end: newAllocated.start + inputtedSizeSector - 4097,
							filesystem,
							mountpoint,
							format: true,
							label,
							flags
						};

						highestNumber += 1;

						if (partitionTable === 'gpt') {
							newGap = {
								...tempModifiedPartition[index],
								path: null,
								number: 0,
								size: actualSize - tempModifiedPartition[index].size - 2048,
								start: tempModifiedPartition[index].end + 1,
								end: actualSize - tempModifiedPartition[index].size + tempModifiedPartition[index].end - 2048,
								format: false,
								filesystem: null,
								mountpoint: null,
								label: null,
								flags: null
							};

							currentIndex += 1;
							tempModifiedPartition.splice(index + 1, 0, newGap);
						} else {
							tempModifiedPartition[index + 1] = {
								...tempModifiedPartition[index + 1],
								size: tempModifiedPartition[index + 1].size + 2048,
								end: tempModifiedPartition[index + 1].end + 2048
							};
						}
					}
				} else {
					if (bootPartitionIndex === null) {
						newEspPartition = {
							...newAllocated,
							number: newAllocated.number,
							path: `#${newPartitionIndex}`,
							size: 2048,
							end: newAllocated.start + 2048 - 1,
							filesystem: null,
							label: 'bios_grub',
							format: true,
							mountpoint: null,
							flags: ['bios_grub']
						};

						highestNumber += 1;
						newPartitionIndex += 1;
						tempModifiedPartition[index] = newEspPartition;

						newAllocated = {
							...newAllocated,
							number: newEspPartition.number + 1,
							path: `#${newPartitionIndex}`,
							size: inputtedSizeSector - 2048 - 4096,
							start: newEspPartition.end + 1,
							end: newAllocated.size - 2048 + newEspPartition.end - 4096,
							format: true,
							filesystem,
							mountpoint,
							label,
							flags
						};

						highestNumber += 1;
						newPartitionIndex += 1;

						tempModifiedPartition.splice(index + 1, 0, newAllocated);
						bootPartitionIndex = index;
						currentIndex += 1;

						if (partitionTable === 'gpt') {
							newGap = {
								...newAllocated,
								path: null,
								number: 0,
								size: actualSize - newAllocated.size - newEspPartition.size - 2048,
								start: newAllocated.end + 1,
								end: actualSize - newAllocated.size - newEspPartition.size + newAllocated.end - 2048,
								format: false,
								filesystem: null,
								mountpoint: null,
								label: null,
								flags: null
							};

							tempModifiedPartition.splice(index + 2, 0, newGap);
							currentIndex += 1;
						} else {
							tempModifiedPartition[index + 1] = {
								...tempModifiedPartition[index + 1],
								size: tempModifiedPartition[index + 1].size + 2048,
								end: tempModifiedPartition[index + 1].end + 2048
							};
						}
					}
				}
			} else {
				if (mountpoint === '/boot/efi') {
					bootPartitionIndex = index;
					if (!flags.includes('boot')) flags.push('boot');
					if (!flags.includes('esp')) flags.push('esp');
				}

				tempModifiedPartition[index] = {
					...newAllocated,
					size: inputtedSizeSector - 4096,
					end: newAllocated.start + inputtedSizeSector - 4097,
					filesystem,
					mountpoint,
					label,
					flags
				};

				highestNumber += 1;

				if (partitionTable === 'gpt') {
					newGap = {
						...tempModifiedPartition[index],
						path: null,
						number: 0,
						size: actualSize - tempModifiedPartition[index].size - 2048,
						start: tempModifiedPartition[index].end + 1,
						end: actualSize - tempModifiedPartition[index].size + tempModifiedPartition[index].end - 2048,
						format: false,
						filesystem: null,
						mountpoint: null,
						label: null,
						flags: null
					};

					tempModifiedPartition.splice(index + 1, 0, newGap);
				} else {
					tempModifiedPartition[index + 1] = {
						...tempModifiedPartition[index + 1],
						size: tempModifiedPartition[index + 1].size + 2048,
						end: tempModifiedPartition[index + 1].end + 2048
					};
				}
			}
		}

		modifiedPartition = JSON.parse(JSON.stringify(tempModifiedPartition));

		newPartition = false;
		showEdit = false;
	};

    const ignoreNonNumeric = (event) => {
        inputtedSize = event.target.value.replace(/[^0-9]/g, '');
    };

    $effect(() => {
        if (filesystem === "swap") mountpoint = "swap";
        if (filesystem === "fat32") mountpoint = "/boot/efi";
        if (filesystem === "ext4" || filesystem === "btrfs") mountpoint = null;
    })

    $effect(() => {
        if (mountpoint === "/boot/efi") filesystem = "fat32";
    })

    $effect(() => {
        if (tempModifiedPartition[index]?.filesystem !== modifiedPartition[index]?.filesystem) format = true;
    })

    $effect(() => {
        tempModifiedPartition[index].filesystem = filesystem;
        tempModifiedPartition[index].mountpoint = mountpoint;
        tempModifiedPartition[index].format = format;
        tempModifiedPartition[index].label = label;
        tempModifiedPartition[index].flags = flags;
    })

	onMount(() => {
		tempModifiedPartition = JSON.parse(JSON.stringify(modifiedPartition));

		if (newPartition && !readOnly) {
			let partitionWithTag = modifiedPartition.filter((p) =>
				p.path ? p.path.includes('#') : false
			);

			newPartitionIndex += partitionWithTag.length;

			newAllocated = {
				...modifiedPartition[index],
				format: true,
				path: `#${newPartitionIndex}`,
				number: highestNumber + 1,
				size: modifiedPartition[index].size,
				start: modifiedPartition[index].start,
				end: modifiedPartition[index].end
			};

			actualSize = modifiedPartition[index].size;
			inputtedSize = (actualSize * 512) / (1024 * 1024);
		}

		getFlagList(modifiedPartition[index].flags);
	});
</script>

<div
	class="flex flex-col w-[370px] h-[418px] p-[12px_20px] items-start flex-shrink-0 rounded-[13px] border-[1.3px] border-[#3C6350] bg-[#101010] gap-y-2 {readOnly
		? 'opacity-75 grayscale-70'
		: ''}"
>
	<!-- Partition Title -->
	<div class="w-full">
		<span class="text-[#26A767] font-['Plus_Jakarta_Sans'] text-[16px] font-bold leading-[140%]">
			{#if tempModifiedPartition[index]}
				{tempModifiedPartition[index].path
					? tempModifiedPartition[index].path.includes('#')
						? `New Partition ${tempModifiedPartition[index].path}`
						: tempModifiedPartition[index].path
					: 'Unallocated'}
			{/if}
		</span>
	</div>

	<!-- Size and Format Section -->
	<div class="flex w-full justify-between items-center">
		{#if newPartition && !readOnly}
			<div
				class="flex items-center w-full justify-between p-2 rounded-[14px] border-[1.3px] border-[#3C6350]"
			>
				<div>
					<span class="text-[#FFFEFB]">New Size</span>
				</div>

				<div class="gap-2">
					<input
						type="text"
						bind:value={inputtedSize}
						class="w-16 bg-transparent text-white focus:outline-none"
                        oninput={ignoreNonNumeric}
					/>
					<span class="text-[#FFFEFB]">MB</span>
				</div>
			</div>
		{:else}
			<div
				class="flex items-center w-[150px] justify-between p-2 rounded-[14px] border-[1.3px] border-[#3C6350]"
			>
				<span class="text-[#FFFEFB]">Size</span>
				<span class="text-[#FFFEFB]">
					{tempModifiedPartition[index] ? prettySize(tempModifiedPartition[index].size) : ''}
				</span>
			</div>
		{/if}

		<!-- Format Options -->
		{#if !newPartition}
			{#if !readOnly}
				<div class="flex flex-col space-y-2">
					<div class="flex items-center space-x-2">
						<div
							class="h-4 w-4 border border-[#3C6350] rounded-full flex items-center justify-center"
						>
							<input
								type="radio"
								value={false}
								bind:group={format}
								class="absolute opacity-0 h-4 w-4 cursor-pointer"
							/>
							{#if format === false}
								<div class="h-2 w-2 bg-[#3C6350] rounded-full"></div>
							{/if}
						</div>
						<span class="text-[#FFFEFB] text-sm">Keep data</span>
					</div>
					<div class="flex items-center space-x-2">
						<div
							class="h-4 w-4 border border-[#3C6350] rounded-full flex items-center justify-center"
						>
							<input
								type="radio"
								value={true}
								bind:group={format}
								class="absolute opacity-0 h-4 w-4 cursor-pointer"
							/>
							{#if format === true}
								<div class="h-2 w-2 bg-[#3C6350] rounded-full"></div>
							{/if}
						</div>
						<span class="text-[#FFFEFB] text-sm">Erase data</span>
					</div>
				</div>
			{:else}
				<div class="flex flex-col space-y-2">
					<div class="flex items-center space-x-2">
						<div
							class="h-4 w-4 border border-[#3C6350] rounded-full flex items-center justify-center"
						>
							{#if format === false}
								<div class="h-2 w-2 bg-[#3C6350] rounded-full"></div>
							{/if}
						</div>
						<span class="text-[#FFFEFB] text-sm">Keep data</span>
					</div>
					<div class="flex items-center space-x-2">
						<div
							class="h-4 w-4 border border-[#3C6350] rounded-full flex items-center justify-center"
						>
							{#if format === true}
								<div class="h-2 w-2 bg-[#3C6350] rounded-full"></div>
							{/if}
						</div>
						<span class="text-[#FFFEFB] text-sm">Erase data</span>
					</div>
				</div>
			{/if}
		{/if}
	</div>

	<!-- Filesystem and Mountpoint -->
	<div class="grid grid-cols-2 w-full gap-4">
		<div class="flex flex-col">
			<span class="text-[#FFFEFB] mb-1">Filesystem</span>
			{#if !readOnly}
				<ComponentSelect
					options={[
						{ value: 'btrfs', name: 'btrfs' },
						{ value: 'fat32', name: 'fat32' },
						{ value: 'ext4', name: 'ext4' },
						{ value: 'swap', name: 'swap' }
					]}
					bind:value={filesystem}
					displayField="name"
					width="100%"
				/>
			{:else}
				<div class="bg-[#101010] text-[#FFFEFB] border-[1.3px] border-[#3C6350] rounded-[14px] p-2">
					{filesystem || 'None'}
				</div>
			{/if}
		</div>
		<div class="flex flex-col">
			<span class="text-[#FFFEFB] mb-1">Mountpoint</span>
			{#if !readOnly}
				{#if filesystem === 'swap'}
					<div class="bg-[#101010] text-[#FFFEFB] border-[1.3px] border-[#3C6350] rounded-[14px] p-2 opacity-50">
						 [SWAP]
					</div>
				{:else}
					<ComponentSelect
						options={[
							{ value: null, name: 'None' },
							{ value: '/', name: '/' },
							{ value: '/boot/efi', name: '/boot/efi' },
							{ value: '/home', name: '/home' }
						]}
						bind:value={mountpoint}
						displayField="name"
						width="100%"
					/>
				{/if}
			{:else}
				<div
					class="bg-[#101010] text-[#FFFEFB] border-[1.3px] border-[#3C6350] rounded-[14px] p-2 min-h-[46px]"
				>
					{mountpoint || null}
				</div>
			{/if}
		</div>
	</div>

	<!-- Label -->

	<div class="flex flex-col w-full">
		<span class="text-[#FFFEFB] mb-1">Label</span>
		{#if !readOnly}
			<input
				type="text"
				bind:value={label}
				oninput={(e) => {
					if (!e.target.value.length) label = null;
				}}
				class="w-full bg-[#101010] text-[#FFFEFB] border-[1.3px] border-[#3C6350] rounded-[14px] p-2 focus:outline-none"
			/>
		{:else}
			<div
				class="w-full bg-[#101010] text-[#FFFEFB] border-[1.3px] border-[#3C6350] rounded-[14px] p-2"
			>
				{label || 'None'}
			</div>
		{/if}
	</div>

	<!-- Flags ini kang -->
	<div class="w-full">
		<span class="text-[#FFFEFB] mb-1">Flags</span>
		<div class="grid grid-cols-3 gap-2">
			{#each flagList as flag}
				<div class="flex items-center space-x-2">
					{#if !readOnly}
						{#key flags}
							<div class="h-4 w-4 border border-[#3C6350] rounded flex items-center justify-center">
								<input
									type="checkbox"
									id={flag}
									checked={flags.includes(flag)}
									onchange={(e) => {
										const checked = e.target.checked;

										flags = checked ? [...flags, flag] : flags.filter((f) => f !== flag);

										getFlagList(flags);
									}}
									class="absolute opacity-0 h-4 w-4 cursor-pointer"
								/>
								{#if flags.includes(flag)}
									<div class="h-2 w-2 bg-[#3C6350] rounded-sm"></div>
								{/if}
							</div>
						{/key}
					{:else}
						<div class="h-4 w-4 border border-[#3C6350] rounded flex items-center justify-center">
							{#if flags.includes(flag)}
								<div class="h-2 w-2 bg-[#3C6350] rounded-sm"></div>
							{/if}
						</div>
					{/if}
					<label for={flag} class="text-[#FFFEFB] text-sm cursor-pointer">{flag}</label>
				</div>
			{/each}
		</div>
	</div>

	<!-- Buttons -->
	{#if !readOnly}
		<div class="flex w-full justify-end space-x-2 mt-4">
			{#if newPartition}
				<button
					onclick={cancelModifiedPartition}
					class="px-4 py-2 rounded text-[#FF453A] border border-[#3C6350] hover:bg-[#1a1a1a] active:shadow-[0_0_7.167px_rgba(38,167,104,0.8)]"
				>
					Cancel
				</button>
				<button
					onclick={createPartition}
					class="px-4 py-2 rounded text-[#26A768] border border-[#3C6350] hover:bg-[#1a1a1a] active:shadow-[0_0_7.167px_rgba(38,167,104,0.8)]"
				>
					Create
				</button>
			{:else}
				<button
					onclick={cancelModifiedPartition}
					class="px-4 py-2 rounded text-[#FF453A] border border-[#3C6350] hover:bg-[#1a1a1a] active:shadow-[0_0_7.167px_rgba(38,167,104,0.8)] disabled:opacity-50"
				>
					Cancel
				</button>
				<button
					onclick={applyModifiedPartition}
					disabled={isArrayIdentical(tempModifiedPartition, modifiedPartition)}
					class="px-4 py-2 rounded text-[#26A768] border border-[#3C6350] hover:bg-[#1a1a1a] active:shadow-[0_0_7.167px_rgba(38,167,104,0.8)] disabled:opacity-50"
				>
					Apply
				</button>
			{/if}
		</div>
	{/if}
</div>
