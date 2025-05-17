<div class="game-tree">
    <Blokus board={board}  />
    <button onclick={generateChildren}>Generate Children</button>
    <div class="children">
        {#each children as child}
            <BlokusTree polyominos={polyominos} board={child} />
        {/each}
    </div>
</div>

<script lang="ts">
    import { Blokus as BlokusBitset  } from '../../../cgtjs/cgtjs/index.ts';
    import Blokus from './Blokus.svelte';
    import BlokusTree from './BlokusTree.svelte';

    const {
        board,
        polyominos
    }: {
        board: BlokusBitset,
        polyominos: BlokusBitset[]
    } = $props();

    let children: BlokusBitset[] = $state([]);


    function generateChildren() {
        children = []
        for (const child of board.moves(polyominos)) {
            let exists = false;
            for (const existingChild of children) {
                if (existingChild.isEqualTo(child)) {
                    exists = true;
                    break;
                }
            }
            if (!exists) {
                children.push(child);
            }
        }
    }
    

</script>

<style>
    .children {
        display: flex;
        flex-flow: row wrap;
    }
</style>
