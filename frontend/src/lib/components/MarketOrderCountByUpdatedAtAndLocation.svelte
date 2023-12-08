<script lang="ts">
	import type { MarketOrderCountByUpdatedAtAndLocation } from "$lib/types";
	import { Chart } from "chart.js/auto";
	import "chartjs-adapter-date-fns";
	import { onMount } from "svelte";

	export let data: MarketOrderCountByUpdatedAtAndLocation[];
	let canvas: HTMLCanvasElement;

	onMount(() => {
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
		let dates = new Array(...new Set(data.map((d) => d.updated_at)));

		let datasets: { label: string; data: number[], backgroundColor: string, borderColor: string }[] = [];

		labels.forEach((label, i) => {
			let dataset = datasets.find((d) => d.label === label) ?? {
				label,
				data: [],
				backgroundColor: colors[i % colors.length],
				borderColor: colors[i % colors.length],
			};

			data.filter((d) => d.location === label).forEach((d) =>
				dataset.data.push(d.count),
			);
			datasets.push(dataset);
		});


		for (let label of labels) 

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
