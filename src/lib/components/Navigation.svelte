<script>
	import GlowingText from './ui/GlowingText.svelte';

	let currentStep = $state(1); // Use Svelte 5 reactive state
	const totalSteps = 4;

	let percentage = $derived((currentStep / totalSteps) * 100);
</script>

<div class="rounded-sm w-[1050px] h-16 mx-auto text-glow">
	<div class="flex items-center justify-between w-full h-full bg-black/30 px-4">
		<!-- Back Button -->
		<button
			class="cursor-pointer grid place-items-center py-[9px] px-[30px] text-xl bg-[#101010] rounded-sm hover:shadow-[0_0px_5px_1px_#00B85E] transition-all duration-300 border-[0.3px] border-[#3C6350]"
			disabled={currentStep === 1}
			onclick={() => (currentStep = Math.max(1, currentStep - 1))}
			aria-label="Go to previous step"
		>
			<GlowingText text="Back" />
		</button>

		<!-- pages -->
		<div class="flex flex-col items-center justify-between space-y-2 my-[10px]">
			<GlowingText text="About System" />
		</div>

		<!-- Next Button -->
		<button
			class="cursor-pointer grid place-items-center py-[9px] px-[30px] text-xl bg-[#101010] rounded-sm hover:shadow-[0_0px_5px_1px_#00B85E] transition-all duration-300 border-[0.3px] border-[#3C6350]"
			disabled={currentStep === totalSteps}
			onclick={() => (currentStep = Math.min(totalSteps, currentStep + 1))}
			aria-label="Go to next step"
		>
			<GlowingText text="Next" />
		</button>
	</div>
	<!-- Step Indicator -->

	<div class="h-1 w-full bg-[#234132]">
		<div style={`width: ${percentage}%`} class={`h-full  bg-[#26A768] border-r-2 border-white`}>
			.
		</div>
	</div>
</div>
