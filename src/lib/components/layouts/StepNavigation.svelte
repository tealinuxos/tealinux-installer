<script>
	export let currentStep;
	export let totalSteps;
	export let stepTitles;
	export let onNavigate;

	import Button from './ui/Button.svelte';
	import GlowingText from './ui/GlowingText.svelte';
</script>

<div class="rounded-sm w-[1050px] flex flex-col h-[72px] mx-auto">
	<div class="flex items-center justify-between w-full bg-black/30 px-4 p-1">
		<!-- Tombol Kembali -->
		<div class="flex items-center gap-6">
			<Button
				isDisabled={currentStep === 1}
				onclick={() => onNavigate(Math.max(1, currentStep - 1))}
				btnText="Back"
			/>
			<div class="flex items-center gap-1">
				{#each Array(totalSteps).fill() as _, index}
					<div
						class="rounded-[8px] transition-all ease-in-out duration-300"
						style={
							currentStep === index + 1
								? "background: #26A768; width: 25px; height: 4px;"
								: "background: #D9D9D9; width: 15px; height: 4px;"
						}
					></div>
				{/each}
			</div>
		</div>

		
		<div class="flex flex-col items-center justify-between space-y-2 my-[10px]">
			<GlowingText text={stepTitles[currentStep]} />
		</div>

		<!-- Tombol Selanjutnya -->
		<Button
			isDisabled={currentStep === totalSteps}
			onclick={() => onNavigate(Math.min(totalSteps, currentStep + 1))}
			btnText="Next"
		/>
	</div>
</div>
