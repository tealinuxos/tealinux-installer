<script>
	import Modal from './Modal.svelte';

	let {
		show = $bindable(),
		data = $bindable(),
		field = null,
		title = 'This is a title',
		notFoundMessage = 'Message not found',
		onclick = () => console.log('clicked'),
		selected = $bindable(),
		nullValue
	} = $props();
</script>

<div
	class="flex p-[10px] border border-border bg-[#101010] rounded-[14px] items-center text-[15px] justify-between h-fit w-full cursor-pointer"
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
	<div>
		<span>{selected ? selected : nullValue}</span>
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
	<Modal bind:show {data} {onclick} {field} {title} {notFoundMessage} {selected} {nullValue} />
{/if}
