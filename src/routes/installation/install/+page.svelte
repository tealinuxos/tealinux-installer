<script>
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';
	import Button from '$lib/components/ui/Button.svelte';

	let percentage = $state(0);

	let message = $state('Starting installation...');


	let errorMessage = $state(null);
	let isFinalizing = $state(false);

	const listenInstall = listen('INSTALL', (event) => {
		percentage = event.payload.percentage;
		message = event.payload.message;
		if (percentage >= 95) {
			isFinalizing = true;
		}
	});

	const listenError = listen('ERROR', (event) => {
		errorMessage = event.payload.message;
	});

	const startInstall = async () => {
		try {
			await invoke('start_install'); // !!! COMMENT THIS DURING DEVELOPMENT !!!
		} catch (err) {
			errorMessage = err.toString();
		}
	};

	const reboot = async () => {
		await invoke('reboot');
	};
</script>

<section class="flex flex-col justify-between h-screen items-center text-center p-8 text-white font-archivo">
	<div class="flex-1 flex text-left min-w-[812px] w-[812px] justify-between items-center">
		<div class="max-w-[541px]">
			<p class="font-[500] text-5xl leading-[1.4] h-[146px]">
				Fast, efficient, and <br /> optimized for developers
			</p>
			<p class="font-jakarta text-xl w-[482px] mt-2">
				Tea Linux OS, built on Arch Linux, offers cutting-edge tools and features for creators,
				coders, and administrators. Empower your productivity with the latest innovations in this
				release.
			</p>
		</div>
		<img src="/tealinux.svg" alt="logo" class="w-[222px] h-[233px]" />
	</div>

	{#await startInstall()}
		<div class="w-full">
			{#if isFinalizing}
				<div class="text-center mb-8">
					<h2 class="font-poppinsemibold text-2xl tracking-[-5%] text-[#4CDA95] mb-2">
						Finalizing Installation
					</h2>
					<p class="font-jakarta text-sm text-gray-300">
						Reset user initial complete
					</p>
					<p class="font-jakarta text-sm text-gray-300 mt-1">
						EOSX
					</p>
				</div>
			{/if}
			
			<span class="font-poppinsemibold text-2xl tracking-[-5%] block mb-4">{message}</span>
			<div class="flex flex-col justify-center w-[1050px] items-center mt-5 mx-auto">
				<div class="flex w-[1050px] h-[60px] p-[20px] items-center flex-shrink-0 rounded-[7px] bg-[rgba(0,0,0,0.3)]">
					<div class="w-[900px] h-[8px] rounded-[128px] bg-[#2D2D2D] mr-3">
						<div 
							class="h-full rounded-[2px] bg-gradient-to-r from-[#0F4128] to-[#26A768]"
							style="width: {percentage}%"
						></div>
					</div>
					<span class="font-poppinmedium text-xl text-[#4CDA95] whitespace-nowrap">{percentage}%</span>
				</div>
			</div>
		</div>
	{:then}
		<div class="flex flex-col items-center w-full">
			<div class="text-center mb-8">
				<h2 class="font-poppinsemibold text-2xl tracking-[-5%] text-[#4CDA95] mb-2">
					Finalizing Installation
				</h2>
				<p class="font-jakarta text-sm text-gray-300">
					Reset user initial complete
				</p>
				<p class="font-jakarta text-sm text-gray-300 mt-1">
					EOSX
				</p>
			</div>
			
			<span class="font-jakarta font-semibold text-sm text-[#4CDA95] mb-6">
				Reboot upon install completion
			</span>
			
			<Button 
				on:click={reboot}
				class="bg-greenTealinux hover:bg-[#3EC585] text-white font-poppinmedium font-medium px-8 py-3 rounded-md transition-colors"
			>
				Reboot Now
			</Button>
		</div>
	{/await}
</section>