<script lang="ts">
	import type { MarketOrderCountByUpdatedAt } from "$lib/types";
	import { Chart } from "chart.js/auto";
	import 'chartjs-adapter-date-fns';
	import { onMount } from "svelte";

	export let data: MarketOrderCountByUpdatedAt[];
	let canvas: HTMLCanvasElement;

	onMount(() => {
		new Chart(canvas, {
			type: "line",
			data: {
				labels: data.map(
					(d) => new Date(d.updated_at),
				),
				datasets: [
					{
						data: data.map(
							(d) => d.count,
						),
						borderColor: "#7480ff",
						backgroundColor: "#7480ff"
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
				animation: false,
				scales: {
					x: {
						type: "time",
						time: {
							unit: "hour",
						},
						title: {
							display: true,
						},
                        grid: {
                            display: false
                        }
					},
					y: {
						title: {
							display: false,
						},
                        grid: {
                            display: false
                        }
					},
				},
			},
		});
	});
</script>

<canvas bind:this={canvas}></canvas>