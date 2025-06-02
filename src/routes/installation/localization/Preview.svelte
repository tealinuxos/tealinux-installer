<script>
    import { getCurrency } from 'locale-currency';

    let {
        selectedLocale,
        selectedRegion = "Asia",
        selectedCity = "Jakarta"
    } = $props();

    let selectedTimezone = $state(null);

    let date = $state(new Date());
	let dateOptions = {
		weekday: 'short',
		year: 'numeric',
		month: 'short',
		day: 'numeric'
	};
	let number = 1234567.89;
	let price = 1234.56;

	let timePreview = $state('');
	let datePreview = $state('');
	let numberPreview = $state('');
	let currencyPreview = $state('');

    const getShortLocale = (locale) => {

        let pattern = /([a-z]+)_([A-Z]+)/;
        let matched = locale.match(pattern);

        let shortLocale = matched[1] + '-' + matched[2];

        return shortLocale;
    }

	const handlePreview = () => {
		if (selectedLocale != null) {
			let short = getShortLocale(selectedLocale);

			// timePreview = date.toLocaleDateString(short, dateOptions);
			numberPreview = number.toLocaleString(short);

			let currencyCode = getCurrency(short);

			let currency = new Intl.NumberFormat(short, {
				style: 'currency',
				currency: currencyCode
			});

			currencyPreview = currency.format(price);
        }

        if (selectedTimezone !== null) {
			let timeFormat = new Intl.DateTimeFormat([], {
				timeZone: selectedTimezone,
				hour: 'numeric',
				minute: 'numeric',
				second: 'numeric',
				hour12: false
			});
			let dateFormat = new Intl.DateTimeFormat([], {
				timeZone: selectedTimezone,
				year: 'numeric',
				month: 'long',
				day: 'numeric'
			});
			datePreview = dateFormat.format(date);
			timePreview = timeFormat.format(date);
		}
	};

    setInterval(() => {
        date = new Date();
    }, 1000);

	$effect(() => {
		selectedLocale, handlePreview();
        selectedTimezone = selectedRegion
            ? selectedCity
                ? `${selectedRegion}/${selectedCity}`
                : null
            : null;
	});
</script>

<div class="flex flex-col gap-y-[10px]">
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
