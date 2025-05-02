<script>
	import { onMount } from "svelte";
    import randomColor from "randomcolor";

    let { disk, colors } = $props();

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

    onMount(() => {
        colors = colors ? colors : getColors(disk);
    })
</script>

{#if colors}
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
            {@const path =
                partition.partitionPath == null ? 'Unallocated' : partition.partitionPath.slice(5)}
            {@const filesystem =
                partition.filesystem == null
                    ? path == 'Unallocated'
                        ? 'Unallocated'
                        : 'Unknown'
                    : partition.filesystem}
            <div class="flex items-center pr-2 gap-x-[2px]">
                <div style="background-color: {color}" class="w-2 h-2 rounded-full"></div>
                <div class="flex flex-col text-[11px] font-jakarta">
                    <span class="pl-1">{filesystem}</span>
                </div>
            </div>
        {/each}
    </div>

{/if}
