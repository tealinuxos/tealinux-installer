<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { getBlueprint } from '../global.js';
	import { goto } from '$app/navigation';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import GlowingText from '$lib/components/ui/GlowingText.svelte';
	import SearchButton from './SearchButton.svelte';
	import Preview from './Preview.svelte';
	import Navigation from '../../../lib/components/Navigation.svelte';

	let json = $state([]);

	// Locale
	let locales = $state([]);
	let filteredLocales = $state([]);
	let selectedLocale = $state(null);
	let showLocaleModal = $state(false);
	let localeSearchTerm = $state('');

	// Timezone
	let timezones = $state([]);
	let showRegionModal = $state(false);
	let regionSearchTerm = $state('');
	let showCityModal = $state(false);
	let citySearchTerm = $state('');
	let filteredRegion = $state([]);
	let filteredCity = $state([]);
	let selectedTimezone = $state(null);
	let selectedRegion = $state(null);
	let selectedCity = $state(null);

	// Keyboard Layout
	let filteredLayouts = $state([]);
	let filteredVariants = $state([]);
	let selectedKeyboard = $state(null);
	let selectedLayout = $state(null);
	let selectedVariant = $state(null);
	let showLayoutModal = $state(false);
	let showVariantModal = $state(false);
	let layoutSearchTerm = $state('');
	let variantSearchTerm = $state('');
	let showVariants = {};

	const getKeyboard = async () => {
		let result = await invoke('get_keyboard_json');

		return JSON.parse(result);
	};

	const getLocale = async () => {
		let result = await invoke('get_locale_json');

		return JSON.parse(result);
	};

	const getTimezone = async () => {
		let result = await invoke('get_timezone_json');

		return JSON.parse(result);
	};

	const selectKeyboardLayout = (keyboard) => {
		selectedKeyboard = keyboard;
		selectedLayout = selectedKeyboard.code;
		showLayoutModal = false;

		layoutSearchTerm = selectedKeyboard.name;

		// Set variant selection when changing keyboard to the first entry
		variantSearchTerm = selectedKeyboard.variant.length ? selectedKeyboard.variant[0].name : null;

		selectedVariant = selectedKeyboard.variant.length ? selectedKeyboard.variant[0].code : null;

		filteredVariants = selectedKeyboard.variant;
	};

	const selectKeyboardVariant = (variant) => {
		selectedVariant = variant.code;
		variantSearchTerm = variant.name;
		showVariantModal = false;
	};

	const selectTimezoneRegion = (timezone) => {
		selectedTimezone = timezone;
		selectedRegion = selectedTimezone.region;
		showRegionModal = false;

		regionSearchTerm = selectedRegion;

		// Set city selection when changing timezone to the first entry
		citySearchTerm = selectedCity = selectedTimezone.city.length ? selectedTimezone.city[0] : null;

		filteredCity = selectedTimezone.city;
	};

	const selectTimezoneCity = (city) => {
		selectedCity = city;
		citySearchTerm = city;
		showCityModal = false;
	};

	const selectLocale = (locale) => {
		selectedLocale = locale.name;
		localeSearchTerm = locale.name;
		showLocaleModal = false;
	};

	const handleSetLocalization = async () => {

		await invoke('blueprint_set_locale', { locale: selectedLocale });
		await invoke('blueprint_set_timezone', { region: selectedRegion, city: selectedCity });
		await invoke('blueprint_set_keyboard', { layout: selectedLayout, variant: selectedVariant });

        goto("/installation/partitioning");
	};

	onMount(async () => {
		json = await getKeyboard();
		filteredLayouts = json;

		timezones = await getTimezone();
		filteredRegion = timezones;

		locales = await getLocale();
		filteredLocales = locales;

		let defaultKeyboard = json.length ? json[0] : null;
		let defaultTimezone = timezones.length
			? timezones.find((zone) => zone.region === 'Asia')
			: null;
		let defaultLocale = locales.length
			? locales.find((locale) => locale.name === 'en_US.UTF-8 UTF-8')
			: null;

		getBlueprint().then((blueprint) => {
			if (blueprint.locale !== null) {
				selectedLayout = blueprint.keyboard.layout;
				selectedVariant = blueprint.keyboard.variant;
				layoutSearchTerm = '';
				variantSearchTerm = '';
			}
		});

		selectLocale(defaultLocale);
		selectTimezoneRegion(defaultTimezone);
		selectKeyboardLayout(defaultKeyboard);
	});

	// debugging purpose; remove this later
	$effect(() => {
		$inspect(`Selected Keyboard: ${selectedLayout} - ${selectedVariant}`);
	});
