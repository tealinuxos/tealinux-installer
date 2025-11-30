import type { BootPath } from '$types/installation/partitioning/partitioning.types';
import { writable } from 'svelte/store';

export const partitionMethod = writable<BootPath | ''>('');
