<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { getBlueprint } from '../global.js';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import GlowingText from '$lib/components/ui/GlowingText.svelte';
	import SearchButton from './SearchButton.svelte';

	let json = $state([]);

    // Locale
	let locales = [];
    let filteredLocales = [];
    let selectedLocale = null;
    let showLocaleModal = false;

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
		invoke('get_locale_json').then((response) => {
			locales = JSON.parse(response);
			filteredLocales = locales;
		});
	};

	const toggleVariants = (keyboardName) => {
		showVariants = {
			...showVariants,
			[keyboardName]: !showVariants[keyboardName]
		};
	};

	const selectKeyboardLayout = (keyboard) => {

		selectedKeyboard = keyboard;
		selectedLayout = selectedKeyboard.code;
		showLayoutModal = false;

        layoutSearchTerm = selectedKeyboard.name;

		// Set variant selection when changing keyboard to the first entry
		variantSearchTerm = selectedKeyboard.variant.length
            ? selectedKeyboard.variant[0].name
            : null;

        selectedVariant = selectedKeyboard.variant.length
            ? selectedKeyboard.variant[0].code
            : null;

        filteredVariants = selectedKeyboard.variant;
	};

	const selectKeyboardVariant = (variant) => {

		selectedVariant = variant.code;
		variantSearchTerm = variant.name;
		showVariantModal = false;
	};

	const handleSetKeyboard = async () => {
		await invoke('blueprint_set_keyboard', { layout: selectedLayout, variant: selectedVariant });
	};

	onMount(async () => {

        json = await getKeyboard();
        filteredLayouts = await getKeyboard();

        let defaultKeyboard = json.length ? json[0] : null;

        // getLocale();

        getBlueprint().then((blueprint) => {

            if (blueprint.locale !== null) {
                selectedLayout = blueprint.keyboard.layout;
                selectedVariant = blueprint.keyboard.variant;
                layoutSearchTerm = '';
                variantSearchTerm = '';
            }
        });

        selectKeyboardLayout(defaultKeyboard);
	});

    // debugging purpose; remove this later
    $effect(() => {
        $inspect(`Selected Keyboard: ${selectedLayout} - ${selectedVariant}`);
    })

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
				<div
					class="flex p-[10px] border border-border bg-[#101010] rounded-[14px] text-[15px] justify-between h-fit w-full"
				>
					<div>
						<span>US</span>
					</div>
					<div>
						<span>united state</span>
					</div>
				</div>
			</div>
			<!-- select Timezone -->
			<div class=" space-y-[10px]">
				<!-- label -->
				<GlowingText size="[11]" text="Timezone" />
				<!-- selector? -->
				<div
					class="flex p-[10px] border border-border bg-[#101010] rounded-[14px] text-[15px] items-center justify-between h-fit w-full"
				>
					<div>
						<span>Region/City</span>
					</div>
					<div>
						<sp1an>							
							<svg width="14" height="9" viewBox="0 0 14 9" fill="none" xmlns="http://www.w3.org/2000/svg">
							<path d="M1 1.5L7 7.5L13 1.5" stroke="#26A768" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
							</svg></sp1an>
					</div>
				</div>
			</div>
			<!-- select Keyboard Layout -->
			<div class=" space-y-[10px]">
				<!-- label -->
				<GlowingText size="[11]" text="Keyboard Layout" />
				<!-- keyboard name -->
                <SearchButton
                    bind:show={showLayoutModal}
                    bind:keyword={layoutSearchTerm}
                    data={filteredLayouts}
                    onclick={selectKeyboardLayout}
                />
				<!-- keyboard varian -->
                <SearchButton
                    bind:show={showVariantModal}
                    bind:keyword={variantSearchTerm}
                    bind:result={selectedVariant}
                    data={filteredVariants}
                    onclick={selectKeyboardVariant}
                />
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
				<div class="flex flex-col gap-y-[10px]">
					<!-- preview item -->
					<div class="flex gap-x-4">
						<img src="/icons/clock-vector.svg" alt="clock" />
						<span>21:26:21</span>
					</div>
					<!-- preview item -->
					<div class="flex gap-x-4">
						<img src="/icons/calendar-vector.svg" alt="clock" />
						<span>Sabtu, 14 Juni 2025</span>
					</div>
					<!-- preview item -->
					<div class="flex gap-x-4">
						<img src="/icons/currency-vector.svg" alt="clock" />
						<span>1.234.567,89 - Rp 1.234,56</span>
					</div>
				</div>
			</div>
		</div>
	{/snippet}
</TwoSide>
