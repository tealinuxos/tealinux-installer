<script>
	import Navigation from "$lib/components/Navigation.svelte";

    Navigation
</script>

<div>
    <h1 class="text-white">manual</h1>
</div>

<Navigation
	totalSteps={5}
	currentStep={4}
	currentTitle="Manual Partitioning"
	prevPath="/installation/partitioning"
	nextPath="/installation"
	nextAction={null}
/>
