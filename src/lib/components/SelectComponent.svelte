<script>
    let { 
      options = [], 
      value = null, 
      displayField = '', 
      sizeField = '', 
      formatter = null,
      width = '100%',
      height = '46.656px'
    } = $props();
  
    let isOpen = $state(false);
    let selectElement = $state(null);
    
    let selectedOption = $derived(
      options.find(opt => {
        return typeof opt === 'object' ? opt === value : opt === value;
      })
    );
  
    function handleClickOutside(event) {
      if (selectElement && !selectElement.contains(event.target)) {
        isOpen = false;
      }
    }
    
    function toggleDropdown() {
      isOpen = !isOpen;
    }
    
    function selectOption(option) {
      value = option;
      isOpen = false;
    }
    
    function getDisplayText(option) {
      if (!option) return "Select an option";
      
      if (typeof option === 'object') {
        const display = displayField ? option[displayField] : option.value || option.name;
        const size = sizeField && option[sizeField] ? ` (${formatSize(option[sizeField])})` : '';
        return `${display}${size}`;
      }
      return option;
    }
    
    function formatSize(size) {
      return formatter ? formatter(size) : size;
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
    >
      <div class="selected-text">
        {getDisplayText(selectedOption || value)}
      </div>
      
      <div class="icon" class:rotate={isOpen}>
        <svg width="14" height="9" viewBox="0 0 14 9" fill="none">
          <path d="M1 1.33325L7 7.33325L13 1.33325" stroke="#26A768" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
      </div>
    </div>
    
    {#if isOpen}
      <div class="dropdown-options">
        {#each options as option (option.value || option)}
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
</style>