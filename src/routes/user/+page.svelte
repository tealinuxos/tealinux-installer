<script>
    import { goto } from "$app/navigation";
    import "../../app.css"

    let username = "";
    let password = "";
    let confirmPassword = "";
    let message = "";
    let showAlert = false;

    async function handleSubmit(event) {
        event.preventDefault();
        if (username === "" || password === "" || confirmPassword === "") {
            message = "Field username, password, dan confirm tidak boleh kosong"
            showAlert = true;
        }else if(password !== confirmPassword){
            message = "Value password harus sama dengan confirm"
            showAlert = true;
        } else {
            goto("/");
        }
    }
</script>

<nav class="font-sans flex flex-col text-center sm:flex-row sm:text-left sm:justify-between py-4 px-6 bg-white shadow sm:items-baseline w-full">
    <div class="mb-2 sm:mb-0">
      <a href="/" class="text-4xl"><span class="bg-green-400 px-1 text-white">Tea</span>Linux Installer</a>
    </div>
    <div>
      <a href="/" class="ml-2"><img class="w-20 -mt-14 -mb-5" src="../icon/back.svg" alt="Back"></a>
    </div>
</nav>


{#if showAlert}
    <div class="flex items-center p-4 mb-4 text-sm text-red-800 rounded-lg bg-red-50 dark:bg-gray-800 dark:text-red-400" role="alert">
        <svg class="flex-shrink-0 inline w-4 h-4 me-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
            <path d="M10 .5a9.5 9.5 0 1 0 9.5 9.5A9.51 9.51 0 0 0 10 .5ZM9.5 4a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3ZM12 15H8a1 1 0 0 1 0-2h1v-3H8a1 1 0 0 1 0-2h2a1 1 0 0 1 1 1v4h1a1 1 0 0 1 0 2Z"/>
        </svg>
        <span class="sr-only">Info</span>
        <div>
            {message}
        </div>
    </div>
{/if}

<form action="/" on:submit|preventDefault={handleSubmit} class="mt-8 max-w-md mx-auto">
    <div class="relative z-0 w-full mb-5 group">
        <!-- svelte-ignore a11y-autofocus -->
        <input type="text" bind:value={username} class="block py-2.5 px-0 w-full text-xl text-gray-900 bg-transparent border-0 border-b-2 border-black appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer" placeholder=" " autofocus/>
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="peer-focus:font-medium absolute text-xl text-black duration-500 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6">Username</label>
    </div>
    <div class="relative z-0 w-full mb-5 group">
        <input type="password" bind:value={password} class="block py-2.5 px-0 w-full text-xl text-gray-900 bg-transparent border-0 border-b-2 border-black appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer" placeholder=" " />
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="peer-focus:font-medium absolute text-xl text-black duration-500 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6">Password</label>
    </div>
    <div class="relative z-0 w-full mb-5 group">
        <input type="password" bind:value={confirmPassword} class="block py-2.5 px-0 w-full text-xl text-gray-900 bg-transparent border-0 border-b-2 border-black appearance-none dark:text-white dark:border-gray-600 dark:focus:border-blue-500 focus:outline-none focus:ring-0 focus:border-blue-600 peer" placeholder=" " />
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="peer-focus:font-medium absolute text-xl text-black duration-500 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:start-0 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto peer-focus:text-blue-600 peer-focus:dark:text-blue-500 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6">Confirm Password</label>
    </div>
    <button type="submit" class="bg-green-400 border text-xl font-bold text-white rounded-lg block w-full p-2.5">CREATE</button>
</form>