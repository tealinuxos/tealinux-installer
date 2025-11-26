<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import type { RouteId } from '$app/types';
	import GlowingText from './ui/GlowingText.svelte';
	import Link from './ui/Link.svelte';

	interface Props {
		currentStep: number;
		currentTitle: string;
		prevPath?: RouteId;
		nextPath: RouteId;
		prevAction?: null | (() => Promise<void>);
		nextAction?: null | (() => Promise<void>);
		disableNext?: boolean;
	}

	interface Loading {
		prev: boolean;
		next: boolean;
	}

	const totalSteps: number = 7;

	let {
		currentStep = 1,
		currentTitle = 'Installation',
		prevPath = '/',
		nextPath = '/',
		disableNext = false,
		nextAction = null,
		prevAction = null
	}: Props = $props();

	let isLoading = $state<Loading>({
		next: false,
		prev: false
	});

	async function handleNext() {
		isLoading.next = true;

		if (nextAction) {
			await nextAction();
			isLoading.next = false;

			if (nextPath) {
				goto(resolve(nextPath));
			}
		} else {
			goto(resolve(nextPath));
		}
	}

	async function handlePrev() {
		isLoading.prev = true;

		if (prevAction) {
			await prevAction();
			isLoading.prev = false;

			if (nextPath) {
				goto(resolve(prevPath));
			}
		} else {
			goto(resolve(prevPath));
		}
	}
</script>

<div class="rounded-sm w-[1050px] flex flex-col h-[72px] mx-auto">
	<div class="flex items-center justify-between w-full bg-black/30 px-4 p-1">
		<!-- Tombol Kembali -->
		<div class="flex items-center gap-6">
			<Link
				isDisabled={isLoading.next || isLoading.prev || currentStep === 1 || !prevPath}
				btnText={isLoading.prev ? '....' : 'Previous'}
				onclick={handlePrev}
				aria-label="Go to previous step"
				href={prevPath || '#'}
			/>
			<div class="flex items-center gap-1">
				{#each Array(totalSteps), index}
					<div
						class={`${currentStep === index + 1 ? 'bg-[#26A768] w-[25px]' : 'bg-[#D9D9D9] w-[15px]'} h-1 rounded-lg transition-all ease-in-out duration-300`}
					></div>
				{/each}
			</div>
		</div>

		<!-- Judul halaman dinamis -->
		<div class="flex flex-col items-center justify-between space-y-2 my-2.5">
			<GlowingText text={currentTitle} />
		</div>

		<!-- Tombol Selanjutnya -->
		<Link
			isDisabled={disableNext ||
				isLoading.next ||
				isLoading.prev ||
				currentStep === totalSteps ||
				(!nextPath && !nextAction)}
			btnText={isLoading.prev ? '....' : 'Next'}
			aria-label="Go to next step"
			onclick={handleNext}
			href={nextPath || '#'}
		/>
	</div>
</div>
