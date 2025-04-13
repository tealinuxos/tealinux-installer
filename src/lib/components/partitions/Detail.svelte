<script>

    import { prettySize } from '$lib/essentials.js';
	import { onMount } from 'svelte';

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

    let inputtedSize = $state(0);
    let actualSize = $state(0);

    $inspect(actualSize);

    $effect(() => {
        selectedPartition;
    })

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
        showEdit = false;
    }

    const createPartition = () => {
        modifiedPartition = [];

        let lastSize = Number(tempModifiedPartition[selectedPartition].size);
        let lastStart = Number(tempModifiedPartition[selectedPartition].start);
        let lastEnd = Number(tempModifiedPartition[selectedPartition].end);

        let inputtedSizeSector = getSectorFromMB(inputtedSize);

        let newSize = lastSize - inputtedSizeSector;

        if (newSize >= 0) {

            if (newSize !== 0) {

                let newUnallocated = {
                    number: Number(tempModifiedPartition[selectedPartition].number) + 1,
                    diskPath,
                    path: null,
                    size: newSize,
                    start: Number(lastStart + inputtedSizeSector),
                    end: Number(lastStart + inputtedSizeSector + newSize) - 1,
                    filesystem: null,
                    label: null,
                    format: false,
                    mountpoint: null,
                    label: null,
                    flags: []
                };

                tempModifiedPartition.splice(selectedPartition + 1, 0, newUnallocated);
            }

            tempModifiedPartition[selectedPartition].size = inputtedSizeSector;
            tempModifiedPartition[selectedPartition].end = lastStart + inputtedSizeSector - 1;

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

            newPartitionIndex += 1;

            tempModifiedPartition[selectedPartition] = {
                ...modifiedPartition[selectedPartition],
                path: `#${newPartitionIndex}`,
                format: true,
                filesystem: "ext4",
                label: null,
                flags: []
            };

            actualSize = modifiedPartition[selectedPartition].size;

            inputtedSize = ( actualSize * 512 ) / ( 1024 * 1024 );
        }
    })

</script>

<div class="flex flex-col border-4 border-black rounded-lg p-4 space-y-4">
    <span>{
        tempModifiedPartition[selectedPartition].path
            ? tempModifiedPartition[selectedPartition].path.includes("#")
                ? `New Partition ${tempModifiedPartition[selectedPartition].path}`
                : tempModifiedPartition[selectedPartition].path
            : tempModifiedPartition.path
    }</span>
    <div class="flex flex-row space-x-2 justify-between">
        <div class="border-4 border-black rounded-lg flex items-center p-4 space-x-2">
            {#if newPartition}
                <span>New Size</span>
                <input type="number" bind:value={inputtedSize} />
                <span>MB</span>
            {:else}
                <span>Size</span>
                <span>
                    {prettySize(tempModifiedPartition[selectedPartition].size)}
                </span>
            {/if}
        </div>
        <div class="flex flex-col items-start justify-center">
            {#if !newPartition}
                <div class="flex flex-row items-center justify-center space-x-2">
                    <input type="radio" value={false} bind:group={tempModifiedPartition[selectedPartition].format} />
                    <span>Keep data</span>
                </div>
                <div class="flex flex-row items-center justify-center space-x-2">
                    <input type="radio" value={true} bind:group={tempModifiedPartition[selectedPartition].format} />
                    <span>Erase data</span>
                </div>
            {/if}
        </div>
    </div>
    <div class="flex flex-row space-x-2 justify-between">
        <div class="flex flex-col">
            <span>Filesystem</span>
            <select bind:value={tempModifiedPartition[selectedPartition].filesystem}>
                <option value="btrfs">btrfs</option>
                <option value="fat32">fat32</option>
                <option value="ext4">ext4</option>
            </select>
        </div>
        <div class="flex flex-col">
            <span>Mountpoint</span>
            <select bind:value={tempModifiedPartition[selectedPartition].mountpoint}>
                <option value={null}>None</option>
                <option>/</option>
                <option>/boot/efi</option>
                <option>/home</option>
            </select>
        </div>
    </div>
    <div class="flex flex-col">
        <span>Label</span>
        <input type="text" class="border-2" bind:value={tempModifiedPartition[selectedPartition].label} />
    </div>
    <div class="flex flex-col">
        <span>Flags</span>
        <div class="flex flex-row justify-start items-center space-x-2">
            {#each getFlagList(tempModifiedPartition[selectedPartition].flags) as flag}
                {@const existFlag = tempModifiedPartition[selectedPartition].flags}
                <input
                    type="checkbox"
                    id={flag}
                    checked={tempModifiedPartition[selectedPartition].flags.includes(flag)}
                    onchange={(event) => {
                        let checked = event.target.checked;

                        if (checked) {
                            if (!existFlag.includes(flag)) {
                                tempModifiedPartition[selectedPartition].flags.push(flag)
                            }
                        } else {
                            if (existFlag.includes(flag)) {
                                tempModifiedPartition[selectedPartition].flags = existFlag.filter(f => f !== flag);
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
