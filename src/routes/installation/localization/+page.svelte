<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { getBlueprint, refreshDisk } from '../global.js';
	import { goto } from '$app/navigation';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import GlowingText from '$lib/components/ui/GlowingText.svelte';
	import SearchButton from './SearchButton.svelte';
	import Preview from './Preview.svelte';
	import Navigation from '../../../lib/components/Navigation.svelte';

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
	let keyboards = $state([]);
	let filteredLayouts = $state([]);
	let filteredVariants = $state([]);
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
        if (keyboard) {
            selectedLayout = keyboard;
            showLayoutModal = false;

            let defaultVariant = { code: null, name: selectedLayout.name };

            selectedVariant = defaultVariant;

            filteredVariants = [defaultVariant];

            filteredVariants = filteredVariants.concat(selectedLayout.variant);


            await setCosmicKeymapPreview(selectedLayout, selectedVariant);
        }
	};

	const selectKeyboardVariant = async (variant) => {
		selectedVariant = variant;
		showVariantModal = false;

        await setCosmicKeymapPreview(selectedLayout, selectedVariant);
	};

    const setCosmicKeymapPreview = async (layout, variant) => {
        if (layout && variant) {
            await invoke("set_cosmic_keymap", {
                live: false,
                layout: layout.code,
                variant: variant.code
            });
        }
    };


	const selectTimezoneRegion = (timezone) => {
        if (timezone) {
            selectedTimezone = timezone;
            selectedRegion = selectedTimezone.region;
            showRegionModal = false;


            // Set city selection when changing timezone to the first entry
            selectedCity = selectedTimezone.city?.length ? selectedTimezone.city[0] : null ?? null;

            filteredCity = selectedTimezone.city?.length ? selectedTimezone.city : null ?? null;
        }
	};

	const selectTimezoneCity = (city) => {
		selectedCity = city;
		citySearchTerm = city;
		showCityModal = false;
	};

	const selectLocale = (locale) => {
        if (locale) {
            selectedLocale = locale.name;
            localeSearchTerm = locale.name;
            showLocaleModal = false;
        }
	};

	const handleSetLocalization = async () => {

		await invoke('blueprint_set_locale', { locale: selectedLocale });
		await invoke('blueprint_set_timezone', { region: selectedRegion, city: selectedCity });
		await invoke('blueprint_set_keyboard', {
            layout: selectedLayout.code,
            variant: selectedVariant.code
        });

        await refreshDisk();
	};

    const getDefault = async () => {

		keyboards = await getKeyboard();
		filteredLayouts = keyboards;

		timezones = await getTimezone();
		filteredRegion = timezones;

		locales = await getLocale();
		filteredLocales = locales;
        
		getBlueprint().then((blueprint) => {
            if (blueprint.locale) {
                selectedLocale = blueprint.locale.main;
            } else {
                let temp = locales.length
                    ? locales.find((locale) => locale.name === 'en_US.UTF-8 UTF-8')
                    : null;

                selectedLocale = temp
                    ? temp.name
                    : null;
            }

            if (blueprint.timezone) {
				selectedTimezone = timezones.find(zone => zone.region === blueprint.timezone.region);
                selectedRegion = selectedTimezone.region;

                filteredCity = selectedTimezone
                    ? selectedTimezone.city
                    : null;

                selectedCity = selectedTimezone.city?.find(city => city === blueprint.timezone.city) ?? null;
            } else {
                selectedTimezone = timezones.length
                    ? timezones.find((zone) => zone.region === 'Asia')
                    : null;

                selectedRegion = selectedTimezone
                    ? selectedTimezone.region
                    : 'Asia';

                filteredCity = selectedTimezone
                    ? selectedTimezone.city
                    : null;

                selectedCity = selectedTimezone
                    ? selectedTimezone.city.find((city) => city === 'Jakarta')
                    : 'Jakarta';
            }

			if (blueprint.keyboard) {
				selectedLayout = keyboards.find(layout => layout.code === blueprint.keyboard.layout);

                let defaultVariant = { code: null, name: selectedLayout?.name };

                if (blueprint.keyboard.variant) {
                    selectedVariant = selectedLayout.variant.find(variant => variant.code === blueprint.keyboard.variant);
                } else {
                    selectedVariant = defaultVariant;
                }

                filteredVariants = selectedLayout
                    ? [defaultVariant].concat(selectedLayout.variant)
                    : null;

			} else {
                selectedLayout = keyboards.length
                    ? keyboards.find(keyb => keyb.code === "us")
                    : null;

                selectedVariant = { code: null, name: selectedLayout?.name }

                filteredVariants = selectedLayout ?
                    [selectedVariant].concat(selectedLayout.variant) :
                    null;
            }
		});

    }

	onMount(async () => {

        await getDefault();

		selectLocale(selectedLocale);
		selectTimezoneRegion(selectedRegion);
        selectTimezoneCity(selectedCity);
		selectKeyboardLayout(selectedLayout);
		selectKeyboardVariant(selectedVariant);
        setCosmicKeymapPreview(selectedLayout, selectedVariant);
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
					title={selectedLocale || "Select Locale"}
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
			<div class=" space-y-[10px]">
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
	currentStep={2}
	currentTitle="Localization"
	prevPath="/installation"
	nextPath="/installation/partitioning"
	nextAction={handleSetLocalization}
/>
