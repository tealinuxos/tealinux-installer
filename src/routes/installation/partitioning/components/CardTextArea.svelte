<script lang="ts">
	import { cn } from '$lib/utils/cn';
	import type { HTMLAttributes } from 'svelte/elements';

	interface Props extends HTMLAttributes<HTMLButtonElement> {
		disabled?: boolean;
		initialDevice: string;
		caption?: string;
		showCaption: boolean;
		showIcon: boolean;
		isSelected: boolean;
		description?: string;
	}

	let {
		disabled,
		initialDevice: device,
		caption,
		showCaption,
		showIcon,
		isSelected,
		description,
		...props
	}: Props = $props();

	let backgroundColor = $derived(isSelected ? '#032B17' : '#101010');
	let iconColor = $derived(isSelected ? '#4CDA95' : '#3C6350');
	let borderColor = $derived(isSelected ? '#4CDA95' : '#3C6350');
</script>

<!-- UI -->
<button
	class={cn(
		'flex items-center justify-between h-12 px-3 py-3 rounded-lg w-full',
		{
			'cursor-not-allowed': disabled,
			'cursor-pointer': !disabled
		},
		`border-[1.3px] border-[${borderColor}] bg-[${backgroundColor}]`
	)}
	{...props}
	{disabled}
>
	<!-- Left Section -->
	<div class="flex flex-col justify-center">
		<div class="flex items-center gap-2">
			{#if showIcon}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill={iconColor}
					viewBox="0 0 24 24"
					class="w-5 h-5"
				>
					<path d="M3 3h18v6H3V3zm0 12h18v6H3v-6zm0-6h18v2H3v-2zm2 8h2v2H5v-2zm0-12h2v2H5V5z" />
				</svg>
			{/if}
			<span class="text-[15px] font-medium leading-[140%] font-jakarta text-[{iconColor}]">
				{device}
			</span>
		</div>
		{#if showCaption}
			<div class="text-[11px] font-normal font-jakarta text-[#9F9F9F] leading-[140%]">
				{caption}
			</div>
		{/if}
	</div>

	<!-- Right Section -->
	<div class="text-[11px] font-normal leading-[140%] font-jakarta text-right text-[{iconColor}]">
		{description}
	</div>
</button>
