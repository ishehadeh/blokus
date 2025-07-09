<script lang="ts">
	import * as base64 from '$lib/base64.ts';
	import Blokus from '$lib/Blokus.svelte';
	import { Blokus as BlokusBitmap, TileState } from '$lib/cgtjs';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	const url = data.url;
	const board = BlokusBitmap.empty(3n, 9n);
	let newPolyomino: BlokusBitmap | null = $state(null);
	let newPolyominoW = $state(0);
	let newPolyominoH = $state(0);
	let newPolyominoTile = $state(TileState.Interior);

	board.set(0n, 0n, TileState.Corner);
	const serializePolyominos = (poly: BlokusBitmap[]) =>
		poly.map((p) => base64.encode(p.serialize())).join(',');
	const gameUrl = new URL(url);
	gameUrl.pathname = '/game/' + serializePolyominos([board]);
	$effect(() => {
		const w = BigInt(newPolyominoW);
		const h = BigInt(newPolyominoH);
		if (newPolyomino && (w !== newPolyomino.width || h !== newPolyomino.height)) {
			newPolyomino = newPolyomino.resize(w, h);
		}
	});
	const L4 = BlokusBitmap.fromString(`csc
										sis
										sisc
										siis
										cssc`);
	const newBoard = (n: bigint) => {
		const bitboard = BlokusBitmap.empty(n, 3n);
		bitboard.set(0n, 0n, TileState.Corner);
		return bitboard;
	};
	const alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
	const firstMoves: [string, BlokusBitmap][] = [...newBoard(5n).movesDeduplicated([L4])]
		.sort((a, b) => a.toStringBoard().localeCompare(b.toStringBoard()))
		.map((board, i) => [alphabet[i], board]);
	const ASubMoves: [string, BlokusBitmap][] = [...Object.fromEntries(firstMoves)['A'].resize(6n, 3n).movesDeduplicated([L4])]
		.sort((a, b) => a.toStringBoard().localeCompare(b.toStringBoard()))
		.map((board, i) => [`A${i + 1}`, board]);
	const ESubMoves: [string, BlokusBitmap][] = [...Object.fromEntries(firstMoves)['E'].resize(6n, 3n).movesDeduplicated([L4])]
		.sort((a, b) => a.toStringBoard().localeCompare(b.toStringBoard()))
		.map((board, i) => [`E${i + 1}`, board]);
	const CSubMoves: [string, BlokusBitmap][] = [...Object.fromEntries(firstMoves)['C'].resize(6n, 3n).movesDeduplicated([L4])]
		.sort((a, b) => a.toStringBoard().localeCompare(b.toStringBoard()))
		.map((board, i) => [`C${i + 1}`, board]);
</script>

