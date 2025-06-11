<script>
	import { onMount } from 'svelte';
	import randomColor from 'randomcolor';
	import prettyBytes from 'pretty-bytes';
	import { prettySize } from '../essentials';
	import { parse } from 'svelte/compiler';

	let { disk, colors, showMountpoint = true, showLabel = false } = $props();

	let noPartitionTable = $state(true);

	const getColors = (disk) => {
		let generated_colors = [];

		if (disk?.partitions ?? false) {
			let length = disk.partitions.length;

			for (let i = 0; i < length; i++) {
				generated_colors.push(
					randomColor({
						luminosity: 'bright',
						hue: 'random'
					})
				);
			}

			return generated_colors;
		}

		return ['#454545'];
	};

	onMount(() => {
		noPartitionTable = disk?.label ? false : true ?? true;
		colors = colors ? colors : getColors(disk);
		console.log('DiskPreview mounted with disk:', disk);
	});
</script>

{#if colors}
	<!-- color bar -->
    <div class="grid place-items-center mb-2 h-5 w-full overflow-hidden relative">
        <span
            class="absolute right-0 bg-[#303030]/50 text-center px-2 font-jakarta text-xs font-[600] h-full flex items-center justify-center"
        >
            {prettySize(disk.size.slice(0, -1))}
        </span>
		<div class="h-full flex overflow-hidden w-full rounded-xs">
			{#key noPartitionTable}
				{#if noPartitionTable || !disk}
					<div style="width: 100%; background-color: #525252" class="h-full"></div>
				{:else}
					{#each disk.partitions as partition, i}
						{@const diskSize = disk.size.slice(0, -1)}
						{@const partitionSize = partition.size.slice(0, -1)}
						{@const percentage = (partitionSize / diskSize) * 100}
						{@const path =
							partition.partitionPath == null ? 'Unallocated' : partition.partitionPath.slice(5)}
						{@const filesystem =
							partition.filesystem == null
								? path == 'Unallocated'
									? 'Unallocated'
									: 'Unknown'
								: partition.filesystem}
						{@const color = filesystem ? colors[i] : '#525252'}

						<div style="width: {percentage}%; background-color: {color}" class="h-full"></div>
					{/each}
				{/if}
			{/key}
		</div>
	</div>

	<!-- information -->
	<div class="flex flex-wrap gap-y-2 max-h-[35px] overflow-y-auto mb-4 w-fit">
		{#key noPartitionTable}
			{#if noPartitionTable || !disk}
				<div class="flex items-start pr-2 gap-x-[2px]">
					<div style="background-color: #545454" class="w-2 h-2 rounded-full mt-1"></div>
					<div class="flex flex-col text-[11px] font-jakarta">
						<span class="pl-1 font-semibold tracking-wide"> </span>
						{#if disk}
							<span class="pl-1 uppercase whitespace-nowrap"
								>{prettySize(Number(disk?.size.slice(0, -1) ?? 0))} Unallocated</span
							>
						{:else}
							<span class="pl-1 uppercase whitespace-nowrap">No Partition</span>
						{/if}
					</div>
				</div>
			{:else}
				{#each disk.partitions as partition, i}
					{@const color = colors[i]}
					{@const size = prettySize(partition?.size.slice(0, -1))}
					{@const path =
						partition.partitionPath == null ? 'Unallocated' : partition.partitionPath.slice(5)}
					{@const filesystem =
						partition.filesystem == null
							? path == 'Unallocated'
								? 'Unallocated'
								: 'Unknown'
							: partition.filesystem}
					<div class="flex items-start pr-2 gap-x-[2px]">
						<div style="background-color: {color}" class="w-2 h-2 rounded-full mt-1"></div>
						<div class="flex flex-col text-[11px] font-jakarta">
							<span class="pl-1 font-semibold tracking-wide"
								>{path}
								{partition.mountpoint && showMountpoint ? `- ${partition.mountpoint}` : ''}
								{partition.name && showLabel ? `- ${partition.name}` : ''}
							</span>
							<span class="pl-1 uppercase whitespace-nowrap">{size} {filesystem}</span>
						</div>
					</div>
				{/each}
			{/if}
		{/key}
	</div>
{/if}
