<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { getBlueprint } from '../global.js';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import GlowingText from '$lib/components/ui/GlowingText.svelte';

	let json = [];
	let filteredKeyboards = [];
	let filteredVariants = [];
	let selectedKeyboard = null;
	let selectedLayout = null;
	let selectedVariant = null;
	let showLayoutModal = false;
	let showVariantModal = false;
	let searchTerm = '';
	let variantSearchTerm = '';
	let showVariants = {};

	let locales = [];

	const getKeyboard = async () => {
		invoke('get_keyboard_json').then((response) => {
			json = JSON.parse(response);
			filteredKeyboards = json;
			console.log(json);
		});
	};

	const toggleVariants = (keyboardName) => {
		showVariants = {
			...showVariants,
			[keyboardName]: !showVariants[keyboardName]
		};
	};

	function filterOptions() {
		const term = searchTerm.toLowerCase();
		filteredKeyboards = json.filter((e) => e.name.toLowerCase().includes(term));
	}

	function filterVariantOptions() {

		const term = variantSearchTerm.toLowerCase();

		if (selectedKeyboard) {
			filteredVariants = selectedKeyboard.variant
				.filter(variant => variant.name.toLowerCase().includes(term))
		}
	}

	$: searchTerm, filterOptions();
	$: variantSearchTerm, filterVariantOptions();
    $: console.log(`Selected Keyboard: ${selectedLayout} - ${selectedVariant}`);

	const selectKeyboardLayout = (keyboard) => {

		selectedKeyboard = keyboard;
		selectedLayout = selectedKeyboard.code;
		searchTerm = selectedKeyboard.name;
		showLayoutModal = false;

		// Set variant selection when changing keyboard to the first entry
		variantSearchTerm = selectedKeyboard.variant.length
            ? selectedKeyboard.variant[0].name
            : null;

        selectedVariant = selectedKeyboard.variant.length
            ? selectedKeyboard.variant[0].code
            : null;

        filterVariantOptions()
	};

	const selectKeyboardVariant = (variant) => {

		selectedVariant = variant.code;
		variantSearchTerm = variant.name;
		showVariantModal = false;
	};

	const handleSetKeyboard = async () => {
		await invoke('blueprint_set_keyboard', { layout: selectedLayout, variant: selectedVariant });
	};

	onMount(() => {
		getKeyboard();
		getBlueprint().then((blueprint) => {
			console.log(blueprint);
			if (blueprint.locale === null) {
				selectedLayout = 'us';
				selectedVariant = 'euro';
				searchTerm = 'English (US)';
				variantSearchTerm = 'English (US)';
			} else {
				selectedLayout = blueprint.keyboard.layout;
				selectedVariant = blueprint.keyboard.variant;
				searchTerm = '';
				variantSearchTerm = '';
			}
		});
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
				<div
					class="flex p-[10px] border border-border bg-[#101010] rounded-[14px] items-center text-[15px] justify-between h-fit w-full"
				>
					<div>
						<span>{searchTerm || "Select a keyboard layout"}</span>
				  	</div>
				  
					<div>
						<span>
							<svg
								width="14"
								height="9"
								viewBox="0 0 14 9"
								fill="none"
								xmlns="http://www.w3.org/2000/svg"
								class="cursor-pointer"
								on:click={() => (showLayoutModal = true)}
							>
								<path d="M1 1.5L7 7.5L13 1.5" stroke="#26A768" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
							</svg>
						</span>
					</div>
				</div>
				<!-- keyboard varian -->
				<div
					class="flex p-[10px] border border-border bg-[#101010] rounded-[14px] text-[15px] items-center justify-between h-fit w-full"
				>
					<div>
						<span>{variantSearchTerm || "Select a keyboard variant"}</span>
					</div>
					<div>
						<span>
							<svg
								width="14"
								height="9"
								viewBox="0 0 14 9"
								fill="none"
								xmlns="http://www.w3.org/2000/svg"
								class="cursor-pointer"
								on:click={() => {
									if (!selectedLayout) {
										showLayoutModal = true;
									} else {
										showVariantModal = true;
									}
								}}
							>
								<path d="M1 1.5L7 7.5L13 1.5" stroke="#26A768" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
							</svg>
						</span>
					</div>
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

<!-- Keyboard Name Modal -->
{#if showLayoutModal}
	<div class="fixed inset-0 flex items-center justify-center backdrop-blur-sm z-80">
		<div class="absolute inset-0 bg-black/50" on:click={() => (showLayoutModal = false)}></div>
		<div class="flex flex-col min-w-[434px] max-h-full justify-center items-center p-4 bg-black rounded-lg border border-[#3C6350] shadow-[0_0_10px_rgba(38,167,104,0.25)] overflow-auto z-90">
			<div class="w-full p-6 z-10">
				<h2 class="text-xl font-bold mb-4 text-white">Select Keyboard Layout</h2>
				<input
					type="text"
					bind:value={searchTerm}
					placeholder="Search keyboard layout"
					class="w-full p-2 border rounded-lg mb-4 bg-[#1c1c1c] text-white"
				/>
				<!-- daftar keyboard -->
				<div class="max-h-60 overflow-auto space-y-2">
					{#if filteredKeyboards.length > 0}
						{#each filteredKeyboards as keyboard}
							<div
								class="flex items-center justify-between p-2 bg-[#303030] text-white rounded-md cursor-pointer hover:bg-gray-700"
								style="height: 28px; padding: 3px 16px;"
								on:click={() => selectKeyboardLayout(keyboard)}
							>
								<span>{keyboard.name}</span>
								<span class="text-sm text-gray-400">{keyboard.description || ''}</span>
							</div>
						{/each}
					{:else}
						<div class="text-white">No keyboards found</div>
					{/if}
				</div>
				<button
					class="mt-4 w-full p-2 bg-red-500 text-white rounded-lg hover:bg-red-600"
					on:click={() => (showLayoutModal = false)}
				>
					Close
				</button>
			</div>
		</div>
	</div>
{/if}

<!-- Keyboard Variant Modal -->
{#if showVariantModal}
	<div class="fixed inset-0 flex items-center justify-center backdrop-blur-sm z-80">
		<div class="absolute inset-0 bg-black/50" on:click={() => (showVariantModal = false)}></div>
		<div class="flex flex-col min-w-[434px] max-h-full justify-center items-center p-4 bg-black rounded-lg border border-[#3C6350] shadow-[0_0_10px_rgba(38,167,104,0.25)] overflow-auto z-90">
			<div class="w-full p-6 z-10">
				<h2 class="text-xl font-bold mb-4 text-white">Select Keyboard Variant for {selectedKeyboard?.name || 'Selected Layout'}</h2>
				<input
					type="text"
					bind:value={variantSearchTerm}
					placeholder="Search keyboard variant"
					class="w-full p-2 border rounded-lg mb-4 bg-[#1c1c1c] text-white"
				/>
				<!-- daftar variant -->
				<div class="max-h-60 overflow-auto space-y-2">
					{#if filteredVariants.length > 0}
						{#each filteredVariants as variant}
							<div
								class="flex items-center justify-between p-2 bg-[#303030] text-white rounded-md cursor-pointer hover:bg-gray-700"
								style="height: 28px; padding: 3px 16px;"
								on:click={() => selectKeyboardVariant(variant)}
							>
								<span>{variant.name}</span>
								<span class="text-sm text-gray-400">{variant.description || ''}</span>
							</div>
						{/each}
					{:else}
						<div class="text-white">No variants found</div>
					{/if}
				</div>
				<button
					class="mt-4 w-full p-2 bg-red-500 text-white rounded-lg hover:bg-red-600"
					on:click={() => (showVariantModal = false)}
				>
					Close
				</button>
			</div>
		</div>
	</div>
{/if}
