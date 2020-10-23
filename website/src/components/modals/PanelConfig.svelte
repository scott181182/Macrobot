<script lang="ts">
import type ButtonPanel from "../../model/ButtonPanel";

export let panel: ButtonPanel;
let rows = panel.buttons.length;
let cols = panel.buttons[0].length;

</script>

<div class="fluid-container">
    <form>
        <div class="row">
            <label for="title-field" class="col-xs-12">
                Panel Title:
                <input id="title-field" type="text" bind:value={panel.title}/>
            </label>
        </div>
        <div class="row">
            <label for="row-field" class="col-xs-6">
                Button Rows:
                <input id="row-field" type="number" min="1" bind:value={rows} on:change={_ => panel.reshape(rows, cols)}/>
            </label>
            <label for="col-field" class="col-xs-6">
                Button Columns:
                <input id="col-field" type="number" min="1" bind:value={cols} on:change={_ => panel.reshape(rows, cols)}/>
            </label>
        </div>
    </form>
</div>
