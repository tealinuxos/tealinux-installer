const mibToSector = (mib) => {
    return Number((mib * 1024 * 1024) / 512);
};

export const getDiskAfter = ( diskBefore, rootFilesystem = "ext4", partitionTable, swapSize = 0) => {

    let size = Number(diskBefore.size.slice(0, -1));
    let diskPath = diskBefore.diskPath;
    let newPartitions = [];
    let partitionNumber = 1;


    /// EFI PARTITION

    if (partitionTable === "gpt") {

        newPartitions.push({
            number: partitionNumber,
            partitionPath: `${diskPath}${partitionNumber}`,
            filesystem: "fat32",
            flags: [ "boot", "esp" ],
            mountpoint: [ "/boot/efi" ],
            name: null,
            size: `${mibToSector(512)}s`,
            start: "2048s",
            end: `${mibToSector(512) + 2048}s`,
            typePartisi: "primary",
            typeUuid: null,
            uuid: null
        })

        size -= mibToSector(512) + 2048;
        partitionNumber += 1;
    }


    /// SWAP PARTITION

    if (swapSize !== 0) {

        let start = newPartitions.length
            ? Number(newPartitions[newPartitions.length - 1].end.slice(0, -1)) + 1
            : 2048;

        newPartitions.push({
            number: partitionNumber,
            partitionPath: `${diskPath}${partitionNumber}`,
            filesystem: "linux-swap(v1)",
            flags: null,
            mountpoint: [ "swap" ],
            name: null,
            size: `${mibToSector(swapSize)}s`,
            start: `${start}s`,
            end: `${mibToSector(swapSize) + start}s`,
            typePartisi: "primary",
            typeUuid: null,
            uuid: null
        })

        size -= mibToSector(swapSize);
        partitionNumber += 1;
    }

    /// ROOT PARTITION

    let start = newPartitions.length
        ? Number(newPartitions[newPartitions.length - 1].end.slice(0, -1)) + 1
        : 2048;
    
    newPartitions.push({
        number: partitionNumber,
        partitionPath: `${diskPath}${partitionNumber}`,
        filesystem: rootFilesystem,
        flags: partitionTable === "mbr" ? [ "boot" ] : null,
        mountpoint: [ "/" ],
        name: null,
        size: `${size - 2048}s`,
        start: `${start}s`,
        end: `${size - 2048 + start}s`,
        typePartisi: "primary",
        typeUuid: null,
        uuid: null
    });

    return { ...diskBefore, partitions: newPartitions };
};
