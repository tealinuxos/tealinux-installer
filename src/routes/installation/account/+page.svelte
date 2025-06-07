<script>
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { getBlueprint, refreshDisk } from '../global.js';
	import { goto } from '$app/navigation';
	import Navigation from '$lib/components/Navigation.svelte';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import { afterNavigate } from '$app/navigation';

	let prevRoute = '';

	let fullname, username, hostname, password, confirmPassword, autologin;
	let isAdministrator = false;
	let passwordsMatch = false;
	let passwordVisible = false;
	let passwordConfirmVisible = false;
	let showPasswordIndicator = false;

	function togglePasswordVisibility() {
		passwordVisible = !passwordVisible;
	}

	function togglePasswordConfirmVisibility() {
		passwordConfirmVisible = !passwordConfirmVisible;
	}

	const handleSetAccount = async () => {
		if (password !== confirmPassword) {
			passwordsMatch = false;
			return;
		}
		passwordsMatch = true;

		await invoke('blueprint_set_account', { fullname, username, hostname, password, autologin });
	};

	$: if (password && confirmPassword && password === confirmPassword) {
		passwordsMatch = true;
	} else {
		passwordsMatch = false;
	}

	function checkPasswordStrength(password) {
		if (!password) return 0;

		let strength = 0;

		// Length check
		if (password.length > 8) strength += 1;
		if (password.length > 12) strength += 1;

		// Character variety checks
		if (/[A-Z]/.test(password)) strength += 1;
		if (/[a-z]/.test(password)) strength += 1;
		if (/[0-9]/.test(password)) strength += 1;
		if (/[^A-Za-z0-9]/.test(password)) strength += 1;

		return Math.min(strength, 4);
	}

	function getStrengthWidth(password) {
		const strength = checkPasswordStrength(password);
		return Math.min(strength, 3) * 33.33;
	}

	function getStrengthColor(password) {
		const strength = checkPasswordStrength(password);
		switch (strength) {
			case 0:
				return 'bg-transparent';
			case 1:
				return 'bg-[#FF453A]'; // weak
			case 2:
				return 'bg-[#FF9F0B]'; // medium
			case 3:
				return 'bg-[#26A768]'; // strong
			default:
				return 'bg-transparent';
		}
	}

	$: showPasswordIndicator = password && password.length > 0;

	function getStrengthText(password) {
		const strength = checkPasswordStrength(password);
		switch (strength) {
			case 0:
				return '';
			case 1:
				return 'Weak';
			case 2:
				return 'Medium';
			case 3:
			case 4:
				return 'Strong';
			default:
				return '';
		}
	}

	afterNavigate(({ from }) => {
		prevRoute = from?.url.pathname;
	});

	onMount(() => {
		getBlueprint().then((blueprint) => {
			if (blueprint.account !== null) {
				fullname = blueprint.account.fullname;
				username = blueprint.account.username;
				hostname = blueprint.account.hostname;
				password = blueprint.account.password;
				confirmPassword = blueprint.account.password;
			}
		});
	});
</script>

