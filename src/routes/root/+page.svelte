<script>
    import "../../app.css"
</script>

<nav class="font-sans flex flex-col text-center sm:flex-row sm:text-left sm:justify-between py-4 px-6 bg-white shadow sm:items-baseline w-full">
    <div class="mb-2 sm:mb-0">
      <a href="/" class="text-4xl"><span class="bg-green-400 px-1 text-white">Tea</span>Linux Installer</a>
    </div>
    <div>
      <a href="/" class="ml-2"><img class="w-20 -mt-14 -mb-5" src="../icon/back.svg" alt="Back"></a>
    </div>
</nav> 



<form class="mt-5 max-w-sm mx-auto" action="/">
    <div class="mb-5">
      <!-- svelte-ignore a11y-label-has-associated-control -->
      <label class="text-2xl">Root Password</label>
      <input type="password" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" required />
    </div>
    <div class="mb-7">
        <!-- svelte-ignore a11y-label-has-associated-control -->
        <label class="text-2xl">Confirm</label>
        <input type="password" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" required />
    </div>
    <button type="submit" class="bg-green-400 border text-xl font-bold text-white rounded-lg block w-full p-2.5">DONE</button>
</form>  