/**
 * This type came from get_keyboard_json() method
 * This type is the result type of the Keyboard::list();
 */

export interface Variant {
	code: string | null; // Idk if this was intended or not
	name: string;
}

export interface Keyboard {
	code: string;
	name: string;
	variant: Variant[];
}
