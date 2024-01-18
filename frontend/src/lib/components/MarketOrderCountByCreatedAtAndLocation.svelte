<script lang="ts">
	import { get_backend_url } from "$lib/env";
	import type { MarketOrderCountByCreatedAtAndLocation } from "$lib/types";
	import { Chart } from "chart.js/auto";
	import "chartjs-adapter-date-fns";
	import { onMount } from "svelte";

	let data: MarketOrderCountByCreatedAtAndLocation[];
	export let minAmount: number;
	let canvas: HTMLCanvasElement;

	onMount(async () => {
		let response = await fetch(
			`${get_backend_url()}/statistics/orders?group_by=created_at, location`,
		);
		data = await response.json();

		let colors = [
			"#7480ff",
			"#6873e5",
			"#5c66cc",
			"#5159b2",
			"#454c99",
			"#3a407f",
			"#2e3366",
			"#292d5b",
		];

		let labels = new Array(...new Set(data.map((d) => d.location)));
		let dates = new Array(...new Set(data.map((d) => d.created_at)));

		let datasets: {
			label: string;
			data: number[];
			backgroundColor: string;
			borderColor: string;
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
				backgroundColor: colors[i % colors.length],
				borderColor: colors[i % colors.length],
			});
		});

		datasets.sort((a, b) => {
			return a.data.reduce((sum, val) => (sum += val)) <
				b.data.reduce((sum, val) => (sum += val))
				? 1
				: -1;
		});

		datasets.forEach((dataset, i) => {
			dataset.backgroundColor = colors[i % colors.length];
			dataset.borderColor = colors[i % colors.length];
		});

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
