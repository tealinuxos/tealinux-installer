<!-- HexagonLoader.svelte -->
<script lang="ts">
	import { fade, fly, scale } from 'svelte/transition';
	import { quintOut, backOut } from 'svelte/easing';

	export let text: string = 'Loading';
	export let show: boolean = true;

	const layers = 3;
</script>

{#if show}
	<div
		class="fixed inset-0 z-50 flex flex-col items-center justify-center bg-tealinux text-white"
		in:fade={{ duration: 400 }}
		out:fade={{ duration: 600, easing: quintOut }}
	>
		<!-- Stacked Animated Hexagons -->
		<div
			class="relative w-32 h-32 mb-8"
			in:scale={{ duration: 800, delay: 200, easing: backOut }}
			out:scale={{ duration: 500, easing: quintOut }}
		>
			{#each Array(layers) as _, i (i)}
				<div
					class="absolute inset-0 flex items-center justify-center hexagon-layer"
					style="
              transform: translate({i * 8}px, {i * 4}px);
              opacity: {1 - i * 0.25};
              z-index: {layers - i};
              animation-delay: {i * 200}ms;
            "
					in:fly={{
						y: 50,
						duration: 600,
						delay: 300 + i * 100,
						easing: backOut
					}}
					out:fly={{
						y: -50,
						duration: 400,
						delay: i * 100,
						easing: quintOut
					}}
				>
					<!-- Hexagon Shape -->
					<img
						src="logo-tealinux.svg"
						alt="logo-tealinux"
						class="relative h-32 animate-float animate-out"
						style="animation-delay: {i * 200}ms;"
					/>
				</div>
			{/each}
		</div>

		<!-- Loading Text -->
		<div
			class="text-xl font-medium animate-pulse mb-4"
			in:fly={{ y: 20, duration: 600, delay: 600, easing: quintOut }}
			out:fly={{ y: -20, duration: 400, delay: 200, easing: quintOut }}
		>
			{text}
		</div>

		<!-- Loading dots animation -->
		<div
			class="flex space-x-1"
			in:fade={{ duration: 600, delay: 800 }}
			out:fade={{ duration: 300, delay: 100 }}
		>
			{#each Array(3) as _, i (i)}
				<div
					class="w-2 h-2 bg-green-500 rounded-full animate-bounce"
					style="animation-delay: {i * 100}ms;"
				/>
			{/each}
		</div>
	</div>
{/if}

<style>
	@keyframes float {
		0%,
		100% {
			transform: translateY(0px) scale(1);
		}
		50% {
			transform: translateY(-10px) scale(1.05);
		}
	}

	@keyframes pulse-glow {
		0%,
		100% {
			filter: drop-shadow(0 0 20px rgba(34, 197, 94, 0.3));
		}
		50% {
			filter: drop-shadow(0 0 30px rgba(34, 197, 94, 0.6));
		}
	}

	@keyframes float-out {
		0% {
			transform: translateY(0px) scale(1);
		}
		100% {
			transform: translateY(-30px) scale(0.8);
			opacity: 0;
		}
	}

	.animate-float {
		animation: float 2s ease-in-out infinite;
	}

	.animate-out {
		transition: all 0.5s ease-out;
	}

	/* Enhanced exit animations */
	.hexagon-layer:nth-child(1) img {
		animation:
			float 2s ease-in-out infinite,
			pulse-glow 2s ease-in-out infinite;
	}

	.hexagon-layer:nth-child(2) img {
		animation: float 2s ease-in-out infinite;
		animation-delay: 0.2s;
	}

	.hexagon-layer:nth-child(3) img {
		animation: float 2s ease-in-out infinite;
		animation-delay: 0.4s;
	}

	/* Smooth transition when hiding */
	:global(.loading-exit) .animate-float {
		animation: float-out 0.6s ease-out forwards;
	}
</style>
