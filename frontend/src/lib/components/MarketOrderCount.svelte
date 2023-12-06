<script lang="ts">
	import type { MarketOrderCount } from "$lib/types";

    export let data: MarketOrderCount;
    let formatter = Intl.NumberFormat("en", { notation: "compact" })
</script>

<div class="stat">
    <div class="text-xl stat-title">Total Market Orders</div>
    <div class="stat-value text-primary">
        {formatter.format(data.count)}
    </div>
</div>