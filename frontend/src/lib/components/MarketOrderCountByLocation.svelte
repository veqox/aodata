<script lang="ts">
	import type { MarketOrderCountByLocation } from "$lib/types";
	import { Chart, Colors } from "chart.js/auto";
	import { onMount } from "svelte";

	export let data: MarketOrderCountByLocation[];
	let canvas: HTMLCanvasElement;

	onMount(() => {
		new Chart(canvas, {
			type: "bar",
			data: {
				labels: data.map((d) => d.location),
				datasets: [
					{
						data: data.map((d) => d.count),
						borderRadius: 7,
						backgroundColor: [
							"#7480ff",
							"#6873e5",
							"#5c66cc",
							"#5159b2",
							"#454c99",
							"#3a407f",
							"#2e3366",
							"#292d5b",
						],
						
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
				},
				borderColor: "#1D232A",
				backgroundColor: "#7480ff",
				animation: false,
			},
		});
	});
</script>

<canvas bind:this={canvas}> </canvas>
