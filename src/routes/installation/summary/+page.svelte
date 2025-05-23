<script>
	import TwoSide from '$lib/components/layouts/TwoSide.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { getRead, getBlueprint } from '../global.js';
	import prettyBytes from 'pretty-bytes';
	import { randomColor } from 'randomcolor';
	import Navigation from '../../../lib/components/Navigation.svelte';
	import GlowingText from '../../../lib/components/ui/GlowingText.svelte';

	let timezone;
	let mainLocale;
	let locales;
	let formattedPartitions;
	let assignedPartitions;
	let passwordVisible = $state(false);
    let blueprint = $state(null);

    const getBlueprintJSON = async () => {
        let blueprint = await getBlueprint();
        return blueprint;
    }

	const getDisk = async () => {
		let blueprint = await getBlueprint();
		return blueprint.storage.partitions;
	};

	const getDiskSize = async () => {
		let disk = await getDisk();
		let size = 0;

		for (let i of disk.keys()) {
			size += disk[i].size;
		}

		return size;
	};

	const getStorageJSON = async () => {
		let json = await getRead();
		json = json.disk.filter((disk) => disk.partitions !== null);

		return json;
	};

	const setSummary = async () => {
		let json = await getBlueprint();
		console.log(json);

		timezone = json.timezone.region + '/' + json.timezone.city;
		mainLocale = json.locale.main;

		let partitions = json.disk.filter(
			(partition) => partition.format !== false || partition.mountpoint !== null
		);

		formattedPartitions = partitions.filter((partition) => partition.format !== null);
		assignedPartitions = partitions.filter((partition) => partition.mountpoint !== null);
	};

	const getColors = (disk) => {
		let length = disk.length;

		let colors = [];

		for (let i = 0; i < length; i++) {
			colors.push(
				randomColor({
					luminosity: 'bright',
					hue: 'random'
				})
			);
		}
		return colors;
	};

	function togglePasswordVisibility() {
		passwordVisible = !passwordVisible;
	}

	const printJson = async () => {
		await invoke('print_json');
	};

    onMount(async () => {
        blueprint = await getBlueprintJSON();
    })
</script>

<TwoSide>
	{#snippet left()}
		<div class="w-[288px] space-y-[15px]">
			<div class="flex space-x-[14px]">
				<h1 class="font-archivo font-[600] text-[30px] tracking-[-1.8px]">Review your choices</h1>
			</div>
			<p class="font-jakarta text-sm font-[200] tracking-[-0.56px] text-center">
				Review your choices carefully to ensure everything 
				is ready before proceeding with the installation.
			</p>
		</div>
	{/snippet}

	{#snippet right()}
			{#await getBlueprint() then blueprint}
				{@const keyboard =
					blueprint.keyboard === null
						? 'To be filled'
						: blueprint.keyboard.layout + ' - ' + blueprint.keyboard.variant}
				{@const timezoneRegion =
					blueprint.timezone === null ? 'To be filled' : blueprint.timezone.region}
				{@const timezoneCity = blueprint.timezone === null ? 'To be filled' : blueprint.timezone.city}
				{@const locale = blueprint.locale === null ? 'To be filled' : blueprint.locale.main}
				{@const userFullname = blueprint.account === null ? 'To be filled' : blueprint.account.fullname}
				{@const userUsername = blueprint.account === null ? 'To be filled' : blueprint.account.username}
				{@const userHostname = blueprint.account === null ? 'To be filled' : blueprint.account.hostname}
				{@const userPassword = blueprint.account === null ? 'To be filled' : blueprint.account.password}

				<div class="flex space-x-5 mb-[10px]">
					<div class="w-1/2 bg-[#101010] border-[1.3px] border-[#3C6350] p-[15px] rounded-[14px] space-y-5">
						<GlowingText size="[15]" text="User account" />
						<div class="space-y-4 text-[15px]">
							<div class="leading-none space-y-[10px]">
								<p class="font-[500]">Full name</p>
								<span class="ml-[4px] font-poppin text-gray-500 text-[14px]">{userFullname}</span>
							</div>

							<div class="leading-none space-y-[10px]">
								<p class="font-[500]">Computer name</p>
								<span class="ml-[4px] font-poppin text-gray-500 text-[14px]">{userHostname}</span>
							</div>

							<div class="leading-none space-y-[10px]">
								<p class="font-[500]">Username</p>
								<span class="ml-[4px] font-poppin text-gray-500 text-[14px]">{userUsername}</span>
							</div>
						</div>
					</div>
					
					<!-- Localization -->
					<div class="w-1/2 bg-[#101010] border-[1.3px] border-[#3C6350] p-[15px] rounded-[14px] space-y-5">
						<GlowingText size="[15]" text="Localization" />
						<div class="space-y-4 text-[15px]">
							<div class="leading-none space-y-[10px]">
								<p class="font-[500]">Locale</p>
								<span class="ml-[4px] font-poppin text-gray-500 text-[14px]">{locale}</span>
							</div>
							<div class="leading-none space-y-[10px]">
								<p class="font-[500]">Time Zone</p>
								<span class="ml-[4px] font-poppin text-gray-500 text-[14px]">{timezoneRegion}/{timezoneCity}</span>
							</div>
							<div class="leading-none space-y-[10px]">
								<p class="font-[500]">Keyboard</p>
								<span class="ml-[12px] font-poppin text-gray-500 text-[14px]">{keyboard}</span>
							</div>
						</div>
					</div>
				</div>
							

				
			{/await}

		
	{/snippet}
</TwoSide>

<Navigation
	currentStep={5}
	currentTitle="Summary"
	prevPath="/installation"
	nextPath="/installation/localization"
	nextAction={null}
/> 