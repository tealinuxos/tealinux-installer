<script>
	import { getRead } from './../../global.js';
	import { onMount } from 'svelte';
	import { writable } from 'svelte/store';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import GlowingText from '$lib/components/ui/GlowingText.svelte';
	import Navigation from "$lib/components/Navigation.svelte";
	import DiskPreview from '$lib/components/DiskPreview.svelte';
	import InfoCard from './InfoCard.svelte';
	import PreviewButton from '../components/PreviewButton.svelte';
	
  
	const Method = {
	  SINGLE: 'single',
	  DUAL: 'dual',
	  MANUAL: 'manual'
	};
  
	const Preview = {
	  BEFORE: 'Before',
	  AFTER: 'After'
	};
  
	const disks = writable([]);
	const selectedDisk = writable(null);
	const selectedMethod = writable(null);
	const diskAfter = writable(null);
	const selectedPreview = writable(Preview.BEFORE);
  
	const getStorageJSON = async () => {
	  let json = await getRead();
	  disks.set(json.disk.filter((disk) => disk.partitions !== null));
	};
  
	function updateDiskPreview(disk) {
	  if (!disk) return;
  
	  // Simulasi preview AFTER dengan menambah partisi dummy
	  diskAfter.set({
		...disk,
		partitions: [...disk.partitions, { name: 'New Partition', size: '500MB' }]
	  });
	}
  
	const selectDisk = (disk) => {
	  console.log(`Selected Disk: ${disk.name}`);
	  selectedDisk.set(disk);
	  updateDiskPreview(disk);
	};
  
	const selectMethod = (method) => {
	  console.log(`Selected Method: ${method}`);
	  selectedMethod.set(method);
	};
  
	onMount(() => {
	  getStorageJSON();
	});
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
		<div class="flex flex-col gap-2">
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
		</div>

  
		<GlowingText size="[11]" text="File System" />
		<div class="flex flex-col gap-2">
			<InfoCard initialDevice="BTRFS" caption="Stable and widely used!" showCaption={true} showIcon={false} borderColor="#3C6350" backgroundColor="#101010" />
			<InfoCard initialDevice="EXT4" caption="Stable and widely used!" showCaption={true} showIcon={false} borderColor="#3C6350" backgroundColor="#101010" />
		</div>

  
		<GlowingText size="[11]" text="Swap Option" />
		<div class="flex flex-col gap-2">
			<InfoCard initialDevice="SWAP" caption="Recommended" showCaption={true} showIcon={false} borderColor="#3C6350" backgroundColor="#101010" />
			<InfoCard initialDevice="NO SWAP" caption="Stable and widely used!" showCaption={true} showIcon={false} borderColor="#3C6350" backgroundColor="#101010" />
	  
		</div>

		<div class="flex flex-col p-[15px] gap-[10px] self-stretch rounded-[10.267px] border border-[#3C6350] bg-[#101010]">
		  {#if $selectedDisk}
			<div class="flex flex-row space-x-2">
			  <PreviewButton
				title={Preview.BEFORE}
				selected={$selectedPreview === Preview.BEFORE}
				onclick={() => selectedPreview.set(Preview.BEFORE)}
			  />
			  <PreviewButton
				title={Preview.AFTER}
				selected={$selectedPreview === Preview.AFTER}
				onclick={() => selectedPreview.set(Preview.AFTER)}
			  />
			</div>
			<div class="space-y-[10px] w-full">
			  {#if $selectedPreview === Preview.BEFORE}
				<DiskPreview disk={$selectedDisk} />
			  {:else}
				<DiskPreview disk={$diskAfter} />
			  {/if}
			</div>
		  {:else}
			<PreviewButton title="Before" />
			<div class="text-[#E4E4E4] font-jakarta text-[9.46px] font-[500] leading-[17.659px] text-center py-4">
			  Select a disk to see preview
			</div>
		  {/if}
		</div>
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
  