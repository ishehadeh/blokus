<script lang="ts">
	import * as base64 from '$lib/base64';
	import Blokus from '$lib/Blokus.svelte';
	import { CanonicalForm, canonicalForm, MoveSet, NumberUpStar, Blokus as BlokusBitmap } from '$lib/cgtjs.ts';
	import type { PageProps } from './$types';
	import { page } from '$app/state';

	let { data }: PageProps = $props();
	const { board, children, polyominos } = data;


	let score: CanonicalForm|null = $state(null);
	function solve(blokus: BlokusBitmap, poly: BlokusBitmap[]): NumberUpStar|MoveSet {
		const solveChildren: BlokusBitmap[] = []
		for (const child of blokus.moves(poly)) {

			let exists = false;
			for (const existingChild of solveChildren) {
				if (existingChild.isEqualTo(child)) {
					exists = true;
					break;
				}
			}
			if (!exists) {
				solveChildren.push(child);
			}
		}
		const canonMoves = solveChildren.map(child => solve(child, poly));
		return canonicalForm(canonMoves, canonMoves);
	}

	const gameUrl = (polyomino: BlokusBitmap) => {
		const url = new URL(page.url);
		url.pathname = '/game/' + base64.encode(polyomino.serialize());
		return url.toString();
	}

</script>

<div class="game-tree">
    <h1>Game Detail View</h1>
	<Blokus board={board}  />
	
	{#if score !== null}
		<p class="score">{score.toString()}</p>
	{/if}

	<button onclick={(ev) => score = solve(board, polyominos)}>Calculate Value</button>

    <div class="children">
        {#each children as child}
			<div>
				<Blokus board={child} />
				<a data-sveltekit-reload href={gameUrl(child)}>
					View
				</a>
			</div>
        {/each}
    </div>
</div>

<style>
	.game-tree {
		display: flex;
		flex-flow: column nowrap;
		align-items: center;
		gap: 4em;
	}

	.children {
		display: flex;
		flex-flow: row wrap;
		gap: 4em;
	}
</style>