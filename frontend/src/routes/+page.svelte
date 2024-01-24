<script lang="ts">
	import { narrow_relative_time_formatter } from '$lib/utils/formatters';
	import { standard_number_formatter } from '$lib/utils/formatters';
	import type { Location, MarketOrder } from '$lib/types';
	import { onMount } from 'svelte';
	import type { LocalizedNames } from "$lib/types";

    let tiers: number[] = [1, 2, 3, 4, 5, 6, 7, 8];
    let enchantments: number[] = [0, 1, 2, 3, 4];
    let qualities: { name: string, value: number }[] = [
        { name: "Normal", value: 1 },
        { name: "Good", value: 2 },
        { name: "Outstanding", value: 3 },
        { name: "Excellent", value: 4 },
        { name: "Masterpiece", value: 5 },
    ]
    
    let filters: {
        name: string,
        location?: string,
        tier?: number,
        enchantment?: number,
        quality?: number,
    } = {
        name: "",
    }

    let page = 0;
    let page_size = 20;
    let orders: MarketOrder[] = [];

    let locations: Location[] = [];
    
    onMount(async() => {
        locations = await get_locations();
    })

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

        let request_url = 
            `https://veqox.dedyn.io/api/orders` +
            `?item_name=${filters.name}` +
            `&auction_type=offer` +
            `&limit=${page_size}` +
            `&from=${from_string}` +
            `&offset=${page * page_size}`;

        if(filters.enchantment) request_url += `&enchantment_level=${filters.enchantment}`;
        if(filters.quality) request_url += `&quality_level=${filters.quality}`;
        if(filters.location) request_url += `&location_id=${filters.location}`;
		
		const res = await fetch(request_url);

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
</script>

<div class="flex flex-col h-screen">
    <div class="flex justify-around w-full">
        <input 
            class="input input-bordered" 
            type="text" 
            bind:value={filters.name}>
    
        <select class="select select-bordered" bind:value={filters.tier}>
            <option value={undefined} disabled selected>Tier</option>
            {#each tiers as tier}
                <option>{tier}</option>
            {/each}
        </select>
    
        <select class="select select-bordered" bind:value={filters.enchantment}>
            <option value={undefined} disabled selected>Enchantment</option>
            {#each enchantments as enchantment}
                <option>{enchantment}</option>
            {/each}
        </select>
    
        <select class="select select-bordered" bind:value={filters.quality}>
            <option value={undefined} disabled selected>Quality</option>
            {#each qualities as quality}
                <option value={quality.value}>{quality.name}</option>
            {/each}
        </select>
        
        <select class="select select-bordered" bind:value={filters.location}>
            <option value={undefined} disabled selected>Location</option>
            {#each locations as location}
                <option value={location.id}>{location.name}</option>
            {/each}
        </select>

        <button class="btn" on:click={async() => {
            orders = await get_prices();
        }}>
            ⟳
        </button>
    </div>
    
	<div class="h-full overflow-auto">
		<table class="table table-pin-rows">
			<thead class="text-lg">
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
                            <img
                                class="h-12"
                                src={`https://render.albiononline.com/v1/item/${order.item_unique_name}.png?quality=${order.quality_level}&size=128`}
                                alt="item"
                            />
                        </td>
                        <td>
                            {order.item_unique_name}
                        </td>
                        <td
                            >{standard_number_formatter.format(
                                order.unit_price_silver,
                            )}</td
                        >
                        <td>{order.amount}</td>
                        <td
                            >{narrow_relative_time_formatter.format(
                                get_date_diff_hours(order.updated_at),
                                "hours",
                            )}</td
                        >
                        <td
                            >{narrow_relative_time_formatter.format(
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
		<div class="join">
            <button class="join-item btn" on:click={async() => {
                if(page > 0) page--;
                orders = await get_prices();
            }}>«</button>
            <button class="join-item btn">Page {page}</button>
            <button class="join-item btn" on:click={async() => {
                page++;
                orders = await get_prices();
            }}>»</button>
        </div>
	</div>
</div>