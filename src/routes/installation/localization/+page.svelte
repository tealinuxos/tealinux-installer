<script lang="ts">
	import type { Keyboard, Variant } from '$types/keyboard-variant.js';
	import type { Locale } from '$types/locale-variant.js';
	import type { Timezone } from '$types/timezone-variant.js';
	import type { RouteId } from '$app/types';
	import { onMount } from 'svelte';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import GlowingText from '$lib/components/ui/GlowingText.svelte';
	import SearchButton from './SearchButton.svelte';
	import Preview from './Preview.svelte';
	import {
		getBlueprintInfo,
		getKeyboardJSON,
		getLocaleJSON,
		getTimezoneJSON
	} from '$lib/utils/read_utils.js';
	import { commands } from '$types/commands.js';
	import Navigation from '$lib/components/Navigation.svelte';
	import { afterNavigate } from '$app/navigation';

	// Additional variables
	let prevRoute = $state<RouteId | null>(null);

	// Locale
	let locales = $state<Locale[] | null>([]);
	let filteredLocales = $state<Locale[] | null>([]);
	let selectedLocale = $state<string | null>(null);
	let showLocaleModal = $state(false);
	let localeSearchTerm = $state('');

	// Timezone
	let timezones = $state<Timezone[] | null>([]);
	let filteredRegion = $state<Timezone[] | null>([]);
	let selectedTimezone = $state<Timezone | null>(null);
	let selectedRegion = $state<string | null>(null);
	let selectedCity = $state<string | null>(null);
	let filteredCity = $state<string[] | null>([]);
	let showRegionModal = $state(false);
	let regionSearchTerm = $state('');
	let showCityModal = $state(false);
	let citySearchTerm = $state('');

	// Keyboard Layout
	let keyboards = $state<Keyboard[] | null>([]);
	let filteredLayouts = $state<Keyboard[] | null>([]);
	let selectedLayout = $state<Keyboard | null>(null);
	let filteredVariants = $state<Variant[] | null>([]);
	let selectedVariant = $state<Variant | null>(null);
	let showLayoutModal = $state(false);
	let showVariantModal = $state(false);
	let layoutSearchTerm = $state('');
	let variantSearchTerm = $state('');

	const getDefault = async () => {
		const blueprint = await getBlueprintInfo();

		keyboards = await getKeyboardJSON();
		filteredLayouts = keyboards;

		timezones = await getTimezoneJSON();
		filteredRegion = timezones;

		locales = await getLocaleJSON();
		filteredLocales = locales;

		if (blueprint?.locale?.main) {
			selectedLocale = blueprint.locale.main;
		} else {
			selectedLocale = locales?.find((l) => l.name === 'en_US.UTF-8 UTF-8')?.name ?? null;
		}

		if (blueprint?.timezone) {
			const targetRegion = blueprint.timezone.region;

			selectedTimezone = timezones?.find((z) => z.region === targetRegion) ?? null;

			selectedRegion = targetRegion;

			filteredCity = selectedTimezone?.city ?? [];

			selectedCity = blueprint.timezone.city;
		} else {
			selectedTimezone = timezones?.find((zone) => zone.region === 'Asia') ?? null;
			selectedRegion = selectedTimezone?.region ?? 'Asia';
			filteredCity = selectedTimezone?.city ?? [];

			const defaultCity = 'Jakarta';
			selectedCity = filteredCity.find((c) => c === defaultCity) ?? defaultCity;
		}

		if (blueprint?.keyboard) {
			const layoutCode = blueprint.keyboard.layout;
			selectedLayout = keyboards?.find((layout) => layout.code === layoutCode) ?? null;

			if (selectedLayout) {
				const defaultVariant: Variant = {
					code: null,
					name: selectedLayout.name
				};

				filteredVariants = [defaultVariant, ...(selectedLayout.variant ?? [])];

				const variantCode = blueprint.keyboard.variant || null;

				if (variantCode) {
					selectedVariant = filteredVariants.find((v) => v.code === variantCode) ?? defaultVariant;
				} else {
					selectedVariant = defaultVariant;
				}
			}
		} else {
			selectedLayout = keyboards?.find((keyb) => keyb.code === 'us') ?? null;

			if (selectedLayout) {
				const defaultVariant: Variant = {
					code: null,
					name: selectedLayout.name
				};
				selectedVariant = defaultVariant;
				filteredVariants = [defaultVariant, ...(selectedLayout.variant ?? [])];
			} else {
				selectedLayout = null;
				selectedVariant = null;
				filteredVariants = [];
			}
		}
	};

	const selectKeyboardLayout = async (keyboard: Keyboard | null) => {
		if (keyboard) {
			selectedLayout = keyboard;
			showLayoutModal = false;

			const defaultVariant: Variant = { code: null, name: selectedLayout.name };

			selectedVariant = defaultVariant;

			filteredVariants = [defaultVariant, ...(selectedLayout.variant ?? [])];

			await setCosmicKeymapPreview(selectedLayout, selectedVariant);
		}
	};

	const selectKeyboardVariant = async (variant: Variant) => {
		selectedVariant = variant;
		showVariantModal = false;

		if (selectedLayout) {
			await setCosmicKeymapPreview(selectedLayout, selectedVariant);
		}
	};

	const setCosmicKeymapPreview = async (layout: Keyboard, variant: Variant) => {
		if (layout && variant) {
			await commands.setCosmicKeymap(false, layout.code, variant.code);
		}
	};

	const selectTimezoneRegion = (timezone: Timezone | null) => {
		if (timezone) {
			selectedTimezone = timezone;
			selectedRegion = timezone.region;
			showRegionModal = false;

			filteredCity = timezone.city ?? [];

			selectedCity = filteredCity.length > 0 ? filteredCity[0] : null;
		}
	};

	const selectTimezoneCity = (city: string) => {
		selectedCity = city;
		citySearchTerm = city;
		showCityModal = false;
	};

	const selectLocale = (locale: Locale) => {
		if (locale) {
			selectedLocale = locale.name;
			localeSearchTerm = locale.name;
			showLocaleModal = false;
		}
	};

	const handleSetLocalization = async () => {
		if (!selectedLocale || !selectedRegion || !selectedCity || !selectedLayout) {
			console.error('Incomplete localization data');
			return;
		}

		await commands.blueprintSetLocale(selectedLocale);
		await commands.blueprintSetTimezone(selectedRegion, selectedCity);
		await commands.blueprintSetKeyboard(selectedLayout.code, selectedVariant?.code ?? null);

		await commands.readRefreshDisk();
	};

	afterNavigate(({ from }) => {
		prevRoute = from?.url.pathname as RouteId;
		console.log(prevRoute);
	});

	onMount(async () => {
		await getDefault();

		if (selectedLayout && selectedVariant) {
			await setCosmicKeymapPreview(selectedLayout, selectedVariant);
		}
	});
