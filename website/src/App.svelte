<script lang="ts">
import { setContext } from "svelte";

import Grid from "./components/Grid.svelte";
import MacroModel from "./components/MacroModal.svelte";
import Navbar from "./components/Navbar.svelte";

import ButtonPanel from "./model/ButtonPanel";
import PanelController from "./model/PanelController";



let controller = new PanelController(
    ButtonPanel.default("Panel 1")
);

function updateApp() { controller = controller; }
function addPanel() {
    const panelIndex = controller.panels.length + 1;
    controller.panels.push(ButtonPanel.default(`Panel ${panelIndex}`));
    updateApp();
}
function selectPanel(panel: ButtonPanel) {
    controller.changePanel(panel);
    updateApp();
}
setContext("macrobot", { addPanel, selectPanel, updateApp });
</script>

<MacroModel>
    <main>
        <Navbar controller={controller}/>
        <Grid panel={controller.activePanel}/>
    </main>
</MacroModel>
<style>
    main {
        width: 100%;
        height: 100%;

        display: flex;
        flex-direction: column;
    }
    Navbar { flex: 0 0; }
</style>
