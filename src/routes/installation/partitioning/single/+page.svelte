<script>
	import Navigation from "$lib/components/Navigation.svelte";

    Navigation
</script>

<div>
    <h1 class="text-white">single</h1>
</div>

<Navigation
	totalSteps={5}
	currentStep={4}
	currentTitle="Single Boot"
	prevPath="/installation/partitioning"
	nextPath="/installation"
	nextAction={null}
/>
