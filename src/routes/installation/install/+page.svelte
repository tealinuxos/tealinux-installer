<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { json } from "../stores.js";

    const setBlueprint = async () => {
        let blueprint = JSON.stringify(json);
        console.log(blueprint);

        await invoke("set_blueprint_json", { json: blueprint });
    }

    const startInstall = async () => {
        await invoke("start_install");
    }

    const printJson = async () => {
        await invoke("print_json");
    }

    onMount(() => {
        console.log(json);
        setBlueprint();
    })

</script>

<h1>WIP</h1>
<button class="border-2 border-black p-2" on:click={startInstall}>Start Install(Nginstal tenan)</button>
<button class="border-2 border-black p-2" on:click={printJson}>Print JSON</button>
