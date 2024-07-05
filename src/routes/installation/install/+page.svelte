<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';
	import { exit } from '@tauri-apps/api/process';
	import { onMount } from 'svelte';
	import Error from '$lib/components/modals/Error.svelte';

	let percentage = 100;
	let message = '';
    let errorMessage = null;

	const listenInstall = listen('INSTALL', (event) => {
		percentage = event.payload.percentage;
		message = event.payload.message;
	});

	const listenError = listen('ERROR', (event) => {
		errorMessage = event.payload.message;
	});

	const startInstall = async () => {
		await invoke('start_install');
	};

	const exitOk = async () => {
		await exit(0);
	};

	const reboot = async () => {
		await invoke('reboot');
	};
</script>

{#if errorMessage !== null}
    <Error errorMessage={errorMessage} />
{/if}

<section class="flex flex-col justify-between h-screen items-center text-center p-8">
	<h1 class="font-archivo text-3xl font-bold tracking-[-4.5%]">Installing...</h1>
	<div>
		<img src="/tealinux.png" alt="logo" class=" w-72 mx-auto" />
		<p class="font-poppinsemibold text-3xl tracking-[-5%] mb-3 mt-10">Tealinux</p>
		<p class="font-archivo text-6xl font-[500] tracking-[-4.5%]">Nikmatnya Sebuah Racikan</p>
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

	{#await startInstall()}
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
	{:then}
		<div class="flex flex-col">
			<span class=" font-poppinmedium font-semibold text-3xl tracking-[-5%]"
				>Installation Completed</span
			>
			<div class="flex gap-x-8 justify-center mt-4">
				<button
					class="bg-greenTealinux text-white font-poppinmedium font-medium p-2 rounded-md min-w-20"
					on:click={exitOk}>Exit</button
				>
				<button
					class="bg-greenTealinux text-white font-poppinmedium font-medium p-2 rounded-md min-w-20"
					on:click={reboot}>Reboot</button
				>
			</div>
		</div>
	{/await}
</section>
