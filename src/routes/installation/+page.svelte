<script>
    
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";

    let online = false;

    const isOnline = async () => {
        invoke("is_online").then((status) => online = status);
    }

    onMount(() => {
        isOnline();
        console.log("/installation");
    });

</script>

<h1>Welcome to TealinuxOS Installer</h1>

{#if online}
    <a href="/installation/locale">
        <button>Next</button>
    </a>
{:else}
    <h3>Please connect to internet!</h3>
    <a href="/installation/locale">Bypass Internet</a>
{/if}
