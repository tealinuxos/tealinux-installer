export interface Variant {
	code: string | null;
	name: string;
}

export interface Keyboard {
	code: string;
	name: string;
	variant: Variant[];
}