<section id="l3-l4">
	<section id="intro">
		<h1>Blokus L4 on 3xN</h1>
		<div>
			<p>We call the following polyomino L4.</p>
			<figure>
				<Blokus board={L4} />
			</figure>
		</div>

		<div>
			<p>Suppose we're playing on a 3xN board, the first player then has the following options:</p>

			<div id="first-moves-figure-container">
				{#each firstMoves as [name, move] }
					<figure>
						<figcaption>{name}</figcaption>
						<Blokus board={move} />
					</figure>
				{/each}
			</div>

			<ul>
				<li><b>D</b> ends the game immediately.</li>
				<li><b>B</b> forces B or D (mirrored) on a board of <code>N - 4</code>.</li>
				<li><b>F</b> forces B or D (mirrored) on a board of <code>N - 4</code>.</li>
				<li><b>A</b>, <b>C</b>, and <b>E</b> are more complex.</li>
			</ul>
		</div>
	</section>

	<section>
		<h2>The Spiral</h2>

		<div>
			<p>Below are the moves from <b>B</b>, notice that we are forced to move to <b>D</b> (i.e. 0) or <b>B</b> mirrored.</p>
			<div id="first-moves-figure-container">
				{#each Object.fromEntries(firstMoves)['B'].resize(6n, 3n).movesDeduplicated([L4]) as move }
					<figure>
						<Blokus board={move} />
					</figure>
				{/each}
			</div>

			<p>
				So, we can define <code>B(N) = &lbrace;0, B(N - 4)&rbrace;</code> 
			</p>
		</div>
		<div>
			<p>Now, let's look at <b>F</b>:</p>
			<div id="first-moves-figure-container">
				{#each Object.fromEntries(firstMoves)['F'].resize(6n, 3n).movesDeduplicated([L4]) as move }
					<figure>
						<Blokus board={move} />
					</figure>
				{/each}
			</div>

			<p>
				These are the exact same moves as <b>B</b>, but each is mirrored. Mirrored moves don't matter (TODO: prove).
				So, it follows (<b>F(N) = B(n)</b>)
			</p>
		</div>

		<p>
			In summary, we've found that <b>F(n) = B(n) = &lbrace;0, B(N - 4)&rbrace;</b>, and <b>D(n) = 0</b>
		</p>
	</section>

	<section>
		<h2>The Hard Ones</h2>

		<div>
			<p>Let's investigate the moves of <b>A</b></p>
			<div>
				{#each ASubMoves as [name, move] }
					<figure>
						<figcaption>{name}</figcaption>
						<Blokus board={move} />
					</figure>
				{/each}
			</div>

			<ul>
				<li><b>A1</b> =&lbrace;B(N - 5), D(N - 5) &rbrace;</li>
				<li><b>A2</b> = A(N - 4)</li>
				<li><b>A3</b> = <b>A4</b> = &lbrace;B(N - 4), D(N - 4) &rbrace;</li>
				<li><b>A5</b> = F(N - 4)</li>
			</ul>
		</div>

		<div>
			<p>Now, moving on to investigate the moves of <b>E</b></p>
			<div>
				{#each ESubMoves as [name, move] }
					<figure>
						<figcaption>{name}</figcaption>
						<Blokus board={move} />
					</figure>
				{/each}
			</div>

			<ul>
				<li><b>E1</b> = A(N - 2)</li>
				<li><b>E2</b> = F(N - 2)</li>
				<li><b>E3</b> = <b>E4</b> = B(N - 2)</li>
			</ul>
		</div>

		<div>
			<p>Finally, let's look at <b>C</b></p>
			<div>
				{#each CSubMoves as [name, move] }
					<figure>
						<figcaption>{name}</figcaption>
						<Blokus board={move} />
					</figure>
				{/each}
			</div>

			<ul>
				<li><b>C1</b> = A(N - 2)</li>
				<li><b>C2</b> = <b>C4</b> = B(N - 2)</li>
				<li><b>C3</b> = F(N - 2)</li>
			</ul>
		</div>

		<div>
			<p>So, in summary:
				<br />
				<b>A = &lbrace;
						&lbrace;B(N - 5), 0 &rbrace;,
						A(N - 4),
						&lbrace;B(N - 4), 0 &rbrace;,
						F(N - 4)
					   &rbrace;</b>,
				<br />
				<b>E = &lbrace;
						A(N - 2),
						B(N - 2),
						F(N - 2)
					   &rbrace;</b>
				<br />
				<b>C = &lbrace;
							A(N - 2),
							B(N - 2),
							F(N - 2),
					   &rbrace;</b>
			</p>
		</div>
	</section>

	<section>
		<h2>Putting it Together</h2>

		<div>
			<p>So, in summary:
				<br />
				<b>A(N) = &lbrace;
						&lbrace;B(N - 5), 0 &rbrace;,
						A(N - 4),
						&lbrace;B(N - 4), 0 &rbrace;,
						F(N - 4)
					   &rbrace;</b>,
				<br />
				<b>E(N) = C(N) = &lbrace;
						A(N - 2),
						B(N - 2),
						F(N - 2)
					   &rbrace;</b>
				<br />
				<b>F(n) = B(n) = &lbrace;0, B(N - 4)&rbrace;</b>
				<br />
				<b>D(n) = 0</b>
			</p>
		</div>
	</section>
</section>

<style>
	.game-group {
		display: flex;
		gap: 1rem;
	}

	.move-sequence {
		display: flex;
		gap: calc(3rem + 8px * 2);
	}

	.move-sequence > *:not(:last-child)::after {
		content: '🠒';
		font-size: 3rem;
		padding: 0 8px;
		position: absolute;
	}

	section#l3-l4 {
		margin: auto;
		max-width: 640px;
		font-size: 1.2rem;
		--bg-color: white;
	}

	figure {
		border: 1px solid black;
		display: inline-block;
		padding: 1rem;

		counter-increment: figure;
	}

	figcaption {
		padding: 8px;
		margin-top: -2rem;
		font-size: 1rem;
		border-radius: 10px;
		background-color: var(--bg-color);
		display: block;
	}

	figcaption::before {
		content: 'Fig. ' counter(figure);
		margin-right: 8px;
	}

	.cell-ref-corner::after {
		content: '⬤';
		color: var(--color-corner);
		padding-left: 4px;
		padding-right: 4px;
	}
	.cell-ref-side::after {
		content: '⬤';
		color: var(--color-side);
		padding-left: 4px;
		padding-right: 4px;
	}
	.cell-ref-interior::after {
		content: '⬤';
		color: var(--color-interior);
		padding-left: 4px;
		padding-right: 4px;
	}
	.controls {
		padding: 10px;
		z-index: 1000;
		position: sticky;
		border: 1px solid black;
		border-radius: 10px;
		top: 10px;
		background-color: white;
	}
	.controls label, .controls input {
		cursor: pointer;
	}
	section > section:has(.controls) {
		display: initial;
	}

</style>
