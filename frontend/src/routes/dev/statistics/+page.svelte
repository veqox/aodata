<script lang="ts">
	import { onMount } from "svelte";
	import type { PageServerData } from "./$types";
	import Chart from "chart.js/auto";
	import "chartjs-adapter-date-fns";
	import MarketOrderCount from "$lib/components/MarketOrderCount.svelte";
	import MarketOrderCountByLocation from "$lib/components/MarketOrderCountByLocation.svelte";
	import MarketOrderCountByAuctionType from "$lib/components/MarketOrderCountByAuctionType.svelte"
	import MarketOrderCountByUpdatedAt from "$lib/components/MarketOrderCountByUpdatedAt.svelte";

	export let data: PageServerData;
</script>

<div class="stats stats-vertical">
	<MarketOrderCount data={data.props.data.market_order_count} />
	<MarketOrderCountByLocation
		data={data.props.data.market_order_count_by_location.filter(
			(i) => i.count > 50000,
		)}
	/>
	<MarketOrderCountByAuctionType data={data.props.data.market_order_count_by_auction_type} />
	<MarketOrderCountByUpdatedAt data={data.props.data.market_order_count_by_updated_at} />
</div>
