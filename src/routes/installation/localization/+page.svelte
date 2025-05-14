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

	const selectKeyboardLayout = async (keyboard) => {
		selectedKeyboard = keyboard;
		selectedLayout = selectedKeyboard.code;
		showLayoutModal = false;

		layoutSearchTerm = selectedKeyboard.name;

		// Set variant selection when changing keyboard to the first entry
		variantSearchTerm = selectedKeyboard.variant.length ? selectedKeyboard.variant[0].name : null;

		selectedVariant = selectedKeyboard.variant.length ? selectedKeyboard.variant[0].code : null;

		filteredVariants = selectedKeyboard.variant;

        await setPreview(selectedLayout, selectedVariant);
	};

	const selectKeyboardVariant = async (variant) => {
		selectedVariant = variant.code;
		variantSearchTerm = variant.name;
		showVariantModal = false;

        await setPreview(selectedLayout, selectedVariant);
	};

    const setPreview = async (layout, variant) => {
        console.log('invokeing')
        await invoke("set_cosmic_keymap", { layout, variant });
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
			class="flex flex-col h-[562px] p-5 space-y-[15px] mb-[15px] bg-black/30 border-[0.5px] border-gray-900 rounded-[10px] font-jakarta"
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
