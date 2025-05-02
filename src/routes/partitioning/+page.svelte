<!-- @migration-task Error while migrating Svelte code: `<tr>` cannot be a child of `<table>`. `<table>` only allows these children: `<caption>`, `<colgroup>`, `<tbody>`, `<thead>`, `<tfoot>`, `<style>`, `<script>`, `<template>`. The browser will 'repair' the HTML (by moving, removing, or inserting elements) which breaks Svelte's assumptions about the structure of your components.
https://svelte.dev/e/node_invalid_placement -->
<script>

    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let disks = [];
    let selectedDisk = 0;
    let ready = false;

    async function getStorageJSON() {
        invoke('get_read_json').then((response) => {
            let json = JSON.parse(response).disk;
            disks = json
            ready = true;
            console.log(disks);
        }).await
    }

    onMount(() => {
        getStorageJSON();
    });

</script>

<h1 class="py-5">gparted lite</h1>

<select bind:value={selectedDisk}>
    {#each disks as disk, i}
        <option value="{i}">{disk.diskPath}</option>
    {/each}
</select>

{#if ready}

    <div class="w-full flex flex-row items-center">

        {#each disks[selectedDisk].partitions as partition}

            {@const disk = disks[selectedDisk]}
            {@const diskSize = disk.size.slice(0, -1)}
            {@const partitionPath = partition.partitionPath}
            {@const partitionSize = partition.size.slice(0, -1)}
            {@const percentage = partitionSize / diskSize * 100}

            <div style="width: {percentage}%;" class="h-[60px] text-center border-4 border-black mx-[1px] flex flex-col items-center">

                {#if percentage > 5}
                    {#if partition.typePartisi == "free"}
                        <span>Unallocated</span>
                    {:else}
                        <span>{partitionPath}</span>
                    {/if}
                    
                    <span>{Math.floor(partitionSize / 2048)}</span>
                {/if}

            </div>

        {/each}

    </div>

    <table class="w-full">
        <thead>
            <tr>
                <th class="p-5">Partition</th>
                <th class="p-5">Name</th>
                <th class="p-5">File System</th>
                <th class="p-5">Mount Point</th>
                <th class="p-5">Size</th>
                <th class="p-5">Flags</th>
            </tr>
        </thead>
        <tbody>

        {#each disks[selectedDisk].partitions as partition}
            
            {@const disk = disks[selectedDisk]}
            {@const partitionSize = partition.size.slice(0, -1) / 2048}

            <tr>
                {#if partition.typePartisi == "free"}
                    
                    <td class="px-5 text-center">unallocated</td>
                    <td class="px-5 text-center"></td>
                    <td class="px-5 text-center">unallocated</td>
                    <td class="px-5 text-center"></td>
                    <td class="px-5 text-center">{Math.floor(partitionSize)}</td>
                    <td class="px-5 text-center"></td>

                {:else}

                    <td class="px-5 text-center">{partition.partitionPath}</td>

                    {#if partition.name !== null}
                        <td class="px-5 text-center">{partition.name}</td>
                    {:else}
                        <td class="px-5 text-center"></td>
                    {/if}

                    {#if partition.filesystem !== null}
                        <td class="px-5 text-center">{partition.filesystem}</td>
                    {:else}
                        <td class="px-5 text-center"></td>
                    {/if}

                    <td class="px-5 text-center">{partition.mountpoint}</td>

                    <td class="px-5 text-center">{Math.floor(partitionSize)}</td>

                    {#if partition.flags !== null}

                        <td class="px-5 text-center">

                        {#each partition.flags as flag, i}

                            {#if i !== partition.flags.length - 1}
                                <span>{flag}, </span>
                            {:else}
                                <span>{flag}</span>
                            {/if}

                        {/each}

                        </td>

                    {:else}
                        <td class="px-5 text-center"></td>
                    {/if}

                {/if}

            </tr>
        {/each}
        </tbody>
    </table>

{/if}
<a href="/" class="px-4 py-2 bg-slate-300 rounded-md">Home</a>
