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

    onMount(() => {
        console.log(json);
        setBlueprint();
    })

</script>

<h1>WIP</h1>
<button on:click={startInstall}>Start Install</button>
