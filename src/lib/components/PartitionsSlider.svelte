<script>
	import prettyBytes from 'pretty-bytes';
	import GlowingText from './ui/GlowingText.svelte';
	import DiskPreview from './DiskPreview.svelte';
	import randomColor from 'randomcolor';
	import { onMount } from 'svelte';

	let {
		title,
		partitions = [],
		disk,
		selectedPartition = $bindable(),
		partitionTable,
		disableNext = $bindable(),
        disabled = true
	} = $props();

	let colors = $state([]);
	let currentSlide = $state(0);
	let message = $state('');

	function prevSlide() {
		currentSlide = (currentSlide - 1 + partitions.length) % partitions.length;
	}

	function nextSlide() {
		currentSlide = (currentSlide + 1) % partitions.length;
	}

	const getColors = (disk) => {
		let generated_colors = [];
		let length = disk.partitions?.length ?? 0;

		for (let i = 0; i < length; i++) {
			generated_colors.push(
				randomColor({
					luminosity: 'bright',
					hue: 'random'
				})
			);
		}

		return generated_colors;
	};

	const changeSelectedPartition = (index) => {
		selectedPartition = disk.partitions[index];
		currentSlide = index;
	};

	$effect(() => {
		if (selectedPartition) {
			if (Number(selectedPartition.size.slice(0, -1)) < 41943040) {
				message = 'The selected partition does not meet minimum size requirement of 20GB';
				disableNext = true;
			} else {
				message = 'TealinuxOS can be installed on this partition';
				disableNext = false;
			}
		}
	});

	$effect(() => {
		if (disk.partitions) {
			selectedPartition = disk.partitions[currentSlide];
		}
	});

	onMount(() => {
		colors = getColors(disk);
	});
</script>

<div class="bg-[#101010] border-[1.3px] border-[#3C6350] p-[15px] py-[5px] rounded-[14px]">
	{#if title}
		<GlowingText size="lg" text={title} />
	{/if}

	{#if partitions.length > 0}
		<!-- Display only the current slide -->
		{@const partition = partitions[currentSlide]}

		<div class="mb-2 mt-1 flex justify-between items-center">
			<GlowingText
				size="md"
				text={`${partition.partitionPath || 'Unallocated'} ${partition.name ? ` - ${partition.name}` : ''}`}
			/>
			<p class="text-xs font-[200] uppercase font-bold">
				{partitionTable ? partitionTable : 'Unknown Partition Table'}
			</p>
		</div>

		<div class="flex gap-x-4 items-start">
			<div class="w-full">
				<!-- color bar -->
				<div class="flex mb-2 h-5 w-full overflow-hidden">
					<div class="h-full flex overflow-hidden w-full">
						{#each partitions as partition, i}
							{@const diskSize = disk.size.slice(0, -1)}
							{@const partitionSize = partition.size.slice(0, -1)}
							{@const percentage = (partitionSize / diskSize) * 100}
							{@const color = i === currentSlide ? '#86EFAC' : colors[i]}

							<div
								style="width: {percentage}%; background-color: {color}"
								class="h-full {i === currentSlide ? 'scale-y-125 border border-black' : ''}"
							></div>
						{/each}
					</div>
				</div>
				<!-- information -->
				<div class="flex flex-wrap gap-y-2 max-h-[50px] overflow-y-auto mb-0 w-fit">
					{#each partitions as partition, i}
						{@const color = colors[i]}
						{@const prettySize = prettyBytes(parseInt(partition.size) * 512, { binary: true })}
						{@const path = partition.partitionPath == null ? 'Unallocated' : partition.partitionPath.slice(5)}
						{@const filesystem = partition.filesystem == null 
							? path == 'Unallocated' 
								? 'Unallocated' 
								: 'Unknown' 
							: partition.filesystem}

						<div
							class="flex items-start pr-1 gap-x-[2px] mx-2 p-1 rounded { disabled ? 'cursor-not-allowed' : 'cursor-pointer' }"
							style="{
								i === currentSlide 
									? 'border-radius: 9.489px; border: 1.423px solid #26A768; background: #032A17; color: white'
									: 'border-radius: 9.489px; border: 1.423px solid #3C6350; background: #101010; color: white'
							}"
							on:click={() => { if (!disabled) changeSelectedPartition(i) }}
						>
							<div
								style="background-color: {color}"
								class="w-3 h-3 rounded-full mt-1 border border-gray-600"
							></div>
							<div class="flex flex-col text-[11px] font-jakarta">
								<span class="pl-1 font-semibold tracking-wide">{path}</span>
								<span class="pl-1 uppercase whitespace-nowrap">{prettySize} {filesystem}</span>
							</div>
						</div>
					{/each}
				</div>
				{#key message}
					<div
                        class="flex items-center justify-between w-full font-semibold gap-4 pb-2
                            {disableNext ? 'text-red-500' : 'text-green-tealinux'}"
                    >
						<p class="text-sm">{message}</p>
					</div>
				{/key}
			</div>
		</div>
	{:else}
		<p>No partition data available</p>
	{/if}
</div>
