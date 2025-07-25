<script lang='ts'>
	import * as base64 from "$lib/base64.ts";
	import { goto } from '$app/navigation';
	import Blokus from "$lib/Blokus.svelte";
	import { Blokus as BlokusBitmap, TileState } from "../../../cgtjs/cgtjs/index.ts";
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	const polyominos = data.polyominos;
	const url = data.url;

	let boardWidth = $state(3n);
	let boardHeight = $state(9n);
	let board = $derived.by(() => {
		if (!boardWidth || !boardHeight) return;

		const w = BigInt(boardWidth);
		const h = BigInt(boardHeight);
		if (w <= 0n || h <= 0n) return;

		const board = BlokusBitmap.empty(BigInt(boardWidth), BigInt(boardHeight));
		board.set(0n, 0n, TileState.Corner);
		return board;
	});
	let newPolyomino: BlokusBitmap|null = $state(null);
	let newPolyominoW = $state(0);
	let newPolyominoH = $state(0);
	let newPolyominoTile = $state(TileState.Interior);

	const serializePolyominos = (poly: BlokusBitmap[]) => poly.map(p => base64.encode(p.serialize())).join(',');
	const arrayWithout = <T>(arr: T[], indexToRemove: number) => arr.filter((_, i) => indexToRemove !== i);
	const urlWithPolyominos = (polyominos:  BlokusBitmap[]) => {
		let newSearchParams = new URLSearchParams(url.search);
		newSearchParams.set('polyominos', serializePolyominos(polyominos));
		const location = new URL(url);
		location.search = newSearchParams.toString();
		return location.toString();
	}

	const serializedBoard = $derived(serializePolyominos([board]));
	const gameUrl = $derived.by(() => {
		const u = new URL(url);
		u.pathname = `/game/${serializedBoard}`;
		return u;
	});
	$effect(() => {
		const w = BigInt(newPolyominoW);
		const h = BigInt(newPolyominoH);
		if (newPolyomino && (w !== newPolyomino.width || h !== newPolyomino.height)) {
			newPolyomino = newPolyomino.resize(w, h);
		}
	});
	let polyominosSerialized: string = $state('');
</script>
<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

<div>
	{#each polyominos as poly, i }
		<Blokus board={poly} />
		<a data-sveltekit-reload href={urlWithPolyominos(arrayWithout(polyominos, i))}>Remove</a>
	{/each}
</div>

<div>
	<input bind:value={boardWidth} type="number" placeholder="Width" />
	<input bind:value={boardHeight} type="number" placeholder="Height" />
</div>

<div>
	<button onclick={(_ev) => newPolyomino = BlokusBitmap.empty(BigInt(newPolyominoW), BigInt(newPolyominoH))}>Create New Polyomino</button>
	<input bind:value={newPolyominoW} type="number" placeholder="Width" />
	<input bind:value={newPolyominoH} type="number" placeholder="Height" />

	{#if newPolyomino}
		<form method="GET" data-sveltekit-reload action="">
		<input hidden name="polyominos" value={polyominosSerialized}>
		<Blokus placeState={TileState.Interior} autofillAroundInterior={true} showStates={true} board={newPolyomino} />
		<input type="submit" onclick={(ev) => polyominosSerialized = serializePolyominos([...polyominos, newPolyomino ])}/>
	</form>
	{/if}
</div>

<Blokus 
	board={board} />

<a data-sveltekit-reload href={gameUrl.toString()}>View</a>