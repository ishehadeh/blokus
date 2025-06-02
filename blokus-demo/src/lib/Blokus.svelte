<div class={['board', ...(showStates ? ['show-states'] : [])]} style="--board-width: {board.width}; --board-height: {board.height}">
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
        placeState: editCell = undefined,
        showStates = false,
    }: {
        board: Blokus,
        userPolyomino?: Blokus,
        placeState?: TileState,
        showStates?: boolean
    } = $props();

    let previewBoard = board.clone();
    let cells = $state(new Array(Number(board.width * board.height)).fill(TileState.Empty));
    let hoveredCell = $state({ x: 0n, y: 0n});
    updateCellsArray();

    function updateCellsArray() {
        if (BigInt(cells.length) !== board.width * board.height) {
            cells = new Array(Number(board.width * board.height)).fill(TileState.Empty);
        }

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
        } else if (editCell) {
            previewBoard = board.clone();

            previewBoard.set(hoveredCell.x, hoveredCell.y, editCell);
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
        } else if (editCell) {
            board.set(hoveredCell.x, hoveredCell.y, editCell);
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
        display: inline-grid;
        grid-template-rows: repeat(var(--board-height), 1rem);
        grid-template-columns: repeat(var(--board-width), 1rem);

        gap: var(--cell-border-width, 2px);
        background-color: var(--cell-border-color, lightgrey);
        border: var(--cell-border-width, 2px) solid var(--cell-border-color, lightgrey);
    }

    .cell {
        background-color: var(--cell-color);
    }

    .interior {
        --cell-color: black;
    }

    .corner.show-states {
        --cell-color: #6A2E35;
    }

    .side.show-states {
        --cell-color: #AAC0AA;
    }

    .side:not(.show-states),
    .corner:not(.show-states),
    .empty {
        --cell-color: white;
    }
</style>
