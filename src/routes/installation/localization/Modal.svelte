<script>
	import { onMount } from "svelte";

	let {
		show = $bindable(),
		onclick = () => console.log('clicked'),
		data,
		title = 'This is a title',
		notFoundMessage = 'Message not found',
		field = null,
		selected
	} = $props();

	let filteredData = $state([]); // Inisialisasi sebagai array kosong
	let tempSelected = $state(null);
	let keyword = $state('');
	let selectedIndex = $state(0);

	function filter(term) {
		if (!data) return;
		
		term = term.toLowerCase();
		if (field) {
			filteredData = data.filter((e) => 
				e[field] && e[field].toString().toLowerCase().includes(term)
			);
		} else {
			filteredData = data.filter((e) => 
				e && e.toString().toLowerCase().includes(term)
			);
		}
	}

	function handleSelect(item, index) {
		tempSelected = item;
		selectedIndex = index;
	}

	function confirmSelection() {
		let value = tempSelected ? tempSelected : selected;
		onclick(value);
		show = false;
	}

	function cancelSelection() {
		show = false;
	}

	const scrollToSelected = (selected) => {
		let el = document.getElementById(selected);
		if (el) {
			el.scrollIntoView({
				behavior: "smooth",
				block: "nearest"
			});
		}
	}

	const onKeyDown = (event) => {
		// Hanya handle tombol panah
		if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
			event.preventDefault();
			
			if (!tempSelected && filteredData.length > 0) {
				tempSelected = filteredData[0];
				selectedIndex = 0;
			}

			if (filteredData.length > 0) {
				switch(event.key) {
					case 'ArrowDown':
						if (selectedIndex < filteredData.length - 1) {
							selectedIndex += 1;
							tempSelected = filteredData[selectedIndex];
							scrollToSelected(field ? tempSelected[field] : tempSelected);
						}
						break;
					case 'ArrowUp':
						if (selectedIndex > 0) {
							selectedIndex -= 1;
							tempSelected = filteredData[selectedIndex];
							scrollToSelected(field ? tempSelected[field] : tempSelected);
						}
						break;
				}
			}
		}
	}

	$effect(() => {
		filter(keyword);
		// Reset selection saat pencarian berubah
		if (keyword.length > 0) {
			tempSelected = null;
			selectedIndex = 0;
		}
	});

	onMount(() => {
		// Inisialisasi filteredData dengan data awal
		filteredData = [...data];
		if (selected) {
			scrollToSelected(field ? selected[field] : selected);
		}
	});
</script>

<div class="fixed inset-0 flex items-center justify-center backdrop-blur-sm z-80">
    <!-- Backdrop -->
    <div
        style="-webkit-backdrop-filter: blur(10px); backdrop-filter: blur(10px)"
        class="absolute inset-0 bg-black/50"
        on:click={() => (show = false)}
    ></div>
    
    <!-- Modal Content -->
    <div
        class="flex flex-col min-w-[434px] max-h-full justify-center items-center p-4 bg-black rounded-[4px] border border-[#3C6350] shadow-[0_0_10px_rgba(38,167,104,0.25)] overflow-auto z-90"
    >
        <div class="w-full p-6 z-10">
            <!-- Search Input -->
            <div class="relative flex items-center w-full">
                <div class="absolute left-3 text-[#26A768]">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <circle cx="11" cy="11" r="8"></circle>
                        <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
                    </svg>
                </div>

                <input
                    type="text"
                    bind:value={keyword} 
                    class="flex items-center justify-between w-full h-[35px] pl-9 pr-8 bg-[#122C1F] text-white rounded-[4px] border border-[#26A768] border-opacity-100 focus:outline-none focus:ring-1 focus:ring-[#26A768]"
                    style="border-width: 1.3px"
                    autofocus
                    on:keydown={(e) => e.key === 'Escape' && (show = false)}
                />

                <div class="absolute right-3 text-[#26A768] cursor-pointer" on:click={() => (show = false)}>
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <line x1="18" y1="6" x2="6" y2="18"></line>
                        <line x1="6" y1="6" x2="18" y2="18"></line>
                    </svg>
                </div>
            </div>
            
            <br>
            
            <!-- Results List -->
            <div class="max-h-60 overflow-auto space-y-2">
                {#if filteredData.length > 0}
                    {#each filteredData as itemData, index (itemData)}
                        {@const itemValue = field ? itemData[field] : itemData}
                        <div 
                            class={`flex items-center justify-between p-[3px_16px] h-[40px] rounded-[8px] cursor-pointer 
                                ${tempSelected === itemData ? 
                                    'bg-[#122C1F] text-white' : 
                                    'bg-[rgba(29,33,31,0.7)] text-white hover:bg-[#122C1F]'}`}
                            on:click={() => handleSelect(itemData, index)}
                            id={itemValue}
                        >
                            <span>{itemValue}</span>
                            <span class="text-sm text-gray-400">{itemData.description || ''}</span>
                        </div>
                    {/each}
                {:else}
                    <div class="text-white">{notFoundMessage}</div>
                {/if}
            </div>
            
            <!-- Buttons -->
            <div class="flex gap-2 mt-4">
                <button
                    class="w-full px-4 py-2 rounded text-white border border-[#3C6350] hover:bg-[#1a1a1a] active:shadow-[0_0_7.167px_rgba(38,167,104,0.8)] disabled:opacity-50"
                    on:click={cancelSelection}
                >
                    Cancel
                </button>
                <button
                    class="w-full px-4 py-2 rounded text-white bg-[#26A768] border border-[#3C6350] hover:bg-[#1E8A56] active:shadow-[0_0_7.167px_rgba(38,167,104,0.8)] disabled:opacity-50"
                    on:click={confirmSelection}
                    disabled={!tempSelected}
                >
                    Confirm
                </button>
            </div>
        </div>
    </div>
</div>

<svelte:window on:keydown={onKeyDown} />