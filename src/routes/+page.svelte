<script lang="ts">
	import { onMount } from 'svelte';
	import Loading from '$lib/components/Loading.svelte';
	import { commands } from '$types/commands';
	import { resolve } from '$app/paths';

	let isLoading = $state<boolean>(true);
	let loadingText = $state<string>('Initializing TeaLinux...');
	let isInitialized = $state<boolean>(false);

	const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

	const initStep = async (text: string, action: () => Promise<unknown>, minDuration = 500) => {
		loadingText = text;
		const [result] = await Promise.all([action(), delay(minDuration)]);
		return result;
	};

	const initializeSystem = async () => {
		try {
			// Step 1: Check Online Status
			await initStep('Checking connection...', async () => {
				try {
					return await commands.isOnline();
				} catch (e) {
					console.warn('Network check failed:', e);
					return false;
				}
			});

			// Step 2: Set Empty Blueprint
			await initStep('Setting up blueprint...', () => commands.setEmptyBlueprint());

			// Step 3: Read JSON Configuration
			await initStep('Loading configuration...', () => commands.setReadJson());

			// Final Step: Finishing touches
			loadingText = 'Almost ready...';
			await delay(800);

			// Success State
			isInitialized = true;
			isLoading = false;
		} catch (error) {
			console.error('Initialization critical error:', error);
			loadingText = 'Error occurred. Restarting interface...';

			await delay(2000);
			isLoading = false;
			isInitialized = true;
		}
	};

	const openWebsite = async () => {
		await commands.openWebsite();
	};

	onMount(() => {
		initializeSystem();
	});
</script>

<!-- Loading Screen -->
{#if isLoading}
	<Loading text={loadingText} show={isLoading} />
{/if}

<!-- Main Content -->
{#if isInitialized && !isLoading}
	<div
		class="flex items-center justify-center min-h-screen text-white bg-linear-to-br from-gray-900 to-black"
	>
		<div class="text-center animate-fade animate-ease-in-out animate-normal">
			<!-- Welcome Text -->
			<div class="animate-fade-up animate-ease-in-out animate-normal pb-14">
				<h1
					class="font-archivo font-semibold text-6xl -tracking-[1.5%] mb-4 bg-linear-to-br from-green-tealinux to-red-200 bg-clip-text text-transparent"
				>
					Welcome to<br />TeaLinuxOS Celia!
				</h1>
			</div>

			<div class="flex flex-col space-y-3">
				<!-- Start Button -->
				<div class="p-2 animate-fade-up animate-delay-[6ms] animate-ease-in-out animate-normal">
					<a
						href={resolve('/installation')}
						class="bg-green-tealinux hover:-translate-y-1 hover:shadow-2xl hover:shadow-green-500/25
                       transition-all duration-300 rounded-full hover:bg-green-600 text-white
                       font-semibold text-xl py-4 px-14 border-2 border-green-600/30
                       transform hover:scale-105 active:scale-95
                       focus:outline-none focus:ring-4 focus:ring-green-500/50"
					>
						Install TealinuxOS
					</a>
				</div>
				<div class="p-0 animate-fade-up animate-delay-[6ms] animate-ease-in-out animate-normal">
					<button
						onclick={openWebsite}
						class="hover:-translate-y-1
                       transition-all duration-300 rounded-full hover:text-green-tealinux text-gray-500
                       font-semibold text-lg py-4 px-28
                       transform hover:scale-105 active:scale-95
                       focus:outline-none"
					>
						How to Install?
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}
