<div class="cell {state}">
    
</div>

<script lang="ts">
    import { TileState } from '../../../cgtjs/cgtjs/index.ts';
    const {
        state,
    }: {
        state: TileState,
    } = $props();    
</script>

<style>
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