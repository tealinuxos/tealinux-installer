import { commands, type BluePrint } from '$types/commands';
import type { ReadFromOpt } from '$types/read-from-opt';

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
