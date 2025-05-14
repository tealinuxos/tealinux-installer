<script>
	let {
		show = $bindable(),
		keyword = $bindable(),
		onclick = () => console.log('clicked'),
		data,
		title = 'This is a title',
		notFoundMessage = 'Message not found',
		field = null
	} = $props();

	let filteredData = $state(data);

	function filter(term) {
		term = term.toLowerCase();

		if (field) {
			filteredData = data.filter((e) => e[field].toLowerCase().includes(term));
		} else {
			filteredData = data.filter((e) => e.toLowerCase().includes(term));
		}
	}

	$effect(() => {
		filter(keyword);
	});
</script>

{#if show}
	<div class="fixed inset-0 flex items-center justify-center backdrop-blur-sm z-80">
		<div
			style="-webkit-backdrop-filter: blur(10px); backdrop-filter: blur(10px)"
			class="absolute inset-0 bg-black/50"
			onclick={() => (show = false)}
		></div>
		<div
			class="flex flex-col min-w-[434px] max-h-full justify-center items-center p-4 bg-black rounded-lg border border-[#3C6350] shadow-[0_0_10px_rgba(38,167,104,0.25)] overflow-auto z-90"
		>
			<div class="w-full p-6 z-10">
				<!-- <h2 class="text-xl font-bold mb-4 text-white">{title}</h2> -->
				<div class="relative flex items-center w-full">
				<!-- Search icon (left) -->
				<div class="absolute left-3 text-[#26A768]">
					<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<circle cx="11" cy="11" r="8"></circle>
					<line x1="21" y1="21" x2="16.65" y2="16.65"></line>
					</svg>
				</div>

				<!-- Input field -->
				<input
					type="text"
					class="flex items-center justify-between w-full h-[35px] pl-9 pr-8 bg-[#122C1F] text-white rounded-[4px] border border-[#26A768] border-opacity-100 focus:outline-none focus:ring-1 focus:ring-[#26A768]"
					style="border-width: 1.3px"
				/>

				<!-- ESC icon (right) -->
				<div class="absolute right-3 text-[#26A768] cursor-pointer">
					<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<line x1="18" y1="6" x2="6" y2="18"></line>
					<line x1="6" y1="6" x2="18" y2="18"></line>
					</svg>
				</div>
				</div>
				<br>
				<!-- daftar keyboard -->
				<div class="max-h-60 overflow-auto space-y-2">
					{#if data.length > 0}
						{#each filteredData as item}
							<div 
								class="flex items-center justify-between p-[3px_16px] h-[40px] bg-[rgba(29,33,31,0.7)] text-white rounded-[8px] cursor-pointer hover:bg-[#122C1F]"
								onclick={() => onclick(item)}
							>
								{#if field}
									<span>{item[field]}</span>
								{:else}
									<span>{item}</span>
								{/if}
								<span class="text-sm text-gray-400">{item.description || ''}</span>
							</div>
						{/each}
					{:else}
						<div class="text-white">{notFoundMessage}</div>
					{/if}
				</div>
				<button
					class="mt-4 w-full p-2 bg-red-500 text-white rounded-lg hover:bg-red-600"
					onclick={() => (show = false)}
				>
					Close
				</button>
			</div>
		</div>
	</div>
{/if}
