<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import laptop from '../assets/laptop.png';

	let json;
	let showModelName = false;

	async function handleGetJSON() {
		invoke('get_read_json').then((response) => {
			json = JSON.parse(response);
			console.log(json);
			showModelName = true;
		}).await;
	}

	onMount(() => {
		handleGetJSON();
	});
</script>

<nav class="flex flex-col flex-1 overflow-y-auto bg-white">
	<div class="bg-[#D9D9D9] flex items-center py-4 px-8 w-full font-sans font-medium text-2xl">Tealinux Installer</div>
</nav>

<main class="p-8">
	<div class="bg-[#D9D9D9] w-full grid-cols-3 grid place-items-center py-3 px-16 h-[40vh] rounded-2xl">
		<div class="border border-black flex flex-col items-center gap-y-1">
			<img src="{laptop}" alt="gambar laptop" class="w-1/2">
			<p class="text-2xl font-medium">82SV</p>
			<p>Yoga Slim 7Pro 14|AP7</p>
			<p>80 %</p>
			<p>online</p>
		</div>
		<div>ini bagian tengah</div>
		<div>ini bagian kanan</div>
	</div>
	<div class="w-full mt-8 py-4 grid grid-cols-2 gap-8 max-h-96 overflow-y-auto">
		{#each [1,2,3,4,5,6] as e}			
			<div class="aspect-square bg-[#D9D9D9]"></div>
		{/each}
	</div>
	<a href="/partitioning">Partitioning</a>
</main>




{#if showModelName}
	<div
		class="flex flex-col justify-center p-5 gap-y-4 bg-slate-200 w-fit rounded-xl shadow-xl hover:scale-105 hover:translate-x-2 hover:translate-y-2 transition-all"
	>
		<h1 class="font-bold">Device Information</h1>
		<div class="flex justify-between">
			<h1 class="font-bold">
                {#if json.battery.capacity != null}
                    Battery : {json.battery.capacity}%
                {:else}
                    No Battery Found
                {/if}
            </h1>
			<h1 class="font-bold flex items-center">
				{#if json.online.status}
					online <span
						class="w-3 ml-2 aspect-square rounded-full bg-green-400 inline-block border border-slate-600"
					></span>
				{:else}
					offline <span
						class="w-3 ml-2 aspect-square rounded-full bg-red-500 inline-block border border-slate-600"
					></span>
				{/if}
			</h1>
		</div>
		<hr class="border border-slate-800" />
		<p>{json.model.systemProductName + ' - ' + json.model.systemVersion}</p>
		<table>
			<tr>
				<td>OS</td>
				<td>: {json.operatingSystems ? json.operatingSystems.join(', ') : ''}</td>
			</tr>
			<tr>
				<td>CPU</td>
				<td>: {json.lspci.cpu ? json.lspci.cpu : ''}</td>
			</tr>
			<tr>
				<td>VGA</td>
				<td>: {json.lspci.vga ? json.lspci.vga : ''}</td>
			</tr>
			<tr>
				<td>Memory Capacity</td>
				<td>: {json.memory.capacity ? json.memory.capacity : ''} Mb</td>
			</tr>
			<tr>
				<td>Memory Used</td>
				<td>: {json.memory.used ? json.memory.used : ''} Mb</td>
			</tr>
			<tr>
				<td>Memory Free</td>
				<td>: {json.memory.capacity - json.memory.used} Mb</td>
			</tr>
		</table>
	</div>
{/if}


