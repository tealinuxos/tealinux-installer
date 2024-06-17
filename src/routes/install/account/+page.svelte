<script>

    import { onMount } from "svelte";
    import { json } from "../stores.js";

    let fullname, username, hostname, password;

    const handleSetAccount = () => {
        json.account.fullname = fullname;
        json.account.username = username;
        json.account.hostname = hostname;
        json.account.password = password;
    }

</script>

<h1>Account</h1>

<form class="flex flex-col">
    <input class="p-2" type="text" bind:value={fullname} placeholder="Fullname" />
    <input class="p-2" type="text" bind:value={username} placeholder="username" />
    <input class="p-2" type="text" bind:value={hostname} placeholder="hostname" />
    <input class="p-2" type="password" bind:value={password} placeholder="password" />
    <a href="/install/partition" on:click={handleSetAccount}>Next</a>
</form>
