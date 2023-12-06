<script lang="ts">
	import type { MarketOrderCountByUpdatedAt } from "$lib/types";
	import { Chart, Colors } from "chart.js";
	import { onMount } from "svelte";

	export let data: MarketOrderCountByUpdatedAt[];
	let canvas: HTMLCanvasElement;
	let formatter = Intl.NumberFormat("en", { notation: "compact" });
	let chart: Chart<"doughnut"> | undefined;

	onMount(() => {
		Colors.defaults.forceOverride = true;


        new Chart(canvas, {
			type: "line",
			data: {
				labels: data.map(d => new Date(d.updated_at)),
				datasets: [
					{
						label: "Count",
						data: data.map(d => d.count),
					},
				],
			},
			options: {
				responsive: true,
				maintainAspectRatio: true,
				plugins: {
					legend: {
						display: false,
					},
					colors: Colors.defaults,
				},
				borderColor: "#1D232A",
				animation: false,
				scales: {
					x: {
						type: "time",
						time: {
							unit: "hour", // Adjust as needed (hour, month, year, etc.)
						},
						title: {
							display: true,
							text: "Date",
							
						},
					},
					y: {
						title: {
							display: true,
							text: "Count",
						},
					},
				},
			},
		});
	});
</script>

<div class="stat">
	<div class="text-xl stat-title">Market Orders by Date</div>
	<div class="stat-value">    
		<canvas bind:this={canvas}> </canvas>
	</div>
</div>
