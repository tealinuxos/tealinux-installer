<script lang="ts">
	import { type BootPath } from '$types/installation/partitioning/partitioning.types';
	import Navigation from '$lib/components/Navigation.svelte';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import { partitionMethod } from '$lib/stores/informationStore.js';
	import { commands } from '$types/commands';

	interface AccountInfo {
		fullname: string;
		username: string;
		hostname: string;
		password: string;
		confirmPassword: string;
		autologin: boolean;
	}

	interface PasswordControl {
		passwordVisible: boolean;
		passwordConfirmVisible: boolean;
	}

	type StrengthLevel = 0 | 1 | 2 | 3 | 4;

	let accountInfo = $state<AccountInfo>({
		fullname: '',
		username: '',
		hostname: '',
		password: '',
		confirmPassword: '',
		autologin: false
	});

	let passwordControl = $state<PasswordControl>({
		passwordVisible: false,
		passwordConfirmVisible: false
	});

	const partitioningMethod = $partitionMethod;

	let passwordMatch = $derived.by(() => {
		if (accountInfo.password !== accountInfo.confirmPassword) {
			return false;
		}
		return true;
	});

	let passwordStrength: StrengthLevel = $derived.by(() => {
		let strength = 0;

		if (accountInfo.password.length > 8) strength += 1;
		if (accountInfo.password.length > 12) strength += 1;
		if (/[A-Z]/.test(accountInfo.password)) strength += 1;
		if (/[a-z]/.test(accountInfo.password)) strength += 1;
		if (/[0-9]/.test(accountInfo.password)) strength += 1;
		if (/[^A-Za-z0-9]/.test(accountInfo.password)) strength += 1;

		return Math.min(strength, 4) as StrengthLevel;
	});

	let passwordStrengthWidth: Record<StrengthLevel, number> = $derived.by(() => {
		return {
			0: 0,
			1: 25,
			2: 50,
			3: 75,
			4: 100
		};
	});

	const passwordColor: Record<StrengthLevel, string> = {
		0: '',
		1: 'bg-[#FF453A]',
		2: 'bg-[#FF9F0B]',
		3: 'bg-[#FFD60A]',
		4: 'bg-[#30D158]'
	};

	const passwordStrengthText: Record<StrengthLevel, string> = {
		0: '',
		1: 'Weak',
		2: 'Medium',
		3: 'Strong',
		4: 'Very Strong'
	};

	function togglePasswordVisibility() {
		passwordControl.passwordVisible = !passwordControl.passwordVisible;
	}

	function togglePasswordConfirmVisibility() {
		passwordControl.passwordConfirmVisible = !passwordControl.passwordConfirmVisible;
	}

	const handleSetAccount = async () => {
		if (passwordMatch === false) {
			return;
		}

		await commands.blueprintSetAccount(
			accountInfo.fullname,
			accountInfo.username,
			accountInfo.hostname,
			accountInfo.password,
			accountInfo.autologin
		);
	};
</script>

