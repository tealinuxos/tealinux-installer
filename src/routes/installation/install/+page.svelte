<script>

    import { invoke } from "@tauri-apps/api/tauri";
    import { listen } from "@tauri-apps/api/event";
    import { exit } from "@tauri-apps/api/process";
    import { onMount } from "svelte";

    let percentage = 0;
    let message = "";

    const unlisten = listen("INSTALL", (event) => {
        percentage = event.payload.percentage;
        message = event.payload.message;

        if (message === "Installation completed")
        {
            alert(message);
        }
    })

    const startInstall = async () => {
        await invoke("start_install");
    }

    const exitOk = async () => {
        await exit(0);
    }

    const reboot = async () => {
        await invoke("reboot");
    }

</script>

<h1>Installing</h1>

{#await startInstall()}
    <div class="flex flex-col">
        <span>{percentage}%</span>
        <span>{message}</span>
    </div>
{:then}
    <div class="flex flex-col">
        <span>Installation Completed</span>
        <button class="border-2 border-black p-2" on:click={exitOk}>Exit</button>
        <button class="border-2 border-black p-2" on:click={reboot}>Reboot</button>
    </div>
{/await}
