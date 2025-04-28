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

<table class="border-4 border-black">
    <thead>
        <tr>
            <td>Path</td>
            <td>Label</td>
            <td>Size</td>
            <td>Format</td>
            <td>Filesystem</td>
            <td>Mountpoint</td>
            <!-- <td>Selected</td> -->
        </tr>
    </thead>
    <tbody>
        {#key selectedDisk}
            {#if modifiedPartition}
                {#each modifiedPartition as p, num}
                    <tr onclick={() => changeSelectedPartition(num)} class="{ num === selectedPartition ? 'bg-green-500' : '' }">
                        <td>{
                            p.path 
                                ? p.path.includes("#")
                                    ? `New Partition ${p.path}`
                                    : p.path
                                : 'Unallocated'
                        }</td>
                        <td>{p.label}</td>
                        <td>{prettySize(p.size)}</td>
                        <td>{p.format}</td>
                        <td>{p.filesystem ? p.filesystem : p.path ? 'Unknown' : 'Unallocated'}</td>
                        <td>{p.mountpoint ? p.mountpoint : ''}</td>
                    </tr>
                {/each}
            {/if}
        {/key}
    </tbody>
</table>
