import { commands, type BluePrint} from '$types/commands';
import type { Keyboard } from '$types/keyboard-variant';
import type { Locale } from '$types/locale-variant';
import type { ReadFromOpt } from '$types/read-from-opt';
import type { Timezone } from '$types/timezone-variant';

export const getSystemInfo = async (): Promise<ReadFromOpt | null> => {
	try {
		const data = await commands.getReadFromOpt();
		const json: ReadFromOpt = typeof data === 'string' ? JSON.parse(data) : data;
		return json;
	} catch (error) {
		console.error('Error fetching system info:', error);
		return null;
	}
};

export const getBlueprintInfo = async (): Promise<BluePrint | null> => {
	try {
		const data = await commands.getBlueprintFromOpt();
		const json: BluePrint = typeof data === 'string' ? JSON.parse(data) : data;
		return json;
	} catch (error) {
		console.error('Error fetching blueprint info:', error);
		return null;
	}
};

export const getKeyboardJSON = async (): Promise<Keyboard[] | null> => {
	try {
		const data = await commands.getKeyboardJson();
		const json: Keyboard[] = typeof data === 'string' ? JSON.parse(data) : data;
		return json;
	} catch (error) {
		console.error('Error fetching keyboard info:', error);
		return null;
	}
};

export const getLocaleJSON = async (): Promise<Locale[] | null> => {
	try {
		const data = await commands.getLocaleJson();
		const json: Locale[] = typeof data === 'string' ? JSON.parse(data) : data;
		return json;
	} catch (error) {
		console.error('Error fetching locale info:', error);
		return null;
	}
};

export const getTimezoneJSON = async (): Promise<Timezone[] | null> => {
	try {
		const data = await commands.getTimezoneJson();
		const json: Timezone[] = typeof data === 'string' ? JSON.parse(data) : data;
		return json;
	} catch (error) {
		console.error('Error fetching timezone info:', error);
		return null;
	}
};
