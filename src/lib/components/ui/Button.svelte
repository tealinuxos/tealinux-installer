<script>
	import { on } from 'svelte/events';
	import GlowingText from './GlowingText.svelte';

	let { isDisabled, onclick, btnText, variant } = $props();
	// Default values
	isDisabled = isDisabled || false;
	onclick = onclick || (() => {});
	btnText = btnText || 'Button';
	variant = variant || 'glow';
</script>

{#if variant === 'glow'}
	<button
		class="cursor-pointer grid place-items-center py-[6px] px-[27px] text-md bg-[#101010] disabled:opacity-50 rounded-sm enabled:hover:shadow-[0_0px_5px_1px_#00B85E] transition-all duration-300 border-[0.3px] border-[#3C6350] disabled:cursor-not-allowed"
		disabled={isDisabled}
		{onclick}
		aria-label="Go to previous step"
	>
		<GlowingText text={btnText} size={'lg'} />
	</button>
{/if}
