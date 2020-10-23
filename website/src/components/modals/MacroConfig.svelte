<script lang="ts">
import type MacroButton from "../../model/MacroButton";

export let macroBtn: MacroButton;

</script>

<div>
    <form>
        <label for="label-field">
            Macro Label:
            <input id="label-field" type="text" bind:value={macroBtn.label}/>
        </label>
        <label for="macro-field">
            Macro Command:
            <input id="macro-field" type="text" bind:value={macroBtn.macro}/>
        </label>
    </form>
</div>
