<script lang="ts">
	import type { LocalizedNames } from "$lib/types";

    let searchValue: string;
    let suggestions: string[];

    const search = async() => {
        let response = await fetch(`https://veqox.dedyn.io/api/items/${searchValue}`)
        let data = await response.json() as LocalizedNames[];

        suggestions = data.map(d => d.en_us);
    }
</script>

<div class="flex items-center justify-center w-full">
    <input type="text" class="w-full max-w-xs input input-bordered input-sm" bind:value={searchValue} on:input={search} />
    <div>
        
    </div>
</div>