<TwoSide>
	{#snippet left()}
		<div class="mx-[35px] space-y-[15px]">
			<h1 class="font-archivo font-semibold text-[28px]">
				Create a <span class="text-green-tealinux">User</span><br />
			</h1>
			<p class="font-jakarta text-sm font-extralight">
				Set up a user by defining a username, creating a password, or even enabling automatic login
				for quicker access, though this may reduce security, especially on shared devices.
			</p>
		</div>
	{/snippet}
	{#snippet right()}
		<div
			class="flex flex-col h-[600px] p-6 space-y-[15px] mb-[15px] bg-black/30 border-[0.5px] border-gray-900 rounded-[10px] font-jakarta justify-center"
		>
			<form class="flex flex-col h-[85dvh] space-y-4">
				<!-- Full Name -->
				<div class="w-[400px] mx-auto">
					<label for="fullName" class="block text-sm font-medium text-[#26A768] mb-2"
						>Full Name</label
					>
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
								bind:value={accountInfo.fullname}
								placeholder="Enter your full name"
							/>
						</div>
					</div>
				</div>

				<!-- Computer Name -->
				<div class="w-[400px] mx-auto">
					<label for="computerName" class="block text-sm font-medium text-[#26A768] mb-2"
						>Computer Name</label
					>
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
								bind:value={accountInfo.hostname}
								placeholder="Enter your computer name"
							/>
						</div>
					</div>
				</div>

				<!-- Username -->
				<div class="w-[400px] mx-auto">
					<label for="userName" class="block text-sm font-medium text-[#26A768] mb-2"
						>User Name</label
					>
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
								bind:value={accountInfo.username}
								placeholder="Enter your user name"
							/>
						</div>
					</div>
				</div>

				<!-- Password -->
				<div class="w-[400px] mx-auto">
					<label for="password" class="block text-sm font-medium text-[#26A768] mb-2"
						>Password</label
					>
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
							<input
								class="w-full h-full outline-none text-sm text-white text-opacity-70 placeholder-[#3C6350] focus:placeholder-white/40 px-3 bg-transparent"
								type={passwordControl.passwordVisible ? 'text' : 'password'}
								bind:value={accountInfo.password}
								placeholder="Enter your password"
							/>

							<button
								type="button"
								class="absolute right-3 top-1/2 -translate-y-1/2 group"
								onclick={togglePasswordVisibility}
							>
								<svg
									width="20"
									height="20"
									viewBox="0 0 24 24"
									fill="none"
									xmlns="http://www.w3.org/2000/svg"
									class="transition-all duration-300 group-hover:scale-110"
								>
									{#if passwordControl.passwordVisible}
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

					<div class="flex items-center mt-2 ml-12">
						<div class="flex-1 h-1 bg-gray-700 rounded-full overflow-hidden">
							<div
								class="h-full rounded-full transition-all duration-300 {passwordColor[
									passwordStrength
								]}"
								style="width: {passwordStrengthWidth[passwordStrength]}%"
							></div>
						</div>
						<span class="ml-3 text-xs text-white font-medium"
							>{passwordStrengthText[passwordStrength]}</span
						>
					</div>
				</div>
				<!-- Confirm Password -->
				<div class="w-[400px] mx-auto">
					<label for="confirmPassword" class="block text-sm font-medium text-[#26A768] mb-2"
						>Confirm Password</label
					>
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
							<input
								class="w-full h-full outline-none text-sm text-white text-opacity-70 focus:placeholder-white/40 px-3 bg-transparent"
								type={passwordControl.passwordConfirmVisible ? 'text' : 'password'}
								bind:value={accountInfo.confirmPassword}
								placeholder="Confirm your password"
							/>

							<button
								type="button"
								class="absolute right-3 top-1/2 -translate-y-1/2 group"
								onclick={togglePasswordConfirmVisibility}
							>
								<svg
									width="20"
									height="20"
									viewBox="0 0 24 24"
									fill="none"
									xmlns="http://www.w3.org/2000/svg"
									class="transition-all duration-300 group-hover:scale-110"
								>
									{#if passwordControl.passwordConfirmVisible}
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
					<div class="flex items-center mt-1 ml-12">
						{#if passwordMatch === false && accountInfo.password}
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
								bind:checked={accountInfo.autologin}
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
						<label for="automaticLogin" class="ml-2 text-sm text-[#2ecc71]">Automatic Login</label>
					</div>
				</div>
			</form>
		</div>
	{/snippet}
</TwoSide>

<Navigation
	currentStep={5}
	currentTitle="User"
	prevPath={`/installation/partitioning/${partitioningMethod as BootPath}`}
	nextPath="/installation/summary"
	nextAction={handleSetAccount}
	disableNext={!passwordMatch ||
		!accountInfo.fullname ||
		!accountInfo.username ||
		!accountInfo.hostname ||
		!accountInfo.password ||
		!accountInfo.confirmPassword}
/>
