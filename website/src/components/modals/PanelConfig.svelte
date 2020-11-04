<script lang="ts">
import { createEventDispatcher } from "svelte";

import type ButtonPanel from "../../model/ButtonPanel";



export let panel: ButtonPanel;
export let onsave: () => void;
export let ondelete: (panel: ButtonPanel) => void;



let title = panel.title;
let rows = panel.buttons.length;
let cols = panel.buttons[0].length;

function update()
{
    panel.title = title;
    panel.reshape(rows, cols);
    onsave();
}

</script>

<div class="fluid-container">
    <form>
        <div class="row">
            <label for="title-field" class="col-12">
                Panel Title:
                <input id="title-field" type="text" bind:value={title}/>
            </label>
        </div>
        <div class="row">
            <label for="row-field" class="col-6">
                Button Rows:
                <input id="row-field" type="number" min="1" bind:value={rows}/>
            </label>
            <label for="col-field" class="col-6">
                Button Columns:
                <input id="col-field" type="number" min="1" bind:value={cols}/>
            </label>
        </div>
        <div class="row">
            <div class="col-6">
                <button class="btn btn-block btn-danger" on:click={_ => ondelete(panel)}>Delete Panel</button>
            </div>
            <div class="col-6">
                <button class="btn btn-block btn-success" on:click={_ => update()}>Save</button>
            </div>
        </div>
    </form>
</div>
