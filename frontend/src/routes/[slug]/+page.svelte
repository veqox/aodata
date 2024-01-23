<script lang="ts">
	import type { LocalizedNames, MarketOrder, Location } from "$lib/types";
	import { onMount } from "svelte";
	import type { PageData } from "./$types";

	let locations: Location[] = [];
	let orders: MarketOrder[] = [];
	let page = 0;
	let page_size = 20;

    export let data: PageData;

	let item =  data.props.slug
	let number_formatter = new Intl.NumberFormat("en-US", {
		notation: "standard",
	});
	let time_formatter = new Intl.RelativeTimeFormat("en-US", {
		style: "narrow",
	});

	const get_locations = async () => {
		const res = await fetch(
			`https://veqox.dedyn.io/api/locations?min_market_orders=1`,
		);
		const json = await res.json();
		return json;
	};

	const get_prices = async () => {
		let yesterday = new Date();
		yesterday.setDate(yesterday.getDate() - 1);
		
		let from_string = yesterday.toISOString().split('T')[0];
		
		const res = await fetch(
			`https://veqox.dedyn.io/api/items/${item}/orders?auction_type=offer&limit=${page_size}&offset=${
				page * page_size
			}&from=${from_string}`,
		);

		const json = await res.json() as MarketOrder[];

        if (!json) return [];

        return json.map((order) => {
            order.updated_at = new Date(order.updated_at);
            order.created_at = new Date(order.created_at);
            order.expires_at = new Date(order.expires_at);
            return order;
        });
	};

	const get_date_diff_hours = (date: Date) => {
		const now = new Date();
		const diff = date.getTime() - now.getTime();
		return Math.floor(diff / (1000 * 60 * 60));
	};

    const get_date_diff_days = (date: Date) => {
        const now = new Date();
        const diff = date.getTime() - now.getTime();
        return Math.floor(diff / (1000 * 60 * 60 * 24));
    };

	onMount(async () => {
		locations = await get_locations();
        orders = await get_prices();
	});
</script>

<div class="flex flex-col h-screen">
	<div class="h-full overflow-auto">
		<table class="table table-pin-rows">
			<thead>
				<tr>
                    <th></th>
					<th>Item</th>
					<th>Price</th>
					<th>Amount</th>
					<th>Last update</th>
                    <th>Expires</th>
                    <th>Location</th>
				</tr>
			</thead>
			<tbody>
                {#each orders as order, i}
                    <tr class="hover:bg-base-200">
                        <td>
                            {i + 1 + page * page_size}
                        </td>
                        <td>
                            <img
                                class="h-12"
                                src={`https://render.albiononline.com/v1/item/${order.item_unique_name}.png?quality=${order.quality_level}&size=128`}
                                alt="item"
                            />
                        </td>
                        <td
                            >{number_formatter.format(
                                order.unit_price_silver,
                            )}</td
                        >
                        <td>{order.amount}</td>
                        <td
                            >{time_formatter.format(
                                get_date_diff_hours(order.updated_at),
                                "hours",
                            )}</td
                        >
                        <td
                            >{time_formatter.format(
                                get_date_diff_days(order.expires_at),
                                "days",
                            )}</td>
                        <td
                            >{locations.find(
                                (l) => l.id === order.location_id,
                            )?.name}</td
                        >
                    </tr>
                {/each}
			</tbody>
		</table>
	</div>
	<div class="flex justify-center w-full join">
		<button
			class="join-item btn"
			on:click={async() => {
				if (page > 0) {
					page--;
				}
				orders = await get_prices();
			}}>«</button
		>
		<button class="join-item btn btn-disabled">{page}</button>
		<button
			class="join-item btn"
			on:click={async() => {
				page++;
				orders = await get_prices();
			}}>»</button
		>
	</div>
</div>