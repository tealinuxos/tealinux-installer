<script>
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { fly } from 'svelte/transition';
	import { getBlueprint } from '../global.js';

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

<Sidebar />
<div class="relative w-full">
	<header class="absolute top-0 flex items-center justify-center w-full gap-[10px] mt-[40px]">
		<div class="w-[20px] h-[20px] bg-greenTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-grayTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-grayTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-grayTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-grayTealinux rounded-full"></div>
		<div class="w-[20px] h-[20px] bg-grayTealinux rounded-full"></div>
	</header>
	<section class="flex flex-col items-center justify-center h-[85dvh]">
		<form class="text-center w-[50dvw] rounded-md min-h-[50dvh]">
			<div>
				<h1 class="text-center mb-6 font-bold text-[32px] font-archivo">Select Keyboard Layout</h1>
			</div>
			<div class="relative max-w-md mx-auto mb-4">
				<h2 class="font-poppinsemibold text-left mb-2">Region</h2>
				<div
					class="relative flex items-center w-full bg-grayTealinux h-[45px] {showOptions
						? ' rounded-t-lg border border-greyBorder'
						: 'rounded-lg border-2 border-black'} overflow-hidden shadow-lg"
				>
					<input
						type="text"
						placeholder="select keyboard.."
						class="peer h-full w-full outline-hidden text-sm text-black text-opacity-50 bg-transparent pr-2 pl-[12px] font-poppin"
						bind:value={searchTerm}
						on:click={toggleOptions}
					/>
					<svg
						class="mr-[16px]"
						width="20"
						height="21"
						viewBox="0 0 20 21"
						fill="none"
						xmlns="http://www.w3.org/2000/svg"
					>
						<path
							d="M19 19.5L14.65 15.15M17 9.5C17 13.9183 13.4183 17.5 9 17.5C4.58172 17.5 1 13.9183 1 9.5C1 5.08172 4.58172 1.5 9 1.5C13.4183 1.5 17 5.08172 17 9.5Z"
							stroke="black"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						/>
					</svg>
				</div>
				{#if showOptions}
					<div
						class="z-10 absolute w-full bg-white border border-greyBorder rounded-b-xl max-h-[30vh] overflow-y-auto"
						in:fly={{ y: -10, duration: 1000 }}
						out:fly={{ y: 10, duration: 300 }}
					>
						{#each filteredKeyboards.sort((a, b) => a.name.localeCompare(b.name)) as keyboard}
							{@const name = keyboard.name}
							{@const code = keyboard.code}
							<div
								class="border border-b-grayBorder last:border-none bg-white hover:bg-slate-100 transition-all"
							>
								<div
									class="flex w-full items-center justify-between py-4 px-4 border border-b-grayBorder last:border-none bg-white transition-all"
									on:click={() => toggleVariants(name)}
								>
									<p>{name} - {code}</p>
									<img
										src="/dropDownMain.svg"
										alt="arrow"
										class="{showVariants[name]
											? 'rotate-180'
											: ''} transition-transform duration-300"
									/>
								</div>
								{#if showVariants[name]}
									{#each keyboard.variant as variant}
										<label
											class="flex flex-row-reverse w-full items-center justify-between py-4 px-4 border border-b-grayBorder last:border-none bg-greyVariant transition-all"
											for="{name}-{code}"
										>
											<input
												required
												type="radio"
												id="{name}-{code}"
												value={variant.code}
												class="w-5 h-5"
												on:click={() => selectKeyboards(code, variant.code, name)}
											/>
											<div class="text-start text-[14px]">
												<p>{variant.code}</p>
												<p class=" text-[#0D1814] text-opacity-50">{name} - {code}</p>
											</div>
										</label>
									{/each}
								{/if}
							</div>
						{/each}
					</div>
				{/if}

				<div class="fixed bottom-[0px] w-[28rem] max-w-md mx-auto my-30 h-[15dvh]">
					<div class="flex justify-between w-full font-poppin">
						<a
							href="/installation"
							class="text-white cursor-pointer bg-greenTealinux focus:ring-4 font-medium rounded-lg text-sm px-5 py-2.5 mb-2"
							>Back</a
						>
						<a
							href="/installation/timezone"
							on:click={handleSetKeyboard}
							class="text-white bg-greenTealinux {selectedKeyboards
								? ''
								: ' brightness-75 pointer-events-none'}  focus:ring-4 focus:ring-gray-900 font-medium rounded-lg text-sm px-5 py-2.5 mb-2 focus:outline-hidden"
							>Next</a
						>
					</div>
				</div>
			</div>
		</form>
	</section>
</div>
