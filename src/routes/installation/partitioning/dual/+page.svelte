<script>
	import Navigation from "$lib/components/Navigation.svelte";

    Navigation
</script>

<div>
    <h1 class="text-white">dual</h1>
</div>

<Navigation
	totalSteps={5}
	currentStep={4}
	currentTitle="Dual Boot"
	prevPath="/installation/partitioning"
	nextPath="/installation"
	nextAction={null}
/>
