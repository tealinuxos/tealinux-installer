<script>
    import { fade } from 'svelte/transition';
  
    // Props
    export let show = false;
    
    // Actions
    function handleCancel() {
      // Handle cancel logic
      show = false;
    }
    
    function handleConfirm() {
      // Handle confirm logic
      show = false;
    }
  </script>
  
  {#if show}
  <div transition:fade
       class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
    <div class="inline-flex p-[27px_35px] flex-col justify-center items-center gap-[10px] rounded-[20px] border-2 border-[#F00] bg-[#101010]">
      <!-- Main Content -->
      <div class="flex flex-col items-center gap-[25px]">
        <!-- Warning Icon -->
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M12 8V12M12 16H12.01M22 12C22 17.5228 17.5228 22 12 22C6.47715 22 2 17.5228 2 12C2 6.47715 6.47715 2 12 2C17.5228 2 22 6.47715 22 12Z" stroke="#F00" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
        
        <!-- Warning Text -->
        <div class="text-white text-center font-['Plus_Jakarta_Sans'] text-[24px] font-extrabold leading-none">
          WARNING
        </div>
        
        <!-- Description -->
        <div class="text-white text-center font-['Plus_Jakarta_Sans'] text-[16px] font-normal max-w-[300px]">
          This configure will reset all your data. Please backup your data before run.
        </div>
        
        <!-- Buttons -->
        <div class="flex gap-[18px]">
          <button 
            on:click={handleCancel}
            class="px-[15px] py-[8px] rounded-[23px] border-[1.3px] border-[#F00] bg-[#101010] text-white font-['Plus_Jakarta_Sans'] text-[14px] hover:bg-[#1a1a1a] transition-colors"
          >
            cancel
          </button>
          <button 
            on:click={handleConfirm}
            class="px-[15px] py-[8px] rounded-[23px] border-[1.3px] border-[#F00] bg-[#F00] text-white font-['Plus_Jakarta_Sans'] text-[14px] hover:bg-[#ff3333] transition-colors"
          >
            confirm
          </button>
        </div>
      </div>
    </div>
  </div>
  {/if}
  
