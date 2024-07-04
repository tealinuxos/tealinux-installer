<script>
	import SysInfoSide from './SysInfoSide.svelte';
	import { sysInfoActivated } from './store.js'; // Path ke file store.js

	let isOpen = false;

	function toggleSidebar() {
		isOpen = !isOpen;
	}

	function openSysInfo() {
		sysInfoActivated.set(true);
	}
</script>

<SysInfoSide />
<sidebar
	class="sidebar fixed top-0 left-0 z-30 h-screen transition-colors duration-500 flex flex-col p-4 space-y-4 {isOpen
		? ' bg-[#C8E8D6]'
		: 'collapsed bg-greenTealinux'}"
>
	<button on:click={toggleSidebar} class=" self-start transition-all duration-1000">
		{#if isOpen}
			<svg
				class="h-8 w-8 text-black p-0.5 border-2 border-black rounded-md"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M6 18L18 6M6 6l12 12"
				></path>
			</svg>
		{:else}
			<svg
				class="h-8 w-8 text-white"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M4 6h16M4 12h16M4 18h16"
				></path>
			</svg>
		{/if}
	</button>
	{#if isOpen}
		<button
			class="flex items-center space-x-2 bg-white p-2 rounded-md border-2 border-black"
			on:click={openSysInfo}
		>
			<svg
				class="h-6 w-6 text-black"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M9.75 17l5.25-5.25-5.25-5.25"
				></path>
			</svg>
			<span class="font-semibold">System information</span>
		</button>
		<button class="flex items-center space-x-2 bg-white p-2 rounded-md border-2 border-black">
			<svg
				class="h-6 w-6 text-black"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M8 6h13M8 12h13m-7 6h7"
				></path>
			</svg>
			<span class="font-semibold">Summary</span>
		</button>
	{/if}
</sidebar>

<style>
	.sidebar {
		width: 15rem; /* Default width */
		transition: width 0.3s ease;
	}
	.sidebar.collapsed {
		width: 4rem; /* Smaller width for collapsed state */
	}
</style>
