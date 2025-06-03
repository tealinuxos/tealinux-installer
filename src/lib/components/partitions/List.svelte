<script>
	import { prettySize } from '$lib/essentials.js';

	export let selectedPartition;
	export let modifiedPartition;
	export let showEdit;
	export let newPartition;
	export let tempModifiedPartition;
	export let highestNumber;
	export let bootPartitionIndex;

	const changeSelectedPartition = async (selected) => {
		selectedPartition = selected;
		showEdit = false;
		newPartition = false;
	};

	const isUnallocated = (partition) => {
		return !partition?.path && !partition?.filesystem;
	};

	const showCreateDetail = () => {
		newPartition = true;
		showEdit = true;
	};

	const removePartition = () => {
		if (
			selectedPartition === null ||
			!modifiedPartition ||
			selectedPartition >= modifiedPartition.length
		)
			return;

		if (modifiedPartition[selectedPartition].path) highestNumber -= 1;

		if (
			(modifiedPartition[selectedPartition].flags &&
				modifiedPartition[selectedPartition].flags.includes('esp')) ||
			(modifiedPartition[selectedPartition].flags &&
				modifiedPartition[selectedPartition].flags.includes('bios_grub'))
		) {
			bootPartitionIndex = null;
		}
        
        let wasUnallocated = modifiedPartition[selectedPartition].path.includes("#");
        let highestUnallocated = Math.max(...modifiedPartition.map(p => p.path?.includes("#") ?? false ? Number(p.path.slice(1)) : 0));

        if (wasUnallocated) {
            modifiedPartition = modifiedPartition.map(partition => {
                if (partition.path?.includes('#')) {
                    if (partition.path && partition.path.includes('#')) {
                        const num = parseInt(partition.path.replace('#', ''));
                        if (num > 1 && num !== highestUnallocated - 1) {
                            return { ...partition, path: `#${num - 1}` };
                        }
                    }
                }
                return partition;
            });
        }

        modifiedPartition[selectedPartition] = {
            ...modifiedPartition[selectedPartition],
            number: 0,
            path: null,
            filesystem: null,
            format: false,
            mountpoint: null,
            label: null,
            flags: []
        }

		tempModifiedPartition = modifiedPartition;
		showEdit = false;
		newPartition = false;

		if (selectedPartition === tempModifiedPartition.length) selectedPartition -= 1;
	};

	const editPartition = () => {
		if (
			selectedPartition === null ||
			!modifiedPartition ||
			selectedPartition >= modifiedPartition.length
		)
			return;
		showEdit = true;
		newPartition = false;
	};
</script>

<div
	class="flex flex-col justify-between w-[1050px] max-h-[418px] rounded-[13px] border-[1.3px] border-[#3C6350] bg-[#101010] p-4"
>
	<div class="w-full overflow-y-auto block h-[350px]">
		<table class="w-full">
			<thead class="text-[#FFFEFB] font-['Poppins'] text-[14px] sticky top-0 bg-[#101010]">
				<tr class="border-b border-[#3C6350]">
					<th class="p-3 text-left">Device</th>
					<th class="p-3 text-left">Size</th>
					<th class="p-3 text-left">Format</th>
					<th class="p-3 text-left">Type</th>
					<th class="p-3 text-left">MountPoint</th>
					
				</tr>
			</thead>
			<tbody class="text-[#FFFEFB] font-['Poppins'] text-[14px] overflow-y-auto">
				{#if modifiedPartition}
					{#each modifiedPartition as p, num}
						<tr
							on:click={() => changeSelectedPartition(num)}
							class="border-b border-[#3C6350] {num === selectedPartition
								? 'bg-[rgba(38,167,104,0.2)]'
								: 'bg-[#101010]'}"
							style="cursor: default;"
						>
							<td class="p-3">
								{#if p.path}
									{#if p.path.includes('#')}
										New Partition {p.path}
									{:else}
										{p.path}
									{/if}
								{:else}
									Unallocated
								{/if}
							</td>
							<td class="p-3">{p?.size ? prettySize(p.size) : ''}</td>
							<td class="p-3">
								{#if p.format}
									<div class="relative inline-block group">
										<span>Yes</span>
										<svg
											class="w-4 h-4 ml-1 inline-block text-[#ffcf30]"
											viewBox="0 0 24 24"
											fill="currentColor"
										>
											<path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z" />
										</svg>
										<div class="absolute z-10 hidden group-hover:block bg-[#ffcf30] text-black font-bold text-xs rounded p-2 whitespace-nowrap bottom-full left-1/2 transform -translate-x-1/2 mb-1">
										    The data on this partition will be erased!
										</div>
									</div>
								{:else}
									No
								{/if}
							</td>
							<td class="p-3">
								{#if p.filesystem}
									{p.filesystem}
								{:else if p.path}
									Unknown
								{:else}
									Unallocated
								{/if}
							</td>
							<td class="p-3">{p.mountpoint || ''}</td>
						</tr>
					{/each}
				{/if}
			</tbody>
		</table>
	</div>

	<div class="flex justify-between mt-4">
		<button
			class="flex h-8 px-[9px] items-center justify-center gap-[10px] rounded-[4px] border-[0.3px] border-[#3C6350] bg-[#101010] text-white font-['Poppins'] text-[14px] transition-all duration-200 hover:shadow-[0_0_9px_#00B85E] active:shadow-[0_0_9px_#00B85E] disabled:opacity-50 disabled:hover:shadow-none"
			on:click={showCreateDetail}
			disabled={!modifiedPartition || 
				modifiedPartition[selectedPartition]?.path !== null ||
				modifiedPartition[selectedPartition]?.size <= 4096}
		>
			+ Add
		</button>
		<div class="flex gap-3">
			<button
				class="flex h-8 px-[9px] items-center justify-center gap-[10px] rounded-[4px] border-[0.3px] border-[#3C6350] bg-[#101010] text-white font-['Poppins'] text-[14px] transition-all duration-200 hover:shadow-[0_0_9px_#00B85E] active:shadow-[0_0_9px_#00B85E] disabled:opacity-50 disabled:hover:shadow-none"
				on:click={editPartition}
				disabled={selectedPartition === null ||
					isUnallocated(modifiedPartition?.[selectedPartition])}
			>
				Edit
			</button>
			<button
				class="flex h-8 px-[9px] items-center justify-center gap-[10px] rounded-[4px] border-[0.3px] border-[#633C3C] bg-[#101010] text-white font-['Poppins'] text-[14px] transition-all duration-200 hover:shadow-[0_0_9px_#FF453A] active:shadow-[0_0_9px_#FF453A] disabled:opacity-50 disabled:hover:shadow-none"
				on:click={removePartition}
				disabled={selectedPartition === null ||
					!modifiedPartition?.[selectedPartition] ||
					!modifiedPartition?.[selectedPartition].path}
			>
				Delete
			</button>
		</div>
	</div>
</div>
