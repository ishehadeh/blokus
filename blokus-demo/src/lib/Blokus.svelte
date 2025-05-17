<div class="board" style="--board-width: {board.width}; --board-height: {board.height}">
    {#each cells as state, i}
        <div class="cell {state}" onmousedown={(e) => onMouseDown(e, i)} onmouseenter={(e) => onMouseEnter(e, i)}>
        </div>
    {/each}
</div>

<script lang="ts">
    import { Blokus, TileState } from '../../../cgtjs/cgtjs/index.ts';

    const {
        board,
        userPolyomino = undefined,
    }: {
        board: Blokus,
        userPolyomino?: Blokus,
    } = $props();

    let previewBoard = board.clone();
    const cells = $state(new Array(Number(board.width * board.height)).fill(TileState.Empty));
    let hoveredCell = $state({ x: 0n, y: 0n});
    updateCellsArray();

    function updateCellsArray() {
        for (const cellIndex in cells) {
            const currentState = previewBoard.getByIndex(BigInt(cellIndex));
            const oldState = cells[cellIndex];
            if (oldState !== currentState) {
                cells[cellIndex] = currentState;
            }
        }
    }

    $effect(() => {
        if (userPolyomino) {
            previewBoard = board.clone();
            previewBoard.tryPlacePolyomino(hoveredCell.x, hoveredCell.y, userPolyomino, userPolyomino.width/2n, userPolyomino.height/2n);
            updateCellsArray();
        }
    })

    function onMouseDown(event: MouseEvent, cellIndex: number) {
        hoveredCell.x = BigInt(cellIndex) % previewBoard.width;
        hoveredCell.y = BigInt(cellIndex) / previewBoard.width;
        if (userPolyomino) {
            board.tryPlacePolyomino(hoveredCell.x, hoveredCell.y, userPolyomino, userPolyomino.width/2n, userPolyomino.height/2n);
            previewBoard = board.clone();

            updateCellsArray();
        }
    }

    function onMouseEnter(event: MouseEvent, cellIndex: number) {
        hoveredCell.x = BigInt(cellIndex) % previewBoard.width;
        hoveredCell.y = BigInt(cellIndex) / previewBoard.width;
    }

</script>

<style>
    .board {
        display: grid;
        gap: 4px;
        grid-template-rows: repeat(var(--board-height), 1rem);
        grid-template-columns: repeat(var(--board-width), 1rem);
    }

    .cell {
        background-color: var(--cell-color);
    }

    .interior {
        --cell-color: blue;
    }
    .corner {
        --cell-color: green;
    }

    .side {
        --cell-color: orange;
    }

    .empty {
        --cell-color: black;
    }
</style>
