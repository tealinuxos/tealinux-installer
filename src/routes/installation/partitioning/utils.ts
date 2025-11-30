import type { Disk, Partition } from '$types/read-from-opt';

const SECTOR_SIZE = 512;
const ALIGNMENT_OFFSET = 2048;

const formatSector = (val: number): string => {
	return `${Math.floor(val)}s`;
};

const parseSector = (val: string): number => {
	if (!val || typeof val !== 'string') return 0;
	const num = parseInt(val.replace(/\D/g, ''), 10);
	return isNaN(num) ? 0 : num;
};

const mibToSector = (mib: number): number => {
	return Math.floor((mib * 1024 * 1024) / SECTOR_SIZE);
};

export const getDiskAfter = (
	diskBefore: Disk,
	rootFilesystem: string = 'ext4',
	partitionTable: string, //TODO: CHECK TIPE DATA PARTITION TABLE ITU APA SAJA
	swapSizeMib: number = 0
): Disk => {
	const newDisk: Disk = { ...diskBefore, partitions: [] };

	let availableSectors = parseSector(diskBefore.size);
	const diskPath = diskBefore.diskPath;

	const getPartPath = (idx: number) => {
		const prefix = diskPath.includes('nvme') || diskPath.includes('mmcblk') ? 'p' : '';
		return `${diskPath}${prefix}${idx}`;
	};

	const newPartitions: Partition[] = [];
	let currentPartNum = 1;

	// Reserve EFI For GPT
	if (partitionTable === 'gpt') {
		const efiSizeSector = mibToSector(512);
		const start = ALIGNMENT_OFFSET;
		const end = start + efiSizeSector;

		newPartitions.push({
			number: String(currentPartNum),
			partitionPath: getPartPath(currentPartNum),
			filesystem: 'fat32',
			flags: ['boot', 'esp'],
			mountpoint: ['/boot/efi'],
			name: 'EFI System',
			size: formatSector(efiSizeSector),
			start: formatSector(start),
			end: formatSector(end),
			typePartisi: 'primary'
		});

		availableSectors -= efiSizeSector + ALIGNMENT_OFFSET;
		currentPartNum++;
	}

	// Swap Size Partition
	if (swapSizeMib > 0) {
		const prevEnd =
			newPartitions.length > 0
				? parseSector(newPartitions[newPartitions.length - 1].end)
				: ALIGNMENT_OFFSET - 1;

		const start = prevEnd + 1;
		const swapSectorSize = mibToSector(swapSizeMib);
		const end = start + swapSectorSize;

		newPartitions.push({
			number: String(currentPartNum),
			partitionPath: getPartPath(currentPartNum),
			filesystem: 'linux-swap(v1)',
			flags: [],
			mountpoint: ['swap'],
			name: 'Linux Swap',
			size: formatSector(swapSectorSize),
			start: formatSector(start),
			end: formatSector(end),
			typePartisi: 'primary'
		});

		availableSectors -= swapSectorSize;
		currentPartNum++;
	}

	// Root Partition
	const prevEnd =
		newPartitions.length > 0
			? parseSector(newPartitions[newPartitions.length - 1].end)
			: ALIGNMENT_OFFSET - 1;

	const start = prevEnd + 1;

	const rootSectorSize = availableSectors - ALIGNMENT_OFFSET;
	const end = start + rootSectorSize;

	newPartitions.push({
		number: String(currentPartNum),
		partitionPath: getPartPath(currentPartNum),
		filesystem: rootFilesystem,
		// IF MBR = BOOT
		flags: partitionTable === 'mbr' ? ['boot'] : undefined,
		mountpoint: ['/'],
		name: 'TeaLinuxOS',
		size: formatSector(rootSectorSize),
		start: formatSector(start),
		end: formatSector(end),
		typePartisi: 'primary'
	});

	newDisk.partitions = newPartitions;
	return newDisk;
};

export const getIdealSwapSize = (memoryBytes: number): number => {
	let idealSize = 0;

	if (memoryBytes < 8192) {
		idealSize = memoryBytes * 2;
	} else if (memoryBytes < 16384) {
		idealSize = memoryBytes * 1.5;
	} else if (memoryBytes < 32768) {
		idealSize = memoryBytes;
	} else {
		// memory >= 32768
		idealSize = memoryBytes / 2;
	}

	return Math.floor(idealSize);
};
