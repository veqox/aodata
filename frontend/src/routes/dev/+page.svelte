<script lang="ts">
	import type { PageServerData } from "./$types";

	import MarketOrderCount from "$lib/components/MarketOrderCount.svelte";
	import MarketOrderCountByLocation from "$lib/components/MarketOrderCountByLocation.svelte";
	import MarketOrderCountByAuctionType from "$lib/components/MarketOrderCountByAuctionType.svelte";
	import MarketOrderCountByCreatedAt from "$lib/components/MarketOrderCountByCreatedAt.svelte";
	import MarketOrderCountByUpdatedAt from "$lib/components/MarketOrderCountByUpdatedAt.svelte";

	export let data: PageServerData;
</script>

<div class="flex justify-center">
	<div class="w-11/12">
		<div class="grid grid-cols-1 justify-evenly md:grid-cols-3">
			<MarketOrderCount data={data.props.data.market_order_count} />

			{#each data.props.data.market_order_count_by_auction_type as { auction_type, count }}
				<MarketOrderCountByAuctionType
					data={{ auction_type, count }}
					type={auction_type}
				/>
			{/each}
		</div>

		<div class="col-span-3 shadow stat">
			<div class="stat-title">Market Orders By Creation Date</div>
			<div class="stat-value">
				<MarketOrderCountByCreatedAt
					data={data.props.data.market_order_count_by_created_at}
				/>
			</div>
		</div>
		<div class="col-span-3 shadow stat">
			<div class="stat-title">Market Orders By Last Update</div>
			<div class="stat-value">
				<MarketOrderCountByUpdatedAt
					data={data.props.data.market_order_count_by_updated_at}
				/>	
			</div>
		</div>

		<div class="col-span-3 shadow stat">
			<div class="stat-title">Market Orders By Location</div>
			<div class="stat-value">
				<MarketOrderCountByLocation
				data={data.props.data.market_order_count_by_location}
			/>
			</div>
		</div>
	</div>
</div>
