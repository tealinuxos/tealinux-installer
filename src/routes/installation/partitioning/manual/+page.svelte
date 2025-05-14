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
import SelectComponent from '$lib/components/SelectComponent.svelte';



let disks = $state([]);
let selectedDisk = $state(0);
let selectedPartition = $state(0);

let showEdit = $state(false);
let newPartition = $state(false);
let disable = $state(false)

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
    <div class="flex flex-col p-5 gap-y-2">
        <div class="flex flex- space-between space-x-69  ">
            <div>
                <h1 class="text-[#26A768] font-['Plus_Jakarta_Sans'] text-[28px] font-bold leading-[39.2px]">
                    Manual Partition
                  </h1>            
            </div>
            <div class="flex gap-2.5">		  
                <!-- <button 
                    onclick={revertChanges}
                    class="flex w-[169px] h-[40px] justify-center items-center gap-[7.963px] rounded-[14px] border-[0.239px] border-[#3C6350] bg-[#101010] "
                >
                    <svg width="17" height="17" viewBox="0 0 17 17" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path id="Vector" d="M8.71289 16.2334C6.47956 16.2334 4.58789 15.4584 3.03789 13.9084C1.48789 12.3584 0.712891 10.4667 0.712891 8.2334C0.712891 6.00007 1.48789 4.1084 3.03789 2.5584C4.58789 1.0084 6.47956 0.233402 8.71289 0.233402C9.86289 0.233402 10.9629 0.470735 12.0129 0.945402C13.0629 1.42007 13.9629 2.0994 14.7129 2.9834V1.2334C14.7129 0.950069 14.8089 0.712735 15.0009 0.521402C15.1929 0.330069 15.4302 0.234069 15.7129 0.233402C15.9956 0.232735 16.2332 0.328735 16.4259 0.521402C16.6186 0.714069 16.7142 0.951402 16.7129 1.2334V6.2334C16.7129 6.51674 16.6169 6.7544 16.4249 6.9464C16.2329 7.1384 15.9956 7.23407 15.7129 7.2334H10.7129C10.4296 7.2334 10.1922 7.1374 10.0009 6.9454C9.80956 6.7534 9.71356 6.51607 9.71289 6.2334C9.71222 5.95074 9.80822 5.7134 10.0009 5.5214C10.1936 5.3294 10.4309 5.2334 10.7129 5.2334H13.9129C13.3796 4.30007 12.6506 3.56674 11.7259 3.0334C10.8012 2.50007 9.79689 2.2334 8.71289 2.2334C7.04622 2.2334 5.62956 2.81674 4.46289 3.9834C3.29622 5.15007 2.71289 6.56674 2.71289 8.2334C2.71289 9.90007 3.29622 11.3167 4.46289 12.4834C5.62956 13.6501 7.04622 14.2334 8.71289 14.2334C9.84622 14.2334 10.8839 13.9461 11.8259 13.3714C12.7679 12.7967 13.4969 12.0257 14.0129 11.0584C14.1462 10.8251 14.3339 10.6627 14.5759 10.5714C14.8179 10.4801 15.0636 10.4757 15.3129 10.5584C15.5796 10.6417 15.7712 10.8167 15.8879 11.0834C16.0046 11.3501 15.9962 11.6001 15.8629 11.8334C15.1796 13.1667 14.2046 14.2334 12.9379 15.0334C11.6712 15.8334 10.2629 16.2334 8.71289 16.2334Z" fill="#26A768"/>
                    </svg>
                    <span class="text-[#4CDA95] font-['Plus_Jakarta_Sans'] text-[13px] font-bold leading-[140%]">
                            revert Changes
                    </span>
                        
                </button> -->
                <button 
                    onclick={newPartitionTable}
                    class="flex w-[169px] h-[40px] justify-center items-center gap-[7.963px] rounded-[14px] border-[0.239px] border-[#3C6350] bg-[#101010] hover:shadow-[0_0_7.167px_rgba(38,167,104,0.8)] active:shadow-[0_0_7.167px_rgba(38,167,104,0.8)] transition-all duration-200 "
                >
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path d="M11 13H6C5.71667 13 5.47934 12.904 5.288 12.712C5.09667 12.52 5.00067 12.2827 5 12C4.99934 11.7173 5.09534 11.48 5.288 11.288C5.48067 11.096 5.718 11 6 11H11V6C11 5.71667 11.096 5.47934 11.288 5.288C11.48 5.09667 11.7173 5.00067 12 5C12.2827 4.99934 12.5203 5.09534 12.713 5.288C12.9057 5.48067 13.0013 5.718 13 6V11H18C18.2833 11 18.521 11.096 18.713 11.288C18.905 11.48 19.0007 11.7173 19 12C18.9993 12.2827 18.9033 12.5203 18.712 12.713C18.5207 12.9057 18.2833 13.0013 18 13H13V18C13 18.2833 12.904 18.521 12.712 18.713C12.52 18.905 12.2827 19.0007 12 19C11.7173 18.9993 11.48 18.9033 11.288 18.712C11.096 18.5207 11 18.2833 11 18V13Z" fill="#26A768"/>
                        <path d="M11 13H6C5.71667 13 5.47934 12.904 5.288 12.712C5.09667 12.52 5.00067 12.2827 5 12C4.99934 11.7173 5.09534 11.48 5.288 11.288C5.48067 11.096 5.718 11 6 11H11V6C11 5.71667 11.096 5.47934 11.288 5.288C11.48 5.09667 11.7173 5.00067 12 5C12.2827 4.99934 12.5203 5.09534 12.713 5.288C12.9057 5.48067 13.0013 5.718 13 6V11H18C18.2833 11 18.521 11.096 18.713 11.288C18.905 11.48 19.0007 11.7173 19 12C18.9993 12.2827 18.9033 12.5203 18.712 12.713C18.5207 12.9057 18.2833 13.0013 18 13H13V18C13 18.2833 12.904 18.521 12.712 18.713C12.52 18.905 12.2827 19.0007 12 19C11.7173 18.9993 11.48 18.9033 11.288 18.712C11.096 18.5207 11 18.2833 11 18V13Z" fill="url(#paint0_linear_1976_2711)"/>
                        <defs>
                        <linearGradient id="paint0_linear_1976_2711" x1="12" y1="5" x2="12" y2="19" gradientUnits="userSpaceOnUse">
                            <stop offset="0.92" stop-color="#26A768" stop-opacity="0"/>
                            <stop offset="1" stop-color="#26A768"/>
                        </linearGradient>
                        </defs>
                    </svg>
                    <span class="text-[#4CDA95] font-['Plus_Jakarta_Sans'] text-[13px] font-bold leading-[140%] ">
                            New Partition Table
                    </span>
                    
                </button>
                <button 
                    onclick={revertChanges}
                    class="flex w-[169px] h-[40px] justify-center items-center gap-[7.963px] rounded-[14px] border-[0.239px] border-[#3C6350] bg-[#101010] hover:shadow-[0_0_7.167px_rgba(38,167,104,0.8)] active:shadow-[0_0_7.167px_rgba(38,167,104,0.8)] transition-all duration-200 "
                >
                    <svg width="17" height="17" viewBox="0 0 17 17" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path id="Vector" d="M8.71289 16.2334C6.47956 16.2334 4.58789 15.4584 3.03789 13.9084C1.48789 12.3584 0.712891 10.4667 0.712891 8.2334C0.712891 6.00007 1.48789 4.1084 3.03789 2.5584C4.58789 1.0084 6.47956 0.233402 8.71289 0.233402C9.86289 0.233402 10.9629 0.470735 12.0129 0.945402C13.0629 1.42007 13.9629 2.0994 14.7129 2.9834V1.2334C14.7129 0.950069 14.8089 0.712735 15.0009 0.521402C15.1929 0.330069 15.4302 0.234069 15.7129 0.233402C15.9956 0.232735 16.2332 0.328735 16.4259 0.521402C16.6186 0.714069 16.7142 0.951402 16.7129 1.2334V6.2334C16.7129 6.51674 16.6169 6.7544 16.4249 6.9464C16.2329 7.1384 15.9956 7.23407 15.7129 7.2334H10.7129C10.4296 7.2334 10.1922 7.1374 10.0009 6.9454C9.80956 6.7534 9.71356 6.51607 9.71289 6.2334C9.71222 5.95074 9.80822 5.7134 10.0009 5.5214C10.1936 5.3294 10.4309 5.2334 10.7129 5.2334H13.9129C13.3796 4.30007 12.6506 3.56674 11.7259 3.0334C10.8012 2.50007 9.79689 2.2334 8.71289 2.2334C7.04622 2.2334 5.62956 2.81674 4.46289 3.9834C3.29622 5.15007 2.71289 6.56674 2.71289 8.2334C2.71289 9.90007 3.29622 11.3167 4.46289 12.4834C5.62956 13.6501 7.04622 14.2334 8.71289 14.2334C9.84622 14.2334 10.8839 13.9461 11.8259 13.3714C12.7679 12.7967 13.4969 12.0257 14.0129 11.0584C14.1462 10.8251 14.3339 10.6627 14.5759 10.5714C14.8179 10.4801 15.0636 10.4757 15.3129 10.5584C15.5796 10.6417 15.7712 10.8167 15.8879 11.0834C16.0046 11.3501 15.9962 11.6001 15.8629 11.8334C15.1796 13.1667 14.2046 14.2334 12.9379 15.0334C11.6712 15.8334 10.2629 16.2334 8.71289 16.2334Z" fill="#26A768"/>
                    </svg>
                    <span class="text-[#4CDA95] font-['Plus_Jakarta_Sans'] text-[13px] font-bold leading-[140%] ">
                        revert Changes
                    </span>
                    
                </button>
                <button 
                    onclick={revertChanges}
                    class="flex w-[169px] h-[40px] justify-center items-center gap-[7.963px] rounded-[14px] border-[0.239px] border-[#3C6350] bg-[#101010] hover:shadow-[0_0_7.167px_rgba(38,167,104,0.8)] active:shadow-[0_0_7.167px_rgba(38,167,104,0.8)] transition-all duration-200 "
                >
                    <svg width="17" height="17" viewBox="0 0 17 17" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path id="Vector" d="M8.71289 16.2334C6.47956 16.2334 4.58789 15.4584 3.03789 13.9084C1.48789 12.3584 0.712891 10.4667 0.712891 8.2334C0.712891 6.00007 1.48789 4.1084 3.03789 2.5584C4.58789 1.0084 6.47956 0.233402 8.71289 0.233402C9.86289 0.233402 10.9629 0.470735 12.0129 0.945402C13.0629 1.42007 13.9629 2.0994 14.7129 2.9834V1.2334C14.7129 0.950069 14.8089 0.712735 15.0009 0.521402C15.1929 0.330069 15.4302 0.234069 15.7129 0.233402C15.9956 0.232735 16.2332 0.328735 16.4259 0.521402C16.6186 0.714069 16.7142 0.951402 16.7129 1.2334V6.2334C16.7129 6.51674 16.6169 6.7544 16.4249 6.9464C16.2329 7.1384 15.9956 7.23407 15.7129 7.2334H10.7129C10.4296 7.2334 10.1922 7.1374 10.0009 6.9454C9.80956 6.7534 9.71356 6.51607 9.71289 6.2334C9.71222 5.95074 9.80822 5.7134 10.0009 5.5214C10.1936 5.3294 10.4309 5.2334 10.7129 5.2334H13.9129C13.3796 4.30007 12.6506 3.56674 11.7259 3.0334C10.8012 2.50007 9.79689 2.2334 8.71289 2.2334C7.04622 2.2334 5.62956 2.81674 4.46289 3.9834C3.29622 5.15007 2.71289 6.56674 2.71289 8.2334C2.71289 9.90007 3.29622 11.3167 4.46289 12.4834C5.62956 13.6501 7.04622 14.2334 8.71289 14.2334C9.84622 14.2334 10.8839 13.9461 11.8259 13.3714C12.7679 12.7967 13.4969 12.0257 14.0129 11.0584C14.1462 10.8251 14.3339 10.6627 14.5759 10.5714C14.8179 10.4801 15.0636 10.4757 15.3129 10.5584C15.5796 10.6417 15.7712 10.8167 15.8879 11.0834C16.0046 11.3501 15.9962 11.6001 15.8629 11.8334C15.1796 13.1667 14.2046 14.2334 12.9379 15.0334C11.6712 15.8334 10.2629 16.2334 8.71289 16.2334Z" fill="#26A768"/>
                    </svg>
                    <span class="text-[#4CDA95] font-['Plus_Jakarta_Sans'] text-[13px] font-bold leading-[140%] ">
                        refresh
                    </span>
                    
                </button>
                <!-- <SelectComponent
                    height="46.66px"
                    width="216px"
                    options={disks}
                    bind:value={selectedDisk}
                    displayField="diskPath"
                    sizeField="size"
                    formatter={prettySize}
                    on:change={({ detail }) => changeSelectedDisk(detail.value)}
                    class="w-full max-w-md"
                />   -->
            </div>



        </div>
        <div class="">
            <Preview
            bind:modifiedPartition
            bind:diskSize
        />
        </div>
        <div class="flex flex-row flex-auto space-x-2 text-white">
            <List
                bind:selectedDisk
                bind:selectedPartition
                bind:modifiedPartition
                bind:originalPartition
                bind:showEdit
                bind:newPartition
                bind:newPartitionIndex>
                
                
            </List>


        
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
                {:else }
                        <Detail
                        readOnly={true}
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
                {/if}
            {/if}
        </div>
    </div>
    <!-- <div class="flex flex-row justify-between p-2">
        <button class="bg-green-500 disabled:bg-green-900 px-4" onclick={() => { showEdit = true; newPartition = true} } disabled={!isUnallocated(modifiedPartition[selectedPartition])}>+</button>
        <div>
            <button class="bg-green-500 px-4 disabled:bg-green-900" disabled={isUnallocated(modifiedPartition[selectedPartition])} onclick={() => showEdit = true}>Edit</button>
            <button class="bg-red-500 px-4" onclick={removePartition}>Remove</button>
        </div>
    </div> -->
    <!-- <button class="text-white bg-green-900" onclick={handleSetStorage}>Apply Without Summary</button> -->
{/await}

<Navigation
	totalSteps={5}
	currentStep={4}
	currentTitle="Manual Partitioning"
	prevPath="/installation/partitioning"
	nextPath="/installation"
	nextAction={null}
/>
