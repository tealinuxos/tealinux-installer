<script>

    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { json } from "../stores.js";

    let disks;
    let ready = false;
    let showPartition = false;
    let selectedDisk = 0;
    let partitionDetail = [];

    const getStorageJSON = async () => {
        invoke('get_storage_json').then((response) => {
            disks = JSON.parse(response);
            console.log(disks[selectedDisk].partitions);
            ready = true;
            handlePartitionDetail();
        });
    };

    const handlePartitionDetail = () => {
        for (let i of disks[selectedDisk].partitions.keys())
        {
            let partitionPath = disks[selectedDisk].partitions[i].partitionPath;

            partitionDetail.push({
                path: partitionPath,
                format: false,
                mountpoint: null
            });
        }
        console.log(partitionDetail);
    }

    const handleSetPartition = () => {
        json.partition = partitionDetail;
    }

    $: console.log(partitionDetail);

    onMount(() => {
        getStorageJSON();
    });

</script>

<h1>Partitioning</h1>

{#if ready}
<div class="flex flex-col">
    <select class="w-1/3" bind:value={selectedDisk} on:change={handlePartitionDetail}>
        {#each disks as disk, i}
            {@const model = disk.model}
            {@const path = disk.diskPath}
            <option value={i}>{model + ' (' + path + ')'}</option>
        {/each}
    </select>

    {#each disks[selectedDisk].partitions as partition, i}
        {@const path = partition.partitionPath}
        <div class="flex flex-row">
            <span>{path}</span>
            <select bind:value={partitionDetail[i].format}>
                <option value={false}>Do not format</option>
                <option value={true}>Format</option>
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
{/if}
