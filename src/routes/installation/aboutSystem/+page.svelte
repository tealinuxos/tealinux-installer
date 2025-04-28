<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { getBlueprint } from '../global.js';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import GlowingText from '$lib/components/ui/GlowingText.svelte';
	import prettyBytes from 'pretty-bytes';

	let disks = [];
	let selectedDisk = null;
	let selectedTool = null;
	let diskPreview = null;
	let showSingleBootWarning = false;
	let warningPosition = { x: 0, y: 0 };

	const getDisks = async () => {
		invoke('get_available_disks').then((response) => {
			disks = JSON.parse(response);
			console.log("Available disks:", disks);
		});
	};

	const selectDisk = (disk) => {
		console.log(`Selected Disk: ${disk.name}`);
		selectedDisk = disk;
		updateDiskPreview(disk);
	};

	const selectTool = (tool) => {
		console.log(`Selected Tool: ${tool}`);
		selectedTool = tool;
	};

	const updateDiskPreview = (disk) => {
		diskPreview = {
			name: disk.name,
			size: disk.size,
			partitions: disk.partitions || []
		};
	};

	const handleSetDisk = async () => {
		if (selectedDisk && selectedTool) {
			await invoke('blueprint_set_disk', { 
				disk: selectedDisk, 
				tool: selectedTool 
			});
		}
	};

	const showWarning = (event) => {
		showSingleBootWarning = true;
		warningPosition = { x: event.clientX, y: event.clientY };
	};
	
	const hideWarning = () => {
		showSingleBootWarning = false;
	};

	const getDiskColor = (diskName) => {
		const colors = ['#3C6350', '#26A768', '#7A14EF', '#EF7E20', '#0D44E1'];
		const index = diskName.split('').reduce((acc, char) => acc + char.charCodeAt(0), 0) % colors.length;
		return colors[index];
	};

	onMount(() => {
		getDisks();
		getBlueprint().then((blueprint) => {
			console.log(blueprint);
			if (blueprint.disk) {
				selectedDisk = blueprint.disk;
				selectedTool = blueprint.tool || 'single_boot';
				updateDiskPreview(selectedDisk);
			}
		});
	});
</script>

