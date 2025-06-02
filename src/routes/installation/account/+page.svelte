<script>
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { getBlueprint, refreshDisk } from '../global.js';
	import { goto } from '$app/navigation';
	import Navigation from '$lib/components/Navigation.svelte';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';


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
    switch(strength) {
      case 0: return 'bg-transparent';
      case 1: return 'bg-[#FF453A]'; // weak
      case 2: return 'bg-[#FF9F0B]'; // medium
      case 3: return 'bg-[#26A768]'; // strong
      default: return 'bg-transparent';
    }
  }

$: showPasswordIndicator = password && password.length > 0;
  

  function getStrengthText(password) {
    const strength = checkPasswordStrength(password);
    switch(strength) {
      case 0: return '';
      case 1: return 'Weak';
      case 2: return 'Medium';
      case 3: 
      case 4: return 'Strong';
      default: return '';
    }
  }

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
			<h1 class="font-jakarta font-[800] text-[28px]">
				Create a <span class="text-green-tealinux">User</span><br />
			</h1>
			<p class="font-jakarta text-sm font-[200]">
                Set up a user by defining a username, creating a password, or even enabling automatic login for quicker access, though this may reduce security, especially on shared devices.
			</p>
		</div>
	{/snippet}
	{#snippet right()}
		<div class="flex flex-col h-[562px] p-6 space-y-[15px] mb-[15px] bg-black/30 border-[0.5px] border-gray-900 rounded-[10px] font-jakarta justify-center">
			<form class="flex flex-col h-[85dvh] space-y-4">
				<!-- Full Name -->
				<div class="w-[400px] mx-auto">
					<label class="block text-sm font-medium text-[#26A768] mb-2">Full Name</label>
					<div class="flex items-center gap-3">
						<svg width="36" height="36" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" class="shrink-0">
							<circle cx="24" cy="24" r="24" fill="#1A1F1E"/>
							<circle cx="24" cy="18" r="6" stroke="#4CDA95" stroke-width="2" fill="none"/>
							<path d="M14 34c0-4 4-7 10-7s10 3 10 7" stroke="#4CDA95" stroke-width="2"/>
						</svg>
						<div class="relative flex-1 h-[45px] rounded-[9.489px] overflow-hidden border border-[#4CDA95] bg-[rgba(30,47,39,0.31)]">
						<input
							class="w-full h-full outline-none text-sm text-white text-opacity-70 placeholder:text-white placeholder:text-opacity-40 px-3 bg-transparent"
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
						<svg width="36" height="36" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" class="shrink-0">
							<circle cx="24" cy="24" r="24" fill="#1A1F1E"/>
							<rect x="14" y="14" width="20" height="16" rx="2" stroke="#4CDA95" stroke-width="2"/>
							<path d="M18 34h12M20 31h8" stroke="#4CDA95" stroke-width="2"/>
						</svg>
						<div class="relative flex-1 h-[45px] rounded-[9.489px] overflow-hidden border border-[#4CDA95] bg-[rgba(30,47,39,0.31)]">
							<input
								class="w-full h-full outline-none text-sm text-white text-opacity-70 placeholder:text-white placeholder:text-opacity-40 px-3 bg-transparent"
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
						<svg width="36" height="36" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" class="shrink-0"`>
							<circle cx="24" cy="24" r="24" fill="#1A1F1E"/>
							<rect x="14" y="14" width="20" height="16" rx="2" stroke="#4CDA95" stroke-width="2"/>
							<path d="M18 34h12M20 31h8" stroke="#4CDA95" stroke-width="2"/>
						</svg>
						<div class="relative flex-1 h-[45px] rounded-[9.489px] overflow-hidden border border-[#4CDA95] bg-[rgba(30,47,39,0.31)]">
							<input
								class="w-full h-full outline-none text-sm text-white text-opacity-70 placeholder:text-white placeholder:text-opacity-40 px-3 bg-transparent"
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
		<svg width="36" height="36" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" class="shrink-0">
			<circle cx="24" cy="24" r="24" fill="#1A1F1E"/>
			<rect x="16" y="22" width="16" height="14" rx="2" stroke="#4CDA95" stroke-width="2"/>
			<path d="M19 22v-3a5 5 0 0 1 10 0v3" stroke="#4CDA95" stroke-width="2"/>
		</svg>
		<div class="relative flex-1 h-[45px] rounded-[9.489px] overflow-hidden border border-[#4CDA95] bg-[rgba(30,47,39,0.31)]">
			{#if passwordVisible}
				<input
					class="w-full h-full outline-none text-sm text-white text-opacity-70 placeholder:text-white placeholder:text-opacity-40 px-3 bg-transparent"
					type="text"
					bind:value={password}
					placeholder="Enter your password"
				/>
			{:else}
				<input
					class="w-full h-full outline-none text-sm text-white text-opacity-70 placeholder:text-white placeholder:text-opacity-40 px-3 bg-transparent"
					type="password"
					bind:value={password}
					placeholder="Enter your password"
				/>
			{/if}
			<button type="button" class="absolute right-3 top-1/2 -translate-y-1/2" on:click={togglePasswordVisibility}>
				<svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
					<mask id="mask0_461_173" style="mask-type:luminance" maskUnits="userSpaceOnUse" x="0" y="0" width="24" height="24">
						<path d="M24 0H0V24H24V0Z" fill="white" />
					</mask>
					<g mask="url(#mask0_461_173)">
						<path
							d="M17.8817 19.2971C16.1229 20.4127 14.0824 21.0034 11.9997 21.0001C6.60766 21.0001 2.12166 17.1201 1.18066 12.0001C1.61069 9.67078 2.78229 7.54296 4.52066 5.93407L1.39166 2.80807L2.80666 1.39307L22.6057 21.1931L21.1907 22.6071L17.8817 19.2971ZM5.93466 7.35007C4.57566 8.58566 3.62898 10.2089 3.22266 12.0001C3.53495 13.3666 4.16192 14.6412 5.05366 15.7227C5.9454 16.8041 7.07729 17.6625 8.35921 18.2294C9.64114 18.7963 11.0377 19.0562 12.4378 18.9882C13.8378 18.9203 15.2027 18.5265 16.4237 17.8381L14.3957 15.8101C13.5324 16.3539 12.5099 16.5882 11.4959 16.4745C10.482 16.3609 9.5367 15.906 8.81523 15.1845C8.09376 14.4631 7.63889 13.5178 7.52522 12.5039C7.41156 11.4899 7.64584 10.4674 8.18966 9.60407L5.93466 7.35007ZM12.9137 14.3281L9.67166 11.0861C9.49372 11.539 9.45185 12.0341 9.55117 12.5105C9.65048 12.9868 9.88668 13.4239 10.2308 13.768C10.5749 14.1121 11.012 14.3483 11.4884 14.4476C11.9647 14.5469 12.4598 14.505 12.9127 14.3271L12.9137 14.3281ZM20.8067 16.5921L19.3757 15.1621C20.0442 14.2094 20.5201 13.1353 20.7767 12.0001C20.5049 10.8098 19.994 9.68721 19.2748 8.70056C18.5557 7.71391 17.6435 6.88379 16.5936 6.26067C15.5437 5.63755 14.378 5.23443 13.1674 5.07583C11.9569 4.91723 10.7267 5.00644 9.55166 5.33807L7.97366 3.76007C9.22066 3.27007 10.5797 3.00007 11.9997 3.00007C17.3917 3.00007 21.8777 6.88007 22.8187 12.0001C22.5123 13.6658 21.8236 15.2377 20.8067 16.5921ZM11.7227 7.50807C12.3592 7.46873 12.9968 7.56513 13.5932 7.79088C14.1897 8.01663 14.7313 8.36658 15.1822 8.81752C15.6332 9.26846 15.9831 9.81009 16.2089 10.4066C16.4346 11.003 16.531 11.6406 16.4917 12.2771L11.7227 7.50807Z"
							fill="#757575"
						/>
					</g>
				</svg>
			</button>
		</div>
	</div>
	
	<!-- Password Strength Indicator - Positioned to align with input field -->
	{#if showPasswordIndicator}
		<div class="flex items-center mt-2 ml-[48px]">
			<div class="flex-1 h-[4px] bg-gray-700 rounded-full overflow-hidden">
				<div 
					class="h-full rounded-full transition-all duration-300 {getStrengthColor(password)}" 
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
						<svg width="36" height="36" viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" class="shrink-0">
							<circle cx="24" cy="24" r="24" fill="#1A1F1E"/>
							<rect x="16" y="22" width="16" height="14" rx="2" stroke="#4CDA95" stroke-width="2"/>
							<path d="M19 22v-3a5 5 0 0 1 10 0v3" stroke="#4CDA95" stroke-width="2"/>
						</svg>
						<div class="relative flex-1 h-[45px] rounded-[9.489px] overflow-hidden border border-[#4CDA95] bg-[rgba(30,47,39,0.31)]">
							{#if passwordConfirmVisible}
								<input
									class="w-full h-full outline-none text-sm text-white text-opacity-70 placeholder:text-white placeholder:text-opacity-40 px-3 bg-transparent"
									type="text"
									bind:value={confirmPassword}
									placeholder="Confirm your password"
								/>
							{:else}
								<input
									class="w-full h-full outline-none text-sm text-white text-opacity-70 placeholder:text-white placeholder:text-opacity-40 px-3 bg-transparent"
									type="password"
									bind:value={confirmPassword}
									placeholder="Confirm your password"
								/>
							{/if}
							<button type="button" class="absolute right-3 top-1/2 -translate-y-1/2" on:click={togglePasswordConfirmVisibility}>
								<svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
									<mask id="mask0_461_173" style="mask-type:luminance" maskUnits="userSpaceOnUse" x="0" y="0" width="24" height="24">
										<path d="M24 0H0V24H24V0Z" fill="white" />
									</mask>
									<g mask="url(#mask0_461_173)">
										<path
											d="M17.8817 19.2971C16.1229 20.4127 14.0824 21.0034 11.9997 21.0001C6.60766 21.0001 2.12166 17.1201 1.18066 12.0001C1.61069 9.67078 2.78229 7.54296 4.52066 5.93407L1.39166 2.80807L2.80666 1.39307L22.6057 21.1931L21.1907 22.6071L17.8817 19.2971ZM5.93466 7.35007C4.57566 8.58566 3.62898 10.2089 3.22266 12.0001C3.53495 13.3666 4.16192 14.6412 5.05366 15.7227C5.9454 16.8041 7.07729 17.6625 8.35921 18.2294C9.64114 18.7963 11.0377 19.0562 12.4378 18.9882C13.8378 18.9203 15.2027 18.5265 16.4237 17.8381L14.3957 15.8101C13.5324 16.3539 12.5099 16.5882 11.4959 16.4745C10.482 16.3609 9.5367 15.906 8.81523 15.1845C8.09376 14.4631 7.63889 13.5178 7.52522 12.5039C7.41156 11.4899 7.64584 10.4674 8.18966 9.60407L5.93466 7.35007ZM12.9137 14.3281L9.67166 11.0861C9.49372 11.539 9.45185 12.0341 9.55117 12.5105C9.65048 12.9868 9.88668 13.4239 10.2308 13.768C10.5749 14.1121 11.012 14.3483 11.4884 14.4476C11.9647 14.5469 12.4598 14.505 12.9127 14.3271L12.9137 14.3281ZM20.8067 16.5921L19.3757 15.1621C20.0442 14.2094 20.5201 13.1353 20.7767 12.0001C20.5049 10.8098 19.994 9.68721 19.2748 8.70056C18.5557 7.71391 17.6435 6.88379 16.5936 6.26067C15.5437 5.63755 14.378 5.23443 13.1674 5.07583C11.9569 4.91723 10.7267 5.00644 9.55166 5.33807L7.97366 3.76007C9.22066 3.27007 10.5797 3.00007 11.9997 3.00007C17.3917 3.00007 21.8777 6.88007 22.8187 12.0001C22.5123 13.6658 21.8236 15.2377 20.8067 16.5921ZM11.7227 7.50807C12.3592 7.46873 12.9968 7.56513 13.5932 7.79088C14.1897 8.01663 14.7313 8.36658 15.1822 8.81752C15.6332 9.26846 15.9831 9.81009 16.2089 10.4066C16.4346 11.003 16.531 11.6406 16.4917 12.2771L11.7227 7.50807Z"
											fill="#757575"
										/>
									</g>
								</svg>
							</button>
						</div>
					</div>
					{#if passwordsMatch === false && password}
						<p class="text-red-500 text-[14px] mt-[5px]">Passwords do not match</p>
					{/if}
				</div>

				<!-- Automatic Login -->
				<div class="max-w-md mx-auto mt-4 flex items-center">
					<input 
						type="checkbox" 
						id="automaticLogin"
						class="w-5 h-5 rounded border-[1.423px] border-[#4CDA95] bg-grayTealinux focus:ring-[#4CDA95]"
                        bind:checked={autologin}
					/>
					<label for="automaticLogin" class="ml-2 text-sm text-white">Automatic Login</label>
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
/>
