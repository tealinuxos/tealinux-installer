<script lang="ts" generics="T">
	import type { HTMLAttributes } from 'svelte/elements';
	import { cn } from '$lib/utils/cn';

	type OptionType = T;

	interface Props extends Omit<HTMLAttributes<HTMLElement>, 'onselect'> {
		options?: OptionType[] | null;
		value?: OptionType | null;
		onselect?: (option: OptionType) => void;
		displayField?: string;
		sizeField?: string;
		formatter?: ((size: number) => string) | null;
		loadingText?: string;
		defaultText?: string;
		notFoundText?: string;
		simpleMode?: boolean;
		isLoading?: boolean;
		error?: string | null;
	}

	let {
		options = [],
		value = $bindable(),
		displayField = 'name',
		sizeField = '',
		formatter = null,
		loadingText = 'Loading...',
		defaultText = 'Select an option',
		notFoundText = 'No option',
		simpleMode = false,
		isLoading = false,
		error = null,
		...props
	}: Props = $props();

	let isOpen = $state(false);
	let selectElement = $state<HTMLDivElement | null>(null);

	function handleClickOutside(event: MouseEvent) {
		if (selectElement && event.target instanceof Node && !selectElement.contains(event.target)) {
			isOpen = false;
		}
	}

	function toggleDropdown() {
		if (!isLoading && !error) {
			isOpen = !isOpen;
		}
	}

	function selectOption(option: OptionType) {
		value = option;
		isOpen = false;
	}

	function getDisplayText(option: OptionType | null | undefined): string {
		if (isLoading) return loadingText;
		if (error) return error;
		if (!options || options.length === 0) return notFoundText;
		if (!option) return defaultText;

		if (typeof option === 'object') {
			// eslint-disable-next-line  @typescript-eslint/no-explicit-any
			const opt = option as Record<string, any>;

			const display = displayField && opt[displayField] ? opt[displayField] : opt.name || opt.value;

			if (simpleMode) return String(display);

			const size = sizeField && opt[sizeField] ? ` (${formatSize(opt[sizeField])})` : '';

			return `${display}${size}`;
		}

		return String(option);
	}

	function formatSize(size: string | unknown): string {
		if (!size || typeof size !== 'string') return '';
		try {
			const sizeInBytes = Number(size.slice(0, -1));

			if (isNaN(sizeInBytes)) return '';

			return formatter
				? formatter(sizeInBytes)
				: `${((sizeInBytes * 512) / 1024 ** 3).toFixed(2)} GB`;
		} catch {
			return '';
		}
	}

	$effect(() => {
		document.addEventListener('click', handleClickOutside);
		return () => document.removeEventListener('click', handleClickOutside);
	});
</script>

<div
	class={cn('relative flex h-12 w-full flex-col items-stretch gap-2.5', props.class)}
	bind:this={selectElement}
>
	<div
		onclick={() => {
			if (options) toggleDropdown();
		}}
		role="button"
		tabindex="0"
		onkeydown={(e) => e.key === 'Enter' && toggleDropdown()}
		class={cn(
			'flex min-h-[42px] items-center justify-between px-[15px] py-[9px]',
			'rounded-[14px] border-[1.3px] bg-[#101010]',
			'font-jakarta text-[13px] font-semibold text-[#26a768]',
			'cursor-pointer transition-colors duration-200 ease-in-out',
			isOpen ? 'border-[#26A768]' : 'border-[#3C6350]',
			isLoading || !!error || !options ? 'pointer-events-none cursor-not-allowed opacity-70' : '',
			isLoading ? 'animate-pulse cursor-wait bg-[#0a1f16]' : ''
		)}
	>
		<div class="overflow-hidden text-ellipsis whitespace-nowrap pr-2">
			{getDisplayText(value)}
		</div>

		{#if !isLoading && !error}
			<div class={cn('flex transition-transform duration-200 ease-out', isOpen && 'rotate-180')}>
				<svg width="14" height="9" viewBox="0 0 14 9" fill="none">
					<path
						d="M1 1.33325L7 7.33325L13 1.33325"
						stroke="#26A768"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
					/>
				</svg>
			</div>
		{:else if isLoading}
			<div
				class="size-3.5 animate-spin rounded-full border-2 border-[#26a768]/30 border-t-[#26a768]"
			></div>
		{/if}
	</div>

	{#if isOpen && !isLoading && !error}
		<div
			class={cn(
				'absolute left-0 right-0 top-full z-50 mt-[5px]',
				'max-h-[200px] overflow-y-auto',
				'rounded-[14px] border border-[#3C6350] bg-[#101010] shadow-xl',
				'[&::-webkit-scrollbar-thumb]:rounded-[10px] [&::-webkit-scrollbar-thumb]:bg-border',
				'[&::-webkit-scrollbar]:w-.15'
			)}
		>
			{#each options as option (option)}
				<div
					onclick={() => selectOption(option)}
					role="option"
					tabindex="0"
					aria-selected={value === option}
					onkeydown={(e) => e.key === 'Enter' && selectOption(option)}
					class={cn(
						'cursor-pointer px-[15px] py-2.5 font-jakarta text-[13px] transition-colors duration-200',
						value === option ? 'bg-[#032b17] text-[#4cda95]' : 'text-white hover:bg-[#032b17]'
					)}
				>
					{getDisplayText(option)}
				</div>
			{/each}
		</div>
	{/if}
</div>
