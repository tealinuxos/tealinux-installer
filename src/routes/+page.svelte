<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
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

	const openWebsite = async () => {
		await invoke('open_website');
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
		class="flex items-center justify-center min-h-screen text-white bg-gradient-to-br from-gray-900 to-black"
	>
		<div class="text-center animate-fade animate-ease-in-out animate-normal">
			<!-- Welcome Text -->
			<div class="animate-fade-up animate-ease-in-out animate-normal pb-14">
				<h1
					class="font-archivo font-semibold text-6xl -tracking-[1.5%] mb-4 bg-gradient-to-r from-green-tealinux to-red-200 bg-clip-text text-transparent"
				>
					Welcome to<br />TeaLinuxOS Celia!
				</h1>
			</div>

			<div class="flex flex-col space-y-3">
				<!-- Start Button -->
				<div class="p-2 animate-fade-up animate-delay-[6ms] animate-ease-in-out animate-normal">
					<a
						href="/installation"
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
