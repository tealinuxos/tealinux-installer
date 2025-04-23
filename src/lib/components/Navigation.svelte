<script>
	import { goto } from '$app/navigation';
	import Button from './ui/Button.svelte';
	import GlowingText from './ui/GlowingText.svelte';

	// State untuk step yang sedang aktif
	let currentStep = 1;
	const totalSteps = 4;

	// Map setiap langkah ke judul halaman
	const stepTitles = {
		1: 'Installation',
		2: 'Keyboard Setup',
		3: 'Account Creation',
		4: 'Summary'
	};

	// Navigasi ke langkah tertentu
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
		<!-- Tombol Kembali -->
		 <div class="flex items-center gap-6"> 
			<Button
			isDisabled={currentStep === 1}
			onclick={() => navigateToStep(Math.max(1, currentStep - 1))}
			btnText="Back"
			/>
			<div class="flex items-center gap-1 ">
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


		<!-- Judul halaman dinamis -->
		<div class="flex flex-col items-center justify-between space-y-2 my-[10px]">
			<GlowingText text={stepTitles[currentStep]} />
		</div>

		<!-- Tombol Selanjutnya -->
		<Button
			isDisabled={currentStep === totalSteps}
			onclick={() => navigateToStep(Math.min(totalSteps, currentStep + 1))}
			btnText="Next"
		/>
	</div>

	<!-- Step Indicator -->

</div>