</script>

<TwoSide>
	{#snippet left()}
		<div class="mx-[35px] space-y-[15px]">
			<h1 class="font-jakarta font-[800] text-[28px]">
				Set up your keyboard<br />
				layout, timezone, locale
			</h1>
			<p class="font-jakarta text-sm font-[200]">
				Qorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam eu turpis molestie, dictum
				est a, mattis tellus.
			</p>
		</div>
	{/snippet}
	{#snippet right()}
		<div
			class="flex flex-col <script>
	// ... (keep your existing script section unchanged) ...
</script>

<TwoSide>
	{#snippet left()}
		<div class="mx-[35px] space-y-[15px]">
			<h1 class="font-jakarta font-[800] text-[28px]">
				Setup your user<br />
			</h1>
			<p class="font-jakarta text-sm font-[200]">
				Qorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam eu turpis molestie, dictum
				est a, mattis tellus.
			</p>
		</div>
	{/snippet}
	{#snippet right()}
		<div class="relative w-full h-full flex flex-col">
			<header class="flex items-center justify-center w-full gap-[10px] py-4 bg-whiteTealinux z-30">
				<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
				<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
				<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
				<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
				<div class="w-[20px] h-[20px] bg-grayTealinux rounded-full"></div>
				<div class="w-[20px] h-[20px] bg-grayTealinux rounded-full"></div>
			</header>

			<div class="flex-1 flex items-center justify-center p-6">
				<div class="w-full max-w-md bg-black/30 border-[0.5px] border-gray-900 rounded-[10px] p-6">
					<form class="space-y-4">
						<h1 class="text-center text-2xl font-bold mb-6">Create User</h1>
						
						<div class="space-y-1">
							<div class="flex items-center gap-x-2">
								<h2 class="font-medium">Full name</h2>
								{#if !fullname}
									<p class="text-red-500 text-xs">* Required</p>
								{/if}
							</div>
							<div class="relative flex items-center h-[45px] rounded-lg overflow-hidden border-2 border-white bg-grayTealinux">
								<input
									class="w-full h-full outline-none text-sm text-white text-opacity-70 placeholder:text-white placeholder:text-opacity-40 px-3 bg-transparent"
									type="text"
									bind:value={fullname}
									placeholder="Full name"
								/>
							</div>
						</div>

						<div class="space-y-1">
							<div class="flex items-center gap-x-2">
								<h2 class="font-medium">Username</h2>
								{#if !username}
									<p class="text-red-500 text-xs">* Required</p>
								{/if}
							</div>
							<div class="relative flex items-center h-[45px] rounded-lg overflow-hidden border-2 border-white bg-grayTealinux">
								<input
									class="w-full h-full outline-none text-sm text-white text-opacity-70 placeholder:text-white placeholder:text-opacity-40 px-3 bg-transparent"
									type="text"
									bind:value={username}
									placeholder="Username"
								/>
							</div>
						</div>

						<div class="space-y-1">
							<div class="flex items-center gap-x-2">
								<h2 class="font-medium">Computer Name</h2>
								{#if !hostname}
									<p class="text-red-500 text-xs">* Required</p>
								{/if}
							</div>
							<div class="relative flex items-center h-[45px] rounded-lg overflow-hidden border-2 border-white bg-grayTealinux">
								<input
									class="w-full h-full outline-none text-sm text-white text-opacity-70 placeholder:text-white placeholder:text-opacity-40 px-3 bg-transparent"
									type="text"
									bind:value={hostname}
									placeholder="Computer name"
								/>
							</div>
						</div>

						<div class="space-y-1">
							<div class="flex items-center gap-x-2">
								<h2 class="font-medium">Password</h2>
								{#if !password}
									<p class="text-red-500 text-xs">* Required</p>
								{/if}
							</div>
							<div class="relative flex items-center h-[45px] rounded-lg overflow-hidden border-2 border-white bg-grayTealinux">
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
								<button type="button" on:click={togglePasswordVisibility} class="absolute right-3">
									<svg width="24" height="24" viewBox="0 0 24 24" fill="#757575" xmlns="http://www.w3.org/2000/svg">
										<path d="M17.8817 19.2971C16.1229 20.4127 14.0824 21.0034 11.9997 21.0001C6.60766 21.0001 2.12166 17.1201 1.18066 12.0001C1.61069 9.67078 2.78229 7.54296 4.52066 5.93407L1.39166 2.80807L2.80666 1.39307L22.6057 21.1931L21.1907 22.6071L17.8817 19.2971ZM5.93466 7.35007C4.57566 8.58566 3.62898 10.2089 3.22266 12.0001C3.53495 13.3666 4.16192 14.6412 5.05366 15.7227C5.9454 16.8041 7.07729 17.6625 8.35921 18.2294C9.64114 18.7963 11.0377 19.0562 12.4378 18.9882C13.8378 18.9203 15.2027 18.5265 16.4237 17.8381L14.3957 15.8101C13.5324 16.3539 12.5099 16.5882 11.4959 16.4745C10.482 16.3609 9.5367 15.906 8.81523 15.1845C8.09376 14.4631 7.63889 13.5178 7.52522 12.5039C7.41156 11.4899 7.64584 10.4674 8.18966 9.60407L5.93466 7.35007ZM12.9137 14.3281L9.67166 11.0861C9.49372 11.539 9.45185 12.0341 9.55117 12.5105C9.65048 12.9868 9.88668 13.4239 10.2308 13.768C10.5749 14.1121 11.012 14.3483 11.4884 14.4476C11.9647 14.5469 12.4598 14.505 12.9127 14.3271L12.9137 14.3281ZM20.8067 16.5921L19.3757 15.1621C20.0442 14.2094 20.5201 13.1353 20.7767 12.0001C20.5049 10.8098 19.994 9.68721 19.2748 8.70056C18.5557 7.71391 17.6435 6.88379 16.5936 6.26067C15.5437 5.63755 14.378 5.23443 13.1674 5.07583C11.9569 4.91723 10.7267 5.00644 9.55166 5.33807L7.97366 3.76007C9.22066 3.27007 10.5797 3.00007 11.9997 3.00007C17.3917 3.00007 21.8777 6.88007 22.8187 12.0001C22.5123 13.6658 21.8236 15.2377 20.8067 16.5921ZM11.7227 7.50807C12.3592 7.46873 12.9968 7.56513 13.5932 7.79088C14.1897 8.01663 14.7313 8.36658 15.1822 8.81752C15.6332 9.26846 15.9831 9.81009 16.2089 10.4066C16.4346 11.003 16.531 11.6406 16.4917 12.2771L11.7227 7.50807Z"/>
									</svg>
								</button>
							</div>
						</div>

						<div class="space-y-1">
							<div class="flex items-center gap-x-2">
								<h2 class="font-medium">Confirm Password</h2>
								{#if !password}
									<p class="text-red-500 text-xs">* Required</p>
								{/if}
							</div>
							<div class="relative flex items-center h-[45px] rounded-lg overflow-hidden border-2 border-white bg-grayTealinux">
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
								<button type="button" on:click={togglePasswordConfirmVisibility} class="absolute right-3">
									<svg width="24" height="24" viewBox="0 0 24 24" fill="#757575" xmlns="http://www.w3.org/2000/svg">
										<path d="M17.8817 19.2971C16.1229 20.4127 14.0824 21.0034 11.9997 21.0001C6.60766 21.0001 2.12166 17.1201 1.18066 12.0001C1.61069 9.67078 2.78229 7.54296 4.52066 5.93407L1.39166 2.80807L2.80666 1.39307L22.6057 21.1931L21.1907 22.6071L17.8817 19.2971ZM5.93466 7.35007C4.57566 8.58566 3.62898 10.2089 3.22266 12.0001C3.53495 13.3666 4.16192 14.6412 5.05366 15.7227C5.9454 16.8041 7.07729 17.6625 8.35921 18.2294C9.64114 18.7963 11.0377 19.0562 12.4378 18.9882C13.8378 18.9203 15.2027 18.5265 16.4237 17.8381L14.3957 15.8101C13.5324 16.3539 12.5099 16.5882 11.4959 16.4745C10.482 16.3609 9.5367 15.906 8.81523 15.1845C8.09376 14.4631 7.63889 13.5178 7.52522 12.5039C7.41156 11.4899 7.64584 10.4674 8.18966 9.60407L5.93466 7.35007ZM12.9137 14.3281L9.67166 11.0861C9.49372 11.539 9.45185 12.0341 9.55117 12.5105C9.65048 12.9868 9.88668 13.4239 10.2308 13.768C10.5749 14.1121 11.012 14.3483 11.4884 14.4476C11.9647 14.5469 12.4598 14.505 12.9127 14.3271L12.9137 14.3281ZM20.8067 16.5921L19.3757 15.1621C20.0442 14.2094 20.5201 13.1353 20.7767 12.0001C20.5049 10.8098 19.994 9.68721 19.2748 8.70056C18.5557 7.71391 17.6435 6.88379 16.5936 6.26067C15.5437 5.63755 14.378 5.23443 13.1674 5.07583C11.9569 4.91723 10.7267 5.00644 9.55166 5.33807L7.97366 3.76007C9.22066 3.27007 10.5797 3.00007 11.9997 3.00007C17.3917 3.00007 21.8777 6.88007 22.8187 12.0001C22.5123 13.6658 21.8236 15.2377 20.8067 16.5921ZM11.7227 7.50807C12.3592 7.46873 12.9968 7.56513 13.5932 7.79088C14.1897 8.01663 14.7313 8.36658 15.1822 8.81752C15.6332 9.26846 15.9831 9.81009 16.2089 10.4066C16.4346 11.003 16.531 11.6406 16.4917 12.2771L11.7227 7.50807Z"/>
									</svg>
								</button>
							</div>
							{#if passwordsMatch === false && password}
								<p class="text-red-500 text-sm mt-1">Passwords do not match</p>
							{/if}
						</div>
					</form>
				</div>
			</div>
		</div>
	{/snippet}
</TwoSide>

<Navigation
	totalSteps={5}
	currentStep={2}
	currentTitle="Localization"
	prevPath="/installation"
	nextPath="/installation/summary"
	nextAction={handleSetAccount}
/> bg-black/30 border-[0.5px] border-gray-900 rounded-[10px] font-jakarta"
		>
			<!-- select locale -->
			<div class=" space-y-[10px]">
				<!-- label -->
				<GlowingText size="[11]" text="Locale" />
				<!-- selector? -->
				<SearchButton
					title="Select Locale"
					notFoundMessage="Locale Not Found"
					bind:show={showLocaleModal}
					bind:keyword={localeSearchTerm}
					data={filteredLocales}
					field="name"
					onclick={selectLocale}
				/>
			</div>
			<!-- select Timezone -->
			<div class=" space-y-[10px]">
				<!-- label -->
				<GlowingText size="[11]" text="Timezone" />
				<!-- selector? -->
				 <div class="flex gap-3">
					<SearchButton
					title="Select Timezone Region"
					notFoundMessage="Timezone Region Not Found"
					bind:show={showRegionModal}
					bind:keyword={regionSearchTerm}
					data={filteredRegion}
					field="region"
					onclick={selectTimezoneRegion}
				/>
				<SearchButton
					title="Select Timezone City"
					notFoundMessage="Timezone City Not Found"
					bind:show={showCityModal}
					bind:keyword={citySearchTerm}
					data={filteredCity}
					onclick={selectTimezoneCity}
				/>
				 </div>

			</div>
			<!-- select Keyboard Layout -->
			<div class=" space-y-[10px]">
				<!-- label -->
				<GlowingText size="[11]" text="Keyboard Layout" />
				<!-- keyboard name -->
				 <div class="flex gap-3">
					<SearchButton
					title="Select Keyboard Layout"
					notFoundMessage="Keyboard Layout Not Found"
					bind:show={showLayoutModal}
					bind:keyword={layoutSearchTerm}
					data={filteredLayouts}
					field="name"
					onclick={selectKeyboardLayout}
				/>
				<!-- keyboard varian -->
				<SearchButton
					title="Select Keyboard Variant"
					notFoundMessage="Keyboard Variant Not Found"
					bind:show={showVariantModal}
					bind:keyword={variantSearchTerm}
					field="name"
					data={filteredVariants}
					onclick={selectKeyboardVariant}
				/>
				 </div>

				<!-- keyboard test -->
				<input
					type="text"
					placeholder="test type here"
					class="p-[10px] border border-border bg-[#101010] rounded-[14px] text-[15px] justify-between h-fit w-full"
				/>
			</div>
			<div
				class="flex flex-col p-[10px] gap-y-[15px] border border-border bg-[#101010] rounded-[14px] text-[15px] justify-between h-fit w-full"
			>
				<!-- label -->
				<GlowingText size="[11]" text="Preview" />
				<Preview {selectedLocale} {selectedRegion} {selectedCity} />
			</div>
		</div>
	{/snippet}
</TwoSide>

<Navigation
	totalSteps={5}
	currentStep={2}
	currentTitle="Localization"
	prevPath="/installation"
	nextPath="/installation/partitioning"
	nextAction={handleSetLocalization}
/>
