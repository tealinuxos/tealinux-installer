<script>

    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";

    let disks = [];
    let selectedDisk = 0;
    let ready = false;

    async function handleGetJSON() {
        invoke('get_read_json').then((response) => {
            let json = JSON.parse(response);
            disks = json.disks
            ready = true;
            console.log(disks[0]);
        }).await
    }

    onMount(() => {
        handleGetJSON();
    });

</script>

<h1 class="py-5">gparted lite</h1>

<select bind:value={selectedDisk}>
    {#each disks as disk, i}
        <option value="{i}">{disk.path}</option>
    {/each}
</select>

{#if ready}

    <div class="w-full flex flex-row items-center">

        {#each disks[selectedDisk].partitions as partition}

            {@const disk = disks[selectedDisk]}
            {@const diskSize = disk.size.slice(0, -1)}
            {@const partitionSize = partition.sizeSector.slice(0, -1)}
            {@const percentage = partitionSize / diskSize * 100}

            <div style="width: {percentage}%;" class="h-[60px] text-center border-4 border-black mx-[1px] flex flex-col items-center">

                {#if percentage > 5}
                    {#if partition.isUnallocated}
                        <span>Unallocated</span>
                    {:else}
                        <span>{disk.path + partition.number}</span>
                    {/if}
                    
                    <span>{Math.floor(partitionSize / 2048)}</span>
                {/if}

            </div>

        {/each}

    </div>

    <table class="w-full">
        <tr>
            <th class="p-5">Partition</th>
            <th class="p-5">Name</th>
            <th class="p-5">File System</th>
            <th class="p-5">Mount Point</th>
            <th class="p-5">Size</th>
            <th class="p-5">Flags</th>
        </tr>

        {#each disks[selectedDisk].partitions as partition}
            
            {@const disk = disks[selectedDisk]}
            <tr>
                {#if partition.isUnallocated}
                    
                    <td class="px-5 text-center">unallocated</td>
                    <td class="px-5 text-center">unallocated</td>
                    <td class="px-5 text-center"></td>
                    <td class="px-5 text-center">unallocated</td>
                    <td class="px-5 text-center">{Math.floor(partition.sizeSector.slice(0, -1) / 2048)}</td>
                    <td class="px-5 text-center"></td>

                {:else}

                    <td class="px-5 text-center">{disk.path}{partition.number}</td>

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

                    <td class="px-5 text-center"></td>

                    <td class="px-5 text-center">{Math.floor(partition.sizeSector.slice(0, -1) / 2048)}</td>

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

    </table>

{/if}
