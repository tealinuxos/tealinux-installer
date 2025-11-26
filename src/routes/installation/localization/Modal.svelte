<script lang="ts" generics="T">
	import { onMount } from 'svelte';

	interface Props {
		show: boolean;
		onclick: (item: T) => void;
		data: T[] | null;
		notFoundMessage?: string;
		field?: keyof T | null;
		selectedLabel: string | null;
		keyword?: string;
	}

	let {
		show = $bindable(),
		onclick,
		data,
		notFoundMessage = 'Data not found',
		field = null,
		selectedLabel,
		keyword = $bindable('')
	}: Props = $props();

	let tempSelected = $state<T | null>(null);
	let selectedIndex = $state(0);

	let filteredData = $derived.by(() => {
		if (!data) return [];
		if (!keyword.trim()) return data;

		const term = keyword.toLowerCase();
		return data.filter((item) => {
			const val = getValue(item);
			return String(val).toLowerCase().includes(term);
		});
	});

	const getValue = (item: T): string => {
		if (field && typeof item === 'object' && item !== null) {
			return String(item[field]);
		}
		return String(item);
	};

	const getDescription = (item: T): string => {
		if (typeof item === 'object' && item !== null && 'description' in item) {
			/* eslint-disable @typescript-eslint/no-explicit-any */
			return String((item as any).description);
		}
		return '';
	};

	function handleSelect(item: T) {
		tempSelected = item;
		confirmSelection();
	}

	function confirmSelection() {
		if (tempSelected) {
			onclick(tempSelected);
		} else if (filteredData.length === 1) {
			onclick(filteredData[0]);
		}
		show = false;
		keyword = '';
	}

	function cancelSelection() {
		show = false;
		keyword = '';
	}

	const scrollToSelected = (label: string | null) => {
		if (!label) return;
		setTimeout(() => {
			let el = document.getElementById(`option-${label}`);
			if (el) {
				el.scrollIntoView({ behavior: 'smooth', block: 'center' });
			}
		}, 100);
	};

	const onKeyDown = (event: KeyboardEvent) => {
		if (!show) return;

		const list = filteredData;
		if (!list.length) return;

		switch (event.key) {
			case 'ArrowDown':
				event.preventDefault();
				if (selectedIndex < list.length - 1) {
					selectedIndex++;
					tempSelected = list[selectedIndex];
					scrollToSelected(getValue(tempSelected));
				}
				break;
			case 'ArrowUp':
				event.preventDefault();
				if (selectedIndex > 0) {
					selectedIndex--;
					tempSelected = list[selectedIndex];
					scrollToSelected(getValue(tempSelected));
				}
				break;
			case 'Enter':
				event.preventDefault();
				if (!tempSelected && list.length > 0) {
					tempSelected = list[0];
				}
				confirmSelection();
				break;
			case 'Escape':
				event.preventDefault();
				cancelSelection();
				break;
		}
	};

	$effect(() => {
		if (keyword || keyword === '') {
			selectedIndex = 0;
			tempSelected = filteredData.length > 0 ? filteredData[0] : null;
		}
	});

	onMount(() => {
		if (selectedLabel) {
			scrollToSelected(selectedLabel);
			const found = data?.find((d) => getValue(d) === selectedLabel);
			if (found) tempSelected = found;
		}
	});
</script>

<div class="fixed inset-0 flex items-center justify-center z-9999">
	<div
		class="absolute inset-0 bg-black/60 backdrop-blur-md transition-opacity"
		onclick={cancelSelection}
		role="button"
		tabindex="-1"
		onkeydown={(e) => e.key === 'Escape' && cancelSelection()}
	></div>

	<div
		class="relative flex flex-col w-[434px] max-h-[80vh] p-4 bg-[#0a0a0a] rounded-[14px] border border-[#3C6350] shadow-[0_0_30px_rgba(38,167,104,0.15)] z-10"
	>
		<div class="w-full p-2 space-y-4">
			<div class="relative flex items-center w-full">
				<div class="absolute left-3 text-[#26A768]">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="16"
						height="16"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					>
						<circle cx="11" cy="11" r="8"></circle>
						<line x1="21" y1="21" x2="16.65" y2="16.65"></line>
					</svg>
				</div>

				<!-- svelte-ignore a11y_autofocus -->
				<input
					type="text"
					bind:value={keyword}
					class="w-full h-10 pl-10 pr-10 bg-[#122C1F]/50 text-white rounded-lg border border-[#26A768] focus:outline-none focus:ring-2 focus:ring-[#26A768] focus:border-transparent placeholder-white/30 transition-all"
					placeholder="Search..."
					autofocus
				/>

				{#if keyword}
					<button
						class="absolute right-3 text-[#26A768] hover:text-white transition-colors"
						onclick={() => (keyword = '')}
						aria-label="Clear search"
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<line x1="18" y1="6" x2="6" y2="18"></line>
							<line x1="6" y1="6" x2="18" y2="18"></line>
						</svg>
					</button>
				{/if}
			</div>

			<hr class="border-[#3C6350]/30" />

			<div class="max-h-[300px] overflow-y-auto space-y-1 pr-1 custom-scrollbar">
				{#if filteredData.length > 0}
					{#each filteredData as item, index (index)}
						{@const value = getValue(item)}
						{@const desc = getDescription(item)}
						{@const isSelected =
							tempSelected === item || (!tempSelected && value === selectedLabel)}

						<div
							id={`option-${value}`}
							class={`flex items-center justify-between px-4 py-2.5 rounded-lg cursor-pointer transition-all duration-200
                                ${
																	isSelected
																		? 'bg-[#26A768] text-white shadow-lg shadow-[#26A768]/20'
																		: 'text-gray-300 hover:bg-[#122C1F] hover:text-white'
																}`}
							onclick={() => handleSelect(item)}
							onmouseenter={() => {
								tempSelected = item;
								selectedIndex = index;
							}}
							role="option"
							aria-selected={isSelected}
							tabindex="0"
							onkeydown={(e) => e.key === 'Enter' && handleSelect(item)}
						>
							<span class="font-medium text-sm">{value}</span>
							{#if desc}
								<span class={`text-xs ${isSelected ? 'text-white/80' : 'text-gray-500'}`}
									>{desc}</span
								>
							{/if}
						</div>
					{/each}
				{:else}
					<div class="py-8 text-center text-gray-500 flex flex-col items-center gap-2">
						<span>{notFoundMessage}</span>
					</div>
				{/if}
			</div>
		</div>
	</div>
</div>

<svelte:window onkeydown={onKeyDown} />
