<script>
  let {
    initialDevice = "/dev/sda5",
    initialDescription = "",
    caption = "",
    borderColor = "#4CDA95",
    backgroundColor = "#032B17",
    iconColor = "#4CDA95",
    showIcon = true,
    showCaption = false
  } = $props();

  // State
  let device = initialDevice;
  let description = initialDescription;
</script>

<!-- UI -->
<div
  class="flex items-center justify-between h-[48px] px-3 py-3 rounded-lg w-full"
  style="border: 1.3px solid {borderColor}; background: {backgroundColor};"
>
  <!-- Left Section -->
  <div class="flex flex-col justify-center">
    <div class="flex items-center gap-2">
      {#if showIcon}
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill={iconColor}
          viewBox="0 0 24 24"
          class="w-5 h-5"
        >
          <path d="M3 3h18v6H3V3zm0 12h18v6H3v-6zm0-6h18v2H3v-2zm2 8h2v2H5v-2zm0-12h2v2H5V5z"/>
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
        class="text-[11px] font-normal font-['Plus Jakarta Sans']"
        style="color: #9F9F9F; line-height: 140%;"
      >
        {caption}
      </div>
    {/if}
  </div>

  <!-- Right Section -->
  <div
    class="text-[11px] font-normal leading-[140%] font-['Plus Jakarta Sans'] text-right"
    style="color: {iconColor}"
  >
    {description}
  </div>
</div>