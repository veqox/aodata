<script lang="ts">
	import type { MarketOrderCountByAuctionType } from "$lib/types";
	import { Chart, Colors } from "chart.js";
	import { onMount } from "svelte";

	export let data: MarketOrderCountByAuctionType[];
	let canvas: HTMLCanvasElement;
	let formatter = Intl.NumberFormat("en", { notation: "compact" });
	let chart: Chart<"doughnut"> | undefined;

	onMount(() => {
		Colors.defaults.forceOverride = true;

		chart = new Chart(canvas, {
			type: "doughnut",
            options: {
				responsive: true,
				maintainAspectRatio: false,
				plugins: {
					legend: {
						display: false,
					},
					colors: Colors.defaults,
				},
				borderColor: "#1D232A",
				animation: false,
			},
			data: {
				labels: data.map(
					(d) => d.auction_type,
				),
				datasets: [
					{
						data: data.map((d) => d.count),
                        borderRadius: 7,
					},
				],
			},
		});
	});
</script>

<div class="stat">
	<div class="text-xl stat-title">Market Orders by Auction Type</div>
	<div class="hidden stat-figure md:block">
		<canvas bind:this={canvas}> </canvas>
	</div>
	<div class="stat-value">
		<ul>
			{#each data as { auction_type, count }, i}
				<li style="color: {chart?.data.datasets[0].backgroundColor[i]}">
					{auction_type}
					{formatter.format(count)}
				</li>
			{/each}
		</ul>
	</div>
</div>
