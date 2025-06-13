const mibToSector = (mib) => {
    return Number((mib * 1024 * 1024) / 512);
};

export const getDiskAfter = ( diskBefore, rootFilesystem = "ext4", partitionTable, swapSize = 0, method = "single", selectedPartitionStart = null, selectedPartitionEnd) => {

    let size = Number(diskBefore.size.slice(0, -1));
    let diskPath = diskBefore.diskPath;
    let newPartitions = [];
    let partitionNumber = 1;

    if (method === "single" || (!selectedPartitionStart && !selectedPartitionEnd)) {

        /// EFI PARTITION

        if (partitionTable === "gpt") {

            let path = `${diskPath}${partitionNumber}`;

            if (diskPath && diskPath.includes("nvme")) path = `${diskPath}p${partitionNumber}`;

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
    } else {
        let selectedPartitionIndex = diskBefore?.partitions?.findIndex(p => { return p.start === selectedPartitionStart && p.end === selectedPartitionEnd }) ?? -1;

        if (selectedPartitionIndex !== -1) {
            let partitions = diskBefore.partitions;
            let mapped = partitions.map(p => ({ ...p, mountpoint: null }));
            partitions = mapped;
            partitions[selectedPartitionIndex].name = "TeaLinuxOS";
            partitions[selectedPartitionIndex].mountpoint = "/";
            partitions[selectedPartitionIndex].flags = partitionTable === "mbr" ? [ "boot" ] : null,
            newPartitions = partitions;
        }
    }

    return { ...diskBefore, partitions: newPartitions };
};

export const getIdealSwapSize = async (memory) => {

    let idealSize = 0;

    switch (memory) {
        case memory < 8192:
            idealSize = memory * 2;
            break;
        case memory < 16384:
            idealSize =  memory * 1.5;
            break;
        case memory < 32768:
            idealSize = memory;
            break;
        case memory >= 32768:
            idealSize = memory / 2;
            break;
        default:
            idealSize = memory;
            break;
    }

    return Math.floor(idealSize);
}
