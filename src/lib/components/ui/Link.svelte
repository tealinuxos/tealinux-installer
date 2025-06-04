<script>
	import GlowingText from './GlowingText.svelte';

	let { isDisabled, onclick, btnText, href } = $props();
	// Default values
	isDisabled = isDisabled || false;
	onclick = onclick || (() => {});
	btnText = btnText || 'Button';
	href = href || '#';

	function handleClick(event) {
		if (isDisabled) {
			event.preventDefault();
			return;
		}
		onclick();
	}
</script>

<a
	class="grid w-[102px] max-w-[102px] place-items-center py-[6px] px-[27px] text-md bg-[#101010] rounded-sm border-[0.3px] border-[#3C6350] transition-all duration-300 {isDisabled
		? 'opacity-60 cursor-not-allowed'
		: 'cursor-pointer hover:shadow-[0_0px_5px_1px_#00B85E]'}"
	onclick={handleClick}
	aria-label="Go to previous step"
	href={isDisabled ? null : href}
	aria-disabled={isDisabled}
>
	<GlowingText text={btnText} size={'lg'} />
</a>
