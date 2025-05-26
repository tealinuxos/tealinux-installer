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
