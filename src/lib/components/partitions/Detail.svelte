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
        newPartitionIndex = $bindable()
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

                // if (diskSize === end) {
                //     end -= 1
                // } else {
                //     end -= 512;
                //     newSize -= 511;
                // }

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

            // tempModifiedPartition[index].size = inputtedSizeSector - 511;
            // tempModifiedPartition[index].end = lastStart + inputtedSizeSector - 512;

            tempModifiedPartition[index].size = inputtedSizeSector;
            tempModifiedPartition[index].end = lastStart + inputtedSizeSector - 1;

            // tempModifiedPartition[index + 1].size -= 511;
            // tempModifiedPartition[index + 1].end -= 512;

            modifiedPartition = JSON.parse(JSON.stringify(tempModifiedPartition));
        }

        newPartition = false;
        showEdit = false;
    }

    const getNewPath = () => {

        let newPath;
        let newNumber;
        
        if (modifiedPartition.length === 1) {

            if (!modifiedPartition[0].path) {
                newPath = `${diskPath}1`
            }

        } else {

            let partition = modifiedPartition
                .map(p => p.path)
                .filter(p => p !== null);

            let partitionNumber = partition
                .map(p => Number(p.slice(-1)));

            newNumber = Math.max(...partitionNumber) + 1;
            newPath = `${diskPath}${newNumber}`;
        }


        newPartitionIndex = `${newNumber}`;
        return newPath;
    }

    const getNewPartitionNumber = () => {

        let allocatedPartition = modifiedPartition.filter(p => p.path);

        return allocatedPartition.length + 1;
    }

    onMount(() => {
        tempModifiedPartition = JSON.parse(JSON.stringify(modifiedPartition));

        if (newPartition) {

            let partitionWithTag = modifiedPartition.filter(p => p.path ? p.path.includes("#") : false);

            let number = partitionWithTag.map(p => Number(p.path.replace("#", "")));
            
            let highestIndex = number.length
                ? Math.max(...number)
                : 0;

            console.log('highest index', highestIndex);

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

<div class="flex flex-col w-[370px] h-[418px] p-[12px_20px] justify-between items-center flex-shrink-0 rounded-[13px] border-[1.3px] border-[#3C6350] bg-[#101010]">
    <div>
        <span>{
            tempModifiedPartition[index].path
                ? tempModifiedPartition[index].path.includes("#")
                    ? `New Partition ${tempModifiedPartition[index].path}`
                    : tempModifiedPartition[index].path
                : tempModifiedPartition.path
        }</span>
    </div>

    <div class="flex flex-row space-x-2 justify-between">
        <div class="border-4 border-black rounded-lg flex items-center p-4 space-x-2">
            {#if newPartition}
                <span>New Size</span>
                <input type="number" bind:value={inputtedSize} />
                <span>MB</span>
            {:else}
                <span>Size</span>
                <span>
                    {prettySize(tempModifiedPartition[index].size)}
                </span>
            {/if}
        </div>
        <div class="flex flex-col items-start justify-center">
            {#if !newPartition}
                <div class="flex flex-row items-center justify-center space-x-2">
                    <input type="radio" value={false} bind:group={tempModifiedPartition[index].format} />
                    <span>Keep data</span>
                </div>
                <div class="flex flex-row items-center justify-center space-x-2">
                    <input type="radio" value={true} bind:group={tempModifiedPartition[index].format} />
                    <span>Erase data</span>
                </div>
            {/if}
        </div>
    </div>
    <div class="flex flex-row space-x-2 justify-between text-black">
        <div class="flex flex-col">
            <span>Filesystem</span>
            <select bind:value={tempModifiedPartition[index].filesystem}>
                <option value="btrfs">btrfs</option>
                <option value="fat32">fat32</option>
                <option value="ext4">ext4</option>
            </select>
        </div>
        <div class="flex flex-col">
            <span>Mountpoint</span>
            <select bind:value={tempModifiedPartition[index].mountpoint}>
                <option value={null}>None</option>
                <option value="/">/</option>
                <option value="/boot/efi">/boot/efi</option>
                <option value="/home">/home</option>
            </select>
        </div>
    </div>
    <div class="flex flex-col">
        <span>Label</span>
        <input type="text" class="border-2" bind:value={tempModifiedPartition[index].label} />
    </div>
    <div class="flex flex-col">
        <span>Flags</span>
        <div class="flex flex-row justify-start items-center space-x-2">
            {#each getFlagList(tempModifiedPartition[index].flags) as flag}
                {@const existFlag = tempModifiedPartition[index].flags}
                <input
                    type="checkbox"
                    id={flag}
                    checked={tempModifiedPartition[index].flags.includes(flag)}
                    onchange={(event) => {
                        let checked = event.target.checked;

                        if (checked) {
                            if (!existFlag.includes(flag)) {
                                tempModifiedPartition[index].flags.push(flag)
                            }
                        } else {
                            if (existFlag.includes(flag)) {
                                tempModifiedPartition[index].flags = existFlag.filter(f => f !== flag);
                            }
                        }
                    }}
                />
                <label for={flag}>{flag}</label>
            {/each}
        </div>
    </div>
    <div class="flex flex-col">
        {#if newPartition}
            <button onclick={cancelModifiedPartition} disabled={false} class="bg-red-500 p-4 disabled:bg-red-900">Cancel</button>
            <button onclick={createPartition} disabled={false} class="bg-green-500 p-4 disabled:bg-green-900">Create</button>
        {:else}
            <button onclick={cancelModifiedPartition} disabled={isArrayIdentical(tempModifiedPartition, modifiedPartition)} class="bg-red-500 p-4 disabled:bg-red-900">Cancel</button>
            <button onclick={applyModifiedPartition} disabled={isArrayIdentical(tempModifiedPartition, modifiedPartition)} class="bg-green-500 p-4 disabled:bg-green-900">Apply</button>
        {/if}
    </div>
</div>
