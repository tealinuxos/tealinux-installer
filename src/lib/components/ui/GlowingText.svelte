<script lang="ts">
	interface Props {
		text: string;
		size?: string;
	}

	let { text, size = 'xl' }: Props = $props();
</script>

<div class="relative grid place-items-center w-fit">
	<span
		class={`text-${size} font-normal bg-linear-to-r from-[#4CDA95] via-[#26A768] to-[#26A768] bg-clip-text text-transparent blur-[1px]`}
	>
		{text}
	</span>
	<span
		class={`absolute text-${size} font-normal bg-linear-to-r from-[#4CDA95] via-[#26A768] to-[#26A768] bg-clip-text text-transparent`}
	>
		{text}
	</span>
</div>
