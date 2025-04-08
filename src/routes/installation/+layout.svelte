<script>
	import Navigation from '$lib/components/Navigation.svelte';
</script>

<main
	class="max-h-[720px] max-w-[1080px] h-[720px] w-[1080px] grid place-items-center bg-tealinux font-jakarta"
>
	<div class="flex flex-col justify-between max-h-[720px] min-w-full min-h-full max-w-[1080px]">
		<slot />
		<Navigation />
	</div>
</main>