</script>

<TwoSide>
	{#snippet left()}
		<div class="mx-[35px] space-y-[15px]">
			<h1 class="font-archivo font-semibold text-[28px]">
				Set up your <span class="text-green-tealinux">keyboard</span><br />
				<span class="text-green-tealinux">layout</span>,
				<span class="text-green-tealinux">timezone</span>,
				<span class="text-green-tealinux">locale</span>
			</h1>
			<p class="font-jakarta text-sm font-extralight">
				Select your preffered keyboard layout, adjust the timezone based on your location, and
				configure locale to define regional preferences.
			</p>
		</div>
	{/snippet}
	{#snippet right()}
		<div
			class="flex flex-col h-[562px] p-5 space-y-[15px] mb-[15px] bg-black/30 border-[0.5px] border-gray-900 rounded-[10px] font-jakarta"
		>
			<!-- select locale -->
			<div class="space-y-2.5">
				<!-- label -->
				<GlowingText size="[11]" text="Locale" />
				<!-- selector? -->
				<SearchButton
					title={selectedLocale || 'Select Locale'}
					notFoundMessage="Locale Not Found"
					bind:show={showLocaleModal}
					bind:keyword={localeSearchTerm}
					data={filteredLocales}
					field="name"
					onclick={selectLocale}
					selected={selectedLocale}
				/>
			</div>
			<!-- select Timezone -->
			<div class="space-y-2.5">
				<!-- label -->
				<GlowingText size="[11]" text="Timezone" />
				<!-- selector? -->
				<div class="flex gap-3">
					<SearchButton
						title="Select Timezone Region"
						notFoundMessage="No region found"
						bind:show={showRegionModal}
						bind:keyword={regionSearchTerm}
						data={filteredRegion}
						field="region"
						onclick={selectTimezoneRegion}
						selected={selectedRegion}
					/>
					<SearchButton
						title="Select Timezone City"
						notFoundMessage="No city found for {selectedRegion}"
						bind:show={showCityModal}
						bind:keyword={citySearchTerm}
						data={filteredCity}
						onclick={selectTimezoneCity}
						nullValue={selectedRegion}
						selected={selectedCity}
					/>
				</div>
			</div>
			<!-- select Keyboard Layout -->
			<div class="space-y-2.5">
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
						selected={selectedLayout ? selectedLayout.name : ''}
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
						selected={selectedVariant ? selectedVariant.name : ''}
					/>
				</div>

				<!-- keyboard test -->
				<input
					type="text"
					placeholder="Type here to test your keyboard..."
					class="p-2.5 border border-border bg-[#101010] rounded-[14px] text-[15px] justify-between h-fit w-full"
				/>
			</div>
			<div
				class="flex flex-col p-2.5 gap-y-[15px] border border-border bg-[#101010] rounded-[14px] text-[15px] justify-between h-fit w-full"
			>
				<!-- label -->
				<GlowingText size="[11]" text="Preview" />
				<Preview {selectedLocale} {selectedRegion} {selectedCity} />
			</div>
		</div>
	{/snippet}
</TwoSide>

<Navigation
	currentStep={2}
	currentTitle="Localization"
	prevPath="/installation"
	nextPath={prevRoute === '/installation/summary'
		? '/installation/summary'
		: '/installation/partitioning'}
	nextAction={handleSetLocalization}
/>
