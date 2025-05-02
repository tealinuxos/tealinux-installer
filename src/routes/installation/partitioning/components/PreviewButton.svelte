<script>
    let {
        title = 'default title',
        selected = false,
        onclick = () => console.log('pressed')
    } = $props();
</script>

<button
    onclick={onclick}
    class="{selected ? "text-[#26A768]" :  "text-[#17613D]"} font-jakarta text-[14.667px] font-[600] leading-normal tracking-[-0.66px]"
>
    {title}
</button>
