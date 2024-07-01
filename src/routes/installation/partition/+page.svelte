<script>

    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { getRead } from "../global.js";
    import { randomColor } from "randomcolor";

    let selectedDisk = 0;
    let partitionDetail = [];

    const getStorageJSON = async () => {

        let json = await getRead();

        return json.disk;
    };

    const getFilesystemJSON = async () => {

        let filesystems = await invoke('get_filesystem_json');

        return JSON.parse(filesystems);
    };

    const handlePartitionDetail = async (disks, selectedDisk) => {

        partitionDetail = []

        for (let i of disks[selectedDisk].partitions.keys())
        {
            let partitionPath = disks[selectedDisk].partitions[i].partitionPath;
            let startSector = disks[selectedDisk].partitions[i].start.slice(0, -1);
            let endSector = disks[selectedDisk].partitions[i].end.slice(0, -1);
            let size = disks[selectedDisk].partitions[i].size.slice(0, -1);

            partitionDetail.push({
                path: partitionPath,
                format: null,
                mountpoint: null,
                start: parseInt(startSector),
                end: parseInt(endSector),
                size: parseInt(size)
            });
        }

        return partitionDetail;
    };

    const handleSetPartition = async () => {

        let partition = JSON.stringify(partitionDetail);

        await invoke("blueprint_set_partition", { partition });
    };

    let partitionColors = [];

    const getColors = (disks) => {

        let length = disks[selectedDisk].partitions.length;

        let colors = [];

        for (let i = 0; i < length; i++)
        {
            colors.push(randomColor({
                luminosity: 'bright',
                hue: 'random'
            }));
        }

        partitionColors = colors;
    }

    onMount(() => {
        getStorageJSON().then((disks) => getColors(disks));
    });

</script>

<h1>Partitioning</h1>

{#await getStorageJSON()}
    Loading...
{:then disks}
    <div class="flex flex-col">
        <select class="w-1/3" bind:value={selectedDisk} on:change={getColors(disks)}>
            {#each disks as disk, i}
                {@const model = disk.model}
                {@const path = disk.diskPath}
                <option value={i}>{model + ' (' + path + ')'}</option>
            {/each}
        </select>
    </div>

    <!-- Partition Bar -->
    <div class="p-2 flex">
        {#each disks[selectedDisk].partitions as partition, i}
            {@const diskSize = disks[selectedDisk].size.slice(0, -1)}
            {@const partitionSize = partition.size.slice(0, -1)}
            {@const percentage = partitionSize / diskSize * 100}
            {@const color = partitionColors[i]}

            <div style="width: {percentage}%; background-color: {color}" class="h-16"></div>
        {/each}
    </div>

    <div class="flex flex-row">
        {#each disks[selectedDisk].partitions as partition, i}
            {@const color = partitionColors[i]}
            {@const size = partition.size.slice(0, -1) / 2048}
            {@const path = partition.partitionPath.slice(5)}

            <div style="background-color: {color}" class="p-4"></div>

            <div class="flex flex-col">
                <span class="p-2">{path}</span>
                <span class="p-2">{size}</span>
            </div>
        {/each}
    </div>

    {#await handlePartitionDetail(disks, selectedDisk)}
        Loading...
    {:then partitions}
        <div class="flex flex-col">
            {#each partitions as partition, i}
                {@const path = partition.path}

                {#if path == null}
                    <div class="flex flex-row">
                        <span>Unallocated</span>
                        <select bind:value={partitionDetail[i].format} on:change={handleSetPartition}>
                            <option value={null}>Do not format</option>
                            {#await getFilesystemJSON()}
                                <option disabled={true}>Loading...</option>
                            {:then filesystems}
                                {#each filesystems as filesystem}
                                    <option value={filesystem}>{filesystem}</option>
                                {/each}
                            {/await}
                        </select>
                        <select bind:value={partitionDetail[i].mountpoint} on:change={handleSetPartition}>
                            <option value={null}>No Mountpoint</option>
                            <option value="/">/</option>
                            <option value="/boot">/boot</option>
                            <option value="/home">/home</option>
                        </select>
                    </div>
                {:else}
                    <div class="flex flex-row">
                        <span>{path}</span>
                        <select bind:value={partitionDetail[i].format} on:change={handleSetPartition}>
                            <option value={null}>Do not format</option>
                            {#await getFilesystemJSON()}
                                <option disabled={true}>Loading...</option>
                            {:then filesystems}
                                {#each filesystems as filesystem}
                                    <option value={filesystem}>{filesystem}</option>
                                {/each}
                            {/await}
                        </select>
                        <select bind:value={partitionDetail[i].mountpoint} on:change={handleSetPartition}>
                            <option value={null}>No Mountpoint</option>
                            <option value="/">/</option>
                            <option value="/boot/efi">/boot/efi</option>
                            <option value="/home">/home</option>
                        </select>
                    </div>
                {/if}
            {/each}
        </div>
        <a href="/installation/account" class="pr-4">back</a>
        <a href="/installation/summary" on:click={handleSetPartition}>Next</a>
    {/await}
{/await}
