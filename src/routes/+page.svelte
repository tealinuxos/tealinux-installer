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
		<h1 class="font-sans font-bold text-4xl">Welcome to TealinuxOS Installer</h1>
		<h2 class="mb-4 font-sans text-2xl">Press Start to Install</h2>

		<div class="p-2">
			{#if !online}
				<h3 class="mb-2 text-red-600 font-bold">Please connect to internet!</h3>
				<a href="/installation/locale">Bypass Internet</a>
			{/if}
			<br />
			<div>
				<button
					on:click={toLocale}
					class=" bg-greenTealinux hover:bg-green-800 text-white font-bold py-2 px-2 rounded-full w-64 disabled:bg-green-400"
					disabled={!online}
				>
					Start
				</button>
			</div>
		</div>
	</div>
</div>
