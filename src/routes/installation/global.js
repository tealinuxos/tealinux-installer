import { invoke } from "@tauri-apps/api/tauri";

export const getRead = async () => {

    let json = await invoke("get_read_from_opt");

    return JSON.parse(json);
};

export const getBlueprint = async () => {

    let json = await invoke("get_blueprint_from_opt");

    return JSON.parse(json);
};

export const getShortLocale = (locale) => {

    let shortLocale = locale.slice(0, 5).replace('_', '-').toLowerCase();

    return shortLocale;
}
