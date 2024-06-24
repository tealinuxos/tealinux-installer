<script>

    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { json } from "../stores.js";

    let selectedDisk = 0;
    let partitionDetail = [];
    let filesystems;

    const getStorageJSON = async () => {

        let disks = await invoke('get_storage_json');

        return JSON.parse(disks);
    };

    const getFilesystemJSON = () => {
        invoke('get_filesystem_json').then((response) => {
            filesystems = JSON.parse(response);
        });
    };

    const handlePartitionDetail = async (disks, selectedDisk) => {

        partitionDetail = []

        for (let i of disks[selectedDisk].partitions.keys())
        {
            let partitionPath = disks[selectedDisk].partitions[i].partitionPath;

            partitionDetail.push({
                path: partitionPath,
                format: null,
                mountpoint: null
            });
        }

        return partitionDetail;
    }

    const handleSetPartition = () => {
        json.partition = partitionDetail;
    }

    onMount(() => {
        getStorageJSON();
        getFilesystemJSON();
    });

</script>

<h1>Partitioning</h1>

{#await getStorageJSON()}
    Loading...
{:then disks}
    <div class="flex flex-col">
        <select class="w-1/3" bind:value={selectedDisk}>
            {#each disks as disk, i}
                {@const model = disk.model}
                {@const path = disk.diskPath}
                <option value={i}>{model + ' (' + path + ')'}</option>
            {/each}
        </select>
    </div>

    {#await handlePartitionDetail(disks, selectedDisk)}
        Loading...
    {:then partitions}
        <div class="flex flex-col">
            {#each partitions as partition, i}
                {@const path = partition.path}
                <div class="flex flex-row">
                    <span>{path}</span>
                    <select bind:value={partitionDetail[i].format}>
                        <option value={null}>Do not format</option>
                        {#each filesystems as filesystem}
                            <option value={filesystem}>{filesystem}</option>
                        {/each}
                    </select>
                    <select bind:value={partitionDetail[i].mountpoint}>
                        <option value={null}>No Mountpoint</option>
                        <option value="/">/</option>
                        <option value="/boot">/boot</option>
                        <option value="/home">/home</option>
                    </select>
                </div>
            {/each}
        </div>
        <a href="/installation/summary" on:click={handleSetPartition}>Next</a>
    {/await}
{/await}
