<script lang="ts">
import { getContext, onMount } from "svelte";
import MacroConfig from "./modals/MacroConfig.svelte";

import type ButtonPanel from "../model/ButtonPanel";
import type MacroButton from "../model/MacroButton";
import { WSClient } from "../api";
import { clicker } from "../util";

const { open } = getContext("simple-modal");

export let panel: ButtonPanel;



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
