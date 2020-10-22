<script lang="ts">
import { getContext } from "svelte";
import MacroConfig from "./MacroConfig.svelte";

import MacroButton from "./MacroButton";
// import { longpress } from "./util";

const { open } = getContext("simple-modal");



function makeGrid(rows: number, cols: number): MacroButton[][]
{
    const grid: MacroButton[][] = new Array(rows);
    for(let row = 0; row < rows; row++) {
        grid[row] = new Array(cols);
        for(let col = 0; col < cols; col++) {
            grid[row][col] = MacroButton.default();
        }
    }
    return grid;
}

let btnGrid: MacroButton[][] = makeGrid(3, 4);




function clickHandler(macroBtn: MacroButton)
{
    return () => {
        if(macroBtn.macro) {
            macroBtn.execute();
        } else {
            open(MacroConfig, { macroBtn }, {  }, {
                onClosed: () => {
                    console.log("Oof")
                    btnGrid = btnGrid;
                }
            });
        }
    }
}

</script>


<main>
    <section id="btn-grid">
        {#each btnGrid as btnRow}
            <div class="btn-row">
                {#each btnRow as macroBtn}
                    <div class="btn-container">
                        <button class="macro-btn"
                            on:click={clickHandler(macroBtn)}>
                        {macroBtn.label}
                    </button>
                    </div>
                {/each}
            </div>
        {/each}
    </section>
</main>

<style>
    main {
        width: 100%;
        height: 100%;
    }
    #btn-grid {
        display: flex;
        flex-direction: column;
        height: 100%;
    }
    .btn-row {
        width: 100%;
        flex-grow: 1;
        display: flex;
        flex-direction: row;
    }
    .btn-container {
        flex-grow: 1;

        padding: 1rem;
    }
    .macro-btn {
        width: 100%;
        height: 100%;
    }
</style>
