<script lang="ts">
	import { onMount } from 'svelte';
	import { getCurrency } from 'locale-currency';

	interface Props {
		selectedLocale: string | null;
		selectedRegion: string | null;
		selectedCity: string | null;
	}

	let { selectedLocale, selectedRegion = 'Asia', selectedCity = 'Jakarta' }: Props = $props();

	let date = $state(new Date());

	const demoNumber = 1234567.89;
	const demoPrice = 1234.56;

	let safeLocale = $derived.by(() => {
		if (!selectedLocale) return 'en-US';

		const pattern = /([a-z]{2})_([A-Z]{2})/;
		const matched = selectedLocale.match(pattern);

		if (matched) {
			return `${matched[1]}-${matched[2]}`;
		}

		return selectedLocale.substring(0, 5).replace('_', '-');
	});

	let selectedTimezone = $derived.by(() => {
		if (selectedRegion && selectedCity) {
			return `${selectedRegion}/${selectedCity}`;
		}
		return null;
	});

	let numberPreview = $derived.by(() => {
		try {
			return demoNumber.toLocaleString(safeLocale);
		} catch (e) {
			console.log(e);
			return demoNumber.toLocaleString();
		}
	});

	let currencyPreview = $derived.by(() => {
		try {
			const currencyCode = getCurrency(safeLocale) || 'USD';
			const formatter = new Intl.NumberFormat(safeLocale, {
				style: 'currency',
				currency: currencyCode
			});
			return formatter.format(demoPrice);
		} catch (e) {
			console.log(e);
			return `$${demoPrice}`;
		}
	});

	let datePreview = $derived.by(() => {
		try {
			const dateFormat = new Intl.DateTimeFormat(safeLocale, {
				timeZone: selectedTimezone || undefined,
				year: 'numeric',
				month: 'long',
				day: 'numeric',
				weekday: 'short'
			});
			return dateFormat.format(date);
		} catch (e) {
			console.log(e);
			return date.toDateString();
		}
	});

	let timePreview = $derived.by(() => {
		try {
			const timeFormat = new Intl.DateTimeFormat(safeLocale, {
				timeZone: selectedTimezone || undefined,
				hour: 'numeric',
				minute: 'numeric',
				second: 'numeric',
				hour12: false
			});
			return timeFormat.format(date);
		} catch (e) {
			console.log(e);
			return date.toLocaleTimeString();
		}
	});

	onMount(() => {
		const interval = setInterval(() => {
			date = new Date();
		}, 1000);
		return () => clearInterval(interval);
	});
</script>

<div class="flex flex-col gap-y-2.5">
	<!-- preview item -->
	<div class="flex gap-x-4">
		<img src="/icons/clock-vector.svg" alt="clock" />
		<span>{timePreview}</span>
	</div>
	<!-- preview item -->
	<div class="flex gap-x-4">
		<img src="/icons/calendar-vector.svg" alt="clock" />
		<span>{datePreview}</span>
	</div>
	<!-- preview item -->
	<div class="flex gap-x-4">
		<img src="/icons/currency-vector.svg" alt="clock" />
		<span>{numberPreview} - {currencyPreview}</span>
	</div>
</div>
