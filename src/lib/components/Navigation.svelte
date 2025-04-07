<script>
	import { goto } from '$app/navigation';
	import Button from './ui/Button.svelte';
	import GlowingText from './ui/GlowingText.svelte';

	let currentStep = $state(1); // Use Svelte 5 reactive state
	const totalSteps = 4;

	let percentage = $derived((currentStep / totalSteps) * 100);

	// Function to handle navigation based on the current step
	function navigateToStep(step) {
		currentStep = step;
		switch (step) {
			case 1:
				goto('/installation');
				break;
			case 2:
				goto('/installation/keyboard');
				break;
			case 3:
				goto('/installation/account');
				break;
			case 4:
				goto('/installation/summary');
				break;
		}
	}
</script>

<div class="rounded-sm w-[1050px] flex flex-col h-[72px] mx-auto">
	<div class="flex items-center justify-between w-full bg-black/30 px-4 p-1">
		<!-- Back Button -->
		<Button
			isDisabled={currentStep === 1}
			onclick={() => navigateToStep(Math.max(1, currentStep - 1))}
			btnText="Back"
		/>

		<!-- pages -->
		<div class="flex flex-col items-center justify-between space-y-2 my-[10px]">
			<GlowingText text="About System" />
		</div>

		<!-- Next Button -->
		<Button
			isDisabled={currentStep === totalSteps}
			onclick={() => navigateToStep(Math.min(totalSteps, currentStep + 1))}
			btnText="Next"
		/>
	</div>
	<!-- Step Indicator -->

	<div class="h-1 w-full bg-[#234132]">
		<div
			style={`width: ${percentage}%`}
			class={`h-full  bg-[#26A768] border-r-2 border-white transition-all ease-in-out`}
		></div>
	</div>
</div>
