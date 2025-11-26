<script lang="ts">
	import { cn } from '$lib/utils/cn';
	import type { Snippet } from 'svelte';

	interface Props {
		class?: string;
		left: Snippet;
		right: Snippet;
	}

	let { left, right, class: cls }: Props = $props();
</script>

<div
	class={cn(
		'flex h-full max-h-[630px] flex-1 items-center justify-between p-[15px] py-0 text-white',
		cls
	)}
>
	<div class="flex flex-1 items-center justify-center">
		<!-- <slot name="left"/> -->
		{@render left()}
	</div>
	<div class="w-[640px] h-full py-5 font-jakarta">
		<!-- <slot name="right"/> -->
		{@render right()}
	</div>
</div>
