<script lang="ts">
import { getContext } from "svelte";

import PanelConfig from "./modals/PanelConfig.svelte";

import type ButtonPanel from "../model/ButtonPanel";
import type PanelController from "../model/PanelController";



export let controller: PanelController;



const { open: openModal, close: closeModal } = getContext("macromodal");
const { addPanel, selectPanel, updateApp } = getContext("macrobot");



$: panels = controller.panels;

function configurePanel(panel: ButtonPanel)
{
    openModal(PanelConfig, {
        panel,
        onsave: () => closeModal(),
        ondelete: (panel: ButtonPanel) => {
            controller.deletePanel(panel);
            closeModal();
        }
    }, {  }, {
        onClosed: () => { updateApp(); }
    });
}

</script>

<style lang="scss" scoped>
@import "../variables.scss";

#settings-tab {
    margin-left: auto;
    a { color: $secondary !important; }
}
</style>

<ul class="nav nav-tabs bg-secondary">
    {#each panels as panel}
        <li class="nav-item">
            <a class="nav-link bg-dark" href="#"
                    class:active={controller.activePanel.title == panel.title}
                    on:click={_ => selectPanel(panel)}>
                {panel.title}
            </a>
        </li>
    {/each}
    <li class="nav-item">
        <a class="nav-link bg-dark"  href="#"
                on:click={_ => addPanel()}>
            <i class="fa fa-plus"></i>
        </a>
    </li>
    <!-- <button></button> -->
    <li id="settings-tab" class="nav-item">
        <a class="nav-link bg-dark active" style="color: $secondary" href="#"
                on:click={_ => configurePanel(controller.activePanel)}>
            <i class="fa fa-cog"></i>
        </a>
    </li>
</ul>
