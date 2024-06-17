<script>

    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { json } from "../stores.js";

    let locales;
    let showLocales = false;
    let selectedLocales;

    const getLocale = async () => {
        invoke("get_locale_json").then((response) => {
            locales = JSON.parse(response);
            showLocales = true;
        });
    }

    const handleSetLocale = async () => {
        json.locale.locales = selectedLocales;
        json.locale.main = selectedLocales[0];
        console.log(json);
    }

    onMount(() => {
        getLocale();
    })

</script>

<h1>Locales</h1>

{#if showLocales}
    <form class="flex flex-col" on:submit|preventDefault={handleSetLocale}>
    {#each locales as locale}
        
        {@const name = locale.name}

    <div class="flex flex-row">
        <input type="checkbox" bind:group={selectedLocales} value={name} />
        <label>{name}</label>
    </div>

    {/each}
    <a href="/install/timezone" on:click={handleSetLocale}>Next</a>
    </form>
{/if}
