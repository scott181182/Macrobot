<script lang="ts">
import { getContext, onMount } from "svelte";
import MacroConfig from "./MacroConfig.svelte";

import type ButtonPanel from "./ButtonPanel";
import MacroButton from "./MacroButton";
import { WSClient } from "./api";
import { clicker } from "./util";

const { open } = getContext("simple-modal");

export let panel: ButtonPanel;



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

$: btnGrid = panel.buttons;
let wsClient: WSClient;

onMount(async () => {
    // This is a test websocket server. Change this to the right endpoint.
    wsClient = await WSClient.connect(`ws://${window.location.host}/ws/`);
});



function executeMacro(macroBtn: MacroButton)
{
    if(macroBtn.macro) {
        macroBtn.execute(wsClient);
    } else {
        configureMacro(macroBtn);
    }
}
function configureMacro(macroBtn: MacroButton)
{
    open(MacroConfig, { macroBtn }, {  }, {
        onClosed: () => { btnGrid = btnGrid; }
    });
}

</script>


<section class="fluid-container d-flex flex-column h-100">
    {#each btnGrid as btnRow}
        <div class="btn-row">
            {#each btnRow as macroBtn}
                <div class="btn-container">
                    <button class="macro-btn btn-secondary"
                        use:clicker
                        on:shortclick={_ => executeMacro(macroBtn)}
                        on:longclick={_ => configureMacro(macroBtn)}>
                    {macroBtn.label}
                </button>
                </div>
            {/each}
        </div>
    {/each}
</section>

<style>
    .btn-row {
        width: 100%;
        flex: 1 1;
        flex-grow: 1;
        display: flex;
        flex-direction: row;
    }
    .btn-container {
        flex: 1 1;
        flex-grow: 1;

        padding: 1rem;
    }
    .macro-btn {
        width: 100%;
        height: 100%;

        border-radius: 1rem;
        /* background-color: #999; */
    }
</style>
