import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/tauri";

export let json = {
    account: {
        fullname: null,
        username: null,
        hostname: null,
        password: null
    },
    locale: {
        main: null,
        locales: null
    },
    timezone: {
        region: null,
        city: null
    },
    partition: null
};
