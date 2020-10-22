<script lang="ts">
import Modal from "svelte-simple-modal";

import Grid from "./Grid.svelte";
// import Navbar from "./Navbar.svelte";
</script>

<Modal>
    <main>
        <!-- <Navbar/> -->
        <Grid/>
    </main>
</Modal>
<style>
    main {
        width: 100%;
        height: 100%;

        display: flex;
        flex-direction: column;
        background-color: #4a4a4a;
    }
    /* Navbar {
        flex: 0 0;
    } */
</style>
