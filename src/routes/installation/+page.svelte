<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { getRead } from './global.js';

	function calculateToGB() {
		let partitionSize = json.disk[0].partitions[0].size;
		partitionSize = parseInt(partitionSize.replace('s', ''));
	}
</script>

<main class="max-h-dvh">
{#await getRead()}
{:then json}
    {@const memoryPercent = json.memory.used / json.memory.capacity * 100}
	<div class=" py-8 px-16 mx-auto overflow-auto max-h-[85dvh] scrollbar-none">
		<div class=" bg-greenTealinux bg-opacity-25 w-full p-5 rounded-[43px] mb-6">
			<div class="bg-white grid-cols-3 grid place-items-center py-3 px-16 h-[40vh] rounded-3xl">
					<div class="flex flex-col items-center gap-y-1">
						<img src="/windows.svg" alt="" />
						<p class="text-2xl font-medium mt-[8px]">82SV</p>
						<p>{json.model.systemProductName + ' - ' + json.model.systemVersion}</p>
						<h2 class="font-medium font-poppin text-[16px] flex items-center">
							<img src="/battrey.svg" alt="" class="pr-[8px]" />
							{json.battery.capacity}%
						</h2>
						<div class="flex items-center">
							<img src="/Connection.svg" alt="" class="pr-[8px]" />
							<h2 class="font-medium font-poppin text-[16px] flex items-center">
								Online
								{#if json.online.status}
									<span
										class="w-3 ml-2 aspect-square rounded-full bg-green-400 inline-block border border-slate-600 pl-[6px]"
									></span>
								{:else}
									<span>false</span>
								{/if}
							</h2>
						</div>
					</div>
					<div class="flex flex-col space-y-10">
						<!-- RAM -->
						<div class="flex flex-col items-center h-full">
							<div class="flex items-center justify-center h-full">
								<h2 class="font-archivo font-bold text-[20px]">Ram</h2>
							</div>
							<div class="w-[241px] h-[16px] bg-grayTealinux rounded-[128px]">
								<div
									class="bg-[#F1C21B] h-[16px] rounded-[128px]"
									style="width: {memoryPercent.toFixed()}%"
								></div>
							</div>
							<h2 class="font-medium font-poppin text-[16px] mt-2">
								{memoryPercent.toFixed(2)}% of 100%
							</h2>
						</div>
						<!-- CPU -->
						<div class="flex flex-col items-center h-full">
							<div class="flex items-center justify-center h-full">
								<h2 class="font-archivo font-bold text-[20px] text-center">CPU</h2>
							</div>
							<h2 class="font-poppin font-medium text-[16px]">{json.lspci.cpu}</h2>
						</div>
					</div>
					<div class="flex flex-col space-y-10">
						<!-- Storage -->
						<div class="flex flex-col items-center h-full">
							<div class="flex items-center justify-center h-full">
								<h2 class="font-archivo font-bold text-[20px]">Storage</h2>
							</div>
							<div class="w-[241px] h-[16px] bg-grayTealinux rounded-[128px]">
								<div
									class="bg-[#F1C21B] h-[16px] rounded-[128px] flex items-center"
									style="width: 50%"
								></div>
							</div>
							<h2 class="font-medium font-poppin text-[16px] mt-2">{json.disk[0].size}</h2>
							<!-- {#each json.disk as e,idx}
								{/each} -->
							<!-- {#each json.disk[0].partitions as partition}	
									<h2 class="font-medium font-poppin text-[16px]">{partition.size}</h2>
								{/each} -->
						</div>
						<!-- GPU -->
						<div class="flex flex-col items-center h-full">
							<div class="flex items-center justify-center h-full">
								<h2 class="font-archivo font-bold text-[20px] text-center">GPU</h2>
							</div>
                            <ul class="list-disc">
                                {#each json.lspci.vga as vga}
                                    <li>
                                        <h2 class="font-poppin font-medium text-[16px]">{vga}</h2>
                                    </li>
                                {/each}
                            </ul>
						</div>
					</div>
			</div>
		</div>
		<!-- ============================================================================================ -->
		<div class=" bg-greenTealinux bg-opacity-25 w-full p-5 rounded-2xl mb-6 flex justify-center">
			<div class="bg-white place-items-center py-3 px-16 h-[40vh] min-w-full rounded-3xl">
				<div
					class="flex items-center justify-between bg-gray-300 h-[45px] rounded-[10px] mt-[30.05px] w-full"
				>
					<p
						class="font-poppin font-medium text-[#0D1814] text-[14px] mt-[12px] ml-[12px] mb-[12px]"
					>
						sda
					</p>
					<p class="font-poppin text-[14px] text-[#0D1814] mt-[12px] mr-[12px] mb-[12px]">
						Disk Size : 240 GB
					</p>
				</div>
				<div class="mt-[33px] flex flex-col items-start">
					<div class="flex gap-x-10 items-center w-full">
						<p class="font-poppin font-medium text-[18px]">Current:</p>
						<div class="bg-[#36BA7A] h-[38px] flex-1 rounded-[128px]">
							<div class="bg-[#F1C21B] h-[38px] w-[30%] rounded-[128px]"></div>
						</div>
					</div>
					<div class="flex mt-[13px] ml-[128.31px] mr-[554.35px]">
						<div class="flex items-start gap-x-4 mr-[25px]">
							<div class="flex items-center">
								<div class="bg-[#F1C21B] w-[16px] h-[16px] rounded-[3px]"></div>
							</div>
							<div>
								<p class="font-poppin font-medium text-[15px]">sdb1</p>
								<p class="font-poppin font-medium text-[16px]">13.5 GiB LUKS</p>
							</div>
						</div>
						<div class="flex items-start gap-x-4">
							<div class="flex items-center">
								<div class="bg-[#36BA7A] w-[16px] h-[16px] rounded-[3px]"></div>
							</div>
							<div>
								<p class="font-poppin font-medium text-[15px]">sdb2</p>
								<p class="font-poppin font-medium text-[16px]">8.5 GiB LUKS</p>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>

	<div class="flex justify-between h-[15dvh] items-center w-full px-16 font-poppin">
		<a
			href="/"
			class="py-[10px] px-[20px] bg-greenTealinux text-white font-poppin text-base rounded-lg"
			>Back</a
		>
		<a
			href="/installation/keyboard"
			class="py-[10px] px-[20px] bg-greenTealinux text-white font-poppin text-base rounded-lg"
			>Next</a
		>
	</div>
{/await}
</main>

<style>
	/* .scrollbar-none::-webkit-scrollbar {
		display: none;
	} */

	/* Hide scrollbar for IE, Edge and Firefox */
	.scrollbar-none {
		-ms-overflow-style: none; /* IE and Edge */
		scrollbar-width: none; /* Firefox */
	}
</style>
