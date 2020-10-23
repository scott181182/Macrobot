<script lang="ts">
import { createEventDispatcher, getContext } from "svelte";

import PanelConfig from "./modals/PanelConfig.svelte";

import type ButtonPanel from "../model/ButtonPanel";
import type PanelController from "../model/PanelController";

import { clicker } from "../util";

export let controller: PanelController;

const { open } = getContext("simple-modal");


const dispatch = createEventDispatcher();
$: panels = controller.panels;

function configurePanel(panel: ButtonPanel)
{
    open(PanelConfig, { panel }, {  }, {
        onClosed: () => { dispatch("panelchange"); }
    });
}

</script>



<ul class="nav nav-tabs bg-secondary">
    {#each panels as panel}
        <li class="nav-item">
            <a class="nav-link bg-dark" href="#"
                    class:active={controller.activePanel.title == panel.title}
                    use:clicker
                    on:shortclick={e => dispatch("changepanel", { panel })}
                    on:longclick={e => configurePanel(panel)}>
                {panel.title}
            </a>
        </li>
    {/each}
    <li class="nav-item">
        <a class="nav-link bg-dark"  href="#">
            <i class="fa fa-plus"></i>
        </a>
    </li>
    <!-- <button></button> -->
</ul>
