<script>
	import { goto } from '$app/navigation';
	import Button from './ui/Button.svelte';
	import GlowingText from './ui/GlowingText.svelte';
	import Link from './ui/Link.svelte';

	export let totalSteps = 7;
	export let currentStep = 1;
	export let currentTitle = 'Installation';
	export let prevPath = '';
	export let nextPath = '';
	export let nextAction = null;
	export let prevAction = null;

	let isLoadingNext = false;
	let isLoadingBack = false;

	function handleNext() {
		isLoadingNext = true;

		if (nextAction) {
			Promise.resolve(nextAction()).then(() => {
				isLoadingNext = false;

				if (nextPath) {
					goto(nextPath);
				}
			});
		} else {
			goto(nextPath);
		}
	}

	function handlePrev() {
		isLoadingBack = true;

		if (prevAction) {
			Promise.resolve(prevAction()).then(() => {
				isLoadingBack = false;

				if (prevPath) {
					goto(prevPath);
				}
			});
		} else {
			goto(prevPath);
		}
	}
</script>

<div class="rounded-sm w-[1050px] flex flex-col h-[72px] mx-auto">
	<div class="flex items-center justify-between w-full bg-black/30 px-4 p-1">
		<!-- Tombol Kembali -->
		<div class="flex items-center gap-6">
			<!-- <Button
				isDisabled={isLoadingNext || isLoadingBack || currentStep === 1 || !prevPath}
				onclick={handlePrev}
				btnText={isLoadingBack ? '....' : 'Back'}
			/> -->
			<Link
				isDisabled={isLoadingNext ||
					isLoadingBack ||
					currentStep === totalSteps ||
					(!prevPath && !prevAction)}
				btnText={isLoadingNext ? '....' : 'Back'}
				href={prevPath || '#'}
			/>
			<div class="flex items-center gap-1">
				{#each Array(totalSteps).fill() as _, index}
					<div
						class="rounded-[8px] transition-all ease-in-out duration-300"
						style={currentStep === index + 1
							? 'background: #26A768; width: 25px; height: 4px;'
							: 'background: #D9D9D9; width: 15px; height: 4px;'}
					></div>
				{/each}
			</div>
		</div>

		<!-- Judul halaman dinamis -->
		<div class="flex flex-col items-center justify-between space-y-2 my-[10px]">
			<GlowingText text={currentTitle} />
		</div>

		<!-- Tombol Selanjutnya -->
		<!-- <Button
			isDisabled={isLoadingNext || isLoadingBack || currentStep === totalSteps || (!nextPath && !nextAction)}
			onclick={handleNext}
			btnText={isLoadingNext ? "...." : "Next"}
		/> -->
		<Link
			isDisabled={isLoadingNext ||
				isLoadingBack ||
				currentStep === totalSteps ||
				(!nextPath && !nextAction)}
			btnText={isLoadingNext ? '....' : 'Next'}
			href={nextPath || '#'}
		/>
	</div>
</div>
