<script>
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
    import Error from '$lib/components/modals/Error.svelte';
    import { exit } from '/node_modules/@tauri-apps/plugin-process';
	import { onMount } from 'svelte';

	let percentage = $state(0);
	let message = $state('Starting installation...');
	let errorMessage = $state(null);
	let rebootChecked = $state(false); 

	const listenInstall = listen('INSTALL', (event) => {
		percentage = event.payload.percentage;
		message = event.payload.message;
	});

	const listenError = listen('ERROR', (event) => {
		errorMessage = event.payload.message;
	});

	const startInstall = async () => {
		try {
            // await new Promise(resolve => setTimeout(resolve, 3000)) // Testing purpose

			await invoke('start_install'); // Danger!!! Comment this line when developing

            if (rebootChecked) {
                await reboot();
            }
		} catch (err) {
			errorMessage = err.toString();
		}
	};

	const reboot = async () => {
		await invoke('reboot');
	};

	const toggleRebootCheck = () => {
		rebootChecked = !rebootChecked;
	};
</script>

{#if errorMessage !== null}
    <Error errorMessage={errorMessage} />
    <Error
        title="Installation Failed!"
        message={errorMessage}
        cancelValue="Exit"
        confirmValue="Reboot"
    />
{/if}

<section class="flex flex-col justify-between h-screen items-center text-center p-8 text-white font-archivo">
	
	{#await startInstall()}
	<div class="flex-1 flex text-left min-w-[812px] w-[812px] justify-between items-center">
		<div class="max-w-[541px]">
			<p class="font-[500] text-5xl leading-[1.4] h-[146px]">
				Fast, efficient, and <br /> optimized for everyone
			</p>
			<p class="font-jakarta text-xl w-[482px] mt-2">
				Tea Linux OS, built on Arch Linux, offers cutting-edge tools and features for everyone.
				Empower your productivity with the latest innovations in this
				release.
			</p>
		</div>
		<img src="/tealinux.svg" alt="logo" class="w-[222px] h-[233px]" />
	</div>

		<div class="w-full flex flex-col items-center"> 
			<div class="flex flex-col justify-center items-center mt-5 w-[1050px]">
				<div class="flex flex-col w-full p-[20px] items-center flex-shrink-0 rounded-[7px] bg-[rgba(0,0,0,0.3)]">
					<div class="flex items-center w-full">
						<div class="w-full h-[8px] rounded-[12px] bg-[#2D2D2D] mr-3">
							<div 
								class="h-full rounded-[2px] bg-gradient-to-r from-[#0F4128] to-[#26A768]"
								style="width: {percentage}%"
							></div>
						</div>
						<span class="font-poppinmedium text-xl text-[#4CDA95] whitespace-nowrap ml-3">{percentage}%</span>
					</div>
					<div class="flex justify-between w-full mt-1">
						<div class="animate-pulse text-left font-jakarta text-[#4CDA95] font-medium flex items-center">
							<p>{message}</p>	
						</div>
						<div class="text-right font-jakarta  font-medium flex items-center mr-[56px]">
							<p class="mr-2 text-[#4CDA95]">Reboot upon install completion</p>
							<label class="cursor-pointer">
								<div class={`w-5 h-5 rounded-md border-2 flex items-center justify-center transition-colors
									${rebootChecked ? 'border-green-500 bg-green-500/20' : 'border-gray-500'}`}
                                    onclick={toggleRebootCheck}
                                >
									{#if rebootChecked}
										<svg class="w-4 h-4 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
											<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
										</svg>
									{/if}
								</div>
							</label>
						</div>
					</div>
				</div>
			</div>
		</div>
	{:then}
		
	<div class="flex-1 flex text-left min-w-[812px] w-[812px] justify-between items-center">
		<img src="/tealinux.svg" alt="logo" class="w-[222px] h-[233px]" />
		<div class="max-w-[541px]">
			<p class="font-[500] text-5xl leading-[1.4] h-[146px] text-green-tealinux font-bold">
                Thank you for choosing TeaLinuxOS!
			</p>
			<p class="font-jakarta text-xl w-[482px] mt-2">
                TeaLinuxOS is installed and ready to use.
			</p>
			
			<div class="flex gap-4 mt-6">
				<button 
					class="flex flex-col justify-center items-center gap-[14.545px] w-[136.727px] p-[14.545px_21.818px_14.545px_14.545px] rounded-[7.273px] border-[0.945px] border-[#3C6350] bg-[#101010] text-white font-medium"
					onclick={async () => await exit(0)}
				>
					Exit
				</button>
				<button 
					class="rounded-[7.273px] border-[0.945px] border-[#3C6350] bg-[#26A768] text-white px-6 py-3 font-medium hover:bg-[#1e8a54] transition-colors"
					onclick={reboot}
				>
					Restart Now
				</button>
			</div>
		</div>
	</div>
		
	{/await}
</section>
