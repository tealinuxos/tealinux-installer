export type BootType = 'SINGLE' | 'DUAL' | 'MANUAL';
export type BootPath = 'single' | 'dual' | 'manual';

export type PreviewKey = 'BEFORE' | 'AFTER';
export type PreviewLabel = 'Before' | 'After';

export const Method: Record<BootType, BootPath> = {
	SINGLE: 'single',
	DUAL: 'dual',
	MANUAL: 'manual'
};

export const Preview: Record<PreviewKey, PreviewLabel> = {
	BEFORE: 'Before',
	AFTER: 'After'
};
