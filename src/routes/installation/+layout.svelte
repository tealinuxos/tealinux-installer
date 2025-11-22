<script lang="ts">
	import type { Snippet } from 'svelte';
	import { fade } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { page } from '$app/stores';
	import Modal from '$lib/components/modals/Modal.svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<main
	class="max-h-[720px] max-w-[1080px] min-h-dvh w-[1080px] grid place-items-center bg-tealinux font-jakarta overflow-hidden"
>
	<div class="grid grid-cols-1 grid-rows-1 max-h-[720px] min-h-[720px] min-w-full max-w-[1080px]">
		{#key $page.url.pathname}
			<div
				in:fade={{ duration: 300, easing: cubicOut }}
				class="col-start-1 row-start-1 flex flex-col justify-between w-full h-full bg-tealinux"
			>
				<Modal />
				{@render children?.()}
			</div>
		{/key}
	</div>
</main>
