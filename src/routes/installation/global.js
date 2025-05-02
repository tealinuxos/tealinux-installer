import { invoke } from "@tauri-apps/api/core";

export const getRead = async () => {

    let json = await invoke("get_read_from_opt");

    return JSON.parse(json);
};

export const getBlueprint = async () => {

    let json = await invoke("get_blueprint_from_opt");

    return JSON.parse(json);
};

export const getShortLocale = (locale) => {

    let pattern = /([a-z]+)_([A-Z]+)/;
    let matched = locale.match(pattern);

    let shortLocale = matched[1] + '-' + matched[2];

    return shortLocale;
}
