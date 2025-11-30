<script lang="ts">
	import { cn } from '$lib/utils/cn';
	import type { BootPath } from '$types/installation/partitioning/partitioning.types';
	import type { HTMLAttributes } from 'svelte/elements';

	interface Props extends HTMLAttributes<HTMLElement> {
		selectedMethod: BootPath | null;
		method: BootPath | null;
		title: string;
		description: string;
		disable: boolean;
	}

	let { selectedMethod, title, description, disable, method, ...props }: Props = $props();
</script>

<div
	class={cn(
		'flex flex-col w-[190px] h-[167px] p-2.5 rounded-[14px]',
		'border-[1.3px] transition-all duration-100 bg-[#101010]',
		disable
			? ['cursor-not-allowed border-transparent grayscale opacity-70']
			: [
					'cursor-pointer border-[#3C6352]',
					selectedMethod === method
						? 'bg-[#151515] shadow-[0_0_9px_#26A768]'
						: 'hover:bg-[#151515] hover:border-[#3C6350]'
				]
	)}
	{...props}
>
	<div class="flex items-center gap-2">
		<!-- {@render icon()} -->
		{#if method === 'single'}
			<svg
				width="29"
				height="32"
				viewBox="0 0 29 32"
				fill="none"
				xmlns="http://www.w3.org/2000/svg"
			>
				<rect x="1" y="2.83325" width="21" height="23" rx="2" fill="#0D44E1" />
				<g filter="url(#filter0_df_2074_1684)">
					<rect
						x="7"
						y="7.83325"
						width="21"
						height="23"
						rx="2"
						fill="#849EEA"
						fill-opacity="0.7"
						shape-rendering="crispEdges"
					/>
				</g>
				<path d="M11 18.4047H25V21.2618H11V18.4047Z" fill="#D9D9D9" />
				<path d="M16.3333 25.8333V13.8333H19.6667V25.8333H16.3333Z" fill="#D9D9D9" />
				<defs>
					<filter
						id="filter0_df_2074_1684"
						x="0"
						y="0.833252"
						width="29"
						height="31"
						filterUnits="userSpaceOnUse"
						color-interpolation-filters="sRGB"
					>
						<feFlood flood-opacity="0" result="BackgroundImageFix" />
						<feColorMatrix
							in="SourceAlpha"
							type="matrix"
							values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
							result="hardAlpha"
						/>
						<feOffset dx="-3" dy="-3" />
						<feGaussianBlur stdDeviation="2" />
						<feComposite in2="hardAlpha" operator="out" />
						<feColorMatrix type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.25 0" />
						<feBlend mode="normal" in2="BackgroundImageFix" result="effect1_dropShadow_2074_1684" />
						<feBlend
							mode="normal"
							in="SourceGraphic"
							in2="effect1_dropShadow_2074_1684"
							result="shape"
						/>
						<feGaussianBlur stdDeviation="0.5" result="effect2_foregroundBlur_2074_1684" />
					</filter>
				</defs>
			</svg>
		{/if}
		{#if method === 'dual'}
			<svg
				width="27"
				height="29"
				viewBox="0 0 27 29"
				fill="none"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					d="M0.601904 23.0112C-0.168336 22.6603 -0.210421 20.9971 0.539728 20.5541L13.2008 13.0766C13.3935 12.9628 13.6065 12.9628 13.7992 13.0766L26.4603 20.5541C27.2104 20.9971 27.1683 22.6603 26.3981 23.0112L13.7371 28.7804C13.5823 28.8509 13.4177 28.8509 13.2629 28.7804L0.601904 23.0112Z"
					fill="#EF7E20"
				/>
				<path
					d="M0.601904 20.9849C-0.168336 20.6339 -0.210421 18.9707 0.539728 18.5277L13.2008 11.0503C13.3935 10.9365 13.6065 10.9365 13.7992 11.0503L26.4603 18.5277C27.2104 18.9707 27.1683 20.6339 26.3981 20.9849L13.7371 26.754C13.5823 26.8245 13.4177 26.8245 13.2629 26.754L0.601904 20.9849Z"
					fill="white"
					fill-opacity="0.31"
				/>
				<path
					d="M0.601904 18.9586C-0.168336 18.6076 -0.210421 16.9444 0.539728 16.5014L13.2008 9.02395C13.3935 8.91014 13.6065 8.91014 13.7992 9.02395L26.4603 16.5014C27.2104 16.9444 27.1683 18.6076 26.3981 18.9586L13.7371 24.7277C13.5823 24.7982 13.4177 24.7982 13.2629 24.7277L0.601904 18.9586Z"
					fill="#EF7E20"
				/>
				<path
					d="M0.601904 16.9322C-0.168336 16.5813 -0.210421 14.9181 0.539728 14.475L13.2008 6.99761C13.3935 6.88381 13.6065 6.88381 13.7992 6.99761L26.4603 14.475C27.2104 14.9181 27.1683 16.5813 26.3981 16.9322L13.7371 22.7014C13.5823 22.7719 13.4177 22.7719 13.2629 22.7014L0.601904 16.9322Z"
					fill="white"
					fill-opacity="0.31"
				/>
				<path
					d="M0.601904 14.9059C-0.168336 14.5549 -0.210421 12.8917 0.539728 12.4487L13.2008 4.97128C13.3935 4.85747 13.6065 4.85747 13.7992 4.97128L26.4603 12.4487C27.2104 12.8917 27.1683 14.5549 26.3981 14.9059L13.7371 20.675C13.5823 20.7455 13.4177 20.7455 13.2629 20.675L0.601904 14.9059Z"
					fill="#EF7E20"
				/>
				<path
					d="M0.601904 12.8796C-0.168336 12.5286 -0.210421 10.8654 0.539728 10.4224L13.2008 2.94494C13.3935 2.83114 13.6065 2.83114 13.7992 2.94494L26.4603 10.4224C27.2104 10.8654 27.1683 12.5286 26.3981 12.8796L13.7371 18.6487C13.5823 18.7192 13.4177 18.7192 13.2629 18.6487L0.601904 12.8796Z"
					fill="white"
					fill-opacity="0.31"
				/>
				<path
					d="M0.601904 10.8532C-0.168336 10.5023 -0.210421 8.83907 0.539728 8.39604L13.2008 0.918608C13.3935 0.8048 13.6065 0.8048 13.7992 0.918608L26.4603 8.39604C27.2104 8.83907 27.1683 10.5023 26.3981 10.8532L13.7371 16.6224C13.5823 16.6929 13.4177 16.6929 13.2629 16.6224L0.601904 10.8532Z"
					fill="#EF7E20"
				/>
				<rect x="12" y="14.8333" width="15" height="14" rx="7" fill="#D9D9D9" fill-opacity="0.7" />
				<path
					d="M23.1148 20.119C22.808 19.4722 22.3333 18.9197 21.7401 18.5189C21.147 18.1182 20.4571 17.8839 19.7426 17.8406C19.028 17.7973 18.3149 17.9465 17.6777 18.2726C17.0404 18.5987 16.5024 19.0899 16.1197 19.6948C15.737 20.2997 15.5236 20.9963 15.5018 21.7117C15.4801 22.4272 15.6508 23.1354 15.996 23.7624C16.3413 24.3895 16.8485 24.9124 17.4647 25.2766C18.081 25.6409 18.7837 25.8331 19.4996 25.8333C21.3112 25.8333 23.0085 24.6275 23.5 22.9761M23.1148 20.119V18.041M23.1148 20.119L20.9551 20.3787"
					stroke="white"
					stroke-width="0.9"
					stroke-linecap="round"
					stroke-linejoin="round"
				/>
			</svg>
		{/if}
		{#if method === 'manual'}
			<svg
				width="27"
				height="29"
				viewBox="0 0 27 29"
				fill="none"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					d="M18.383 10.0605C18.383 15.1566 15.168 19.2878 11.2021 19.2878C7.23625 19.2878 4.02128 15.1566 4.02128 10.0605C4.02128 4.96444 7.23625 0.833252 11.2021 0.833252C15.168 0.833252 18.383 4.96444 18.383 10.0605Z"
					fill="#7A14EF"
				/>
				<path
					d="M27 12.606C27 17.7021 23.2706 21.8333 18.6702 21.8333C14.0698 21.8333 10.3404 17.7021 10.3404 12.606C10.3404 7.5099 14.0698 3.37871 18.6702 3.37871C23.2706 3.37871 27 7.5099 27 12.606Z"
					fill="#7A14EF"
				/>
				<path
					d="M16.6596 13.5605C16.6596 18.1294 12.9302 21.8333 8.32979 21.8333C3.72937 21.8333 0 18.1294 0 13.5605C0 8.99162 3.72937 5.2878 8.32979 5.2878C12.9302 5.2878 16.6596 8.99162 16.6596 13.5605Z"
					fill="#7A14EF"
				/>
				<rect
					x="12"
					y="13.8333"
					width="15"
					height="15"
					rx="7.5"
					fill="#D9D9D9"
					fill-opacity="0.7"
				/>
				<path d="M19 15.8333H20.2V21.8333H19V15.8333Z" fill="white" />
				<path d="M19 21.8333V20.6333H25V21.8333H19Z" fill="white" />
			</svg>
		{/if}

		<h3 class="text-white font-jakarta text-[15px] font-semibold tracking-[-0.48px]">
			{title}
		</h3>
	</div>
	<p class="text-white mt-2 font-jakarta text-[12px] font-extralight tracking-[-0.48px]">
		{description}
	</p>
</div>
