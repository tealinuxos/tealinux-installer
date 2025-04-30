<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { getBlueprint } from '../global.js';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import GlowingText from '$lib/components/ui/GlowingText.svelte';
	import SearchableModal from '$lib/components/modals/Modal.svelte';

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
	let isLoading = true;

	const getKeyboard = async () => {
		try {
			const response = await invoke('get_keyboard_json');
			json = JSON.parse(response);
			filteredKeyboards = json;
			isLoading = false;
		} catch (error) {
			console.error('Error loading keyboard data:', error);
			isLoading = false;
		}
	};

	function filterOptions() {
		const term = searchTerm.toLowerCase();
		filteredKeyboards = json.filter((e) => e.name.toLowerCase().includes(term));
	}

	function filterVariantOptions() {
		const term = variantSearchTerm.toLowerCase();
		if (selectedKeyboard) {
			filteredVariants = selectedKeyboard.variant
				.filter(variant => variant.name.toLowerCase().includes(term));
		}
	}

	const handleKeyboardSelect = (event) => {
		const keyboard = event.detail;
		selectedKeyboard = keyboard;
		selectedLayout = keyboard.code;
		searchTerm = keyboard.name;
		
		if (keyboard.variant.length > 0) {
			variantSearchTerm = keyboard.variant[0].name;
			selectedVariant = keyboard.variant[0].code;
		}
	};

	const handleVariantSelect = (event) => {
		const variant = event.detail;
		selectedVariant = variant.code;
		variantSearchTerm = variant.name;
	};

	const handleSetKeyboard = async () => {
		if (selectedLayout && selectedVariant) {
			await invoke('blueprint_set_keyboard', { 
				layout: selectedLayout, 
				variant: selectedVariant 
			});
		}
	};

	onMount(() => {
		getKeyboard();
		getBlueprint().then((blueprint) => {
			if (blueprint.locale === null) {
				selectedLayout = 'us';
				selectedVariant = 'euro';
				searchTerm = 'English (US)';
				variantSearchTerm = 'English (US)';
			} else if (blueprint.keyboard) {
				selectedLayout = blueprint.keyboard.layout;
				selectedVariant = blueprint.keyboard.variant;
				searchTerm = '';
				variantSearchTerm = '';
			}
		});
	});

	$: searchTerm, filterOptions();
	$: variantSearchTerm, filterVariantOptions();
	$: if (selectedLayout && selectedVariant) {
		handleSetKeyboard();
	}
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

<!-- Keyboard Layout Modal -->
<SearchableModal
  title="Select Keyboard Layout"
  bind:showModal={showLayoutModal}
  items={filteredKeyboards}
  on:select={handleKeyboardSelect}
  on:close={() => showLayoutModal = false}
  placeholder="Search keyboard layouts"
  noResultsText={isLoading ? "Loading keyboards..." : "No keyboards found"}
/>

<!-- Keyboard Variant Modal -->
<SearchableModal
  title={`Select Variant for ${selectedKeyboard?.name || 'Keyboard'}`}
  bind:showModal={showVariantModal}
  items={filteredVariants}
  on:select={handleVariantSelect}
  on:close={() => showVariantModal = false}
  placeholder="Search keyboard variants"
  noResultsText={!selectedKeyboard ? "Please select a keyboard layout first" : 
                selectedKeyboard.variant.length === 0 ? "No variants available for this keyboard" : 
                "No variants found"}
/>