<script>
    import { invoke } from "@tauri-apps/api/tauri";

    async function getDiskInfo() {
        let disks = await invoke('get_disk_info');
        console.log(disks);
    }
</script>

<h1 class="text-red-600">Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
<button on:click="{getDiskInfo}">Disk Info</button>
