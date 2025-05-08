<script>
  import { getRead, getBlueprint } from '/src/routes/installation/global.js';
  
  let { 
    options = [], 
    value = null, 
    displayField = '', 
    sizeField = '', 
    formatter = null,
    width = '100%',
    height = '46.656px'
  } = $props();

  let selectedDisk = $state(null);
  let diskOptions = $state([]); 
  let isLoading = $state(false);
  let error = $state(null);
  
  
  const fetchDisks = async () => {
    try {
      isLoading = true;
      error = null;
      
      
      const response = await getRead();
      
      if (response && response.disk) {
        
        diskOptions = response.disk.map(disk => ({
          ...disk,
          value: disk.diskPath,
          name: disk.diskName || disk.diskPath, 
          size: disk.size
        }));

        if (options.length === 0) {
          options = diskOptions;
        }
      }
    } catch (err) {
      error = err.message || 'Failed to fetch disks';
      console.error('Error fetching disks:', err);
    } finally {
      isLoading = false;
    }
  };

  
  $effect(() => {
    fetchDisks();
  });

  const selectDisk = (disk) => {
    console.log('Selected Disk:', disk);
    selectedDisk = disk;
    value = disk; // Update nilai yang dipilih
  };

  let isOpen = $state(false);
  let selectElement = $state(null);
  
  let selectedOption = $derived(
    options.find(opt => {
      return typeof opt === 'object' ? opt === value : opt === value;
    }) || diskOptions.find(opt => opt === value)
  );

  function handleClickOutside(event) {
    if (selectElement && !selectElement.contains(event.target)) {
      isOpen = false;
    }
  }
  
  function toggleDropdown() {
    if (!isLoading && !error) {
      isOpen = !isOpen;
    }
  }
  
  function selectOption(option) {
    value = option;
    isOpen = false;
    
    
    if (option && (option.diskPath || option.value)) {
      selectDisk(option);
    }
  }
  
  function getDisplayText(option) {
    if (isLoading) return "Loading disks...";
    if (error) return "Error loading disks";
    if (!option) return "Select a disk";
    
    if (typeof option === 'object') {
      const display = displayField ? option[displayField] : option.name || option.value;
      const size = sizeField && option[sizeField] ? ` (${formatSize(option[sizeField])})` : '';
      return `${display}${size}`;
    }
    return option;
  }
  
  function formatSize(size) {
    if (!size) {
      return ''
    } else {
      let sizeInBytes = Number(size.slice(0, -1)) * 512;
      return formatter ? formatter(sizeInBytes) : `${(size / (1024 * 1024 * 1024)).toFixed(2)} GB`;
    }
  }
  
  $effect(() => {
    document.addEventListener('click', handleClickOutside);
    return () => document.removeEventListener('click', handleClickOutside);
  });
</script>

<div class="custom-select" bind:this={selectElement} style="width: {width}; height: {height}">
  <div 
    class="selected-value" 
    on:click={toggleDropdown}
    style:border-color={isOpen ? '#26A768' : '#3C6350'}
    class:disabled={isLoading || error}
  >
    <div class="selected-text">
      {getDisplayText(selectedOption || value)}
    </div>
    
    {#if !isLoading && !error}
      <div class="icon" class:rotate={isOpen}>
        <svg width="14" height="9" viewBox="0 0 14 9" fill="none">
          <path d="M1 1.33325L7 7.33325L13 1.33325" stroke="#26A768" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </div>
    {:else if isLoading}
      <div class="spinner"></div>
    {/if}
  </div>
  
  {#if isOpen && !isLoading && !error}
    <div class="dropdown-options">
      {#each diskOptions.length ? diskOptions : options as option (option.value || option)}
        <div 
          class="option {value === option ? 'selected' : ''}"
          on:click={() => selectOption(option)}
        >
          {getDisplayText(option)}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .custom-select {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 10px;
    align-self: stretch;
  }
  
  .selected-value {
    display: flex;
    padding: 9px 15px;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    align-self: stretch;
    border-radius: 14px;
    border: 1.3px solid #3C6350;
    background: #101010;
    cursor: pointer;
    transition: border-color 0.2s ease;
    color: #26A768;
    font-family: 'Jakarta', sans-serif;
    font-size: 13px;
    font-weight: 600;
  }
  
  .selected-value.disabled {
    cursor: not-allowed;
    opacity: 0.7;
  }
  
  .dropdown-options {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    max-height: 200px;
    overflow-y: auto;
    background: #101010;
    border: 1px solid #3C6350;
    border-radius: 14px;
    z-index: 1000;
    margin-top: 5px;
  }
  
  .option {
    padding: 10px 15px;
    cursor: pointer;
    transition: background-color 0.2s;
    color: white;
    font-family: 'Jakarta', sans-serif;
    font-size: 13px;
  }
  
  .option:hover {
    background-color: #032B17;
  }
  
  .option.selected {
    background-color: #032B17;
    color: #4CDA95;
  }
  
  .icon {
    transition: transform 0.2s ease;
  }
  
  .icon.rotate {
    transform: rotate(180deg);
  }
  
  .selected-text {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  
  .spinner {
    width: 14px;
    height: 14px;
    border: 2px solid rgba(38, 167, 104, 0.3);
    border-radius: 50%;
    border-top-color: #26A768;
    animation: spin 1s ease-in-out infinite;
  }
  
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
