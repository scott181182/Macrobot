<script lang="ts">
import { createEventDispatcher } from "svelte";
import {
    Nav,
    NavItem,
    NavLink
} from "sveltestrap";

import type ButtonPanel from "./ButtonPanel";
import type PanelController from "./PanelController";

export let controller: PanelController;



const dispatch = createEventDispatcher();
$: panels = controller.panels;

</script>


<Nav tabs class="bg-secondary">
    {#each panels as panel}
        <NavItem>
            <NavLink class="bg-dark"
                    active={controller.activePanel.title == panel.title}
                    on:click={e => dispatch("changepanel", { panel })}>
                {panel.title}
            </NavLink>
        </NavItem>
    {/each}
    <!-- <button><i class="fa fa-2x fa-cog"></i></button> -->
</Nav>
