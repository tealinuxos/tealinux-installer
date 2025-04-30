<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { getBlueprint } from '../global.js';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import GlowingText from '$lib/components/ui/GlowingText.svelte';
	import Modal from '$lib/components/modals/Modal.svelte';

	// Keyboard state
	let keyboardData = [];
	let filteredKeyboards = [];
	let filteredVariants = [];
	let selectedKeyboard = null;
	let selectedLayout = null;
	let selectedVariant = null;
	let showLayoutModal = false;
	let showVariantModal = false;
	let keyboardSearchTerm = '';
	let variantSearchTerm = '';

	// Timezone state
	let timezoneData = [];
	let filteredTimezones = [];
	let selectedTimezone = { name: 'UTC', code: 'UTC' };
	let showTimezoneModal = false;
	let timezoneSearchTerm = '';

	// Locale state
	let localeData = [];
	let filteredLocales = [];
	let selectedLocale = { name: 'English (US)', code: 'en-US' };
	let showLocaleModal = false;
	let localeSearchTerm = '';

	let isLoading = true;

	// Data fetching
	const fetchData = async () => {
		try {
			const [keyboardRes, timezoneRes, localeRes] = await Promise.all([
				invoke('get_keyboard_json'),
				invoke('get_timezone_json'),
				invoke('get_locale_json')
			]);

			keyboardData = JSON.parse(keyboardRes);
			timezoneData = JSON.parse(timezoneRes);
			localeData = JSON.parse(localeRes);

			filteredKeyboards = keyboardData;
			filteredTimezones = timezoneData;
			filteredLocales = localeData;
		} catch (error) {
			console.error('Error loading data:', error);
		} finally {
			isLoading = false;
		}
	};

	// Filter functions
	function filterKeyboards() {
		const term = keyboardSearchTerm.toLowerCase();
		filteredKeyboards = keyboardData.filter(e => 
			e.name.toLowerCase().includes(term) ||
			(e.description && e.description.toLowerCase().includes(term))
		);
	}

	function filterVariants() {
		const term = variantSearchTerm.toLowerCase();
		if (selectedKeyboard?.variant) {
			filteredVariants = selectedKeyboard.variant.filter(variant => 
				variant?.name?.toLowerCase().includes(term) ||
				(variant?.description?.toLowerCase().includes(term))
			);
		} else {
			filteredVariants = [];
		}
	}

	function filterTimezones() {
		const term = timezoneSearchTerm.toLowerCase();
		filteredTimezones = timezoneData.filter(tz => 
			tz.name.toLowerCase().includes(term) || 
			tz.code.toLowerCase().includes(term) ||
			(tz.region && tz.region.toLowerCase().includes(term))
		);
	}

	function filterLocales() {
		const term = localeSearchTerm.toLowerCase();
		filteredLocales = localeData.filter(locale => 
			locale.name.toLowerCase().includes(term) || 
			locale.code.toLowerCase().includes(term)
		);
	}

	// Selection handlers
	const handleKeyboardSelect = (event) => {
		selectedKeyboard = event.detail;
		selectedLayout = selectedKeyboard.code;
		keyboardSearchTerm = selectedKeyboard.name;
		showLayoutModal = false;

		if (selectedKeyboard.variant?.length > 0) {
			variantSearchTerm = selectedKeyboard.variant[0].name;
			selectedVariant = selectedKeyboard.variant[0].code;
			filterVariants();
		}
	};

	const handleVariantSelect = (event) => {
		selectedVariant = event.detail.code;
		variantSearchTerm = event.detail.name;
		showVariantModal = false;
	};

	const handleTimezoneSelect = (event) => {
		selectedTimezone = event.detail;
		showTimezoneModal = false;
	};

	const handleLocaleSelect = (event) => {
		selectedLocale = event.detail;
		showLocaleModal = false;
	};

	// Save settings
	const saveSettings = async () => {
		try {
			if (selectedLayout && selectedVariant) {
				await invoke('blueprint_set_keyboard', { 
					layout: selectedLayout, 
					variant: selectedVariant 
				});
			}
			if (selectedTimezone) {
				await invoke('blueprint_set_timezone', { 
					timezone: selectedTimezone.code,
					region: selectedTimezone.region // Pastikan ini sesuai dengan yang dibutuhkan backend
				});
			}
			if (selectedLocale) {
				await invoke('blueprint_set_locale', { 
					locale: selectedLocale.code 
				});
			}
		} catch (error) {
			console.error('Error saving settings:', error);
		}
	};

	onMount(async () => {
		await fetchData();
		const blueprint = await getBlueprint();
		
		if (blueprint) {
			// Set keyboard
			if (blueprint.keyboard) {
				selectedLayout = blueprint.keyboard.layout;
				selectedVariant = blueprint.keyboard.variant;
				selectedKeyboard = keyboardData.find(k => k.code === selectedLayout);
				if (selectedKeyboard) {
					keyboardSearchTerm = selectedKeyboard.name;
					if (selectedVariant) {
						const variant = selectedKeyboard.variant?.find(v => v.code === selectedVariant);
						variantSearchTerm = variant?.name || '';
					}
				}
			}

			// Set timezone
			if (blueprint.timezone) {
				selectedTimezone = timezoneData.find(t => t.code === blueprint.timezone) || selectedTimezone;
			}

			// Set locale
			if (blueprint.locale) {
				selectedLocale = localeData.find(l => l.code === blueprint.locale) || selectedLocale;
			}
		}
	});

	// Reactive declarations
	$: keyboardSearchTerm, filterKeyboards();
	$: variantSearchTerm, filterVariants();
	$: timezoneSearchTerm, filterTimezones();
	$: localeSearchTerm, filterLocales();
	$: if (selectedLayout && selectedVariant && selectedTimezone && selectedLocale) {
		saveSettings();
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
				Configure your system preferences
			</p>
		</div>
	{/snippet}
	
	{#snippet right()}
		<div class="flex flex-col h-[562px] p-5 space-y-[15px] mb-[15px] bg-black/30 border-[0.5px] border-gray-900 rounded-[10px] font-jakarta">
			<!-- Locale Selector -->
			<div class="space-y-[10px]">
				<GlowingText size="[11]" text="Locale" />
				<div 
					class="flex p-[10px] border border-border bg-[#101010] rounded-[14px] text-[15px] justify-between h-fit w-full cursor-pointer"
					on:click={() => showLocaleModal = true}
				>
					<div>
						<span>{selectedLocale?.name || "Select locale"}</span>
					</div>
					<div>
						<svg width="14" height="9" viewBox="0 0 14 9" fill="none" xmlns="http://www.w3.org/2000/svg">
							<path d="M1 1.5L7 7.5L13 1.5" stroke="#26A768" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
						</svg>
					</div>
				</div>
			</div>
			
			<!-- Timezone Selector -->
			<div class="space-y-[10px]">
				<GlowingText size="[11]" text="Timezone" />
				<div 
					class="flex p-[10px] border border-border bg-[#101010] rounded-[14px] text-[15px] items-center justify-between h-fit w-full cursor-pointer"
					on:click={() => showTimezoneModal = true}
				>
					<div>
						<span>{selectedTimezone?.name || "Select timezone"}</span>
					</div>
					<div>
						<svg width="14" height="9" viewBox="0 0 14 9" fill="none" xmlns="http://www.w3.org/2000/svg">
							<path d="M1 1.5L7 7.5L13 1.5" stroke="#26A768" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
						</svg>
					</div>
				</div>
			</div>
			
			<!-- Keyboard Selection -->
			<div class="space-y-[10px]">
				<GlowingText size="[11]" text="Keyboard Layout" />
				
				<!-- Layout Selector -->
				<div 
					class="flex p-[10px] border border-border bg-[#101010] rounded-[14px] items-center text-[15px] justify-between h-fit w-full cursor-pointer"
					on:click={() => showLayoutModal = true}
				>
					<div>
						<span>{keyboardSearchTerm || "Select keyboard layout"}</span>
					</div>
					<div>
						<svg width="14" height="9" viewBox="0 0 14 9" fill="none" xmlns="http://www.w3.org/2000/svg">
							<path d="M1 1.5L7 7.5L13 1.5" stroke="#26A768" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
						</svg>
					</div>
				</div>
				
				<!-- Variant Selector -->
				<div 
					class="flex p-[10px] border border-border bg-[#101010] rounded-[14px] text-[15px] items-center justify-between h-fit w-full cursor-pointer"
					on:click={() => {
						if (!selectedLayout) showLayoutModal = true;
						else showVariantModal = true;
					}}
				>
					<div>
						<span>{variantSearchTerm || "Select keyboard variant"}</span>
					</div>
					<div>
						<svg width="14" height="9" viewBox="0 0 14 9" fill="none" xmlns="http://www.w3.org/2000/svg">
							<path d="M1 1.5L7 7.5L13 1.5" stroke="#26A768" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
						</svg>
					</div>
				</div>
				
				<!-- Test Input -->
				<input
					type="text"
					placeholder="Test your keyboard here"
					class="p-[10px] border border-border bg-[#101010] rounded-[14px] text-[15px] justify-between h-fit w-full"
				/>
			</div>
			
			<!-- Preview Section -->
			<div class="flex flex-col p-[10px] gap-y-[15px] border border-border bg-[#101010] rounded-[14px] text-[15px] justify-between h-fit w-full">
				<GlowingText size="[11]" text="Preview" />
				<div class="flex flex-col gap-y-[10px]">
					<div class="flex gap-x-4">
						<img src="/icons/clock-vector.svg" alt="clock" />
						<span>
							{#if selectedLocale}
								{new Date().toLocaleTimeString(selectedLocale.code)}
							{:else}
								--:--:--
							{/if}
						</span>
					</div>
					<div class="flex gap-x-4">
						<img src="/icons/calendar-vector.svg" alt="calendar" />
						<span>
							{#if selectedLocale}
								{new Date().toLocaleDateString(selectedLocale.code, { 
									weekday: 'long', 
									year: 'numeric', 
									month: 'long', 
									day: 'numeric' 
								})}
							{:else}
								--------
							{/if}
						</span>
					</div>
					<div class="flex gap-x-4">
						<img src="/icons/currency-vector.svg" alt="currency" />
						<span>
							{#if selectedLocale}
								{(1234567.89).toLocaleString(selectedLocale.code)} - 
								{selectedLocale.currency ? 
									new Intl.NumberFormat(selectedLocale.code, { 
										style: 'currency', 
										currency: selectedLocale.currency 
									}).format(1234.56) : 
									'---'}
							{:else}
								--- ---
							{/if}
						</span>
					</div>
				</div>
			</div>
		</div>
	{/snippet}
</TwoSide>

<!-- Timezone Modal -->


<!-- Locale Modal -->
<Modal
  title="Select Locale"
  bind:showModal={showLocaleModal}
  items={filteredLocales}
  on:select={handleLocaleSelect}
  on:close={() => showLocaleModal = false}
  placeholder="Search locales"
  noResultsText={isLoading ? "Loading locales..." : "No locales found"}
/>

<Modal
  title="Select Timezone"
  bind:showModal={showTimezoneModal}
  items={filteredTimezones}
  on:select={handleTimezoneSelect}
  on:close={() => showTimezoneModal = false}
  placeholder="Search timezones"
  noResultsText={isLoading ? "Loading timezones..." : "No timezones found"}
/>

<!-- Keyboard Layout Modal -->
<Modal
  title="Select Keyboard Layout"
  bind:showModal={showLayoutModal}
  items={filteredKeyboards}
  on:select={handleKeyboardSelect}
  on:close={() => showLayoutModal = false}
  placeholder="Search keyboard layouts"
  noResultsText={isLoading ? "Loading keyboards..." : "No keyboards found"}
/>

<!-- Keyboard Variant Modal -->
<Modal
  title={`Select Variant for ${selectedKeyboard?.name || 'Keyboard'}`}
  bind:showModal={showVariantModal}
  items={filteredVariants}
  on:select={handleVariantSelect}
  on:close={() => showVariantModal = false}
  placeholder="Search keyboard variants"
  noResultsText={!selectedKeyboard ? "Please select a keyboard layout first" : 
                selectedKeyboard.variant?.length === 0 ? "No variants available for this keyboard" : 
                "No variants found"}
/>