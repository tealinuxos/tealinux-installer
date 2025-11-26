<script lang="ts" generics="T">
	import Modal from './Modal.svelte';

	interface Props {
		show: boolean;
		data: T[] | null;
		field?: keyof T | null;
		title?: string;
		notFoundMessage?: string;
		onclick: (item: T) => void;
		selected: string | null;
		nullValue?: string | null;
		keyword?: string;
	}

	let {
		show = $bindable(),
		data = $bindable(),
		field = null,
		title = 'Select Item',
		notFoundMessage = 'Not found',
		onclick,
		selected = $bindable(),
		nullValue,
		keyword = $bindable('')
	}: Props = $props();
</script>

<div
	class="flex p-2.5 border border-border bg-[#101010] rounded-[14px] items-center text-[15px] justify-between h-fit w-full cursor-pointer hover:border-[#26A768] transition-colors"
	onclick={() => (show = true)}
	onkeydown={(e) => {
		if (e.key === 'Enter') {
			show = true;
			e.preventDefault();
		}
		if (e.key === 'Escape') {
			show = false;
			e.preventDefault();
		}
	}}
	role="button"
	tabindex="0"
>
	<div class="truncate pr-2">
		<span class={!selected ? 'text-gray-500' : 'text-white'}>
			{selected ? selected : (nullValue ?? title)}
		</span>
	</div>

	<div>
		<span>
			<svg width="14" height="9" viewBox="0 0 14 9" fill="none" xmlns="http://www.w3.org/2000/svg">
				<path
					d="M1 1.5L7 7.5L13 1.5"
					stroke="#26A768"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				/>
			</svg>
		</span>
	</div>
</div>

{#if show}
	<Modal
		bind:show
		bind:keyword
		{data}
		{onclick}
		{field}
		{notFoundMessage}
		selectedLabel={selected}
	/>
{/if}
