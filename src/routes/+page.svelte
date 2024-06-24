<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import Tealinux from '$lib/assets/Vector.png';

	let online = false;

	const isOnline = async () => {
		invoke('is_online').then((status) => (online = status));
	};

	function toLocale() {
		if (online) {
			window.location.href = '/installation';
		}
	}

	onMount(() => {
		isOnline();
	});
</script>

<div class="flex items-center justify-center min-h-screen">
	<div class="text-center">
		<img src={Tealinux} alt="" class="mx-auto mb-4" />
		<h1 class="font-archivo font-semibold text-6xl -tracking-[4.5%] mb-4">Welcome to Tealinux</h1>
		<h2 class="font-poppin mb-4 text-4xl tracking-normal">Press start to install</h2>

		<div class="p-2">
			{#if !online}
				<h3 class="mb-2 text-red-600 font-bold">Please connect to internet!</h3>
				<a href="/installation/locale">Bypass Internet</a>
			{/if}
			<br />
			<div>
				<button
					on:click={toLocale}
					class=" bg-greenTealinux rounded-3xl hover:bg-green-800 text-white font-poppinsemibold text-xl py-[10px] px-28 disabled:bg-green-400 border-2 border-black/15"
					disabled={!online}
				>
					START
				</button>
			</div>
		</div>
	</div>
</div>
