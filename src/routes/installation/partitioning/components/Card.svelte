<script>
	let {
		selectedMethod,
		method,
		onclick = () => console.log('clicked'),
		onmouseenter = () => {},
		onmouseleave = () => {},
		title = 'A title',
		description = 'A description',
		disable = false,
		icon
	} = $props();

	let Method = {
		SINGLE: 'single',
		DUAL: 'dual',
		MANUAL: 'manual'
	};
</script>

<div
	class="flex flex-col bg-[#101010] rounded-[14px] p-[10px] w-[190px] h-[167px] cursor-pointer transition-all duration-100
           {disable
		? 'grayscale opacity-70 border-[1.3px] border-transparent cursor-not-allowed!'
		: selectedMethod === method
			? 'border-[1.3px] border-[#3C6350] shadow-[0_0_9px_#26A768]'
			: 'border-[1.3px] border-transparent hover:border-[#3C6350] hover:shadow-[0_0_9px_#26A768]'}"
	{onclick}
	{onmouseenter}
	{onmouseleave}
>
	<div class="flex items-center gap-2">
		{@render icon()}
		<h3
			class="text-white font-bold"
			style="
            color: #FFF;
            leading-trim: both;
            text-edge: cap;
            font-family: 'Plus Jakarta Sans';
            font-size: 15px;
            font-style: normal;
            font-weight: 500;
            line-height: normal;
            letter-spacing: -0.6px;
        "
		>
			{title}
		</h3>
	</div>
	<p
		class="text-white mt-2"
		style="
        color: #FFF;
        leading-trim: both;
        text-edge: cap;
        font-family: 'Plus Jakarta Sans';
        font-size: 12px;
        font-style: normal;
        font-weight: 200;
        line-height: normal;
        letter-spacing: -0.48px;
    "
	>
		{description}
	</p>
</div>
