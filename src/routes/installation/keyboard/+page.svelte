<script>
	import Sidebar from "../../../lib/components/Sidebar.svelte";


</script>
<Sidebar />
<section class="flex flex-col items-center justify-center h-screen">
    <form class="text-center w-[50dvw] p-8 rounded-md min-h-[50dvh]">
        <div>
            <h1 class="text-center mb-6 font-bold text-[32px] font-archivo">Select Keyboard Layout</h1>
        </div>
        <div class="max-w-md mx-auto mb-4">
            <h2 class="font-poppin text-left mb-2 font-medium">Region</h2>
            <div class="relative flex items-center w-[451px] h-[45px] rounded-lg bg-white overflow-hidden border border-greyBorder">
                <input 
                type="text"
                placeholder="select keyboard.."
                class="peer h-full w-full outline-none text-sm text-gray-700 pr-2 pl-[12px] font-poppin"
                >
                <svg class="mr-[16px]" width="20" height="21" viewBox="0 0 20 21" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M19 19.5L14.65 15.15M17 9.5C17 13.9183 13.4183 17.5 9 17.5C4.58172 17.5 1 13.9183 1 9.5C1 5.08172 4.58172 1.5 9 1.5C13.4183 1.5 17 5.08172 17 9.5Z" stroke="black" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
            </div>
            <div class="max-w-md mx-auto mt-30 mt-[26px]">
                <div class="grid grid-cols-2 gap-[295px]">
                    <a href="/installation" class="text-white bg-greyButton hover:bg-gray-500 focus:ring-4 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 h-[44px] w-[76px]">Back</a>
                    <a href="/installation/timezone" class="text-white bg-greyButton hover:bg-gray-500 focus:ring-4 focus:ring-gray-900 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 focus:outline-none h-[44px] w-[76px]">Next</a>
                </div>
            </div>
        </div>
    </form>
</section>