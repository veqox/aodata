<script lang="ts">
	import type { MarketOrderCountByLocation } from "$lib/types";
	import ChartDataLabels from "chartjs-plugin-datalabels";
	import { Chart, Colors } from "chart.js/auto";
	import { onMount } from "svelte";

	export let data: MarketOrderCountByLocation[];
	let canvas: HTMLCanvasElement;

	onMount(() => {
		new Chart(canvas, {
			type: "doughnut",
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
			plugins: [ChartDataLabels],
			options: {
				responsive: true,
				maintainAspectRatio: true,
				plugins: {
					legend: {
						display: false,
					},
					datalabels: {
						display: true,
						font: {
							size: 15
						},
						formatter: (val, ctx) => {
							return `${ctx.chart.data.labels![ctx.dataIndex]}: ${Intl.NumberFormat("en", { notation: "compact" }).format(val)}`
						}
					}
				},
				borderColor: "#1D232A",
				backgroundColor: "#7480ff",
				animation: false,
			},
		});
	});
</script>

<canvas bind:this={canvas}> </canvas>
