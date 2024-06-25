<script>

    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { json } from "../stores.js";

    let timezone = json.timezone.region + '/' + json.timezone.city;
    let mainLocale = json.locale.locales[0];
    let locales = json.locale.locales.toString();
    let partitions = json.partition.filter((partition) => partition.format !== false || partition.mountpoint !== null);
    let formattedPartitions = partitions.filter((partition) => partition.format !== null);
    let assignedPartitions = partitions.filter((partition) => partition.mountpoint !== null);

    const setBlueprint = async () => {

        let blueprint = JSON.stringify(json);

        await invoke("set_blueprint_json", { json: blueprint });
    }

    const printJson = async () => {
        await invoke("print_json");
    }

    onMount(() => {
        setBlueprint();
    })

</script>

<h1>Summary</h1>

<h2 class="font-bold">Timezone</h2>
<p>Set timezone to {timezone}.</p>

<h2 class="font-bold">Locale</h2>
<p>Set locale to ({locales}),</p>
<p>with ({mainLocale}) as main locale</p>

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
