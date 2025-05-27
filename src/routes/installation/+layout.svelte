<script>
	import { onNavigate } from '$app/navigation';
	import Modal from '$lib/components/modals/Modal.svelte';

	onNavigate((navigation) => {
		if (!document.startViewTransition) return;

		return new Promise((resolve) => {
			document.startViewTransition(async () => {
				resolve();
				await navigation.complete;
			});
		});
	});
</script>

<main
	class="max-h-[720px] max-w-[1080px] min-h-dvh w-[1080px] grid place-items-center bg-tealinux font-jakarta"
>
	<div class="flex flex-col justify-between max-h-[720px] min-h-[720px] min-w-full max-w-[1080px]">
		<slot />
		<Modal />
	</div>
</main>

<style>
	@keyframes fade-in {
		from {
			opacity: 0;
		}
	}

	@keyframes fade-out {
		to {
			opacity: 0;
		}
	}

	@keyframes slide-from-right {
		from {
			transform: translatex(30px);
		}
	}

	@keyframes slide-to-left {
		to {
			transform: translateX(-30px);
		}
	}

	:root::view-transition-old(root) {
		animation:
			90ms linear both fade-out,
			200ms linear both slide-to-left;
	}

	:root::view-transition-new(root) {
		animation:
			210ms linear both fade-in,
			200ms linear both slide-from-right;
	}
</style>
