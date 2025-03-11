<script>
	export let gap = '20px'; // Optional: Add a gap between the sides
</script>

<div class="flex items-center justify-between p-[15px] py-0 text-white">
	<div class="flex flex-1 items-center justify-center">
		<slot name="left"></slot>
	</div>
	<div class="w-[640px] h-full p-5 font-jakarta">
		<slot name="right"></slot>
	</div>
</div>
