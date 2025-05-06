<script>
    import { prettySize } from '$lib/essentials.js';
    import { onMount } from 'svelte';
    import SelectComponent from '../SelectComponent.svelte';

    let {
        showEdit = $bindable(),
        modifiedPartition = $bindable(),
        tempModifiedPartition = $bindable(),
        selectedPartition = $bindable(),
        newPartition = $bindable(),
        storage = $bindable(),
        diskSize = $bindable(),
        diskPath = $bindable(),
        newPartitionIndex = $bindable(),
        readOnly = false
    } = $props();

    let index = selectedPartition;

    let inputtedSize = $state(0);
    let actualSize = $state(0);

    const getSectorFromMB = (size) => {
        return Math.floor(( Number(size) * 1024 * 1024 ) / 512);
    }

    const getFlagList = (existFlags) => {
        let flags = existFlags ? existFlags : [];
        
        let flagList = [
            'hidden',
            'boot',
            'efi',
            'esp',
            'bios_grub'
        ];

        flagList = flagList.filter(list => !flags.includes(list));

        return flags.concat(flagList);
    }

    const isArrayIdentical = (a, b) => {
        return JSON.stringify(a) === JSON.stringify(b);
    }

    const applyModifiedPartition = () => {
        modifiedPartition = [];
        modifiedPartition = JSON.parse(JSON.stringify(tempModifiedPartition));
        showEdit = false;
    }

    const cancelModifiedPartition = () => {
        tempModifiedPartition = [];
        tempModifiedPartition = JSON.parse(JSON.stringify(modifiedPartition));

        if (newPartition) {
            newPartitionIndex -= 1;
        }

        showEdit = false;
    }

    const createPartition = () => {
        modifiedPartition = [];

        let lastSize = Number(tempModifiedPartition[index].size);
        let lastStart = Number(tempModifiedPartition[index].start);

        let inputtedSizeSector = getSectorFromMB(inputtedSize);

        let newSize = lastSize - inputtedSizeSector;

        let end = Number(lastStart + inputtedSizeSector + newSize);

        if (newSize >= 0) {
            if (newSize !== 0) {
                let newUnallocated = {
                    number: Number(tempModifiedPartition[index].number) + 1,
                    diskPath,
                    path: null,
                    size: newSize,
                    start: Number(lastStart + inputtedSizeSector),
                    end,
                    filesystem: null,
                    label: null,
                    format: false,
                    mountpoint: null,
                    label: null,
                    flags: []
                };

                tempModifiedPartition.splice(index + 1, 0, newUnallocated);
                tempModifiedPartition[index + 1].size -= 511;
                tempModifiedPartition[index + 1].end -= 512;
            }

            tempModifiedPartition[index].size = inputtedSizeSector;
            tempModifiedPartition[index].end = lastStart + inputtedSizeSector - 1;

            modifiedPartition = JSON.parse(JSON.stringify(tempModifiedPartition));
        }

        newPartition = false;
        showEdit = false;
    }

    onMount(() => {
        tempModifiedPartition = JSON.parse(JSON.stringify(modifiedPartition));

        if (newPartition && !readOnly) {
            let partitionWithTag = modifiedPartition.filter(p => p.path ? p.path.includes("#") : false);
            let number = partitionWithTag.map(p => Number(p.path.replace("#", "")));
            
            let highestIndex = number.length
                ? Math.max(...number)
                : 0;

            newPartitionIndex = highestIndex + 1;

            tempModifiedPartition[index] = {
                ...modifiedPartition[index],
                path: `#${newPartitionIndex}`,
                format: true,
                filesystem: "ext4",
                label: null,
                flags: []
            };

            actualSize = modifiedPartition[index].size;
            inputtedSize = ( actualSize * 512 ) / ( 1024 * 1024 );
        }
    })
</script>

