<script>
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import GlowingText from '$lib/components/ui/GlowingText.svelte';
	import Navigation from "$lib/components/Navigation.svelte";
	import DiskSlider from '$lib/components/DiskSlider.svelte';
	import InfoCard from './InfoCard.svelte';

	const getStorageJSON = async () => {
		let json = await getRead();
		json = json.disk.filter((disk) => disk.partitions !== null);
		return json;
	};

	const getTotalStorage = async () => {
		let storage = await getStorageJSON();
		let total = 0;
		for (let i of storage.keys()) {
			let size = storage[i].size.slice(0, -1);
			total += parseInt(size);
		}
		return total;
	};

	const getColors = (disks, partIdx) => {
		let length = disks[partIdx].partitions.length;
		let colors = [];

		for (let i = 0; i < length; i++) {
			colors.push(
				randomColor({
					luminosity: 'bright',
					hue: 'random'
				})
			);
		}
		return colors;
	};
</script>

<TwoSide>
	{#snippet left()}
		<div class="mx-[35px] space-y-[15px]">
			<h1 class="font-jakarta font-[800] text-[28px]">
				Set up your keyboard<br />
				layout, timezone, locale
			</h1>
			<p class="font-jakarta text-sm font-[200]">
				Qorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam eu turpis molestie, dictum
				est a, mattis tellus.
			</p>
		</div>
	{/snippet}

	{#snippet right()}
		<div class="flex flex-col h-[562px] p-3 space-y-[15px] mb-[15px] bg-black/30 border-[0.5px] border-gray-900 rounded-[10px] font-jakarta">
			<GlowingText size="[11]" text="Selected Disk" />
			
			<InfoCard
				initialDevice="/dev/nvme0n1"
				initialDescription="WD Black SN850X"
				showCaption={false}
				showIcon={true}
				borderColor="#4CDA95"
				backgroundColor="#032B17"
				iconColor="#4CDA95"
			/>

			<GlowingText size="[11]" text="File System" />
			<InfoCard
				initialDevice="BTRFS"
				caption="Stable and widely used!"
				showCaption={true}
				showIcon={false}
				borderColor="#3C6350"
				backgroundColor="#101010"
			/>
			<InfoCard
				initialDevice="EXT4"
				caption="Stable and widely used!"
				showCaption={true}
				showIcon={false}
				borderColor="#3C6350"
				backgroundColor="#101010"
			/>

			<GlowingText size="[11]" text="Swap Option" />
			<InfoCard
				initialDevice="SWAP"
				caption="Recommended"
				showCaption={true}
				showIcon={false}
				borderColor="#3C6350"
				backgroundColor="#101010"
			/>
			<InfoCard
				initialDevice="NO SWAP"
				caption="Stable and widely used!"
				showCaption={true}
				showIcon={false}
				borderColor="#3C6350"
				backgroundColor="#101010"
			/>

			{#await getStorageJSON()}
				Loading...
			{:then disks}
				<DiskSlider {disks} colors={getColors(disks, 0)} />
			{/await}
		</div>
	{/snippet}
</TwoSide>

<Navigation
	totalSteps={5}
	currentStep={4}
	currentTitle="Single Boot"
	prevPath="/installation/partitioning"
	nextPath="/installation"
	nextAction={null}
/>
