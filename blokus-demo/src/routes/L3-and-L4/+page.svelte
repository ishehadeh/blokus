<script lang="ts">
	import * as base64 from '$lib/base64.ts';
	import { goto } from '$app/navigation';
	import Blokus from '$lib/Blokus.svelte';
	import { Blokus as BlokusBitmap, TileState } from '$lib/cgtjs';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	const polyominos = data.polyominos;
	const url = data.url;
	const board = BlokusBitmap.empty(3n, 9n);
	let newPolyomino: BlokusBitmap | null = $state(null);
	let newPolyominoW = $state(0);
	let newPolyominoH = $state(0);
	let newPolyominoTile = $state(TileState.Interior);

	board.set(0n, 0n, TileState.Corner);
	const serializePolyominos = (poly: BlokusBitmap[]) =>
		poly.map((p) => base64.encode(p.serialize())).join(',');
	const arrayWithout = <T,>(arr: T[], indexToRemove: number) =>
		arr.filter((_, i) => indexToRemove !== i);
	const urlWithPolyominos = (polyominos: BlokusBitmap[]) => {
		let newSearchParams = new URLSearchParams(url.search);
		newSearchParams.set('polyominos', serializePolyominos(polyominos));
		const location = new URL(url);
		location.search = newSearchParams.toString();
		return location.toString();
	};
	const gameUrl = new URL(url);
	gameUrl.pathname = '/game/' + serializePolyominos([board]);
	$effect(() => {
		const w = BigInt(newPolyominoW);
		const h = BigInt(newPolyominoH);
		if (newPolyomino && (w !== newPolyomino.width || h !== newPolyomino.height)) {
			newPolyomino = newPolyomino.resize(w, h);
		}
	});
	let polyominosSerialized: string = $state('');
	const L4 = BlokusBitmap.fromString(`csc
										sis
										sisc
										siis
										cssc`);
	const L3 = BlokusBitmap.fromString(`csc
										sisc
										siis
										cssc`);
	const newBoard = () => {
		const bitboard = BlokusBitmap.empty(3n, 5n);
		bitboard.set(0n, 0n, TileState.Corner);
		return bitboard;
	};

	// first example -
	const example1 = newBoard();
	example1.tryPlacePolyomino(0n, 0n, L3, 1n, 1n);

	const example2 = example1.clone();
	example2.tryPlacePolyomino(0n, 1n, L4.flipHorizontal(), 0n, 0n);

	let showStates = $state(false);
	const startingPositions = [
		BlokusBitmap.fromString(`isc
iis
ssc
...
...
`),
		BlokusBitmap.fromString(`iis
isc
sc.
...
...
`),
		BlokusBitmap.fromString(`iis
sis
csc
...
...
`),
		BlokusBitmap.fromString(`is.
isc
iis
ssc
...
`),
		BlokusBitmap.fromString(`iii
iss
sc.
...
...
`),
		BlokusBitmap.fromString(`iis
sis
sis
csc
...
`),
		BlokusBitmap.fromString(`iss
iii
sss
...
...
`),
		BlokusBitmap.fromString(`iis
isc
is.
sc.
...
`),
		BlokusBitmap.fromString(`iii
ssi
.cs
...
...
`)
	];
</script>

