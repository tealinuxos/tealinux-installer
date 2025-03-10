<script>
	let { text } = $props();
</script>

<div class="relative grid place-items-center">
	<span
		class="text-xl font-[400] bg-gradient-to-r from-[#4CDA95] via-[#26A768] to-[#26A768] bg-clip-text text-transparent blur-[1px]"
	>
		{text}
	</span>
	<span
		class="absolute text-xl font-[400] bg-gradient-to-r from-[#4CDA95] via-[#26A768] to-[#26A768] bg-clip-text text-transparent"
	>
		{text}
	</span>
</div>
