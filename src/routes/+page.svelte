<script>

    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";

    let json;
    let showModelName = false;

    async function handleGetJSON() {
        invoke('get_read_json').then((response) => {
            json = JSON.parse(response);
            console.log(json);
            showModelName = true;
        }).await
    }

    onMount(() => {
        handleGetJSON();
    });

</script>

{#if showModelName}
    <div class="flex flex-col justify-center p-5 gap-y-4 bg-slate-200 w-fit rounded-xl shadow-xl hover:scale-105 hover:translate-x-2 hover:translate-y-2 transition-all">
        <h1 class="font-bold ">Device Information</h1> 
        <div class="flex justify-between">

            <p>test</p>
            <p>babababa</p>

            <h1 class="font-bold">Battery : {json.battery.capacity}%</h1>
            <h1 class="font-bold flex items-center">
            {#if json.online.status}
                online <span class="w-3 ml-2 aspect-square rounded-full bg-green-400 inline-block border border-slate-600"></span>
            {:else}
                offline <span class="w-3 ml-2 aspect-square rounded-full bg-red-500 inline-block border border-slate-600"></span>
            {/if}
            </h1>        
        </div>
        <hr class="border border-slate-800"/>
        <p>{json.model.systemProductName+" - "+json.model.systemVersion}</p>
        <table>
            <tr>
                <td>OS</td>
                <td>: {json.operatingSystems ? json.operatingSystems.join(", ") : ""}</td>
            </tr>
            <tr>
                <td>CPU</td>
                <td>: {json.lspci.cpu ? json.lspci.cpu : ""}</td>
            </tr>
            <tr>
                <td>VGA</td>
                <td>: {json.lspci.vga ? json.lspci.vga : ""}</td>
            </tr>
            <tr>
                <td>Memory Capacity</td>
                <td>: {json.memory.capacity ? json.memory.capacity : ""} Mb</td>
            </tr>
            <tr>
                <td>Memory Used</td>
                <td>: {json.memory.used ? json.memory.used : ""} Mb</td>
            </tr>
            <tr>
                <td>Memory Free</td>
                <td>: {json.memory.capacity - json.memory.used} Mb</td>
            </tr>
        </table>
    </div>
{/if}
<a href="/partitioning">Partitioning</a>
