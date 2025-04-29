<script>
	import prettyBytes from 'pretty-bytes';
	import GlowingText from './ui/GlowingText.svelte';
	import DiskPreview from './DiskPreview.svelte';

	let { disks, loading, colors } = $props(); // Props passed from the parent component

	disks = disks || []; // The disks data to display
	loading = loading || false; // Loading state
	colors = colors || ['#3C6350', '#5D9C7F', '#8FD3B2', '#B8E0D2']; // Default color palette

	let currentSlide = $state(0); // Track the current slide index

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
                {#key disk}
                    <DiskPreview { disk } />
                {/key}
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
