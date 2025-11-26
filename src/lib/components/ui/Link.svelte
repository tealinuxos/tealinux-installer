<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import type { RouteId } from '$app/types';
	import { resolve } from '$app/paths';
	import GlowingText from './GlowingText.svelte';

	interface Props extends HTMLAttributes<HTMLElement> {
		isDisabled?: boolean;
		btnText?: string;
		href?: RouteId | string;
	}

	let { isDisabled = false, btnText = 'Button', href = '#', ...rest }: Props = $props();

	const baseClasses =
		'flex items-center justify-center w-[102px] max-w-[102px] py-[6px] px-[27px] text-md bg-[#101010] rounded-sm border-[0.3px] border-[#3C6350] transition-all duration-300';

	const stateClasses = $derived(
		isDisabled
			? 'opacity-60 cursor-not-allowed'
			: 'cursor-pointer hover:shadow-[0_0px_5px_1px_#00B85E]'
	);
</script>

{#if isDisabled}
	<button
		type="button"
		disabled
		class="{baseClasses} {stateClasses}"
		aria-disabled="true"
		{...rest}
	>
		<GlowingText text={btnText} size="lg" />
	</button>
{:else}
	<a href={resolve(href as RouteId)} class="{baseClasses} {stateClasses}" {...rest}>
		<GlowingText text={btnText} size="lg" />
	</a>
{/if}
