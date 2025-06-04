<script>
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	// import { exit } from '@tauri-apps/plugin-process';
	import { onMount } from 'svelte';
	// import Error from '$lib/components/modals/Error.svelte';

	let percentage = $state(100);
	let message = $state('');
	let errorMessage = $state(null);

	const listenInstall = listen('INSTALL', (event) => {
		percentage = event.payload.percentage;
		message = event.payload.message;
	});

	const listenError = listen('ERROR', (event) => {
		errorMessage = event.payload.message;
	});

	const startInstall = async () => {
		//await invoke('start_install'); // !!! PLEASE COMMENT THIS IF YOU ARE DEVELOPING !!!
		console.log('installing');
	};

	// const exitOk = async () => {
	// 	await exit(0);
	// };

	const reboot = async () => {
		await invoke('reboot');
	};
</script>

<!-- {#if errorMessage !== null} -->
<!--     <Error errorMessage={errorMessage} /> -->
<!-- {/if} -->

<section
	class="flex flex-col justify-between h-screen items-center text-center p-8 text-white font-archivo"
>
	<!-- <h1 class="font-archivo text-3xl font-bold tracking-[-4.5%]">Installing...</h1> -->
	<div class="flex-1 flex text-left min-w-[812px] w-[812px] justify-between items-center">
		<div class="max-w-[541px]">
			<p class="font-[500] text-5xl leading-[1.4] h-[146px]">
				Fast, efficient, and <br /> optimized for developers
			</p>
			<p class=" font-jakarta text-xl w-[482px] mt-2">
				Tea Linux OS, built on Arch Linux, offers cutting-edge tools and features for creators,
				coders, and administrators. Empower your productivity with the latest innovations in this
				release."
			</p>
		</div>
		<img src="/tealinux.svg" alt="logo" class=" w-[222px] h-[233px]" />
	</div>

	<!-- 
{#await startInstall()}
	<div class="flex flex-col">
		<span>{percentage}%</span>
		<span>{message}</span>
	</div>
{:then}
	<div class="flex flex-col">
		<span>Installation Completed</span>
		<button class="border-2 border-black p-2" on:click={exitOk}>Exit</button>
		<button class="border-2 border-black p-2" on:click={reboot}>Reboot</button>
	</div>
{/await} -->

	<div>
		<span class=" font-poppinsemibold text-3xl tracking-[-5%]">{message}</span>
		<div class="flex flex-col justify-center w-[90dvw] items-center h-10 mt-5">
			<div
				class="relative flex h-full items-center w-[100%] mx-auto bg-[#white] border border-[#B1B1B1] rounded-[128px]"
			>
				<div
					class="absolute bg-[#26A768] h-full rounded-[128px] flex items-center"
					style="width: {percentage}%"
				>
					<span class="absolute right-[2%] font-poppinsemibold font-bold text-2xl"
						>{percentage}%</span
					>
				</div>
			</div>
		</div>
	</div>
</section>
