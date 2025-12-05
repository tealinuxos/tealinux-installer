<script lang="ts">
	import { partitionMethod } from '$lib/stores/informationStore';
	import { goto } from '$app/navigation';
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import { showModal, closeModal } from '$lib/stores/modalStore.js';
	import GlowingText from '$lib/components/ui/GlowingText.svelte';
	import Navigation from '$lib/components/Navigation.svelte';
	import { getBlueprintInfo } from '$lib/utils/read_utils.js';
	import { resolve } from '$app/paths';

	const partitioningMethod = $partitionMethod;

	const navigateToUserAccount = () => {
		goto(resolve('/installation/account'));
	};

	const navigateToLocalization = () => {
		goto(resolve('/installation/localization'));
	};

	const showInstallWarning = async () => {
		showModal({
			isOpen: false,
			type: 'error',
			title: 'All Data Will be Wiped',
			content:
				'Proceeding with this installation will erase all existing data on the selected drive. This action cannot be undone.',
			confirmText: 'OK',
			cancelText: 'Cancel',
			showCancel: true,
			countdown: 5,
			onConfirm: () => goto(resolve('/installation/install')),
			onCancel: () => closeModal()
		});
	};
</script>

<TwoSide>
	{#snippet left()}
		<div class="mx-[35px] space-y-[15px]">
			<h1 class="font-archivo font-semibold text-[28px]">
				Review your <span class="text-green-tealinux">choices</span><br />
			</h1>
			<p class="font-jakarta text-sm font-extralight">
				Review your choices carefully to ensure everything is ready before proceeding with the
				installation.
			</p>
		</div>
	{/snippet}

	{#snippet right()}
		{#await getBlueprintInfo() then blueprint}
			{@const keyboard = blueprint?.keyboard
				? `${blueprint.keyboard.layout} - ${blueprint.keyboard.variant}`
				: 'To be filled'}
			{@const timezoneRegion = blueprint?.timezone?.region ?? 'To be filled'}
			{@const timezoneCity = blueprint?.timezone?.city ?? 'To be filled'}
			{@const locale = blueprint?.locale?.main ?? 'To be filled'}
			{@const userFullname = blueprint?.account?.fullname ?? 'To be filled'}
			{@const userUsername = blueprint?.account?.username ?? 'To be filled'}
			{@const userHostname = blueprint?.account?.hostname ?? 'To be filled'}

			<div class="flex flex-col space-y-2">
				<div class="flex space-x-2">
					<div
						class="w-1/2 bg-[#101010] border-[1.3px] border-[#3C6350] p-[15px] rounded-[14px] flex flex-col justify-between"
					>
						<div>
							<GlowingText size="[15]" text="User account" />
							<div class="space-y-4 text-[15px] mt-4">
								<div class="leading-none space-y-2.5">
									<p class="font-medium">Full name</p>
									<span class="ml-1 font-poppin text-gray-500 text-[14px]">{userFullname}</span>
								</div>

								<div class="leading-none space-y-2.5">
									<p class="font-medium">Computer name</p>
									<span class="ml-1 font-poppin text-gray-500 text-[14px]">{userHostname}</span>
								</div>

								<div class="leading-none space-y-2.5">
									<p class="font-medium">Username</p>
									<span class="ml-1 font-poppin text-gray-500 text-[14px]">{userUsername}</span>
								</div>
							</div>
						</div>
						<div class="flex justify-end mt-4">
							<button
								onclick={navigateToUserAccount}
								class="flex h-8 px-[9px] items-center justify-center gap-2.5 rounded-sm border-[0.3px] border-[#3C6350] bg-[#101010] text-white font-['Poppins'] text-[14px] transition-all duration-200 hover:shadow-[0_0_9px_#00B85E] active:shadow-[0_0_9px_#00B85E] disabled:opacity-50 disabled:hover:shadow-none"
							>
								Edit
							</button>
						</div>
					</div>

					<div
						class="w-1/2 bg-[#101010] border-[1.3px] border-[#3C6350] p-[15px] rounded-[14px] flex flex-col justify-between"
					>
						<div>
							<GlowingText size="[15]" text="Localization" />
							<div class="space-y-4 text-[15px] mt-4">
								<div class="leading-none space-y-2.5">
									<p class="font-medium">Locale</p>
									<span class="ml-1 font-poppin text-gray-500 text-[14px]">{locale}</span>
								</div>
								<div class="leading-none space-y-2.5">
									<p class="font-medium">Time Zone</p>
									<span class="ml-1 font-poppin text-gray-500 text-[14px]"
										>{timezoneRegion}/{timezoneCity}</span
									>
								</div>
								<div class="leading-none space-y-2.5">
									<p class="font-medium">Keyboard</p>
									<span class="ml-1 font-poppin text-gray-500 text-[14px]">{keyboard}</span>
								</div>
							</div>
						</div>
						<div class="flex justify-end mt-4">
							<button
								onclick={navigateToLocalization}
								class="flex h-8 px-[9px] items-center justify-center gap-2.5 rounded-sm border-[0.3px] border-[#3C6350] bg-[#101010] text-white font-['Poppins'] text-[14px] transition-all duration-200 hover:shadow-[0_0_9px_#00B85E] active:shadow-[0_0_9px_#00B85E] disabled:opacity-50 disabled:hover:shadow-none"
							>
								Edit
							</button>
						</div>
					</div>
				</div>

				<div class="bg-[#101010] border-[1.3px] border-[#3C6350] p-[15px] rounded-[14px]">
					<GlowingText size="[15]" text="Partitions" />
					<div class="mt-4 overflow-x-auto h-[200px]">
						<table class="w-full">
							<thead class="text-[#FFFEFB] font-['Poppins'] text-[14px]">
								<tr class="border-b border-[#3C6350]">
									<th class="p-3 text-left">Partition</th>
									<th class="p-3 text-left">File system</th>
									<th class="p-3 text-left">Used as</th>
									<th class="p-3 text-left">Format</th>
								</tr>
							</thead>
							<tbody class="text-[#FFFEFB] font-['Poppins'] text-[14px]">
								{#each blueprint?.storage?.partitions as partition (partition.path)}
									<tr class="border-b border-[#3C6350]">
										<td class="p-3">{partition.path || 'Unallocated'}</td>
										<td class="p-3">{partition.filesystem || '-'}</td>
										<td class="p-3"
											>{partition.filesystem === 'swap'
												? '[SWAP]'
												: partition.mountpoint || '-'}</td
										>
										<td class="p-3">{partition.format ? 'Yes' : 'No'}</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
					<div class="flex justify-end mt-4">
						<button
							onclick={() => goto(resolve(`/installation/partitioning/${partitioningMethod}`))}
							class="flex h-8 px-[9px] items-center justify-center gap-2.5 rounded-sm border-[0.3px] border-[#3C6350] bg-[#101010] text-white font-['Poppins'] text-[14px] transition-all duration-200 hover:shadow-[0_0_9px_#00B85E] active:shadow-[0_0_9px_#00B85E] disabled:opacity-50 disabled:hover:shadow-none"
						>
							Edit Storage
						</button>
					</div>
				</div>
			</div>
		{/await}
	{/snippet}
</TwoSide>

<Navigation
	currentStep={5}
	currentTitle="Summary"
	prevPath="/installation/account"
	nextPath="/installation/summary"
	nextAction={showInstallWarning}
/>
