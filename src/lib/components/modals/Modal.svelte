<script lang="ts">
    export let title: string;
    export let items: Array<{ name: string; description?: string; [key: string]: any }> = [];
    export let showModal: boolean = false;
    export let placeholder: string = "Search items";
    export let noResultsText: string = "No items found";
    export let closeButtonText: string = "Close";
  
    let searchTerm: string = '';
  
    import { createEventDispatcher } from 'svelte';
    const dispatch = createEventDispatcher();
  
    const selectItem = (item: any) => {
      dispatch('select', item);
      closeModal();
    };
  
    const closeModal = () => {
      searchTerm = '';
      dispatch('close');
    };
  
    $: filteredItems = items?.filter(item => 
        item?.name?.toLowerCase().includes(searchTerm.toLowerCase()) ||
        (item?.description?.toLowerCase().includes(searchTerm.toLowerCase()))
    ) || [];


  </script>
  
  {#if showModal}
  <div class="fixed inset-0 flex items-center justify-center backdrop-blur-sm z-80">
    <div class="absolute inset-0 bg-black/50" on:click={closeModal}></div>
    <div class="flex flex-col min-w-[434px] max-h-full justify-center items-center p-4 bg-black rounded-lg border border-[#3C6350] shadow-[0_0_10px_rgba(38,167,104,0.25)] overflow-auto z-90">
      <div class="w-full p-6 z-10">
        <h2 class="text-xl font-bold mb-4 text-white">{title}</h2>
        <input
          type="text"
          bind:value={searchTerm}
          placeholder={placeholder}
          class="w-full p-2 border rounded-lg mb-4 bg-[#1c1c1c] text-white"
        />
        
        <div class="max-h-60 overflow-auto space-y-2">
          {#if filteredItems.length > 0}
            {#each filteredItems as item}
              <div
                class="flex items-center justify-between p-2 bg-[#303030] text-white rounded-md cursor-pointer hover:bg-gray-700"
                style="height: 28px; padding: 3px 16px;"
                on:click={() => selectItem(item)}
              >
                <span>{item.name}</span>
                {#if item.description}
                  <span class="text-sm text-gray-400">{item.description}</span>
                {/if}
              </div>
            {/each}
          {:else}
            <div class="text-white">{noResultsText}</div>
          {/if}
        </div>
        
        <button
          class="mt-4 w-full p-2 bg-red-500 text-white rounded-lg hover:bg-red-600"
          on:click={closeModal}
        >
          {closeButtonText}
        </button>
      </div>
    </div>
  </div>
  {/if}