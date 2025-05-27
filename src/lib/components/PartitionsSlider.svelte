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
        selectedPartition = $bindable()
    } = $props();

    let colors = $state([]);
	let currentSlide = $state(0); // Track the current slide index

	// Function to navigate to the previous slide
	function prevSlide() {
		currentSlide = (currentSlide - 1 + partitions.length) % partitions.length;
	}

	// Function to navigate to the next slide
	function nextSlide() {
		currentSlide = (currentSlide + 1) % partitions.length;
	}

    const getColors = (disk) => {
		let generated_colors = [];

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
	};

    const changeSelectedPartition = (index) => {
        selectedPartition = disk.partitions[index];
        currentSlide = index;
    };

    $effect(() => {
        selectedPartition = disk.partitions[currentSlide];
    })

	onMount(() => {
		colors = getColors(disk);
	});
</script>

<div class="bg-[#101010] border-[1.3px] border-[#3C6350] p-[15px] py-[10px] rounded-[14px]">
    
    {#if title}
        <GlowingText size="lg" text={title} />
    {/if}

	{#if partitions}
		<!-- Display only the current slide -->
		{@const partition = partitions[currentSlide]}

		<div class="mb-2 mt-1 flex justify-between items-center ">
			<p>
				<span class="text-sm font-[500]">{partition.partitionPath}</span>
				{' '}
				<span class="text-xs font-[400]">{partition.name ? partition.name : ''}</span>
			</p>
			<p class="text-xs font-[200]">
				<!-- {partitions.disk} -->
                GPT
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

                            <div style="width: {percentage}%; background-color: {color}"
                                class="h-full {i === currentSlide ? "scale-y-125 border border-black" : ""}
                                "></div>
                        {/each}
                    </div>
                </div>
                <!-- information -->
                <div class="flex flex-wrap gap-y-2 max-h-[35px] overflow-y-auto mb-4 w-fit">
                    {#each partitions as partition, i}
                        {@const color = colors[i]}
                        {@const prettySize = prettyBytes(parseInt(partition.size) * 512)}
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
                            <div
                                class="flex flex-col text-[11px] font-jakarta {i === currentSlide ? "bg-green-300 text-black opacity-[85%] rounded" : ""}"
                                onclick={changeSelectedPartition(i)}
                            >
                                <span class="pl-1 font-semibold tracking-wide"
                                    >{path}
                                    <!-- {(partition.mountpoint) && showMountpoint -->
                                    <!--     ? `- ${partition.mountpoint}` -->
                                    <!--     : ''} -->
                                    <!-- {(partition.name) && showLabel -->
                                    <!--     ? `- ${partition.name}` -->
                                    <!--     : ''} -->
                                </span
                                >
                                <span class="pl-1 uppercase whitespace-nowrap">{prettySize} {filesystem}</span>
                            </div>
                        </div>
                    {/each}
                </div>
				<div class="flex items-center justify-between w-full gap-4">
					<!-- Slider indicators (centered) -->
					<div class="flex gap-2 mx-auto">
						{#each partitions as _, i}
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
		No partition data available
	{/if}
</div>
