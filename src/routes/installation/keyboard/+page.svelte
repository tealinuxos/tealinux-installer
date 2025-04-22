<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { getBlueprint } from '../global.js';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import GlowingText from '$lib/components/ui/GlowingText.svelte';

	let json = [];
	let filteredKeyboards = [];
	let selectedKeyboards = null;
	let keyboardName = null;
	let keyboardVariant = null;
	let showOptions = false;
	let searchTerm = '';
	let showVariants = {};

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

	$: searchTerm, filterOptions();

	const selectKeyboards = (code, variant, name) => {
		keyboardName = code;
		keyboardVariant = variant;
		selectedKeyboards = variant;
		searchTerm = name;
		toggleOptions();
	};

	const toggleOptions = () => {
		showOptions = !showOptions;
	};

	const handleSetKeyboard = async () => {
		await invoke('blueprint_set_keyboard', { layout: keyboardName, variant: keyboardVariant });
	};

	onMount(() => {
		getKeyboard();
		getBlueprint().then((blueprint) => {
			console.log(blueprint);
			if (blueprint.locale === null) {
				keyboardName = 'us';
				keyboardVariant = 'euro';
				selectedKeyboards = 'euro';
				searchTerm = 'English (US)';
			} else {
				keyboardName = blueprint.keyboard.layout;
				selectedKeyboards = blueprint.keyboard.variant;
				keyboardVariant = blueprint.keyboard.variant;
				searchTerm = '';
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
					class="flex p-[10px] border border-border bg-[#101010] rounded-[14px] text-[15px] justify-between h-fit w-full"
				>
					<div>
						<span>Region/City</span>
					</div>
					<div>
						<span>---</span>
					</div>
				</div>
			</div>
			<!-- select Keyboard Layout -->
			<div class=" space-y-[10px]">
				<!-- label -->
				<GlowingText size="[11]" text="Keyboard Layout" />
				<!-- selector? -->
				<div
					class="flex p-[10px] border border-border bg-[#101010] rounded-[14px] text-[15px] justify-between h-fit w-full"
				>
					<div>
						<span>English (US)</span>
					</div>
					<div>
						<span>---</span>
					</div>
				</div>
				<!-- selector? -->
				<div
					class="flex p-[10px] border border-border bg-[#101010] rounded-[14px] text-[15px] justify-between h-fit w-full"
				>
					<div>
						<span>English (US)</span>
					</div>
					<div>
						<span>---</span>
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
