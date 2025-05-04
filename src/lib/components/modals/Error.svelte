<script>
	import { exit } from '@tauri-apps/plugin-process';

	let { errorMessage = 'Lorem ipsum dolor sit amet consectetur adipisicing elit.' } = $props();

	const exitOk = async () => {
		await exit(0);
	};
</script>

<div class="absolute h-screen w-screen z-90 grid place-items-center bg-black/50 top-0">
	<div
		class=" bg-whiteTealinux border-2 border-greyBorder p-8 min-w-[450px] max-w-[700px] flex flex-col items-center rounded-lg"
	>
		<div class=" mb-6">
			<svg
				class="mx-auto text-red-400 w-16 h-16"
				aria-hidden="true"
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 20 20"
			>
				<path
					stroke="currentColor"
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M10 11V6m0 8h.01M19 10a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
				/>
			</svg>
		</div>
		<h1 class=" text-3xl font-poppin mb-2">Error</h1>
		<p class=" mb-4">{errorMessage}</p>

		<button onclick={exitOk} class=" py-2 px-4 bg-red-500 text-white font-poppinsemibold rounded-md w-fit mx-auto"
			>Close</button
		>
	</div>
</div>
