<script lang="ts">
import { getContext } from "svelte";
const { close: closeModal } = getContext("macromodal");

import type MacroButton from "../../model/MacroButton";



export let macroBtn: MacroButton;
export let onsave: (macroBtn: MacroButton) => void;



let label = macroBtn.label;
let macro = macroBtn.macro;
function update()
{
    macroBtn.label = label;
    macroBtn.macro = macro;
    onsave(macroBtn);
    closeModal();
}
</script>

<form>
    <div class="form-group">
        <label for="label-field">Macro Label:</label>
        <input id="label-field" class="form-control" type="text" bind:value={label}/>
    </div>
    <div class="form-group">
        <label for="macro-field">Macro Command:</label>
        <input id="macro-field" class="form-control" type="text" bind:value={macro}/>
    </div>
    <div class="form-row">
        <div class="col-6">
            <button class="btn btn-block btn-secondary" on:click={_ => closeModal()}>Cancel</button>
        </div>
        <div class="col-6">
            <button class="btn btn-block btn-success" on:click={_ => update()}>Save</button>
        </div>
    </div>
</form>
