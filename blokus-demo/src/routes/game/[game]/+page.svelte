<script lang="ts">
	import Blokus from '$lib/Blokus.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	const { board, children } = data;
</script>

<div class="game-tree">
    <Blokus board={board}  />
    <div class="children">
        {#each children as child}
            <Blokus board={child} />
			<a data-sveltekit-reload href="/game/{child.serializeAscii()}">
				View
			</a>
        {/each}
    </div>
</div>

<style>
	.children {
		display: flex;
		flex-flow: row wrap;
	}
</style>