<div class="flex flex-col w-[370px] h-[418px] p-[12px_20px] justify-between items-start flex-shrink-0 rounded-[13px] border-[1.3px] border-[#3C6350] bg-[#101010] space-y-2 {readOnly ? 'opacity-75' : ''}">
    <!-- Partition Title -->
    <div class="w-full">
        <span class="text-[#26A767] font-['Plus_Jakarta_Sans'] text-[16px] font-bold leading-[140%]">
            {tempModifiedPartition[index].path
                ? tempModifiedPartition[index].path.includes("#")
                    ? `New Partition ${tempModifiedPartition[index].path}`
                    : tempModifiedPartition[index].path
                : tempModifiedPartition.path
            }
        </span>
    </div>

    <!-- Size and Format Section -->
    <div class="flex w-full justify-between items-center">
        <!-- Size Box -->
        <div class="flex items-center w-[157px] gap-2 p-2 rounded-[14px] border-[1.3px] border-[#3C6350]">
            {#if newPartition && !readOnly}
                <span class="text-[#FFFEFB]">New Size</span>
                <input type="number" bind:value={inputtedSize} class="w-16 bg-transparent text-white focus:outline-none" />
                <span class="text-[#FFFEFB]">MB</span>
            {:else}
                <span class="text-[#FFFEFB]">Size</span>
                <span class="text-[#FFFEFB]">
                    {prettySize(tempModifiedPartition[index].size)}
                </span>
            {/if}
        </div>

        <!-- Format Options -->
        {#if !newPartition && !readOnly}
        <div class="flex flex-col space-y-2">
            <div class="flex items-center space-x-2">
                <input type="radio" value={false} bind:group={tempModifiedPartition[index].format} 
                       class="text-[#3C6350] focus:ring-[#3C6350] cursor-pointer" />
                <span class="text-[#FFFEFB] text-sm">Keep data</span>
            </div>
            <div class="flex items-center space-x-2">
                <input type="radio" value={true} bind:group={tempModifiedPartition[index].format} 
                       class="text-[#3C6350] focus:ring-[#3C6350] cursor-pointer" />
                <span class="text-[#FFFEFB] text-sm">Erase data</span>
            </div>
        </div>
        {/if}
    </div>

    <!-- Filesystem and Mountpoint -->
    <div class="grid grid-cols-2 w-full gap-4">
        <div class="flex flex-col">
            <span class="text-[#FFFEFB] mb-1">Filesystem</span>
            {#if !readOnly}
                <select bind:value={tempModifiedPartition[index].filesystem}
                        class="bg-[#101010] text-[#FFFEFB] border-[1.3px] border-[#3C6350] rounded-[8px] p-1 focus:outline-none">
                    <option value="btrfs">btrfs</option>
                    <option value="fat32">fat32</option>
                    <option value="ext4">ext4</option>
                </select>
            {:else}
                <div class="bg-[#101010] text-[#FFFEFB] border-[1.3px] border-[#3C6350] rounded-[8px] p-1">
                    {tempModifiedPartition[index].filesystem || 'None'}
                </div>
            {/if}
        </div>
        <div class="flex flex-col">
            <span class="text-[#FFFEFB] mb-1">Mountpoint</span>
            {#if !readOnly}
                <select bind:value={tempModifiedPartition[index].mountpoint}
                        class="bg-[#101010] text-[#FFFEFB] border-[1.3px] border-[#3C6350] rounded-[8px] p-1 focus:outline-none">
                    <option value={null}>None</option>
                    <option value="/">/</option>
                    <option value="/boot/efi">/boot/efi</option>
                    <option value="/home">/home</option>
                </select>
            {:else}
                <div class="bg-[#101010] text-[#FFFEFB] border-[1.3px] border-[#3C6350] rounded-[8px] p-1">
                    {tempModifiedPartition[index].mountpoint || 'None'}
                </div>
            {/if}
        </div>
    </div>

    <!-- Label -->
    <div class="w-full">
        <span class="text-[#FFFEFB] mb-1">Label</span>
        {#if !readOnly}
            <input type="text" bind:value={tempModifiedPartition[index].label}
                   class="w-full bg-[#101010] text-[#FFFEFB] border-[1.3px] border-[#3C6350] rounded-[8px] p-1 focus:outline-none" />
        {:else}
            <div class="w-full bg-[#101010] text-[#FFFEFB] border-[1.3px] border-[#3C6350] rounded-[8px] p-1">
                {tempModifiedPartition[index].label || 'None'}
            </div>
        {/if}
    </div>

    <!-- Flags -->
    <div class="w-full">
        <span class="text-[#FFFEFB] mb-1">Flags</span>
        <div class="grid grid-cols-3 gap-2">
            {#each getFlagList(tempModifiedPartition[index].flags) as flag}
                <div class="flex items-center space-x-2">
                    {#if !readOnly}
                        <input type="checkbox" id={flag}
                               checked={tempModifiedPartition[index].flags.includes(flag)}
                               onchange={(e) => {
                                   const checked = e.target.checked;
                                   const flags = tempModifiedPartition[index].flags;
                                   tempModifiedPartition[index].flags = checked
                                       ? [...flags, flag]
                                       : flags.filter(f => f !== flag);
                               }}
                               class="h-4 w-4 text-[#3C6350] focus:ring-[#3C6350] border-[#3C6350] rounded" />
                    {:else}
                        <div class="h-4 w-4 border border-[#3C6350] rounded flex items-center justify-center">
                            {#if tempModifiedPartition[index].flags.includes(flag)}
                                <div class="h-2 w-2 bg-[#3C6350] rounded-sm"></div>
                            {/if}
                        </div>
                    {/if}
                    <label for={flag} class="text-[#FFFEFB] text-sm">{flag}</label>
                </div>
            {/each}
        </div>
    </div>

    <!-- Buttons -->
    {#if !readOnly}
        <div class="flex w-full justify-end space-x-2 mt-4">
            {#if newPartition}
                <button onclick={cancelModifiedPartition} 
                        class="px-4 py-2 rounded text-[#FF453A] border border-[#3C6350] hover:bg-[#1a1a1a] active:shadow-[0_0_7.167px_rgba(38,167,104,0.8)]">
                    Cancel
                </button>
                <button onclick={createPartition} 
                        class="px-4 py-2 rounded text-[#26A768] border border-[#3C6350] hover:bg-[#1a1a1a] active:shadow-[0_0_7.167px_rgba(38,167,104,0.8)]">
                    Create
                </button>
            {:else}
                <button onclick={cancelModifiedPartition} 
                        disabled={isArrayIdentical(tempModifiedPartition, modifiedPartition)}
                        class="px-4 py-2 rounded text-[#FF453A] border border-[#3C6350] hover:bg-[#1a1a1a] active:shadow-[0_0_7.167px_rgba(38,167,104,0.8)] disabled:opacity-50">
                    Cancel
                </button>
                <button onclick={applyModifiedPartition} 
                        disabled={isArrayIdentical(tempModifiedPartition, modifiedPartition)}
                        class="px-4 py-2 rounded text-[#26A768] border border-[#3C6350] hover:bg-[#1a1a1a] active:shadow-[0_0_7.167px_rgba(38,167,104,0.8)] disabled:opacity-50">
                    Apply
                </button>
            {/if}
        </div>
    {/if}
</div>