<TwoSide>
	{#snippet left()}
		<div class="mx-[35px] space-y-[15px]">
			<h1 class="font-archivo font-[600] text-[28px]">
				Create a <span class="text-green-tealinux">User</span><br />
			</h1>
			<p class="font-jakarta text-sm font-[200]">
				Set up a user by defining a username, creating a password, or even enabling automatic login
				for quicker access, though this may reduce security, especially on shared devices.
			</p>
		</div>
	{/snippet}
	{#snippet right()}
		<div
			class="flex flex-col h-[562px] p-6 space-y-[15px] mb-[15px] bg-black/30 border-[0.5px] border-gray-900 rounded-[10px] font-jakarta justify-center"
		>
			<form class="flex flex-col h-[85dvh] space-y-4">
				<!-- Full Name -->
				<div class="w-[400px] mx-auto">
					<label class="block text-sm font-medium text-[#26A768] mb-2">Full Name</label>
					<div class="flex items-center gap-3">
						<svg
							width="36"
							height="36"
							viewBox="0 0 48 48"
							fill="none"
							xmlns="http://www.w3.org/2000/svg"
							class="shrink-0"
						>
							<circle cx="24" cy="24" r="24" fill="#1A1F1E" />
							<circle cx="24" cy="18" r="6" stroke="#4CDA95" stroke-width="2" fill="none" />
							<path d="M14 34c0-4 4-7 10-7s10 3 10 7" stroke="#4CDA95" stroke-width="2" />
						</svg>
						<div
							class="relative flex-1 h-[45px] rounded-[9.489px] overflow-hidden border border-[#4CDA95] bg-[rgba(30,47,39,0.31)]"
						>
							<input
								class="w-full h-full outline-none text-sm text-white text-opacity-70 focus:placeholder-white/40 px-3 bg-transparent"
								type="text"
								bind:value={fullname}
								placeholder="Enter your full name"
							/>
						</div>
					</div>
				</div>

				<!-- Computer Name -->
				<div class="w-[400px] mx-auto">
					<label class="block text-sm font-medium text-[#26A768] mb-2">Computer Name</label>
					<div class="flex items-center gap-3">
						<svg
							width="36"
							height="36"
							viewBox="0 0 48 48"
							fill="none"
							xmlns="http://www.w3.org/2000/svg"
							class="shrink-0"
						>
							<circle cx="24" cy="24" r="24" fill="#1A1F1E" />
							<rect x="14" y="14" width="20" height="16" rx="2" stroke="#4CDA95" stroke-width="2" />
							<path d="M18 34h12M20 31h8" stroke="#4CDA95" stroke-width="2" />
						</svg>
						<div
							class="relative flex-1 h-[45px] rounded-[9.489px] overflow-hidden border border-[#4CDA95] bg-[rgba(30,47,39,0.31)]"
						>
							<input
								class="w-full h-full outline-none text-sm text-white text-opacity-70 focus:placeholder-white/40 px-3 bg-transparent"
								type="text"
								bind:value={hostname}
								placeholder="Enter your computer name"
							/>
						</div>
					</div>
				</div>

				<!-- Username -->
				<div class="w-[400px] mx-auto">
					<label class="block text-sm font-medium text-[#26A768] mb-2">User Name</label>
					<div class="flex items-center gap-3">
						<svg
							width="36"
							height="36"
							viewBox="0 0 48 48"
							fill="none"
							xmlns="http://www.w3.org/2000/svg"
							class="shrink-0"
							`
						>
							<circle cx="24" cy="24" r="24" fill="#1A1F1E" />
							<rect x="14" y="14" width="20" height="16" rx="2" stroke="#4CDA95" stroke-width="2" />
							<path d="M18 34h12M20 31h8" stroke="#4CDA95" stroke-width="2" />
						</svg>
						<div
							class="relative flex-1 h-[45px] rounded-[9.489px] overflow-hidden border border-[#4CDA95] bg-[rgba(30,47,39,0.31)]"
						>
							<input
								class="w-full h-full outline-none text-sm text-white text-opacity-70 focus:placeholder-white/40 px-3 bg-transparent"
								type="text"
								bind:value={username}
								placeholder="Enter your user name"
							/>
						</div>
					</div>
				</div>

				<!-- Password -->
				<div class="w-[400px] mx-auto">
					<label class="block text-sm font-medium text-[#26A768] mb-2">Password</label>
					<div class="flex items-center gap-3">
						<svg
							width="36"
							height="36"
							viewBox="0 0 48 48"
							fill="none"
							xmlns="http://www.w3.org/2000/svg"
							class="shrink-0"
						>
							<circle cx="24" cy="24" r="24" fill="#1A1F1E" />
							<rect x="16" y="22" width="16" height="14" rx="2" stroke="#4CDA95" stroke-width="2" />
							<path d="M19 22v-3a5 5 0 0 1 10 0v3" stroke="#4CDA95" stroke-width="2" />
						</svg>
						<div
							class="relative flex-1 h-[45px] rounded-[9.489px] overflow-hidden border border-[#4CDA95] bg-[rgba(30,47,39,0.31)]"
						>
							{#if passwordVisible}
								<input
									class="w-full h-full outline-none text-sm text-white text-opacity-70 placeholder-[#3C6350] focus:placeholder-white/40 px-3 bg-transparent"
									type="text"
									bind:value={password}
									placeholder="Enter your password"
								/>
							{:else}
								<input
									class="w-full h-full outline-none text-sm text-white text-opacity-70 focus:placeholder-white/40 px-3 bg-transparent"
									type="password"
									bind:value={password}
									placeholder="Enter your password"
								/>
							{/if}
							<button
								type="button"
								class="absolute right-3 top-1/2 -translate-y-1/2 group"
								on:click={togglePasswordVisibility}
							>
								<svg
									width="20"
									height="20"
									viewBox="0 0 24 24"
									fill="none"
									xmlns="http://www.w3.org/2000/svg"
									class="transition-all duration-300 group-hover:scale-110"
								>
									{#if passwordVisible}
										<!-- Visible eye icon -->
										<path
											d="M12 5C5.636 5 1 12 1 12C1 12 5.636 19 12 19C18.364 19 23 12 23 12C23 12 18.364 5 12 5Z"
											stroke="#4CDA95"
											stroke-width="2"
											stroke-linecap="round"
											stroke-linejoin="round"
										/>
										<path
											d="M12 15C13.6569 15 15 13.6569 15 12C15 10.3431 13.6569 9 12 9C10.3431 9 9 10.3431 9 12C9 13.6569 10.3431 15 12 15Z"
											stroke="#4CDA95"
											stroke-width="2"
											stroke-linecap="round"
											stroke-linejoin="round"
										/>
									{:else}
										<!-- Invisible eye icon with simple diagonal slash -->
										<path
											d="M12 5C5.636 5 1 12 1 12C1 12 5.636 19 12 19C18.364 19 23 12 23 12C23 12 18.364 5 12 5Z"
											stroke="#4CDA95"
											stroke-width="2"
											stroke-linecap="round"
											stroke-linejoin="round"
										/>
										<path d="M1 1L23 23" stroke="#4CDA95" stroke-width="2" stroke-linecap="round" />
									{/if}
								</svg>
							</button>
						</div>
					</div>

					<!-- Password Strength Indicator -->
					{#if showPasswordIndicator}
						<div class="flex items-center mt-2 ml-[48px]">
							<div class="flex-1 h-[4px] bg-gray-700 rounded-full overflow-hidden">
								<div
									class="h-full rounded-full transition-all duration-300 {getStrengthColor(
										password
									)}"
									style="width: {getStrengthWidth(password)}%"
								></div>
							</div>
							<span class="ml-3 text-xs text-white font-medium">{getStrengthText(password)}</span>
						</div>
					{/if}
				</div>
				<!-- Confirm Password -->
				<div class="w-[400px] mx-auto">
					<label class="block text-sm font-medium text-[#26A768] mb-2">Confirm Password</label>
					<div class="flex items-center gap-3">
						<svg
							width="36"
							height="36"
							viewBox="0 0 48 48"
							fill="none"
							xmlns="http://www.w3.org/2000/svg"
							class="shrink-0"
						>
							<circle cx="24" cy="24" r="24" fill="#1A1F1E" />
							<rect x="16" y="22" width="16" height="14" rx="2" stroke="#4CDA95" stroke-width="2" />
							<path d="M19 22v-3a5 5 0 0 1 10 0v3" stroke="#4CDA95" stroke-width="2" />
						</svg>
						<div
							class="relative flex-1 h-[45px] rounded-[9.489px] overflow-hidden border border-[#4CDA95] bg-[rgba(30,47,39,0.31)]"
						>
							{#if passwordConfirmVisible}
								<input
									class="w-full h-full outline-none text-sm text-white text-opacity-70 focus:placeholder-white/40 px-3 bg-transparent"
									type="text"
									bind:value={confirmPassword}
									placeholder="Confirm your password"
								/>
							{:else}
								<input
									class="w-full h-full outline-none text-sm text-white text-opacity-70 focus:placeholder-white/40 px-3 bg-transparent"
									type="password"
									bind:value={confirmPassword}
									placeholder="Confirm your password"
								/>
							{/if}
							<button
								type="button"
								class="absolute right-3 top-1/2 -translate-y-1/2 group"
								on:click={togglePasswordConfirmVisibility}
							>
								<svg
									width="20"
									height="20"
									viewBox="0 0 24 24"
									fill="none"
									xmlns="http://www.w3.org/2000/svg"
									class="transition-all duration-300 group-hover:scale-110"
								>
									{#if passwordConfirmVisible}
										<!-- Visible eye icon -->
										<path
											d="M12 5C5.636 5 1 12 1 12C1 12 5.636 19 12 19C18.364 19 23 12 23 12C23 12 18.364 5 12 5Z"
											stroke="#4CDA95"
											stroke-width="2"
											stroke-linecap="round"
											stroke-linejoin="round"
										/>
										<path
											d="M12 15C13.6569 15 15 13.6569 15 12C15 10.3431 13.6569 9 12 9C10.3431 9 9 10.3431 9 12C9 13.6569 10.3431 15 12 15Z"
											stroke="#4CDA95"
											stroke-width="2"
											stroke-linecap="round"
											stroke-linejoin="round"
										/>
									{:else}
										<!-- Invisible eye icon with simple diagonal slash -->
										<path
											d="M12 5C5.636 5 1 12 1 12C1 12 5.636 19 12 19C18.364 19 23 12 23 12C23 12 18.364 5 12 5Z"
											stroke="#4CDA95"
											stroke-width="2"
											stroke-linecap="round"
											stroke-linejoin="round"
										/>
										<path d="M1 1L23 23" stroke="#4CDA95" stroke-width="2" stroke-linecap="round" />
									{/if}
								</svg>
							</button>
						</div>
					</div>
					<div class="flex items-center mt-1 ml-[48px]">
						{#if passwordsMatch === false && password}
							<p class="text-red-500 text-[14px] mt-[5px]">Passwords do not match</p>
						{/if}
					</div>

					<div class="max-w-md mx-auto mt-4 gap-x-2 flex items-center">
						<div class="relative w-9 h-9">
							<!-- Checkbox Hidden -->
							<input
								type="checkbox"
								id="automaticLogin"
								class="absolute w-full h-full opacity-0 cursor-pointer z-10"
								bind:checked={autologin}
							/>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="w-9 h-9 transition-transform duration-300"
								viewBox="0 0 48 48"
							>
								<circle
									cx="24"
									cy="24"
									r="22"
									fill="#1a1a1a"
									stroke-width="2"
									class="transition-colors duration-200"
								/>
								<path
									d="M18 24.5l4 4 12-12"
									stroke="#2ecc71"
									stroke-width="3"
									stroke-linecap="round"
									stroke-linejoin="round"
									fill="none"
									class="opacity-0 transition-opacity duration-300"
								/>
							</svg>
						</div>
						<label for="automaticLogin" class="ml-2 text-sm text-white">Automatic Login</label>
					</div>
				</div>
			</form>
		</div>
	{/snippet}
</TwoSide>

<Navigation
	currentStep={5}
	currentTitle="User"
	prevPath="/installation/partitioning"
	nextPath="/installation/summary"
	nextAction={handleSetAccount}
	prevAction={refreshDisk}
	disableNext={!passwordsMatch ||
		!fullname ||
		!username ||
		!hostname ||
		!password ||
		!confirmPassword}
/>
