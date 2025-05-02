<script>

import { onMount } from 'svelte';
import { getRead } from '/src/routes/installation/global.js';
import { invoke } from '@tauri-apps/api/core';
import { prettySize } from '$lib/essentials.js';
import prettyBytes from 'pretty-bytes';
import List from '$lib/components/partitions/List.svelte';
import Detail from '$lib/components/partitions/Detail.svelte';
import Preview from '$lib/components/partitions/Preview.svelte';
import DiskPreview from '$lib/components/DiskPreview.svelte';
import Navigation from "$lib/components/Navigation.svelte";

let selectedDisk = $state(0);
let selectedPartition = $state(0);

let showEdit = $state(false);
let newPartition = $state(false);

let diskSize = $state(0);
let diskPath = $state('');

let firmwareType = $state('');

let originalPartition = $state([]);
let modifiedPartition = $state([]);
let tempModifiedPartition = $state([]);

let newPartitionIndex = $state(0);

let storage = $state({
    diskPath: null,
    partitionTable: null,
    newPartitionTable: false,
    layoutChanged: false,
    partitions: null
});

const getStorageJSON = async () => {

    let json = await getRead();

    firmwareType = json.firmware;

    json = json.disk.filter((disk) => disk.partitions !== null);

    // json.push(json[0])

    // console.log(json) // remove this later

    return json;
};

const changeSelectedDisk = async (selected) => {
    modifiedPartition = [];
    tempModifiedPartition = [];
    originalPartition = [];

    let disks = await getStorageJSON();

    storage.diskPath = disks[selectedDisk].diskPath;
    storage.partitionTable = disks[selectedDisk].label;

    let partitions = disks[selectedDisk].partitions;

    diskSize = disks[selectedDisk].size;
    diskPath = disks[selectedDisk].diskPath;

    for (let i of partitions.keys()) {
        let p = {
            number: Number(partitions[i].number),
            diskPath,
            path: partitions[i].partitionPath,
            size: Number(partitions[i].size.slice(0, -1)),
            start: Number(partitions[i].start.slice(0, -1)),
            end: Number(partitions[i].end.slice(0, -1)),
            filesystem: partitions[i].filesystem,
            label: null,
            format: false,
            mountpoint: null,
            label: partitions[i].name,
            flags: partitions[i].flags ? partitions[i].flags : []
        };

        modifiedPartition = [...modifiedPartition, p];
    }

    // Simulating an unallocated partition
    // modifiedPartition.push({
    //     number: Number(modifiedPartition[modifiedPartition.length - 1].number) + 1,
    //     path: null,
    //     size: 10240000,
    //     start: 1024000,
    //     end: 2048000,
    //     filesystem: null,
    //     format: false,
    //     mountpoint: null,
    //     label: null,
    //     flags: []
    // });
    //

    tempModifiedPartition = JSON.parse(JSON.stringify(modifiedPartition));  // This JSON stuff is needed so the variable is assigned by value
    originalPartition = JSON.parse(JSON.stringify(modifiedPartition));      // This JSON stuff is needed so the variable is assigned by value

    selectedDisk = selected;
    selectedPartition = 0;
}

const revertChanges = () => {
    modifiedPartition = JSON.parse(JSON.stringify(originalPartition));
    tempModifiedPartition = JSON.parse(JSON.stringify(originalPartition));
}

const isUnallocated = (partition) => {
    if (!partition) {
        return true;
    } else {
        return partition.path ? false : true;
    }
}

const newPartitionTable = () => {

    showEdit = false;

    storage.newPartitionTable = true;

    let partition = {
        number: 1,
        diskPath,
        path: null,
        size: Number(diskSize.slice(0, -1)) - 2048,
        start: 2048,
        end: Number(diskSize.slice(0, -1)) - 1,
        filesystem: null,
        format: false,
        mountpoint: null,
        label: null,
        flags: []
    };

    modifiedPartition = [partition];
    tempModifiedPartition = [partition];
    console.log(tempModifiedPartition)
}

