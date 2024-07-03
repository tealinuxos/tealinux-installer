<script>
    import Sidebar from '$lib/components/Sidebar.svelte';
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
<Sidebar />
<section class="flex flex-col items-center justify-center h-auto">
    <form class=" text-center w-[50dvw] p-8 rounded-md min-h-[50dvh]">
        <h1 class="text-center mb-6 font-bold text-[32px] font-archivo">Summary</h1>
        <div class="mb-4">
            <div class="flex justify-between">
                <h2 class="font-poppin mb-2 font-semibold">Keyboard layout</h2>
                <svg class="text-left mb-2" width="30" height="30" viewBox="0 0 30 30" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <rect width="30" height="30" rx="9" fill="#929AAB"/>
                    <path d="M22.625 10.9501L19.0505 7.37499C18.9317 7.25611 18.7906 7.1618 18.6352 7.09746C18.4799 7.03312 18.3135 7 18.1453 7C17.9772 7 17.8107 7.03312 17.6554 7.09746C17.5001 7.1618 17.359 7.25611 17.2401 7.37499L7.37521 17.2402C7.25583 17.3586 7.16119 17.4996 7.09677 17.6549C7.03236 17.8103 6.99947 17.9768 7.00001 18.145V21.72C7.00001 22.0595 7.13486 22.3851 7.37491 22.6251C7.61496 22.8651 7.94054 23 8.28002 23H11.8553C12.0234 23.0005 12.19 22.9676 12.3453 22.9032C12.5007 22.8388 12.6416 22.7442 12.7601 22.6248L22.625 12.7604C22.7439 12.6415 22.8382 12.5004 22.9025 12.3451C22.9669 12.1898 23 12.0233 23 11.8552C23 11.6871 22.9669 11.5206 22.9025 11.3653C22.8382 11.21 22.7439 11.0689 22.625 10.9501ZM8.54482 17.8802L15.7729 10.6525L17.1073 11.9876L9.88004 19.2145L8.54482 17.8802ZM8.28002 19.4249L10.5752 21.72H8.28002V19.4249ZM12.1201 21.4553L10.7849 20.1201L18.0129 12.8924L19.3474 14.2275L12.1201 21.4553Z" fill="white"/>
                </svg>  
            </div>
            <div class="relative flex items-center w-full h-[45px] rounded-[10px] bg-white overflow-hidden border border-greyBorder mb-2 font-poppin text-[14px] mx-auto">
                <input type="text" id="" class="peer h-full w-full outline-none text-sm text-gray-700 pr-2 pl-[12px]">
            </div>
        </div>
        

        <div class="mx-auto mb-4">
            <div class="flex justify-between">
                <h2 class="font-poppin text-left mb-2 font-semibold">Timezone</h2>
                <svg class="mb-2" width="30" height="30" viewBox="0 0 30 30" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <rect width="30" height="30" rx="9" fill="#929AAB"/>
                    <path d="M22.625 10.9501L19.0505 7.37499C18.9317 7.25611 18.7906 7.1618 18.6352 7.09746C18.4799 7.03312 18.3135 7 18.1453 7C17.9772 7 17.8107 7.03312 17.6554 7.09746C17.5001 7.1618 17.359 7.25611 17.2401 7.37499L7.37521 17.2402C7.25583 17.3586 7.16119 17.4996 7.09677 17.6549C7.03236 17.8103 6.99947 17.9768 7.00001 18.145V21.72C7.00001 22.0595 7.13486 22.3851 7.37491 22.6251C7.61496 22.8651 7.94054 23 8.28002 23H11.8553C12.0234 23.0005 12.19 22.9676 12.3453 22.9032C12.5007 22.8388 12.6416 22.7442 12.7601 22.6248L22.625 12.7604C22.7439 12.6415 22.8382 12.5004 22.9025 12.3451C22.9669 12.1898 23 12.0233 23 11.8552C23 11.6871 22.9669 11.5206 22.9025 11.3653C22.8382 11.21 22.7439 11.0689 22.625 10.9501ZM8.54482 17.8802L15.7729 10.6525L17.1073 11.9876L9.88004 19.2145L8.54482 17.8802ZM8.28002 19.4249L10.5752 21.72H8.28002V19.4249ZM12.1201 21.4553L10.7849 20.1201L18.0129 12.8924L19.3474 14.2275L12.1201 21.4553Z" fill="white"/>
                </svg>  
            </div>
            <div class="relative flex items-center w-full h-[45px] rounded-[10px] bg-white overflow-hidden border border-greyBorder mb-2 font-poppin text-[14px] mx-auto">
                <h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">Region: </h2>
                <input type="text" id="" class="peer h-full w-full outline-none text-sm text-gray-700 pr-2 pl-[12px]">
            </div>
            <div class="relative flex items-center w-full h-[45px] rounded-[10px] bg-white overflow-hidden border border-greyBorder mb-2 font-poppin text-[14px] mx-auto">
                <h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">City: </h2>
                <input type="text" id="" class="peer h-full w-full outline-none text-sm text-gray-700 pr-2 pl-[12px]">
            </div>
        </div>

        <div class="mx-auto mb-4">
            <div class="flex justify-between">
                <h2 class="font-poppin text-left mb-2 font-semibold">Locale</h2>
                <svg class="mb-2" width="30" height="30" viewBox="0 0 30 30" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <rect width="30" height="30" rx="9" fill="#929AAB"/>
                    <path d="M22.625 10.9501L19.0505 7.37499C18.9317 7.25611 18.7906 7.1618 18.6352 7.09746C18.4799 7.03312 18.3135 7 18.1453 7C17.9772 7 17.8107 7.03312 17.6554 7.09746C17.5001 7.1618 17.359 7.25611 17.2401 7.37499L7.37521 17.2402C7.25583 17.3586 7.16119 17.4996 7.09677 17.6549C7.03236 17.8103 6.99947 17.9768 7.00001 18.145V21.72C7.00001 22.0595 7.13486 22.3851 7.37491 22.6251C7.61496 22.8651 7.94054 23 8.28002 23H11.8553C12.0234 23.0005 12.19 22.9676 12.3453 22.9032C12.5007 22.8388 12.6416 22.7442 12.7601 22.6248L22.625 12.7604C22.7439 12.6415 22.8382 12.5004 22.9025 12.3451C22.9669 12.1898 23 12.0233 23 11.8552C23 11.6871 22.9669 11.5206 22.9025 11.3653C22.8382 11.21 22.7439 11.0689 22.625 10.9501ZM8.54482 17.8802L15.7729 10.6525L17.1073 11.9876L9.88004 19.2145L8.54482 17.8802ZM8.28002 19.4249L10.5752 21.72H8.28002V19.4249ZM12.1201 21.4553L10.7849 20.1201L18.0129 12.8924L19.3474 14.2275L12.1201 21.4553Z" fill="white"/>
                </svg>  
            </div>
            <div class="relative flex items-center w-full h-[45px] rounded-[10px] bg-white overflow-hidden border border-greyBorder mb-2 font-poppin text-[14px] mx-auto">
                <h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">Main locale: </h2>
                <input type="text" id="" class="peer h-full w-full outline-none text-sm text-gray-700 pr-2 pl-[12px]">
            </div>
        </div>
        
        <div class="mx-auto mb-4">
            <div class="flex justify-between">
                <h2 class="font-poppin text-left mb-2 font-semibold">User</h2>
                <svg class="mb-2" width="30" height="30" viewBox="0 0 30 30" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <rect width="30" height="30" rx="9" fill="#929AAB"/>
                    <path d="M22.625 10.9501L19.0505 7.37499C18.9317 7.25611 18.7906 7.1618 18.6352 7.09746C18.4799 7.03312 18.3135 7 18.1453 7C17.9772 7 17.8107 7.03312 17.6554 7.09746C17.5001 7.1618 17.359 7.25611 17.2401 7.37499L7.37521 17.2402C7.25583 17.3586 7.16119 17.4996 7.09677 17.6549C7.03236 17.8103 6.99947 17.9768 7.00001 18.145V21.72C7.00001 22.0595 7.13486 22.3851 7.37491 22.6251C7.61496 22.8651 7.94054 23 8.28002 23H11.8553C12.0234 23.0005 12.19 22.9676 12.3453 22.9032C12.5007 22.8388 12.6416 22.7442 12.7601 22.6248L22.625 12.7604C22.7439 12.6415 22.8382 12.5004 22.9025 12.3451C22.9669 12.1898 23 12.0233 23 11.8552C23 11.6871 22.9669 11.5206 22.9025 11.3653C22.8382 11.21 22.7439 11.0689 22.625 10.9501ZM8.54482 17.8802L15.7729 10.6525L17.1073 11.9876L9.88004 19.2145L8.54482 17.8802ZM8.28002 19.4249L10.5752 21.72H8.28002V19.4249ZM12.1201 21.4553L10.7849 20.1201L18.0129 12.8924L19.3474 14.2275L12.1201 21.4553Z" fill="white"/>
                </svg>  
            </div>
            <div class="relative flex items-center w-full h-[45px] rounded-[10px] bg-white overflow-hidden border border-greyBorder mb-2 font-poppin text-[14px] mx-auto">
                <h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">Computer name: </h2>
                <input type="text" id="" class="peer h-full w-full outline-none text-sm text-gray-700 pr-2 pl-[12px]">
            </div>
            <div class="relative flex items-center w-full h-[45px] rounded-[10px] bg-white overflow-hidden border border-greyBorder mb-2 font-poppin text-[14px] mx-auto">
                <h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">Username: </h2>
                <input type="text" id="" class="peer h-full w-full outline-none text-sm text-gray-700 pr-2 pl-[12px]">
            </div>
            <div class="relative flex items-center w-full h-[45px] rounded-[10px] bg-white overflow-hidden border border-greyBorder mb-2 font-poppin text-[14px] mx-auto">
                <h2 class="flex whitespace-nowrap font-poppin font-medium text-[14px] ml-[12px]">Password: </h2>
                <input type="text" id="" class="peer h-full w-full outline-none text-sm text-gray-700 pr-2 pl-[12px]">
                <svg class="mr-[17.18px]" width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <mask id="mask0_461_173" style="mask-type:luminance" maskUnits="userSpaceOnUse" x="0" y="0" width="24" height="24">
                        <path d="M24 0H0V24H24V0Z" fill="white"/>
                    </mask>
                    <g mask="url(#mask0_461_173)">
                        <path d="M17.8817 19.2971C16.1229 20.4127 14.0824 21.0034 11.9997 21.0001C6.60766 21.0001 2.12166 17.1201 1.18066 12.0001C1.61069 9.67078 2.78229 7.54296 4.52066 5.93407L1.39166 2.80807L2.80666 1.39307L22.6057 21.1931L21.1907 22.6071L17.8817 19.2971ZM5.93466 7.35007C4.57566 8.58566 3.62898 10.2089 3.22266 12.0001C3.53495 13.3666 4.16192 14.6412 5.05366 15.7227C5.9454 16.8041 7.07729 17.6625 8.35921 18.2294C9.64114 18.7963 11.0377 19.0562 12.4378 18.9882C13.8378 18.9203 15.2027 18.5265 16.4237 17.8381L14.3957 15.8101C13.5324 16.3539 12.5099 16.5882 11.4959 16.4745C10.482 16.3609 9.5367 15.906 8.81523 15.1845C8.09376 14.4631 7.63889 13.5178 7.52522 12.5039C7.41156 11.4899 7.64584 10.4674 8.18966 9.60407L5.93466 7.35007ZM12.9137 14.3281L9.67166 11.0861C9.49372 11.539 9.45185 12.0341 9.55117 12.5105C9.65048 12.9868 9.88668 13.4239 10.2308 13.768C10.5749 14.1121 11.012 14.3483 11.4884 14.4476C11.9647 14.5469 12.4598 14.505 12.9127 14.3271L12.9137 14.3281ZM20.8067 16.5921L19.3757 15.1621C20.0442 14.2094 20.5201 13.1353 20.7767 12.0001C20.5049 10.8098 19.994 9.68721 19.2748 8.70056C18.5557 7.71391 17.6435 6.88379 16.5936 6.26067C15.5437 5.63755 14.378 5.23443 13.1674 5.07583C11.9569 4.91723 10.7267 5.00644 9.55166 5.33807L7.97366 3.76007C9.22066 3.27007 10.5797 3.00007 11.9997 3.00007C17.3917 3.00007 21.8777 6.88007 22.8187 12.0001C22.5123 13.6658 21.8236 15.2377 20.8067 16.5921ZM11.7227 7.50807C12.3592 7.46873 12.9968 7.56513 13.5932 7.79088C14.1897 8.01663 14.7313 8.36658 15.1822 8.81752C15.6332 9.26846 15.9831 9.81009 16.2089 10.4066C16.4346 11.003 16.531 11.6406 16.4917 12.2771L11.7227 7.50807Z" fill="#757575"/>
                    </g>
                </svg>
            </div>
        </div>

        <div class="w-full mx-auto mb-4">
            <div class="flex relative items-center justify-between">
                <h2 class="font-poppin font-semibold text-[15px]">Partition installation</h2>
                <svg class="" width="30" height="30" viewBox="0 0 30 30" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <rect width="30" height="30" rx="9" fill="#929AAB"/>
                    <path d="M22.625 10.9501L19.0505 7.37499C18.9317 7.25611 18.7906 7.1618 18.6352 7.09746C18.4799 7.03312 18.3135 7 18.1453 7C17.9772 7 17.8107 7.03312 17.6554 7.09746C17.5001 7.1618 17.359 7.25611 17.2401 7.37499L7.37521 17.2402C7.25583 17.3586 7.16119 17.4996 7.09677 17.6549C7.03236 17.8103 6.99947 17.9768 7.00001 18.145V21.72C7.00001 22.0595 7.13486 22.3851 7.37491 22.6251C7.61496 22.8651 7.94054 23 8.28002 23H11.8553C12.0234 23.0005 12.19 22.9676 12.3453 22.9032C12.5007 22.8388 12.6416 22.7442 12.7601 22.6248L22.625 12.7604C22.7439 12.6415 22.8382 12.5004 22.9025 12.3451C22.9669 12.1898 23 12.0233 23 11.8552C23 11.6871 22.9669 11.5206 22.9025 11.3653C22.8382 11.21 22.7439 11.0689 22.625 10.9501ZM8.54482 17.8802L15.7729 10.6525L17.1073 11.9876L9.88004 19.2145L8.54482 17.8802ZM8.28002 19.4249L10.5752 21.72H8.28002V19.4249ZM12.1201 21.4553L10.7849 20.1201L18.0129 12.8924L19.3474 14.2275L12.1201 21.4553Z" fill="white"/>
                </svg>  
            </div>
            
            <h2 class="font-poppin font-medium text-[16px] text-center">After</h2>
            <div class="flex items-center justify-center">
                <h2 class="font-poppin font-medium text-[17px] mr-[10px]">/dev/sda</h2>
                <div class="w-[597px] h-[27px] bg-[#C85036] rounded-[128px]">
                    <div class="bg-[#3293C8] h-[27px] w-[50%] rounded-[128px]"></div>
                </div>
            </div>
        </div>
        
    </form>
</section>

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

