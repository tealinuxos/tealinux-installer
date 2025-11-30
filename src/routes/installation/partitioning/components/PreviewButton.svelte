<script lang="ts">
	import type { MouseEventHandler } from 'svelte/elements';
	import { cn } from '$lib/utils/cn';

	interface Props {
		title: string;
		selected: boolean;
		onclick: MouseEventHandler<HTMLButtonElement>;
	}

	let { title, selected = false, onclick }: Props = $props();
</script>

<button
	{onclick}
	class={cn(
		selected ? 'text-[#26A768]' : 'text-[#17613D]',
		'font-jakarta text-[14.667px] font-semibold leading-normal tracking-[-0.66px]'
	)}
>
	{title}
</button>
