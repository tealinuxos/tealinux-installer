<script>

  let {
    options = [],
    value = $bindable(),
    displayField,
    onchange = () => {},
    width = '100%',
    height = '40',
    loadingText = "Loading...",
    defaultText = "Select an option",
    isLoading = false,
    class: customClass = "", 
    error = null,
      noBorder = false  // Prop baru untuk mengontrol transparansi
  } = $props();

  let isOpen = $state(false);
  let selectElement = $state(null);
  



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
    value = option.value;
    isOpen = false;
    onchange();
  }

  function getDisplayText(option) {
    let nullValue = options.find(opt => typeof opt === 'object' ? opt.value === null : opt === null);
    if (isLoading) return loadingText;
    if (error) return error;
    if (!option) return displayField ? nullValue ? nullValue[displayField] : defaultText : nullValue || defaultText;
    
    if (typeof option === 'object') {
      return displayField ? option[displayField] ? option[displayField] : null : option;
    }
    return option;
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
    style:border={noBorder ? 'none' : isOpen ? '1.3px solid #26A768' : '1.3px solid #3C6350'}
    class:disabled={isLoading || error}
  >
    <div class="selected-text">
      {getDisplayText(value)}
    </div>
    
    {#if !isLoading && !error}
      <div class="icon ml-2" class:rotate={isOpen}>
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
      {#each options as option (option.value || option)}
        <div 
          class="option {value && (typeof value === 'object' ? value.value : value) === (typeof option === 'object' ? option.value : option) ? 'selected' : ''}"
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
    font-family: 'Plus Jakarta Sans', sans-serif;
  }
  
  .selected-value {
    display: flex;
    padding: 9px 15px;
    justify-content: space-between;
    align-items: center;
    border-radius: 14px;
    background: #101010; /* Hapus properti border dari sini */
    cursor: pointer;
    transition: border-color 0.2s ease;
    color: #FFFEFB;
    font-size: 13px;
    font-weight: 500;
    height: 100%;
    box-sizing: border-box;
  }
  
  .selected-value.disabled {
    cursor: not-allowed;
    opacity: 0.7;
  }
  
 /* Border untuk dropdown-options selalu ada */
  .dropdown-options {
    position: absolute;
    top: calc(100% + 5px);
    left: 0;
    right: 0;
    max-height: 200px;
    overflow-y: auto;
    background: #101010;
    border: 1.3px solid #3C6350;
    border-radius: 14px;
    z-index: 1000;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  }
  
  
  .option {
    padding: 10px 15px;
    cursor: pointer;
    transition: all 0.2s;
    color: #FFFEFB;
    font-size: 13px;
  }
  
  .option:hover {
    background-color: rgba(60, 99, 80, 0.2);
  }
  
  .option.selected {
    background-color: rgba(38, 167, 104, 0.1);
    color: #26A768;
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
