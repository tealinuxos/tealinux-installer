<script>

    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { getBlueprint } from "../global.js";

    let timezone;
    let mainLocale;
    let locales;
    let formattedPartitions;
    let assignedPartitions;

    const setSummary = async () => {

        let json = await getBlueprint();

        timezone = json.timezone.region + '/' + json.timezone.city;
        mainLocale = json.locale.main;

        let partitions = json.disk.filter((partition) => partition.format !== false || partition.mountpoint !== null);

        formattedPartitions = partitions.filter((partition) => partition.format !== null);
        assignedPartitions = partitions.filter((partition) => partition.mountpoint !== null);
    }

    const printJson = async () => {
        await invoke("print_json");
    }

</script>

<h1>Summary</h1>

{#await setSummary()}
    Loading...
{:then}
    <h2 class="font-bold">Timezone</h2>
    <p>Set timezone to {timezone}.</p>

    <h2 class="font-bold">Locale</h2>
    <p>Set locale to ({mainLocale})</p>

    <h2 class="font-bold">Partition</h2>
    {#if formattedPartitions !== null}
        {#each formattedPartitions as partition}
            {@const path = partition.path}
            {@const filesystem = partition.format}
            <p>Format {path} as {filesystem}</p>
        {/each}
    {/if}

    {#if assignedPartitions !== null}
        {#each assignedPartitions as partition}
            {@const path = partition.path}
            {@const mountPoint = partition.mountpoint}
            <p>Assign {path} as {mountPoint}</p>
        {/each}
    {/if}

    <a class="border-2 border-black p-2" href="/installation/install">Start Install (Nginstal tenan)</a>
    <button class="border-2 border-black p-2" on:click={printJson}>Print JSON</button>
{/await}
