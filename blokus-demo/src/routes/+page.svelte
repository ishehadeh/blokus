<script>
	import Blokus from "$lib/Blokus.svelte";
	import { Blokus as BlokusBitmap, TileState } from "../../../cgtjs/cgtjs/index.ts";
	const polyominos = [
		BlokusBitmap.fromString(`
			csc
			sis
			sis
			csc`)
	];
	const board = BlokusBitmap.empty(3n, 9n);
	board.set(0n, 0n, TileState.Corner);
</script>
<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

<Blokus 
	board={board} />
<a href="/game/{board.serializeAscii()}">View</a>