<TwoSide>
	{#snippet left()}
		<div class="mx-[35px] space-y-[15px]">
			<h1 class="font-jakarta font-[800] text-[28px]">
				Choose your disks and
				<br />
				tool for partitioning
			</h1>
			<p class="font-jakarta text-sm font-[200]">
				Qorem ipsum dolor sit amet, consectetur adipiscing elit.
				Etiam eu turpis molestie, dictum est a, mattis tellus. 
			</p>
		</div>
	{/snippet}
	
	{#snippet right()}
		<div class="flex flex-col h-[562px] p-5 space-y-[15px] mb-[15px] bg-black/30 border-[0.5px] border-gray-900 rounded-[10px] font-jakarta">
			<!-- Disk selection -->
			<div class="space-y-[10px]">
				<GlowingText size="[11]" text="Disk" />
				{#if disks.length > 0}
					<div class="space-y-2">
						{#each disks as disk}
							<div
								class="flex p-[10px] border border-border bg-[#101010] rounded-[14px] text-[15px] justify-between h-fit w-full cursor-pointer hover:bg-gray-900 transition-colors {selectedDisk?.name === disk.name ? 'border-[#3C6350]' : ''}"
								on:click={() => selectDisk(disk)}
							>
								<div>
									<span>{disk.name}</span>
								</div>
								<div>
									<span>{prettyBytes(disk.size)}</span>
								</div>
							</div>
						{/each}
					</div>
				{:else}
					<div class="text-gray-400">Loading disks...</div>
				{/if}
			</div>

			<GlowingText size="[11]" text="Tools" />
			<!-- Partitioning tools -->
			<div class="flex flex-start gap-[15px] p-[10px] w-full">
				<!-- Single Boot Card -->
				<div 
							class="flex flex-col bg-[#101010] rounded-[14px] border-[1.3px] border-[#3C6350] p-[10px] w-[190px] h-[167px] cursor-pointer transition-all duration-100
								{selectedTool === 'single_boot' ? 
									'shadow-[0_0_9px_#26A768]' : 
									'border-transparent hover:border-[#3C6350] hover:shadow-[0_0_9px_#26A768]'}"
							on:click={() => selectTool('single_boot')}
							on:mouseenter={showWarning}
							on:mouseleave={hideWarning}
						>
					<div class="flex items-center gap-2">
						<svg width="29" height="32" viewBox="0 0 29 32" fill="none" xmlns="http://www.w3.org/2000/svg">
							<rect x="1" y="2.83325" width="21" height="23" rx="2" fill="#0D44E1"/>
							<g filter="url(#filter0_df_2074_1684)">
							<rect x="7" y="7.83325" width="21" height="23" rx="2" fill="#849EEA" fill-opacity="0.7" shape-rendering="crispEdges"/>
							</g>
							<path d="M11 18.4047H25V21.2618H11V18.4047Z" fill="#D9D9D9"/>
							<path d="M16.3333 25.8333V13.8333H19.6667V25.8333H16.3333Z" fill="#D9D9D9"/>
							<defs>
							<filter id="filter0_df_2074_1684" x="0" y="0.833252" width="29" height="31" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
							<feFlood flood-opacity="0" result="BackgroundImageFix"/>
							<feColorMatrix in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0" result="hardAlpha"/>
							<feOffset dx="-3" dy="-3"/>
							<feGaussianBlur stdDeviation="2"/>
							<feComposite in2="hardAlpha" operator="out"/>
							<feColorMatrix type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.25 0"/>
							<feBlend mode="normal" in2="BackgroundImageFix" result="effect1_dropShadow_2074_1684"/>
							<feBlend mode="normal" in="SourceGraphic" in2="effect1_dropShadow_2074_1684" result="shape"/>
							<feGaussianBlur stdDeviation="0.5" result="effect2_foregroundBlur_2074_1684"/>
							</filter>
							</defs>
						</svg>
						
						<h3 class="text-white font-bold" style="
							color: #FFF;
							leading-trim: both;
							text-edge: cap;
							font-family: 'Plus Jakarta Sans';
							font-size: 15px;
							font-style: normal;
							font-weight: 500;
							line-height: normal;
							letter-spacing: -0.6px;
						">Single Boot</h3>
					</div>
					<p class="text-white mt-2" style="
						color: #FFF;
						leading-trim: both;
						text-edge: cap;
						font-family: 'Plus Jakarta Sans';
						font-size: 12px;
						font-style: normal;
						font-weight: 200;
						line-height: normal;
						letter-spacing: -0.48px;
					">
						Choose this if you only want Team JuOS to be in early operating when made your computer
					</p>
				</div>
				
				<!-- Double Boot Card -->
				<div 
					class="flex flex-col bg-[#101010] rounded-[14px] p-[10px] w-[190px] h-[167px] cursor-pointer transition-all duration-100
						   {selectedTool === 'double_boot' ? 
							'border-[1.3px] border-[#3C6350] shadow-[0_0_9px_#26A768]' : 
							'border-[1.3px] border-transparent hover:border-[#3C6350] hover:shadow-[0_0_9px_#26A768]'}"
					on:click={() => selectTool('double_boot')}
				>
					<div class="flex items-center gap-2">
						<svg width="27" height="29" viewBox="0 0 27 29" fill="none" xmlns="http://www.w3.org/2000/svg">
							<path d="M0.601904 23.0112C-0.168336 22.6603 -0.210421 20.9971 0.539728 20.5541L13.2008 13.0766C13.3935 12.9628 13.6065 12.9628 13.7992 13.0766L26.4603 20.5541C27.2104 20.9971 27.1683 22.6603 26.3981 23.0112L13.7371 28.7804C13.5823 28.8509 13.4177 28.8509 13.2629 28.7804L0.601904 23.0112Z" fill="#EF7E20"/>
							<path d="M0.601904 20.9849C-0.168336 20.6339 -0.210421 18.9707 0.539728 18.5277L13.2008 11.0503C13.3935 10.9365 13.6065 10.9365 13.7992 11.0503L26.4603 18.5277C27.2104 18.9707 27.1683 20.6339 26.3981 20.9849L13.7371 26.754C13.5823 26.8245 13.4177 26.8245 13.2629 26.754L0.601904 20.9849Z" fill="white" fill-opacity="0.31"/>
							<path d="M0.601904 18.9586C-0.168336 18.6076 -0.210421 16.9444 0.539728 16.5014L13.2008 9.02395C13.3935 8.91014 13.6065 8.91014 13.7992 9.02395L26.4603 16.5014C27.2104 16.9444 27.1683 18.6076 26.3981 18.9586L13.7371 24.7277C13.5823 24.7982 13.4177 24.7982 13.2629 24.7277L0.601904 18.9586Z" fill="#EF7E20"/>
							<path d="M0.601904 16.9322C-0.168336 16.5813 -0.210421 14.9181 0.539728 14.475L13.2008 6.99761C13.3935 6.88381 13.6065 6.88381 13.7992 6.99761L26.4603 14.475C27.2104 14.9181 27.1683 16.5813 26.3981 16.9322L13.7371 22.7014C13.5823 22.7719 13.4177 22.7719 13.2629 22.7014L0.601904 16.9322Z" fill="white" fill-opacity="0.31"/>
							<path d="M0.601904 14.9059C-0.168336 14.5549 -0.210421 12.8917 0.539728 12.4487L13.2008 4.97128C13.3935 4.85747 13.6065 4.85747 13.7992 4.97128L26.4603 12.4487C27.2104 12.8917 27.1683 14.5549 26.3981 14.9059L13.7371 20.675C13.5823 20.7455 13.4177 20.7455 13.2629 20.675L0.601904 14.9059Z" fill="#EF7E20"/>
							<path d="M0.601904 12.8796C-0.168336 12.5286 -0.210421 10.8654 0.539728 10.4224L13.2008 2.94494C13.3935 2.83114 13.6065 2.83114 13.7992 2.94494L26.4603 10.4224C27.2104 10.8654 27.1683 12.5286 26.3981 12.8796L13.7371 18.6487C13.5823 18.7192 13.4177 18.7192 13.2629 18.6487L0.601904 12.8796Z" fill="white" fill-opacity="0.31"/>
							<path d="M0.601904 10.8532C-0.168336 10.5023 -0.210421 8.83907 0.539728 8.39604L13.2008 0.918608C13.3935 0.8048 13.6065 0.8048 13.7992 0.918608L26.4603 8.39604C27.2104 8.83907 27.1683 10.5023 26.3981 10.8532L13.7371 16.6224C13.5823 16.6929 13.4177 16.6929 13.2629 16.6224L0.601904 10.8532Z" fill="#EF7E20"/>
							<rect x="12" y="14.8333" width="15" height="14" rx="7" fill="#D9D9D9" fill-opacity="0.7"/>
							<path d="M23.1148 20.119C22.808 19.4722 22.3333 18.9197 21.7401 18.5189C21.147 18.1182 20.4571 17.8839 19.7426 17.8406C19.028 17.7973 18.3149 17.9465 17.6777 18.2726C17.0404 18.5987 16.5024 19.0899 16.1197 19.6948C15.737 20.2997 15.5236 20.9963 15.5018 21.7117C15.4801 22.4272 15.6508 23.1354 15.996 23.7624C16.3413 24.3895 16.8485 24.9124 17.4647 25.2766C18.081 25.6409 18.7837 25.8331 19.4996 25.8333C21.3112 25.8333 23.0085 24.6275 23.5 22.9761M23.1148 20.119V18.041M23.1148 20.119L20.9551 20.3787" stroke="white" stroke-width="0.9" stroke-linecap="round" stroke-linejoin="round"/>
						</svg>	
						<h3 class="text-white font-bold" style="
							color: #FFF;
							leading-trim: both;
							text-edge: cap;
							font-family: 'Plus Jakarta Sans';
							font-size: 15px;
							font-style: normal;
							font-weight: 500;
							line-height: normal;
							letter-spacing: -0.6px;
						">Double Boot</h3>
					</div>
					<p class="text-white mt-2" style="
						color: #FFF;
						leading-trim: both;
						text-edge: cap;
						font-family: 'Plus Jakarta Sans';
						font-size: 12px;
						font-style: normal;
						font-weight: 200;
						line-height: normal;
						letter-spacing: -0.48px;
					">
						Choose this if you want Team JuOS to be installed alongside other operating system
					</p>
				</div>
				
				<!-- Manual Partition Card -->
				<div 
					class="flex flex-col bg-[#101010] rounded-[14px] p-[10px] w-[190px] h-[167px] cursor-pointer transition-all duration-100
						   {selectedTool === 'manual' ? 
							'border-[1.3px] border-[#3C6350] shadow-[0_0_9px_#26A768]' : 
							'border-[1.3px] border-transparent hover:border-[#3C6350] hover:shadow-[0_0_9px_#26A768]'}"
					on:click={() => selectTool('manual')}
				>
					<div class="flex items-center gap-2">
						<svg width="27" height="29" viewBox="0 0 27 29" fill="none" xmlns="http://www.w3.org/2000/svg">
							<path d="M18.383 10.0605C18.383 15.1566 15.168 19.2878 11.2021 19.2878C7.23625 19.2878 4.02128 15.1566 4.02128 10.0605C4.02128 4.96444 7.23625 0.833252 11.2021 0.833252C15.168 0.833252 18.383 4.96444 18.383 10.0605Z" fill="#7A14EF"/>
							<path d="M27 12.606C27 17.7021 23.2706 21.8333 18.6702 21.8333C14.0698 21.8333 10.3404 17.7021 10.3404 12.606C10.3404 7.5099 14.0698 3.37871 18.6702 3.37871C23.2706 3.37871 27 7.5099 27 12.606Z" fill="#7A14EF"/>
							<path d="M16.6596 13.5605C16.6596 18.1294 12.9302 21.8333 8.32979 21.8333C3.72937 21.8333 0 18.1294 0 13.5605C0 8.99162 3.72937 5.2878 8.32979 5.2878C12.9302 5.2878 16.6596 8.99162 16.6596 13.5605Z" fill="#7A14EF"/>
							<rect x="12" y="13.8333" width="15" height="15" rx="7.5" fill="#D9D9D9" fill-opacity="0.7"/>
							<path d="M19 15.8333H20.2V21.8333H19V15.8333Z" fill="white"/>
							<path d="M19 21.8333V20.6333H25V21.8333H19Z" fill="white"/>
						</svg>
						
						<h3 class="text-white font-bold" style="
							color: #FFF;
							leading-trim: both;
							text-edge: cap;
							font-family: 'Plus Jakarta Sans';
							font-size: 15px;
							font-style: normal;
							font-weight: 500;
							line-height: normal;
							letter-spacing: -0.6px;
						">Manual Partition</h3>
					</div>
					<p class="text-white mt-2" style="
						color: #FFF;
						leading-trim: both;
						text-edge: cap;
						font-family: 'Plus Jakarta Sans';
						font-size: 12px;
						font-style: normal;
						font-weight: 200;
						line-height: normal;
						letter-spacing: -0.48px;
					">
						Choose this if you want Team JuOS to be installed alongside offline operating system
					</p>
				</div>
			</div>
		
			<GlowingText size="[11]" text="Preview Disk" />

			<!-- Add this section after the partitioning tools section -->
			<div class="flex flex-col p-[15px] gap-[10px] self-stretch rounded-[10.267px] border border-[#3C6350] bg-[#101010]">
				<!-- Disk Preview Title -->

				<GlowingText size="[11]" text="Before" />
				
				{#if diskPreview}
					<div class="space-y-[10px] w-full">
						<!-- Before Section -->
						<div class="space-y-[10px]">
							<h4 class="text-[#26A768] font-jakarta text-[14.667px] font-[600] leading-normal tracking-[-0.66px]">
								Before
							</h4>
							
							{#if diskPreview.partitions.length > 0}
								<div class="space-y-[5px] w-full">
									{#each diskPreview.partitions as partition}
										<div class="flex items-center justify-between w-full h-[16px]">
											<span class="text-[#E4E4E4] font-jakarta text-[9.46px] font-[500] leading-[17.659px]">
												{partition.name || 'Unallocated'}
											</span>
											<span class="text-[#E4E4E4] font-jakarta text-[9.46px] font-[500] leading-[17.659px]">
												{partition.size ? prettyBytes(partition.size) : 'Unallocated'}
											</span>
										</div>
									{/each}
								</div>
							{:else}
								<div class="flex items-center justify-between w-full h-[16px]">
									<span class="text-[#E4E4E4] font-jakarta text-[9.46px] font-[500] leading-[17.659px]">
										Unallocated
									</span>
									<span class="text-[#E4E4E4] font-jakarta text-[9.46px] font-[500] leading-[17.659px]">
										{prettyBytes(diskPreview.size)} Unallocated
									</span>
								</div>
							{/if}
						</div>
					</div>
				{:else}
					<div class="text-[#E4E4E4] font-jakarta text-[9.46px] font-[500] leading-[17.659px] text-center py-4">
						Select a disk to see preview
					</div>
				{/if}
			</div>

			<!-- Warning tooltip -->
			{#if showSingleBootWarning}
				<div 
					class="absolute bg-[#3C6350] text-white p-3 rounded-lg shadow-lg z-50 max-w-[200px]"
					style="left: {warningPosition.x}px; top: {warningPosition.y + 20}px"
				>
					<p class="text-xs">Warning: This will erase all existing data on the selected disk. Make sure to backup any important files.</p>
				</div>
			{/if}
		</div>
	{/snippet}
</TwoSide>