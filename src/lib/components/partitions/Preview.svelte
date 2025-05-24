<script>
	import { randomColor } from 'randomcolor';
    import { prettySize } from '$lib/essentials.js';

    let {
        modifiedPartition = $bindable(),
        diskSize = $bindable()
    } = $props();

    let partitionColors = $state([]);

	const getColors = (partitions) => {

        let colors = [];

		for (let i = 0; i < partitions.length; i++) {
			colors.push(
				randomColor({
					luminosity: 'bright',
					hue: 'random'
				})
			);
		}

        partitionColors = colors;
	};

    $effect(() => {
        diskSize; getColors(modifiedPartition)
    })

</script>

{#key diskSize}
    <div class="bg-[#101010] border-[1.3px] border-[#3C6350] p-[15px] rounded-[14px]">
        <!-- <h2 class="text-green-500 text-lg font-semibold mb-4">Partition</h2> -->
            <div class=" flex gap-x-4 items-start">
                <div class="w-full">
                    <div class="flex mb-4 h-7 w-full overflow-hidden">
                        <div class="h-full flex overflow-hidden w-full">
                            {#each modifiedPartition as partition, i}
                                {@const partitionSize = partition.size}
                                {@const percentage = (partitionSize / diskSize) * 100}

                                {@const color = partitionColors[i]}

                                <div
                                    style="width: {percentage}%; background-color: {color}"
                                    class="h-full"
                                ></div>
                            {/each}
                        </div>
                    </div>

                    <div class="flex gap-y-4 flex-wrap mb-4 text-white">
                        {#each modifiedPartition as partition, i}
                            {@const color = partitionColors[i]}
                            {@const size = partition.size}
                            {@const path =
                                partition.path == null
                                    ? 'Unallocated'
                                    : partition.path.includes("#")
                                        ? `New Partition ${partition.path}`
                                        : partition.path.slice(5)}
                            {@const filesystem =
                                partition.filesystem == null
                                    ? path == 'Unallocated'
                                        ? ''
                                        : 'Unknown'
                                    : partition.filesystem}
                            {@const label = 
                                partition.label
                                    ? ` - ${partition.label}`
                                    : ''
                            }
                            <div class="flex items-center pr-2 gap-x-[2px]">
                                <div style="background-color: {color}" class="w-2 h-2 rounded-full"></div>
                                <div class="flex flex-col text-[11px] font-jakarta">
                                    <span class="pl-1">{path} {label}</span>
                                    <span class="pl-1">{prettySize(size)} {filesystem}</span>
                                </div>
                            </div>
                        {/each}
                    </div>
                </div>
            </div>
    </div>
{/key}
