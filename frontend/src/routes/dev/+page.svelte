<script lang="ts">
	import MarketOrderCountByUpdatedAtAndLocation from "$lib/components/MarketOrderCountByUpdatedAtAndLocation.svelte";
	import MarketOrderCountByCreatedAt from "$lib/components/MarketOrderCountByCreatedAt.svelte";
	import MarketOrderCountByUpdatedAt from "$lib/components/MarketOrderCountByUpdatedAt.svelte";
	import MarketOrderCountByLocation from "$lib/components/MarketOrderCountByLocation.svelte";
	import MarketOrderCountByCreatedAtAndLocation from "$lib/components/MarketOrderCountByCreatedAtAndLocation.svelte";
	import MarketOrderCount from "$lib/components/MarketOrderCount.svelte";
	import MarketOrderCountByAuctionTypeOffer from "$lib/components/MarketOrderCountByAuctionTypeOffer.svelte";
	import MarketOrderCountByAuctionTypeRequest from "$lib/components/MarketOrderCountByAuctionTypeRequest.svelte";
</script>

<div class="flex justify-center">
	<div class="w-11/12">
		<div class="grid grid-cols-1 justify-evenly md:grid-cols-6">
			<div class="col-span-2">
				<MarketOrderCount />
			</div>

			<div class="col-span-2">
				<MarketOrderCountByAuctionTypeOffer />
			</div>

			<div class="col-span-2">
				<MarketOrderCountByAuctionTypeRequest />
			</div>
		</div>

		<div class="col-span-6">
			<MarketOrderCountByCreatedAt />
		</div>

		<div class="col-span-6">
			<div class="shadow stat">
				<MarketOrderCountByCreatedAtAndLocation minAmount={10000} />
			</div>
		</div>

		<div class="col-span-3">
			<div class="shadow stat">
				<MarketOrderCountByUpdatedAt />
			</div>
		</div>

		<div class="col-span-3">
			<div class="shadow stat">
				<MarketOrderCountByUpdatedAtAndLocation minAmount={10000} />
			</div>
		</div>

		<div class="col-span-6">
			<div class="shadow stat">
				<MarketOrderCountByLocation minAmount={10000} />
			</div>
		</div>
	</div>
</div>
