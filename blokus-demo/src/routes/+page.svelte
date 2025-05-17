<script>
	import Blokus from "$lib/Blokus.svelte";
	import { Blokus as BlokusBitmap } from "../../../cgtjs/dist";
</script>
<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

<Blokus
	width={10}
	height={10}
	userPolyomino={
		BlokusBitmap.fromString(`
			cssc
			siis
			sis
			csc`)
	}></Blokus>