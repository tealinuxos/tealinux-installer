<script>
	let { text, size } = $props();

	// Default values
	size = size || 'xl'; // Default size is 'xl'
</script>

<div class="relative grid place-items-center w-fit">
	<span
		class={`text-${size} font-[400] bg-gradient-to-r from-[#4CDA95] via-[#26A768] to-[#26A768] bg-clip-text text-transparent blur-[1px]`}
	>
		{text}
	</span>
	<span
		class={`absolute text-${size} font-[400] bg-gradient-to-r from-[#4CDA95] via-[#26A768] to-[#26A768] bg-clip-text text-transparent`}
	>
		{text}
	</span>
</div>
