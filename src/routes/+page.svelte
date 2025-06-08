<!-- +page.svelte -->
<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import Tealinux from '$lib/assets/Vector.png';
	import Loading from '$lib/components/Loading.svelte';

	let isLoading = true;
	let loadingText = 'Initializing TeaLinux...';
	let isInitialized = false;

	const isOnline = async () => {
		try {
			let online = await invoke('is_online');
			return online;
		} catch (error) {
			console.error('Error checking online status:', error);
			return false;
		}
	};

	const setReadJSON = async () => {
		try {
			loadingText = 'Reading configuration...';
			await invoke('set_read_json');
			return true;
		} catch (error) {
			console.error('Error reading JSON:', error);
			throw error;
		}
	};

	const setBlueprintJSON = async () => {
		try {
			loadingText = 'Setting up blueprint...';
			await invoke('set_empty_blueprint');
			return true;
		} catch (error) {
			console.error('Error setting blueprint:', error);
			throw error;
		}
	};

	const initializeSystem = async () => {
		try {
			loadingText = 'Checking connection...';
			await isOnline();
			await new Promise((resolve) => setTimeout(resolve, 500)); // Small delay for UX

			loadingText = 'Preparing installation...';
			await setBlueprintJSON();
			await new Promise((resolve) => setTimeout(resolve, 500));

			loadingText = 'Loading configuration...';
			await setReadJSON();
			await new Promise((resolve) => setTimeout(resolve, 500));

			loadingText = 'Almost ready...';
			await new Promise((resolve) => setTimeout(resolve, 800));

			isInitialized = true;
			isLoading = false;
		} catch (error) {
			console.error('Initialization error:', error);
			loadingText = 'Error occurred. Please restart.';

			setTimeout(() => {
				isLoading = false;
				isInitialized = true;
			}, 2000);
		}
	};

	onMount(() => {
		initializeSystem();
	});

	const handleStartClick = () => {
		window.location.href = '/installation';
	};
</script>

<!-- Loading Screen -->
{#if isLoading}
	<Loading text={loadingText} show={isLoading} />
{/if}

<!-- Main Content -->
{#if isInitialized && !isLoading}
	<div
		class="flex items-center justify-center min-h-screen text-white bg-gradient-to-br from-gray-900 to-black"
	>
		<div class="text-center animate-fade-in">
			<!-- Logo -->
			<div class="mb-8 animate-bounce-in">
				<img
					src={Tealinux}
					alt="TeaLinux Logo"
					class="mx-auto mb-4 w-32 h-32 object-contain drop-shadow-lg"
				/>
			</div>

			<!-- Welcome Text -->
			<div class="animate-slide-up">
				<h1
					class="font-archivo font-semibold text-6xl -tracking-[1.5%] mb-4 bg-gradient-to-r from-green-400 to-green-600 bg-clip-text text-transparent"
				>
					Welcome to TeaLinux OS!
				</h1>
				<h2 class="font-poppins mb-8 text-2xl tracking-normal text-gray-300">
					Press start to installing TeaLinux
				</h2>
			</div>

			<!-- Start Button -->
			<div class="p-2 animate-slide-up-delay">
				<a
					href="/installation"
					class="bg-green-700 hover:-translate-y-1 hover:shadow-2xl hover:shadow-green-500/25
				   transition-all duration-300 rounded-full hover:bg-green-600 text-white
				   font-semibold text-xl py-4 px-28 border-2 border-green-600/30
				   transform hover:scale-105 active:scale-95
				   focus:outline-none focus:ring-4 focus:ring-green-500/50"
				>
					START
				</a>
			</div>

			<!-- Status Indicator -->
			<div class="mt-8 animate-fade-in-delay">
				<div class="flex items-center justify-center space-x-2 text-green-400">
					<div class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
					<span class="text-sm font-medium">System Ready</span>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	/* Custom animations */
	@keyframes fade-in {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	@keyframes bounce-in {
		0% {
			transform: scale(0.3) translateY(-50px);
			opacity: 0;
		}
		50% {
			transform: scale(1.05);
		}
		70% {
			transform: scale(0.9);
		}
		100% {
			transform: scale(1) translateY(0);
			opacity: 1;
		}
	}

	@keyframes slide-up {
		from {
			transform: translateY(30px);
			opacity: 0;
		}
		to {
			transform: translateY(0);
			opacity: 1;
		}
	}

	.animate-fade-in {
		animation: fade-in 0.8s ease-out;
	}

	.animate-fade-in-delay {
		animation: fade-in 0.8s ease-out 0.6s both;
	}

	.animate-bounce-in {
		animation: bounce-in 1s ease-out 0.2s both;
	}

	.animate-slide-up {
		animation: slide-up 0.6s ease-out 0.4s both;
	}

	.animate-slide-up-delay {
		animation: slide-up 0.6s ease-out 0.6s both;
	}

	/* Global styles */
	:global(body) {
		margin: 0;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
		background: linear-gradient(135deg, #1f2937 0%, #000000 100%);
	}
</style>