const removePartition = () => {

    let partitionWithTag = modifiedPartition.filter(p => p.path ? p.path.includes("#") : false);

    let numbers = partitionWithTag.map(p => Number(p.path.replace("#", "")));

    console.log("numbers", numbers)

    if (modifiedPartition[selectedPartition].path.includes("#")) {
        if (numbers) {
            for (let [i, partition] of modifiedPartition.entries()) {
                if (partition.path && partition.path.includes("#")) {
                    let number = Number(partition.path.replace("#", ""));
                    if (number > 1) {
                        modifiedPartition[i].path = `#${number - 1}`;
                    }
                }
            }
        }
    }

    tempModifiedPartition[selectedPartition] = {
        ...tempModifiedPartition[selectedPartition],
        path: null,
        filesystem: null,
        format: false,
        mountpoint: null,
        label: null,
        flags: []
    };

    modifiedPartition[selectedPartition] = {
        ...modifiedPartition[selectedPartition],
        path: null,
        filesystem: null,
        format: false,
        mountpoint: null,
        label: null,
        flags: []
    };

    showEdit = false;
}

const handleSetStorage = () => {

    let partitionWithBoot = modifiedPartition.find(p => p.mountpoint.includes("boot"));
    let bootloaderPath = partitionWithBoot.path;

    let bootloader = {
        firmwareType,
        path: bootloaderPath
    };

    let filteredPartition = modifiedPartition.map(p => {
        return p.path
            ? p.path.includes("#")
                ? {...p, path: null}
                : p
            : p
    });

    storage.partitions = filteredPartition;
    invoke('blueprint_set_bootloader', { bootloader: JSON.stringify(bootloader) });
    invoke('blueprint_set_storage', { storage: JSON.stringify(storage) });
}

$effect(() => {

    for (let i = 0; i < modifiedPartition.length - 1; i += 1) {

        let current = modifiedPartition[i];
        let next = modifiedPartition[i + 1];

        if (!current.path && !next.path) {
            current.size += next.size;
            current.end = current.start + current.size - 1;

            modifiedPartition.splice(i + 1, 1);
            modifiedPartition = modifiedPartition.map(p => p.number > i + 1 ? {...p, number: p.number - 1} : p)

            selectedPartition = i;
        }
    }
})

onMount(async () => {
    await changeSelectedDisk(0);
})

</script>

{#await getStorageJSON() then json}
    <div class="flex flex-col">
        <div class="flex flex-row">
            <select onchange={async (event) => await changeSelectedDisk(event.target.value)}>
                {#each json as d, num}
                    {@const size = prettySize(d.size.slice(0, -1))}
                        <option value={num}>
                            {d.diskPath} ({size})
                        </option>
                {/each}
            </select>
            <div class="text-white">
                <button onclick={revertChanges}>Revert Changes</button>
                <button onclick={newPartitionTable}>New Partition Table</button>
            </div>
        </div>
        <Preview
            bind:modifiedPartition
            bind:diskSize
        />
    </div>
    <div class="flex flex-row space-x-2 text-white">
        <div class="flex flex-col">
            <List
                bind:selectedDisk
                bind:selectedPartition
                bind:modifiedPartition
                bind:originalPartition
                bind:showEdit
                bind:newPartition
                bind:newPartitionIndex
            />
            <div class="flex flex-row justify-between p-2">
                <button class="bg-green-500 disabled:bg-green-900 px-4" onclick={() => { showEdit = true; newPartition = true} } disabled={!isUnallocated(modifiedPartition[selectedPartition])}>+</button>
                <div>
                    <button class="bg-green-500 px-4 disabled:bg-green-900" disabled={isUnallocated(modifiedPartition[selectedPartition])} onclick={() => showEdit = true}>Edit</button>
                    <button class="bg-red-500 px-4" onclick={removePartition}>Remove</button>
                </div>
            </div>
        </div>
        {#if tempModifiedPartition[selectedPartition]}
            {#if showEdit}
                <Detail
                    bind:showEdit
                    bind:tempModifiedPartition
                    bind:modifiedPartition
                    bind:selectedPartition
                    bind:newPartition
                    bind:storage
                    bind:diskSize
                    bind:diskPath
                    bind:newPartitionIndex
                />
            {:else}
                <div class="flex border-4 border-green-900 px-4 justify-center items-center">
                    <span>Select partition to edit!</span>
                </div>
            {/if}
        {/if}
    </div>
    <!-- <a href="/installation/summary" class="bg-green-500 px-4 py-2 rounded-lg" onclick={handleSetBlueprint}>Apply</a> -->
    <button class="text-white bg-green-900" onclick={handleSetStorage}>Apply Without Summary</button>
{/await}

<Navigation
	totalSteps={5}
	currentStep={4}
	currentTitle="Manual Partitioning"
	prevPath="/installation/partitioning"
	nextPath="/installation"
	nextAction={null}
/>
