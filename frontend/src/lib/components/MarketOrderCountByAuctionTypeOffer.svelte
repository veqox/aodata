<script lang="ts">
	import type { MarketOrderCountByAuctionType } from "$lib/types";
	import { onMount } from "svelte";
    import { compact_number_formatter, standard_number_formatter } from '$lib/utils/formatters';

    let data: MarketOrderCountByAuctionType | undefined;

    const get_data = async () => {
        let response = await fetch(
            `https://veqox.dedyn.io/api/statistics/orders/count?auction_type=offer`,
        );

        return response.json();
    }

    onMount(async() => {
        data = await get_data();
    })
</script>

<div class="shadow stat">
    <div class="stat-figure text-primary">
        <svg
            xmlns="http://www.w3.org/2000/svg"
            height="40"
            viewBox="0 -960 960 960"
            width="40"
            class="fill-primary"
            ><path
                d="M830.615-521.077v318.769q0 30.308-21 51.307-21 21-51.308 21H202.923q-30.307 0-51.307-21-21-20.999-21-51.307v-319.538q-24.154-19.847-36.27-51.5-12.115-31.654-.5-68.346l40.462-132.154q8-25.23 27.154-40.692 19.154-15.461 45.769-15.461H753.23q26.616 0 45.462 14.769 18.846 14.769 27.461 40.615l41.231 132.923q11.615 36.692-.5 68.115-12.115 31.423-36.269 52.5Zm-262-28.922q32.77 0 49.27-20.039 16.5-20.038 13.5-43.038l-24.307-156.925h-96.463V-612q0 25.231 17.077 43.616 17.077 18.385 40.923 18.385Zm-180 0q27.616 0 44.809-18.385 17.192-18.385 17.192-43.616v-158.001h-96.463l-24.308 158.463q-3.23 21.308 13.385 41.423 16.616 20.116 45.385 20.116Zm-178 0q22.231 0 38.232-15.5 16-15.501 19.769-38.962l23.539-165.54h-84.924q-6.539 0-10.385 2.885-3.847 2.885-5.77 8.655l-38.461 130.153q-7.924 25.77 7.461 52.039 15.385 26.27 50.539 26.27Zm540 0q32.462 0 49.693-25.5 17.231-25.501 8.308-52.809l-40.461-130.923q-1.924-5.769-5.77-8.27-3.846-2.5-10.385-2.5h-82.924l23.539 165.54q3.769 23.461 19.769 38.962 16 15.5 38.231 15.5Zm-547.692 360h555.384q5.386 0 8.847-3.462 3.462-3.462 3.462-8.847v-291.231q-6.538 2.384-10.923 2.961t-9.078.577q-27 0-47.5-9.769t-39.73-31.308q-16.846 18.769-39.846 29.923-23 11.154-52.462 11.154-25.462 0-48-10.577t-42.462-30.5q-18.538 19.923-42 30.5-23.461 10.577-47.538 10.577-27.077 0-50.769-9.807-23.693-9.808-41.693-31.27-25.231 25.231-46.5 33.154-21.269 7.923-41.5 7.923-4.692 0-9.692-.577-5.001-.577-10.308-2.961v291.231q0 5.385 3.461 8.847 3.462 3.462 8.847 3.462Zm555.384 0H202.923h555.384Z"
            /></svg
        >
    </div>
    <div class="stat-title">Sell Orders</div>
    <div class="stat-value text-primary {data ? "" : "invisible"}">
        {compact_number_formatter.format(
            data?.count ?? 0,
        )}
    </div>
    <div class="stat-desc {data ? "" : "invisible"}">
        {standard_number_formatter.format(
            data?.count ?? 0,
        )}

        Sell Orders
    </div>
</div>