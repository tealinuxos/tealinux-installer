<script>
	import prettyBytes from 'pretty-bytes';
	import GlowingText from './ui/GlowingText.svelte';

	let { disks, loading, colors } = $props(); // Props passed from the parent component

	disks = disks || []; // The disks data to display
	loading = loading || false; // Loading state
	colors = colors || ['#3C6350', '#5D9C7F', '#8FD3B2', '#B8E0D2']; // Default color palette

	let currentSlide = $state(0); // Track the current slide index

	// Function to generate colors for partitions
	function getColors(disk, index) {
		return disk.partitions.map((_, i) => colors[i % colors.length]);
	}

	// Function to navigate to the previous slide
	function prevSlide() {
		currentSlide = (currentSlide - 1 + disks.length) % disks.length;
	}

	// Function to navigate to the next slide
	function nextSlide() {
		currentSlide = (currentSlide + 1) % disks.length;
	}
</script>

<div class="bg-[#101010] border-[1.3px] border-[#3C6350] p-[15px] rounded-[14px]">
	<GlowingText size="lg" text="Disk" />

	{#if loading}
		Loading...
	{:else if disks}
		<!-- Display only the current slide -->
		{@const disk = disks[currentSlide]}
		{@const size = parseInt(disk.size.replace('s', ' '))}
		{@const prettySize = size === 0 ? 'No disk' : prettyBytes(size * 512)}
		{@const colors = getColors(disk, currentSlide)}

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
				<!-- color bar -->
				<div class="flex mb-2 h-5 w-full overflow-hidden">
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

				<!-- information -->
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

				<div class="flex items-center justify-between w-full gap-4">
					<!-- Slider indicators (centered) -->
					<div class="flex gap-2 mx-auto">
						{#each disks as _, i}
							<div
								class="w-1 h-1 bg-[#D9D9D9] rounded-full transition-all {i === currentSlide
									? 'bg-primary w-5'
									: ''}"
							></div>
						{/each}
					</div>

					<!-- Slider controls (right-aligned) -->
					<div class="flex items-center gap-4">
						<button
							class="cursor-pointer text-primary/80 hover:text-primary text-xl transition-colors"
							onclick={prevSlide}>&lsaquo;</button
						>
						<button
							class="cursor-pointer text-primary/80 hover:text-primary text-xl transition-colors"
							onclick={nextSlide}>&rsaquo;</button
						>
					</div>
				</div>
			</div>
		</div>
	{:else}
		No disk data available
	{/if}
</div>
