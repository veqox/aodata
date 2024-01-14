<script lang="ts">
	import type { MarketOrderCountByUpdatedAtAndLocation } from "$lib/types";
	import { Chart } from "chart.js/auto";
	import "chartjs-adapter-date-fns";
	import { onMount } from "svelte";

	let data: MarketOrderCountByUpdatedAtAndLocation[];
	export let minAmount: number;
	let canvas: HTMLCanvasElement;

	onMount(async () => {
		/*let response = await fetch(
			"https://veqox.dedyn.io/api/statistics/orders?group_by=updated_at, location",
		);
		data = await response.json();*/

		let labels = new Array(...new Set(data.map((d) => d.location)));
		let dates = new Array(...new Set(data.map((d) => d.updated_at)));

		let datasets: {
			label: string;
			data: number[];
		}[] = [];

		labels.forEach((label, i) => {
			let orderCounts = data
				.filter((d) => d.location === label)
				.map((d) => d.count);

			if (orderCounts.reduce((sum, val) => (sum += val)) < minAmount) {
				return;
			}

			datasets.push({
				label,
				data: orderCounts,
			});
		});

		datasets.sort((a, b) => {
			return a.data.reduce((sum, val) => (sum += val)) <
				b.data.reduce((sum, val) => (sum += val))
				? 1
				: -1;
		});

		console.log(datasets);

		new Chart(canvas, {
			type: "line",
			data: {
				labels: dates,
				datasets,
			},
			options: {
				responsive: true,
				maintainAspectRatio: true,
				plugins: {
					legend: {
						display: true,
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
							display: false,
						},
					},
					y: {
						title: {
							display: false,
						},
						grid: {
							display: false,
						},
					},
				},
			},
		});
	});
</script>

<canvas bind:this={canvas}></canvas>
