<script>
	import prettyBytes from 'pretty-bytes';
	import GlowingText from './ui/GlowingText.svelte';

	export let disks; // The disks data to display
	export let loading = false; // Loading state
	export let colors = ['#3C6350', '#5D9C7F', '#8FD3B2', '#B8E0D2']; // Default color palette

	// Function to generate colors for partitions
	function getColors(disk, index) {
		// You can customize this based on your needs
		return disk.partitions.map((_, i) => colors[i % colors.length]);
	}
</script>

<div class="bg-[#101010] border-[1.3px] border-[#3C6350] p-[15px] rounded-[14px]">
	<GlowingText size="lg" text="Disk" />

	{#if loading}
		Loading...
	{:else if disks}
		{#each disks as disk, idx}
			{@const size = parseInt(disk.size.replace('s', ' '))}
			{@const prettySize = size === 0 ? 'No disk' : prettyBytes(size * 512)}
			{@const colors = getColors(disk, idx)}

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

			<div class="flex gap-x-4 items-start">
				<div class="w-full">
					<div class="flex mb-4 h-7 w-full overflow-hidden">
						<div class="h-full flex overflow-hidden w-full">
							{#each disk.partitions as partition, i}
								{@const diskSize = disk.size.slice(0, -1)}
								{@const partitionSize = partition.size.slice(0, -1)}
								{@const percentage = (partitionSize / diskSize) * 100}

								{@const color = colors[i]}

								<div style="width: {percentage}%; background-color: {color}" class="h-full"></div>
							{/each}
						</div>
					</div>

					<div class="flex gap-y-4 flex-wrap mb-4">
						{#each disk.partitions as partition, i}
							{@const color = colors[i]}
							{@const size = partition.size.slice(0, -1) * 512}
							{@const path =
								partition.partitionPath == null ? 'Unallocated' : partition.partitionPath.slice(5)}
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
								</div>
							</div>
						{/each}
					</div>
				</div>
			</div>
		{/each}
	{:else}
		No disk data available
	{/if}
</div>
