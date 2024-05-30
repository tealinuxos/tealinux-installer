<script>

    import { invoke } from "@tauri-apps/api/tauri";

    let json;

    let showModelName = false;

    async function handleGetJSON() {

        invoke('get_read_json').then((response) => {
            json = JSON.parse(response);
            console.log(json);
            showModelName = true;
        }).await
    }

</script>

<div class="flex flex-col justify-center p-5"> 
    <button class="bg-black text-white" on:click="{handleGetJSON}">
        Get JSON
    </button>
    {#if showModelName}
        <h1>{json.model.brandName}</h1>
    {/if}
</div>
