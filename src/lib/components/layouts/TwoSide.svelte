<script>
	let { gap, left, right } = $props(); // Optional: Add a gap between the sides
	gap = gap || 'gap-4'; // Default gap if not provided
</script>

<div class="flex items-center justify-between p-[15px] py-0 text-white">
	<div class="flex flex-1 items-center justify-center">
		<!-- <slot name="left"/> -->
		{@render left()}
	</div>
	<div class="w-[640px] h-full p-5 font-jakarta">
		<!-- <slot name="right"/> -->
		{@render right()}
	</div>
</div>
