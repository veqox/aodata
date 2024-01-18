<script lang="ts">
	import type { LocalizedNames } from "$lib/types";

	let items: LocalizedNames[] = [];
	let search_value = "";
	const search = async () => {
		let res = await fetch(
			`https://veqox.dedyn.io/api/items?name=${search_value}`,
		);
		let json = (await res.json()) as LocalizedNames[];
		items = json;
	};
</script>

<div class="flex justify-center w-full">
    <div class="dropdown">
        <input
            class="input input-bordered"
            bind:value={search_value}
            on:input={search}
        />
    
        <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
        <ul
            tabindex="0"
            class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-full"
        >
            {#each items as item}
                <li>
                    <a href={`/${item.item_unique_name}`}>
                        <img
                            class="w-10 h-10"
                            src={`https://render.albiononline.com/v1/item/${item.item_unique_name}.png?size=128`}
                            alt="item"
                        />
                        {item.en_us}
                    </a>
                </li>
            {/each}
        </ul>
    </div>
    
</div>
