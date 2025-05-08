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
				<h2 class="text-xl font-bold mb-4 text-white">{title}</h2>
				<input
					type="text"
					bind:value={keyword}
					placeholder={title}
					class="w-full p-2 border rounded-lg mb-4 bg-[#1c1c1c] text-white"
				/>
				<!-- daftar keyboard -->
				<div class="max-h-60 overflow-auto space-y-2">
					{#if data.length > 0}
						{#each filteredData as item}
							<div
								class="flex items-center justify-between p-2 bg-[#303030] text-white rounded-md cursor-pointer hover:bg-gray-700"
								style="height: 28px; padding: 3px 16px;"
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
