<script>
    // Props
    export let initialDevice = "/dev/sda5";
    export let initialDescription = "";
    export let caption = "";
    export let borderColor = "#4CDA95";
    export let backgroundColor = "#032B17";
    export let iconColor = "#4CDA95";
    export let showIcon = true;
    export let showCaption = false;
  
    // State
    let device = initialDevice;
    let description = initialDescription;
  </script>
  
  <!-- UI -->
  <div
    class="flex items-center justify-between h-[48px] px-3 py-3 rounded-lg w-full"
    style="border: 1.3px solid {borderColor}; background: {backgroundColor};"
  >
    <!-- Kiri -->
    <div class="flex flex-col justify-center">
      <div class="flex items-center gap-2">
        {#if showIcon}
          <svg width="17" height="21" viewBox="0 0 17 21"  fill="{iconColor}" xmlns="http://www.w3.org/2000/svg">
            <path d="M1.38889 0.233398H15.6111C15.8469 0.233398 16.073 0.329177 16.2397 0.499665C16.4064 0.670153 16.5 0.901383 16.5 1.14249V4.77885C16.5 5.01996 16.4064 5.25119 16.2397 5.42168C16.073 5.59216 15.8469 5.68794 15.6111 5.68794H1.38889C1.15314 5.68794 0.927048 5.59216 0.760349 5.42168C0.59365 5.25119 0.5 5.01996 0.5 4.77885V1.14249C0.5 0.901383 0.59365 0.670153 0.760349 0.499665C0.927048 0.329177 1.15314 0.233398 1.38889 0.233398ZM1.38889 7.50613H15.6111C15.8469 7.50613 16.073 7.6019 16.2397 7.77239C16.4064 7.94288 16.5 8.17411 16.5 8.41522V12.0516C16.5 12.2927 16.4064 12.5239 16.2397 12.6944C16.073 12.8649 15.8469 12.9607 15.6111 12.9607H1.38889C1.15314 12.9607 0.927048 12.8649 0.760349 12.6944C0.59365 12.5239 0.5 12.2927 0.5 12.0516V8.41522C0.5 8.17411 0.59365 7.94288 0.760349 7.77239C0.927048 7.6019 1.15314 7.50613 1.38889 7.50613ZM1.38889 14.7789H15.6111C15.8469 14.7789 16.073 14.8746 16.2397 15.0451C16.4064 15.2156 16.5 15.4468 16.5 15.6879V19.3243C16.5 19.5654 16.4064 19.7966 16.2397 19.9671C16.073 20.1376 15.8469 20.2334 15.6111 20.2334H1.38889C1.15314 20.2334 0.927048 20.1376 0.760349 19.9671C0.59365 19.7966 0.5 19.5654 0.5 19.3243V15.6879C0.5 15.4468 0.59365 15.2156 0.760349 15.0451C0.927048 14.8746 1.15314 14.7789 1.38889 14.7789ZM5.83333 3.86976H6.72222V2.05158H5.83333V3.86976ZM5.83333 11.1425H6.72222V9.32431H5.83333V11.1425ZM5.83333 18.4152H6.72222V16.597H5.83333V18.4152ZM2.27778 2.05158V3.86976H4.05556V2.05158H2.27778ZM2.27778 9.32431V11.1425H4.05556V9.32431H2.27778ZM2.27778 16.597V18.4152H4.05556V16.597H2.27778Z" fill="#4CDA95"/>
          </svg>
        {/if}
        <span
          class="text-[15px] font-medium leading-[140%] font-['Plus Jakarta Sans']"
          style="color: {iconColor}"
        >
          {device}
        </span>
      </div>
      {#if showCaption}
        <div
          class=" text-[11px] font-normal font-['Plus Jakarta Sans']"
          style="color: #9F9F9F; line-height: 140%;"
        >
          {caption}
        </div>
      {/if}
    </div>
  
    <!-- Kanan -->
    <div
      class="text-[11px] font-normal leading-[140%] font-['Plus Jakarta Sans'] text-right"
      style="color: {iconColor}"
    >
      {description}
    </div>
  </div>
  