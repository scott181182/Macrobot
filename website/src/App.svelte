<script lang="ts">
import Modal from "svelte-simple-modal";

import Grid from "./Grid.svelte";
import Navbar from "./Navbar.svelte";

import ButtonPanel from "./ButtonPanel";
import PanelController from "./PanelController";



let controller = new PanelController(
    ButtonPanel.default("Panel 1"),
    ButtonPanel.default("Panel 2")
);

function selectPanel(panel: ButtonPanel) {
    controller.changePanel(panel);
    controller = controller;
}
</script>

<Modal>
    <main class="bg-dark">
        <Navbar
            controller={controller}
            on:changepanel={e => selectPanel(e.detail.panel)}/>
        <Grid panel={controller.activePanel}/>
    </main>
</Modal>
<style>
    main {
        width: 100%;
        height: 100%;

        display: flex;
        flex-direction: column;
        /* background-color: #4a4a4a; */
    }
    Navbar {
        flex: 0 0;
    }
</style>