<section id="l3-l4">
	<section id="intro">
		<h1>Blokus L3 & L4 on 3x5</h1>
		<div>
			<p>We call the following polyomino L4.</p>
			<figure>
				<Blokus board={L4} />
			</figure>
		</div>

		<div>
			<p>This is L3.</p>
			<figure>
				<Blokus board={L3} />
			</figure>
		</div>

		<div>
			<p>
				Now, on a 3x5 board, we'll take turns placing the above polyonimos, with one restriction:
				only their corners may touch. For example, here a are few valid moves. The first move must
				be made in the top left corner.
			</p>
			<figure>
				<figcaption>Example Series of Moves.</figcaption>

				<div class="move-sequence">
					<div>
						<Blokus board={example1} />
					</div>

					<div>
						<Blokus board={example2} />
					</div>
				</div>
			</figure>
		</div>

		<div>
			<p>This game is equivalent to *3</p>
		</div>
	</section>

	<section id="proof">
		<div>
			<p>Let's look at all possible starting moves, using L3</p>
			<figure>
				<figcaption>Moves of L3</figcaption>

				<div class="game-group">
					{#each newBoard().movesDeduplicated([L3]) as move}
						<Blokus board={move} {showStates} />
					{/each}
				</div>
			</figure>

			<figure>
				<figcaption>Moves of L4</figcaption>

				<div class="game-group">
					{#each newBoard().movesDeduplicated([L4]) as move}
						<Blokus board={move} {showStates} />
					{/each}
				</div>
			</figure>

			<p>Try clicking the checkbox below to see some more information about these games</p>
			<input bind:checked={showStates} type="checkbox" name="show-states" id="show-states" />
			<label for="show-states"> Detail View </label>

			<div>
				<p>
					The <span class="cell-ref-corner">red</span> tiles are corners, meaning we can place
					another polyomino on them.
					<span class="cell-ref-side">Green</span> tiles side tiles, meaning polyomino squares cannot
					be placed on them.
				</p>
			</div>

			<div>
				<p>
					This game is small enought we won't use any tricks to solve it, let's take a look our all
					our possible starting positions.
				</p>

				<figure>
					<figcaption>Starting Positions</figcaption>

					<div class="game-group">
						{#each startingPositions as move, i}
							<div>
								<Blokus board={move} {showStates} />
								<p>A{i + 1}</p>
							</div>
						{/each}
					</div>
				</figure>

				<p>
					A1 has three possible moves. 
					Each of these moves from A1 would be the final move - notice that there are no L3 or L4 shaped areas of either corner or empty tiles on any of the boards below.
				</p>
				<figure>
					<figcaption>Moves from Position A1</figcaption>

					<div class="game-group">
						{#each startingPositions[0].movesDeduplicated([L3, L4]) as move, i}
							<div>
								<Blokus board={move} {showStates} />
								<p>B{i + 1}</p>
							</div>
						{/each}
					</div>
				</figure>

				<p>
					So, we find <b>A1 = { 0 } = *</b>. By a similar argument, we find that A3, A4, A5, A6, A8, A9 are also equal to <b>*</b>.
				</p>

				<div style="display: flex; flex-wrap: wrap; align-items: center">
					<figure>
						<figcaption>Moves from Position A3</figcaption>

						<div class="game-group">
							{#each startingPositions[2].movesDeduplicated([L3, L4]) as move, i}
								<div>
									<Blokus board={move} {showStates} />
									<p>C{i + 1}</p>
								</div>
							{/each}
						</div>
					</figure>

					<figure>
						<figcaption>Moves from Position A4</figcaption>

						<div class="game-group">
							{#each startingPositions[3].movesDeduplicated([L3, L4]) as move, i}
								<div>
									<Blokus board={move} {showStates} />
									<p>D{i + 1}</p>
								</div>
							{/each}
						</div>
					</figure>

					<figure>
						<figcaption>TODO: WRONG - Moves from Position A5</figcaption>

						<div class="game-group">
							{#each startingPositions[4].movesDeduplicated([L3, L4]) as move, i}
								<div>
									<Blokus board={move} {showStates} />
									<p>E{i + 1}</p>
								</div>
							{/each}
						</div>
					</figure>

					<figure>
						<figcaption>Moves from Position A6</figcaption>

						<div class="game-group">
							{#each startingPositions[5].movesDeduplicated([L3, L4]) as move, i}
								<div>
									<Blokus board={move} {showStates} />
									<p>F{i + 1}</p>
								</div>
							{/each}
						</div>
					</figure>

					<figure>
						<figcaption>Moves from Position A8</figcaption>

						<div class="game-group">
							{#each startingPositions[7].movesDeduplicated([L3, L4]) as move, i}
								<div>
									<Blokus board={move} {showStates} />
									<p>G{i + 1}</p>
								</div>
							{/each}
						</div>
					</figure>

					<figure>
						<figcaption>Moves from Position A9</figcaption>

						<div class="game-group">
							{#each startingPositions[8].movesDeduplicated([L3, L4]) as move, i}
								<div>
									<Blokus board={move} {showStates} />
									<p>H{i + 1}</p>
								</div>
							{/each}
						</div>
					</figure>
				</div>

				<div>
					So, we've reduced our game to the following:
					<figure>
						<figcaption>3x5 Blokus with L3 and L4</figcaption>

						<div class="game-group">
							<div>
								<Blokus board={startingPositions[1]} {showStates} />
								<p>A2</p>
							</div>
							<div>
								<Blokus board={startingPositions[6]} {showStates} />
								<p>A7</p>
							</div>
							<div style="display: flex; align-items: center; justify-content: center">
								*
							</div>
						</div>
					</figure>

					<p>A7 has no corners, therefore no moves from that position, so A7 = 0.</p>

					
					<figure>
						<figcaption>3x5 Blokus with L3 and L4</figcaption>

						<div class="game-group">
							<div>
								<Blokus board={startingPositions[1]} {showStates} />
								<p>A2</p>
							</div>

							<div style="display: flex; align-items: center; justify-content: center">
								0
							</div>
							<div style="display: flex; align-items: center; justify-content: center">
								*
							</div>
						</div>
					</figure>
				</div>

				<div>
					<p>Let's look at A2</p>
					<figure>
						<figcaption>Moves from Position A2</figcaption>

						<div class="game-group">
							{#each startingPositions[1].movesDeduplicated([L3, L4]) as move, i}
								<div>
									<Blokus board={move} {showStates} />
									<p>I{i + 1}</p>
								</div>
							{/each}
						</div>
					</figure>

					<p>
						As discussed before, because of the lack of free space there are no moves from I1, I2, I5, I6, I7, I9, I10.
						However, both I3, and I4 have a single move: they can place L3 in the lower left corner.
						Following this move there would be no more moves, so L3 and L4 are equal to '*'.
						So, we find A2 = &lbrace;*, 0&rbrace; = *2.
					</p>
				</div>

				<div>
					<p>
						So, we find our game is &lbrace;*2, *, 0&rbrace;  or *3.
					</p>
				</div>
			</div>
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
</style>
