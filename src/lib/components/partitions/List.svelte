<script>
    import { prettySize } from '$lib/essentials.js';

    let {
        selectedDisk = $bindable(),
        selectedPartition = $bindable(),
        modifiedPartition = $bindable(),
        originalPartition = $bindable(),
        showEdit = $bindable(),
        newPartition = $bindable(),
        newPartitionIndex
    } = $props();

    const changeSelectedPartition = async (selected = selectedPartition) => {
        selectedPartition = selected;
        showEdit = false;
        newPartition = false;
    }
</script>

<div class="w-[1050px] rounded-[13px] border-[1.3px] border-[#3C6350] bg-[#101010] p-4">
    <table class="w-full">
        <thead class="text-[#FFFEFB] font-['Poppins'] text-[14px]">
            <tr class="border-b border-[#3C6350]">
                <th class="p-3 text-left">Device</th>
                <th class="p-3 text-left">Size</th>
                <th class="p-3 text-left">Format</th>
                <th class="p-3 text-left">Type</th>
                <th class="p-3 text-left">MountPoint</th>
            </tr>
        </thead>
        <tbody class="text-[#FFFEFB] font-['Poppins'] text-[14px]">
            {#key selectedDisk}
                {#if modifiedPartition}
                    {#each modifiedPartition as p, num}
                        <tr 
                            on:click={() => changeSelectedPartition(num)} 
                            class="cursor-pointer hover:bg-[#1E1E1E] { num === selectedPartition ? 'bg-[#26A768]' : '' }"
                        >
                            <td class="p-3">{
                                p.path 
                                    ? p.path.includes("#")
                                        ? `New Partition ${p.path}`
                                        : p.path
                                    : 'Unallocated'
                            }</td>
                            <td class="p-3">{prettySize(p.size)}</td>
                            <td class="p-3">{p.format ? 'Yes' : 'No'}</td>
                            <td class="p-3">{p.filesystem ? p.filesystem : p.path ? 'Unknown' : 'Unallocated'}</td>
                            <td class="p-3">{p.mountpoint ? p.mountpoint : ''}</td>
                        </tr>
                    {/each}
                {/if}
            {/key}
        </tbody>
    </table>
    
    <!-- <div class="flex justify-between mt-4">
        <button class="flex items-center gap-2 px-4 py-2 text-[#26A768] hover:bg-[#1a1a1a] rounded">
            <span class="text-xl">+</span>
            <span>Add</span>
        </button>
        <div class="flex gap-2">
            <button class="px-4 py-2 text-[#26A768] hover:bg-[#1a1a1a] rounded" disabled={isUnallocated(modifiedPartition[selectedPartition])}>
                Edit
            </button>
            <button class="px-4 py-2 text-[#FF453A] hover:bg-[#1a1a1a] rounded" disabled={isUnallocated(modifiedPartition[selectedPartition])}>
                Delete
            </button>
        </div>
    </div> -->
</div>