<div id='board' style="--board-width: {board.width}; --board-height: {board.height}">
    {#each cells as state}
        <Cell state={state} />
    {/each}
</div>

<script lang="ts">
    import { Blokus, TileState } from '../../../cgtjs/dist/index.js';
	import Cell from './Cell.svelte';

    const {
        width,
        height
    }: {
        width: number|bigint,
        height: number|bigint
    } = $props();

    const board = Blokus.empty(BigInt(width), BigInt(height));
    const cells = $state(new Array(Number(board.width * board.height)).fill(TileState.Empty));
    updateCellsArray();

    function updateCellsArray() {
        for (const cellIndex in cells) {
            const currentState = board.getByIndex(BigInt(cellIndex));
            const oldState = cells[cellIndex];
            if (oldState !== currentState) {
                cells[cellIndex] = currentState;
            }
        }
    }
</script>

<style>
    #board {
        display: grid;
        gap: 4px;
        grid-template-rows: repeat(var(--board-height), 1rem);
        grid-template-columns: repeat(var(--board-width), 1rem);
    }
</style>
