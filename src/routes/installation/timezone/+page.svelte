<script>

    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { json } from "../stores.js";

    let timezones;
    let showTimezone = false;
    let search = "";

    const getTimezone = async () => {
        invoke("get_timezone_json").then((response) => {
            timezones = JSON.parse(response);
            showTimezone = true;
        });
    };
    
    let selectedRegion = null;
    let selectedCity = null;

    const handleSetTimezone = async () => {
        json.timezone.region = selectedRegion;
        json.timezone.city = selectedCity;
        console.log(json);
    };

    let showCity = false;

    onMount(() => {
        getTimezone();
    })

</script>

<h3>Timezone</h3>

{#if showTimezone}
<div class="w-1/2 float-left flex flex-col">
    {#each timezones as timezone}
        {@const region = timezone.region}

        <div class="flex flex-row">
            <input type="radio" id={region + "ID"} value={region} bind:group={selectedRegion} on:change={() => showCity = true} />
            <label for={region + "ID"}>{region}</label>
        </div>
    {/each}
</div>
{/if}
{#if showCity}
    {#if timezones[timezones.findIndex(timezone => timezone.region === selectedRegion)].city != null}
        <div class="w-1/2 float-right flex flex-col">
            {#each timezones[timezones.findIndex(timezone => timezone.region === selectedRegion)].city as city}
                <div class="flex flex-row">
                    <input type="radio" id={city + "ID"} value={city} bind:group={selectedCity} />
                    <label for={city + "ID"}>{city}</label>
                </div>
            {/each}
            {#if selectedCity != undefined}
                <a href="/installation/account" on:click={handleSetTimezone}>Next</a>
            {/if}
        </div>
    {:else}
        <a href="/installation/account" on:click={handleSetTimezone}>Next</a>
    {/if}
{/if}
